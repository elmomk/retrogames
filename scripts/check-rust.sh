#!/bin/bash
cd "$(dirname "$0")/.."

PASS=0
FAIL=0

for g in micro space shadow arena dragon mariolike; do
  echo -n "$g: "
  if (cd "miyoo/$g" && cargo check 2>&1 | tail -1 | grep -q "Finished"); then
    echo "OK"
    PASS=$((PASS + 1))
  else
    echo "FAIL"
    (cd "miyoo/$g" && cargo check 2>&1 | grep "^error" | head -3)
    FAIL=$((FAIL + 1))
  fi
done

echo ""
echo "Results: $PASS passed, $FAIL failed"
