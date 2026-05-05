// master_rust — a beginner-friendly, deeply-explained Rustlings-style course.
//
// This is the *runner*. The lessons live in `exercises/` and the annotated
// optimal answers live in `solutions/`. The runner:
//
//   1. Reads `info.toml` to discover the ordered list of exercises.
//   2. Compiles each with `rustc` (or `rustc --test`) into a temp build dir.
//   3. Tracks completion in `.master_rust_progress`.
//   4. Drives an interactive Rustlings-style UI: scrolling output above a
//      pinned progress bar + current-exercise line + keyboard menu, plus
//      filesystem watching that re-runs the current exercise on save.

use anyhow::{anyhow, bail, Context, Result};
use clap::{Parser, Subcommand};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal,
};
use notify::{Event as FsEvent, EventKind, RecursiveMode, Watcher};
use serde::Deserialize;
use std::collections::HashSet;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::time::Duration;

// ---------- ANSI colour helpers ---------------------------------------------
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";

// ---------- CLI definition --------------------------------------------------
#[derive(Parser)]
#[command(
    name = "master_rust",
    version,
    about = "Master Rust from zero — a deeply-explained Rustlings-style course.",
    long_about = "Run with no arguments to enter the interactive course.\n\
                  Each exercise file is heavily commented — read the file top-to-bottom before editing."
)]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    /// List every exercise and whether you've finished it.
    List,
    /// Compile-check (or test) one exercise once and report.
    Run { name: Option<String> },
    /// (Alias for default) Run the interactive course.
    Watch,
    /// Print the hint block for an exercise.
    Hint { name: Option<String> },
    /// Show the annotated optimal solution (only available after pass).
    Solution {
        name: Option<String>,
        #[arg(long)]
        force: bool,
    },
    /// Forget that you completed an exercise.
    Reset { name: String },
    /// Show overall progress.
    Progress,
}

// ---------- Manifest types --------------------------------------------------
#[derive(Debug, Deserialize)]
struct Manifest {
    exercises: Vec<Exercise>,
}

#[derive(Debug, Deserialize, Clone)]
struct Exercise {
    name: String,
    path: String,
    #[serde(default)]
    mode: Mode,
    #[serde(default)]
    hint: String,
}

#[derive(Debug, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum Mode {
    #[default]
    Compile,
    Test,
    Run,
}

// ---------- Paths -----------------------------------------------------------
fn course_root() -> Result<PathBuf> {
    if let Ok(p) = std::env::var("MASTER_RUST_ROOT") {
        return Ok(PathBuf::from(p));
    }
    if let Ok(p) = std::env::var("CARGO_MANIFEST_DIR") {
        return Ok(PathBuf::from(p));
    }
    Ok(std::env::current_dir()?)
}
fn manifest_path() -> Result<PathBuf> { Ok(course_root()?.join("info.toml")) }
fn progress_path() -> Result<PathBuf> { Ok(course_root()?.join(".master_rust_progress")) }
fn build_dir() -> Result<PathBuf> {
    let p = course_root()?.join(".master_rust_build");
    fs::create_dir_all(&p).ok();
    Ok(p)
}

// ---------- Manifest + progress ---------------------------------------------
fn load_manifest() -> Result<Manifest> {
    let p = manifest_path()?;
    let raw = fs::read_to_string(&p)
        .with_context(|| format!("could not read manifest at {}", p.display()))?;
    let m: Manifest = toml::from_str(&raw).context("info.toml is not valid TOML")?;
    Ok(m)
}
fn load_progress() -> HashSet<String> {
    let p = match progress_path() { Ok(p) => p, Err(_) => return HashSet::new() };
    fs::read_to_string(&p).unwrap_or_default().lines()
        .map(|l| l.trim().to_string()).filter(|l| !l.is_empty()).collect()
}
fn save_progress(done: &HashSet<String>) -> Result<()> {
    let mut v: Vec<&String> = done.iter().collect();
    v.sort();
    fs::write(progress_path()?, v.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n"))?;
    Ok(())
}
fn find_exercise<'a>(m: &'a Manifest, name: &str) -> Result<&'a Exercise> {
    m.exercises.iter().find(|e| e.name == name)
        .ok_or_else(|| anyhow!("no exercise named `{name}` — try `master_rust list`"))
}
fn next_pending<'a>(m: &'a Manifest, done: &HashSet<String>) -> Option<&'a Exercise> {
    m.exercises.iter().find(|e| !done.contains(&e.name))
}

