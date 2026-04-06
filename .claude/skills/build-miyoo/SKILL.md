---
name: build-miyoo
description: Build Miyoo Rust/Zig ports (native or cross-compile for ARM)
user_invocable: true
args: "[game-name|all] [--rust|--zig] [--native|--arm]"
---

Build one or all Miyoo game ports. Supports both Rust+SDL2 (`miyoo/`) and Zig+SDL2 (`zig/`) ports.

## Rust SDL2 ports (default)

Cross-compile for Miyoo (ARM, glibc 2.28):
```bash
cd /home/mo/data/Documents/git/retrogames/miyoo/<game>
LIBRARY_PATH=/home/mo/data/Documents/git/retrogames/miyoo/sdl2-stub cargo zigbuild --target armv7-unknown-linux-gnueabihf.2.28 --release
```

Native desktop build:
```bash
cd /home/mo/data/Documents/git/retrogames/miyoo/<game>
cargo build --release
```

## Zig SDL2 ports

Cross-compile for Miyoo:
```bash
cd /home/mo/data/Documents/git/retrogames/zig/<game>
zig build -Dtarget=arm-linux-gnueabihf -Doptimize=ReleaseSafe
```

Native desktop build:
```bash
cd /home/mo/data/Documents/git/retrogames/zig/<game>
zig build
```

## Games

Rust: micro, space, shadow, arena, dragon, mariolike, cyber, neon, nova
Zig: micro (more coming)

## After building

Run `/test-miyoo <binary-path>` to validate in the Docker simulator, or SCP to the Miyoo:
```bash
sshpass -p onion scp <binary> onion@192.168.0.63:/mnt/SDCARD/Roms/PORTS/Games/<GameName>/
```
