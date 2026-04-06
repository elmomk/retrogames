---
name: test-miyoo
description: Test Miyoo ARM binaries in a Docker container that simulates the parasyte runtime
user_invocable: true
args: "[binary-path|game-name]"
---

Test a cross-compiled ARM binary in a Docker container that simulates the Miyoo Mini Plus parasyte runtime environment. Validates shared library resolution, glibc compatibility, and SDL2 linkage.

## How it works

1. Builds a `miyoo-test` Docker image (arm32v7/debian:bullseye-slim) with SDL2, X11, EGL, GLES libs mirroring the parasyte layout
2. Runs the binary inside the container with `LD_LIBRARY_PATH` set to the simulated parasyte path
3. Uses `SDL_VIDEODRIVER=dummy` since there's no real display

## Expected results

- **Success**: Binary loads, all libs resolve. May fail at display init (expected — no framebuffer in container)
- **Failure**: Missing library errors, glibc version mismatch, or SDL2 symbol errors indicate problems

## Usage

If a binary path is provided, test that directly:
```bash
sh miyoo/test/test-miyoo.sh "$ARGUMENTS"
```

If a game name is provided (e.g., `micro`, `space`), find and test the latest build:
```bash
# For Rust SDL2 ports:
sh miyoo/test/test-miyoo.sh miyoo/<game>/target/armv7-unknown-linux-gnueabihf/release/<game>_miyoo

# For Zig ports:
sh miyoo/test/test-miyoo.sh zig/<game>/zig-out/bin/<game>_miyoo
```

## Prerequisites

- Docker with QEMU binfmt support (auto-registered on first run)
- A cross-compiled ARM binary (from `cargo zigbuild --target armv7-unknown-linux-gnueabihf.2.28` or `zig build -Dtarget=arm-linux-gnueabihf`)