// ---------- Running an exercise ---------------------------------------------
enum Outcome {
    NotDone,
    CompileFailed(String),
    TestFailed(String),
    RunFailed(String),
    Passed(String), // captured stdout (run mode) or empty
}
const NOT_DONE_MARKER: &str = "// I AM NOT DONE";

fn run_exercise(ex: &Exercise) -> Result<Outcome> {
    let src = course_root()?.join(&ex.path);
    let source = fs::read_to_string(&src)
        .with_context(|| format!("cannot read exercise at {}", src.display()))?;
    if source.contains(NOT_DONE_MARKER) { return Ok(Outcome::NotDone); }

    let bin = build_dir()?.join(&ex.name);
    let mut cmd = Command::new("rustc");
    cmd.arg("--edition=2024").arg("-A").arg("warnings")
        .arg("-o").arg(&bin).arg(&src);
    if ex.mode == Mode::Test { cmd.arg("--test"); }
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

    let out = cmd.output().context("failed to spawn rustc")?;
    if !out.status.success() {
        return Ok(Outcome::CompileFailed(String::from_utf8_lossy(&out.stderr).into_owned()));
    }
    match ex.mode {
        Mode::Compile => Ok(Outcome::Passed(String::new())),
        Mode::Test => {
            let r = Command::new(&bin).output()?;
            if r.status.success() { Ok(Outcome::Passed(String::new())) } else {
                let mut buf = String::from_utf8_lossy(&r.stdout).into_owned();
                buf.push_str(&String::from_utf8_lossy(&r.stderr));
                Ok(Outcome::TestFailed(buf))
            }
        }
        Mode::Run => {
            let r = Command::new(&bin).output()?;
            if r.status.success() {
                Ok(Outcome::Passed(String::from_utf8_lossy(&r.stdout).into_owned()))
            } else {
                let mut buf = String::from_utf8_lossy(&r.stdout).into_owned();
                buf.push_str(&String::from_utf8_lossy(&r.stderr));
                Ok(Outcome::RunFailed(buf))
            }
        }
    }
}

// One-shot text reporting (used by `run` subcommand).
fn report_oneshot(ex: &Exercise, outcome: &Outcome, done: &mut HashSet<String>) -> bool {
    println!();
    match outcome {
        Outcome::NotDone => {
            println!("{YELLOW}{BOLD}⏸  {}{RESET} — read the comments at the top of\n   {DIM}{}{RESET}\n   then DELETE the line `{}` and save.",
                ex.name, ex.path, NOT_DONE_MARKER);
            false
        }
        Outcome::CompileFailed(msg) => {
            println!("{RED}{BOLD}✗ {} did not compile.{RESET}\n", ex.name);
            println!("{}", msg.trim_end()); false
        }
        Outcome::TestFailed(msg) => {
            println!("{RED}{BOLD}✗ {} compiled, but a test failed.{RESET}\n", ex.name);
            println!("{}", msg.trim_end()); false
        }
        Outcome::RunFailed(msg) => {
            println!("{RED}{BOLD}✗ {} ran but exited with an error.{RESET}\n", ex.name);
            println!("{}", msg.trim_end()); false
        }
        Outcome::Passed(stdout) => {
            if !stdout.is_empty() { print!("{}", stdout); }
            done.insert(ex.name.clone());
            save_progress(done).ok();
            println!("{GREEN}{BOLD}✓ {} passed!{RESET}", ex.name);
            true
        }
    }
}

// ---------- Interactive UI (Rustlings-style) --------------------------------
//
// Layout, top-to-bottom:
//   ┌──────────────────────────────────┐
//   │  scrolling output (welcome /     │
//   │  compiler errors / pass banner / │
//   │  hint / list)                    │
//   ├──────────────────────────────────┤
//   │  Progress: [######>------] X/Y   │
//   │  Current exercise: path/to.rs    │
//   │  n:next / h:hint / l:list / ...  │
//   └──────────────────────────────────┘
//
// We don't implement a full pinned bottom bar (which needs absolute cursor
// positioning and a scroll region). Instead we redraw output → footer as a
// single block whenever something changes. Same visual effect as Rustlings,
// dramatically simpler to maintain.

