#!/usr/bin/env bash
# verify_chapter.sh — for each exercise file in the given chapter directory,
# merge the solution body with the exercise's #[cfg(test)] tests block,
# compile with --test (or --run / plain compile per the file's first arg),
# and print PASS/FAIL.
#
# Usage:  scripts/verify_chapter.sh exercises/04_ownership
set -e
cd "$(dirname "$0")/.."
chapter_dir="${1:?usage: verify_chapter.sh exercises/NN_xxx}"
solutions_dir="${chapter_dir/exercises/solutions}"

fail=0
for ex in "$chapter_dir"/*.rs; do
  base=$(basename "$ex" .rs)
  sol="$solutions_dir/$base.rs"

  if [ ! -f "$sol" ]; then
    echo "MISSING solution for $base"; fail=1; continue
  fi

  if grep -q '#\[cfg(test)\]' "$ex"; then
    awk '/^#\[cfg\(test\)\]/{p=1} p' "$ex" > /tmp/_tests.rs
    cat "$sol" /tmp/_tests.rs > /tmp/_merged.rs
    grep -q '^fn main' /tmp/_merged.rs || echo "fn main(){}" >> /tmp/_merged.rs
    if rustc --edition=2024 -A warnings --test -o /tmp/_bin /tmp/_merged.rs 2>/tmp/_err; then
      if /tmp/_bin --quiet 2>&1 | grep -q "test result: ok"; then
        echo "PASS $base"
      else
        echo "FAIL(tests) $base"; /tmp/_bin 2>&1 | tail -10; fail=1
      fi
    else
      echo "FAIL(compile) $base"; head -20 /tmp/_err; fail=1
    fi
  else
    if rustc --edition=2024 -A warnings -o /tmp/_bin "$sol" 2>/tmp/_err && /tmp/_bin > /tmp/_out 2>&1; then
      echo "PASS $base (run)"
    else
      echo "FAIL $base"; head -20 /tmp/_err; fail=1
    fi
  fi
done
exit $fail
