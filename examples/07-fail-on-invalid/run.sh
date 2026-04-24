#!/usr/bin/env sh
set -eu
here="$(cd "$(dirname "$0")" && pwd)"
wrapper="$here/validate-strict.sh"
chmod +x "$wrapper"

# good.txt should succeed (exit 0)
if ! "$wrapper" < "$here/good.txt" > /dev/null 2>&1; then
  echo "FAIL: expected good.txt to exit 0" >&2
  exit 1
fi

# bad.txt should fail (exit 1)
if "$wrapper" < "$here/bad.txt" > /dev/null 2>&1; then
  echo "FAIL: expected bad.txt to exit non-zero" >&2
  exit 1
fi

echo "OK"