fn print_welcome() {
    let banner = r#"
        Welcome to...

   _ __ ___   __ _ ___| |_ ___ _ __     _ __ _   _ ___| |_
  | '_ ` _ \ / _` / __| __/ _ \ '__|   | '__| | | / __| __|
  | | | | | | (_| \__ \ ||  __/ |      | |  | |_| \__ \ |_
  |_| |_| |_|\__,_|___/\__\___|_|      |_|   \__,_|___/\__|

"#;
    println!("{CYAN}{BOLD}{}{RESET}", banner);
    println!("Each exercise contains a deliberate gap or compile error. Read the");
    println!("comments at the top, fill in the {YELLOW}???{RESET} blanks, delete the");
    println!("{YELLOW}// I AM NOT DONE{RESET} line, then save. The runner re-checks");
    println!("automatically and shows you the result.");
    println!();
    println!("The current exercise's path will always be shown beneath the progress");
    println!("bar — most terminals let you Ctrl-click it to jump there in your editor.");
    println!();
}

fn print_run_block(ex: &Exercise, outcome: &Outcome) {
    println!("{BLUE}{BOLD}── {}{RESET} {DIM}({}){RESET}", ex.name, ex.path);
    println!();
    match outcome {
        Outcome::NotDone => {
            println!("{YELLOW}This exercise is waiting for you to start.{RESET}");
            println!();
            println!("Open {DIM}{}{RESET}, read every comment in the long teaching block at", ex.path);
            println!("the top of the file, then DELETE the line {YELLOW}{}{RESET}", NOT_DONE_MARKER);
            println!("and save. The runner will re-check immediately.");
        }
        Outcome::CompileFailed(msg) => {
            println!("{RED}{BOLD}Compiling of {} failed.{RESET} The compiler error is:\n", ex.name);
            println!("{}", msg.trim_end());
        }
        Outcome::TestFailed(msg) => {
            println!("{RED}{BOLD}{} compiled, but a test failed.{RESET}\n", ex.name);
            println!("{}", msg.trim_end());
        }
        Outcome::RunFailed(msg) => {
            println!("{RED}{BOLD}{} ran but exited with an error.{RESET}\n", ex.name);
            println!("{}", msg.trim_end());
        }
        Outcome::Passed(stdout) => {
            if !stdout.is_empty() {
                println!("{DIM}── output ──{RESET}");
                print!("{}", stdout);
                if !stdout.ends_with('\n') { println!(); }
                println!();
            }
            let sol = solution_path_for(ex);
            println!("{GREEN}{BOLD}Exercise done ✓{RESET}");
            println!("{BOLD}Solution{RESET} for comparison: {CYAN}{}{RESET}", sol.display());
            println!("When done experimenting, press {BOLD}n{RESET} to move on to the next exercise.");
        }
    }
    println!();
}

fn solution_path_for(ex: &Exercise) -> PathBuf {
    Path::new("solutions").join(
        Path::new(&ex.path).strip_prefix("exercises").unwrap_or(Path::new(&ex.path))
    )
}

fn draw_footer(m: &Manifest, done: &HashSet<String>, current: Option<&Exercise>) {
    let total = m.exercises.len();
    let n_done = m.exercises.iter().filter(|e| done.contains(&e.name)).count();
    let label = format!(" {}/{}", n_done, total);

    // Width: cap at 100, scale down for narrow terminals.
    let cols = terminal::size().map(|(c, _)| c as usize).unwrap_or(80);
    let bar_w = cols.saturating_sub(label.len() + 13).min(120).max(10);
    let filled = (bar_w * n_done) / total.max(1);
    let cursor = if filled < bar_w { ">" } else { "" };
    let empty = if filled < bar_w { bar_w - filled - 1 } else { 0 };

    let bar = format!(
        "{GREEN}{}{RESET}{YELLOW}{}{RESET}{RED}{}{RESET}",
        "#".repeat(filled),
        cursor,
        "-".repeat(empty),
    );
    println!("{BOLD}Progress:{RESET} [{}]{}", bar, label);

    match current {
        Some(ex) => println!("{BOLD}Current exercise:{RESET} {CYAN}{}{RESET}", ex.path),
        None => println!("{GREEN}{BOLD}🎉 every exercise complete — course done.{RESET}"),
    }
    println!();
    println!(
        "{BOLD}n{RESET}:next / {BOLD}h{RESET}:hint / {BOLD}s{RESET}:solution / {BOLD}l{RESET}:list / {BOLD}r{RESET}:re-run / {BOLD}x{RESET}:reset / {BOLD}q{RESET}:quit ?"
    );
}

fn clear_screen() {
    // ANSI: clear screen + move cursor home.
    print!("\x1b[2J\x1b[H");
    std::io::stdout().flush().ok();
}

// Interactive loop: blocking key reads + non-blocking filesystem watch.
fn cmd_interactive() -> Result<()> {
    let m = load_manifest()?;
    let mut done = load_progress();
    let exercises_dir = course_root()?.join("exercises");

    let (fs_tx, fs_rx) = channel::<()>();
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<FsEvent>| {
        if let Ok(ev) = res {
            if matches!(ev.kind, EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_))
                && ev.paths.iter().any(|p| p.extension().is_some_and(|e| e == "rs"))
            {
                let _ = fs_tx.send(());
            }
        }
    })?;
    watcher.watch(&exercises_dir, RecursiveMode::Recursive)?;

    let mut current = next_pending(&m, &done).cloned();
    let mut last_outcome: Option<Outcome> = None;

    // First draw — welcome + first exercise + footer.
    clear_screen();
    print_welcome();
    if let Some(ex) = &current {
        let oc = run_exercise(ex)?;
        if matches!(oc, Outcome::Passed(_)) {
            done.insert(ex.name.clone());
            save_progress(&done).ok();
        }
        print_run_block(ex, &oc);
        last_outcome = Some(oc);
    }
    draw_footer(&m, &done, current.as_ref());

    terminal::enable_raw_mode()?;
    let result = run_event_loop(&m, &mut done, &mut current, &mut last_outcome, &fs_rx);
    terminal::disable_raw_mode().ok();
    println!();
    result
}

