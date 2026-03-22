---
name: port-to-miyoo
description: Sync web game changes to the Miyoo Rust port
user_invocable: true
args: "<game-name> [--all]"
---

Port web game changes to the corresponding Miyoo Rust port.

Games: micro, space, shadow, arena, dragon, mariolike, cyber, neon.
Use `--all` to port all games.

For each game, use the `web-to-miyoo` agent to read both versions, identify gaps, port changes, and run `cargo check`.
