---
name: build-miyoo
description: Build Miyoo Rust ports (native or cross-compile)
user_invocable: true
args: "[game-name|all] [--native|--arm]"
---

Build one or all Miyoo Rust game ports.

```bash
cd /home/mo/data/Documents/git/retrogames && ./scripts/build-miyoo.sh $ARGUMENTS
```

Arguments:
- Game name: micro, space, shadow, arena, dragon, mariolike, or `all` (default: all)
- `--native`: desktop build (default)
- `--arm`: cross-compile for Miyoo Mini Plus (requires `gcc-arm-linux-gnueabihf`)

Report build status and binary sizes when done.