#[derive(Copy, Clone)]
enum Action {
    Quit,
    Next,
    Hint,
    Solution,
    List,
    Rerun,
    ResetCurrent,
    None,
}

fn read_key() -> Result<Action> {
    if event::poll(Duration::from_millis(150))? {
        if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            if kind != KeyEventKind::Press { return Ok(Action::None); }
            return Ok(match code {
                KeyCode::Char('q') | KeyCode::Esc => Action::Quit,
                KeyCode::Char('n') | KeyCode::Enter => Action::Next,
                KeyCode::Char('h') => Action::Hint,
                KeyCode::Char('s') => Action::Solution,
                KeyCode::Char('l') => Action::List,
                KeyCode::Char('r') => Action::Rerun,
                KeyCode::Char('x') => Action::ResetCurrent,
                _ => Action::None,
            });
        }
    }
    Ok(Action::None)
}

fn run_event_loop(
    m: &Manifest,
    done: &mut HashSet<String>,
    current: &mut Option<Exercise>,
    last_outcome: &mut Option<Outcome>,
    fs_rx: &std::sync::mpsc::Receiver<()>,
) -> Result<()> {
    loop {
        let action = read_key()?;
        match action {
            Action::Quit => return Ok(()),
            Action::Next => {
                // Reload progress (in case the file changed) and advance.
                *done = load_progress();
                let prev_passed = matches!(last_outcome, Some(Outcome::Passed(_)));
                if prev_passed {
                    *current = next_pending(m, done).cloned();
                }
                // If user pressed n on a non-passing exercise, just re-run it.
                rerun_and_render(m, done, current, last_outcome);
                continue;
            }
            Action::Hint => {
                terminal::disable_raw_mode().ok();
                if let Some(ex) = current {
                    println!();
                    if ex.hint.trim().is_empty() {
                        println!("{DIM}(no hint recorded for {}){RESET}", ex.name);
                    } else {
                        println!("{YELLOW}{BOLD}── hint for {} ──{RESET}", ex.name);
                        println!("{}", ex.hint.trim());
                    }
                    println!();
                }
                draw_footer(m, done, current.as_ref());
                terminal::enable_raw_mode()?;
                continue;
            }
            Action::Solution => {
                terminal::disable_raw_mode().ok();
                println!();
                if let Some(ex) = current {
                    let must_have_passed = !done.contains(&ex.name);
                    let path = solution_path_for(ex);
                    if must_have_passed {
                        println!("{YELLOW}You haven't passed `{}` yet.{RESET}", ex.name);
                        println!("Finish the exercise first — then press {BOLD}s{RESET} to see the annotated optimal solution.");
                    } else {
                        match fs::read_to_string(&path) {
                            Ok(body) => {
                                println!("{CYAN}{BOLD}── annotated solution: {} ──{RESET} {DIM}{}{RESET}\n", ex.name, path.display());
                                println!("{}", body);
                            }
                            Err(_) => println!("{RED}no solution file found at {}{RESET}", path.display()),
                        }
                    }
                }
                println!();
                draw_footer(m, done, current.as_ref());
                terminal::enable_raw_mode()?;
                continue;
            }
            Action::List => {
                terminal::disable_raw_mode().ok();
                println!();
                let n_done = m.exercises.iter().filter(|e| done.contains(&e.name)).count();
                println!("{BOLD}── exercises{RESET}  {DIM}({} / {} done){RESET}\n", n_done, m.exercises.len());
                for ex in &m.exercises {
                    if done.contains(&ex.name) {
                        println!("  {GREEN}✓{RESET} {}  {DIM}{}{RESET}", ex.name, ex.path);
                    } else if current.as_ref().is_some_and(|c| c.name == ex.name) {
                        println!("  {YELLOW}»{RESET} {BOLD}{}{RESET}  {DIM}{}{RESET}", ex.name, ex.path);
                    } else {
                        println!("  {DIM}·{RESET} {}  {DIM}{}{RESET}", ex.name, ex.path);
                    }
                }
                println!();
                draw_footer(m, done, current.as_ref());
                terminal::enable_raw_mode()?;
                continue;
            }
            Action::Rerun => {
                rerun_and_render(m, done, current, last_outcome);
                continue;
            }
            Action::ResetCurrent => {
                if let Some(ex) = current.clone() {
                    done.remove(&ex.name);
                    save_progress(done).ok();
                    rerun_and_render(m, done, current, last_outcome);
                }
                continue;
            }
            Action::None => {}
        }

        // No keypress this tick — drain filesystem events.
        let mut got_fs = false;
        while fs_rx.try_recv().is_ok() { got_fs = true; }
        if got_fs {
            // Debounce a moment so editors finishing their write don't trigger
            // a half-saved compile.
            std::thread::sleep(Duration::from_millis(80));
            while fs_rx.try_recv().is_ok() {}
            rerun_and_render(m, done, current, last_outcome);
        }

    }
}

