#!/bin/bash
cd "$(dirname "$0")/.."

TARGET="${1:-all}"
MODE="${2:---native}"

build_game() {
  local game=$1
  echo "==> Building $game..."
  if [ "$MODE" = "--arm" ]; then
    (cd "miyoo/$game" && \
      CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc \
      cargo build --release --target armv7-unknown-linux-gnueabihf 2>&1 | tail -3)
  else
    (cd "miyoo/$game" && cargo build --release 2>&1 | tail -3)
  fi
  echo ""
}

if [ "$TARGET" = "all" ]; then
  for g in micro space shadow arena dragon mariolike; do
    build_game "$g"
  done
else
  build_game "$TARGET"
fi

echo "==> Done"
if [ "$MODE" = "--arm" ]; then
  echo "Binaries at: miyoo/*/target/armv7-unknown-linux-gnueabihf/release/"
else
  echo "Binaries at: miyoo/*/target/release/"
fi
