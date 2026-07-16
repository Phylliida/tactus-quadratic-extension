#!/usr/bin/env bash
# Verify tactus-quadratic-extension under the Lean backend, importing the
# tactus-algebra export (tactus_algebra.vir/.rlib from ../tactus-algebra).
#
# Usage:
#   ./check.sh                     # verify the whole crate (src/lib.rs)
#   ./check.sh <extra args>        # pass extra flags through to verus
#
# Always passes `-V cache` and tees full output to a log file (default
# /tmp/tactus-qext-check.log, override with $TACTUS_CHECK_LOG).
set -uo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
VERUS="$HERE/../tactus/source/target-verus/release/verus"
ALG="$HERE/../tactus-algebra/export"
LOG="${TACTUS_CHECK_LOG:-/tmp/tactus-qext-check.log}"

if [[ ! -x "$VERUS" ]]; then
  echo "error: tactus verus binary not found at $VERUS" >&2
  echo "build it with: cd ../tactus/source && vargo build --release" >&2
  exit 1
fi

if [[ ! -f "$ALG/tactus_algebra.vir" || ! -f "$ALG/libtactus_algebra.rlib" ]]; then
  echo "building tactus-algebra export first..." >&2
  "$HERE/../tactus-algebra/build-export.sh" >/dev/null
fi

# NOTE: do NOT add --emit-lean here — it emits .lean files WITHOUT running
# Lean (a floor-only measurement mode). This script must run the real
# package-check Lean gate.
"$VERUS" --lean-backend -V cache --crate-type=lib \
  --import tactus_algebra="$ALG/tactus_algebra.vir" \
  --extern tactus_algebra="$ALG/libtactus_algebra.rlib" \
  "$HERE/src/lib.rs" "$@" 2>&1 | tee "$LOG"
rc="${PIPESTATUS[0]}"
echo "[check.sh] full output saved to $LOG (exit $rc)" >&2
exit "$rc"