fn rerun_and_render(
    m: &Manifest,
    done: &mut HashSet<String>,
    current: &mut Option<Exercise>,
    last_outcome: &mut Option<Outcome>,
) {
    // Re-load the manifest in case it changed (rare) and recompute current.
    if current.is_none() {
        *current = next_pending(m, done).cloned();
    }
    terminal::disable_raw_mode().ok();
    clear_screen();
    if let Some(ex) = current.clone() {
        match run_exercise(&ex) {
            Ok(oc) => {
                if matches!(oc, Outcome::Passed(_)) {
                    let was_new = done.insert(ex.name.clone());
                    save_progress(done).ok();
                    let _ = was_new;
                }
                print_run_block(&ex, &oc);
                *last_outcome = Some(oc);
            }
            Err(e) => {
                println!("{RED}runner error: {e:#}{RESET}");
            }
        }
    } else {
        print_complete();
    }
    draw_footer(m, done, current.as_ref());
    terminal::enable_raw_mode().ok();
}

fn print_complete() {
    println!();
    println!("{GREEN}{BOLD}🎉 You finished every exercise. The course is complete.{RESET}");
    println!();
    println!("From here, the next steps are real Rust projects: pick a small CLI");
    println!("you've always wanted, build it with everything you've learned, and");
    println!("publish it to crates.io. The capstone (chapter 20) is a deliberate");
    println!("stepping-stone — extend it.");
    println!();
}

// ---------- One-shot subcommands --------------------------------------------
fn cmd_list() -> Result<()> {
    let m = load_manifest()?;
    let done = load_progress();
    let total = m.exercises.len();
    let n_done = m.exercises.iter().filter(|e| done.contains(&e.name)).count();
    println!("{BOLD}master_rust{RESET} — {n_done}/{total} complete\n");
    for ex in &m.exercises {
        if done.contains(&ex.name) {
            println!("  {GREEN}✓{RESET} {}  {DIM}{}{RESET}", ex.name, ex.path);
        } else {
            println!("  {DIM}·{RESET} {}  {DIM}{}{RESET}", ex.name, ex.path);
        }
    }
    Ok(())
}

