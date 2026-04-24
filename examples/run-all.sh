#!/usr/bin/env sh
# Run every example's run.sh. Exit non-zero if any fail.
set -u
here="$(cd "$(dirname "$0")" && pwd)"

fail=0
for dir in "$here"/*/; do
  name=$(basename "$dir")
  printf '%-26s ' "$name"
  if (cd "$dir" && ./run.sh) > /tmp/nhs-example-$$.out 2>&1; then
    tail -1 /tmp/nhs-example-$$.out
  else
    echo "FAIL"
    cat /tmp/nhs-example-$$.out | sed 's/^/    /'
    fail=1
  fi
  rm -f /tmp/nhs-example-$$.out
done
exit "$fail"