fn cmd_run(name: Option<String>) -> Result<()> {
    let m = load_manifest()?;
    let mut done = load_progress();
    let ex = match name {
        Some(n) => find_exercise(&m, &n)?.clone(),
        None => next_pending(&m, &done)
            .ok_or_else(|| anyhow!("🎉 no pending exercises — course complete!"))?
            .clone(),
    };
    println!("{BLUE}▶ running {}{RESET} {DIM}({}){RESET}", ex.name, ex.path);
    let oc = run_exercise(&ex)?;
    report_oneshot(&ex, &oc, &mut done);
    Ok(())
}

fn cmd_hint(name: Option<String>) -> Result<()> {
    let m = load_manifest()?;
    let done = load_progress();
    let ex = match name {
        Some(n) => find_exercise(&m, &n)?.clone(),
        None => next_pending(&m, &done)
            .ok_or_else(|| anyhow!("nothing pending — pass `master_rust hint <name>`"))?
            .clone(),
    };
    if ex.hint.trim().is_empty() {
        println!("{DIM}(no hint recorded for {}){RESET}", ex.name);
    } else {
        println!("{YELLOW}{BOLD}hint for {}:{RESET}\n\n{}", ex.name, ex.hint.trim());
    }
    Ok(())
}

fn cmd_solution(name: Option<String>, force: bool) -> Result<()> {
    let m = load_manifest()?;
    let done = load_progress();
    let ex = match name {
        Some(n) => find_exercise(&m, &n)?.clone(),
        None => next_pending(&m, &done)
            .ok_or_else(|| anyhow!("nothing pending — pass an exercise name"))?
            .clone(),
    };
    if !force && !done.contains(&ex.name) {
        bail!("you haven't passed `{}` yet — finish it first, or pass --force to peek.", ex.name);
    }
    let sol_path = course_root()?.join(solution_path_for(&ex));
    let body = fs::read_to_string(&sol_path)
        .with_context(|| format!("no solution file found at {}", sol_path.display()))?;
    println!("{CYAN}{BOLD}── annotated solution: {} ──{RESET}\n{DIM}{}{RESET}\n", ex.name, sol_path.display());
    println!("{}", body);
    Ok(())
}

fn cmd_reset(name: String) -> Result<()> {
    let mut done = load_progress();
    if done.remove(&name) {
        save_progress(&done)?;
        println!("{YELLOW}reset {} — it'll come back up next time you run.{RESET}", name);
    } else {
        println!("{DIM}(nothing to reset — {} wasn't marked complete){RESET}", name);
    }
    Ok(())
}

fn cmd_progress() -> Result<()> {
    let m = load_manifest()?;
    let done = load_progress();
    let total = m.exercises.len();
    let n_done = m.exercises.iter().filter(|e| done.contains(&e.name)).count();
    let pct = if total == 0 { 0 } else { (n_done * 100) / total };
    let bar_w = 30usize;
    let filled = (n_done * bar_w) / total.max(1);
    let bar: String = "#".repeat(filled) + &"-".repeat(bar_w - filled);
    println!("{BOLD}{}{RESET} {GREEN}{}{RESET}/{} ({}%)", bar, n_done, total, pct);
    if let Some(next) = next_pending(&m, &done) {
        println!("{DIM}next:{RESET} {}", next.name);
    } else {
        println!("{GREEN}{BOLD}🎉 course complete!{RESET}");
    }
    Ok(())
}

// ---------- Entry point -----------------------------------------------------
fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        None | Some(Cmd::Watch) => cmd_interactive(),
        Some(Cmd::List) => cmd_list(),
        Some(Cmd::Run { name }) => cmd_run(name),
        Some(Cmd::Hint { name }) => cmd_hint(name),
        Some(Cmd::Solution { name, force }) => cmd_solution(name, force),
        Some(Cmd::Reset { name }) => cmd_reset(name),
        Some(Cmd::Progress) => cmd_progress(),
    }
}

// Keep MAGENTA / RecvTimeoutError referenced so we don't warn on unused after edits.
#[allow(dead_code)]
const _UNUSED: &str = MAGENTA;
#[allow(dead_code)]
fn _unused_recv_timeout() -> RecvTimeoutError { RecvTimeoutError::Timeout }
