---
name: port-to-miyoo
description: Sync web game changes to the Miyoo Rust port by reading the current HTML5 game and porting new features, bug fixes, and balance changes to the Rust/Macroquad code
user_invocable: true
args: "<game-name> [--all]"
---

Port web game modifications to the corresponding Miyoo Rust port.

Parse arguments:
- Game name: micro, space, shadow, arena, dragon, mariolike, cyber, neon
- `--all`: Port all games that have both web and Miyoo versions

For each game, use the `web-to-miyoo` agent to:

1. **Read the web version** (`web/<game>/index.html`) and extract:
   - Game constants (physics, speeds, sizes, timings)
   - Enemy types and behavior
   - Player mechanics (movement, abilities, weapons)
   - Level layouts and tile maps
   - Power-up system
   - Score/lives system
   - Story text
   - Visual effects

2. **Read the Miyoo port** (`miyoo/<game>/src/main.rs`) and compare

3. **Identify gaps** — features/changes in web that are missing from Miyoo:
   - New mechanics (e.g., dash, wall jump, growth system)
   - Balance changes (speeds, damage, health values)
   - New enemies or enemy behaviors
   - Updated level layouts
   - New story text
   - Bug fixes (NaN guards, collision fixes, state resets)
   - New visual effects

4. **Port the changes** to the Rust code using Edit, following these patterns:
   - Data-oriented design (flat Vec arrays)
   - Index-based loops to avoid borrow checker issues
   - `f32` annotations on `gen_range()` calls
   - `FilterMode::Nearest` on textures
   - Exhaustive match blocks
   - AABB collision with `overlaps()`

5. **Run `cargo check`** and fix any compile errors

6. **Report** what was ported and what was skipped (if anything)

## Game directory mapping

| Game | Web | Miyoo |
|------|-----|-------|
| micro | web/micro/ | miyoo/micro/ |
| space | web/space/ | miyoo/space/ |
| shadow | web/shadow/ | miyoo/shadow/ |
| arena | web/arena/ | miyoo/arena/ |
| dragon | web/dragon/ | miyoo/dragon/ |
| mariolike | web/mariolike/ | miyoo/mariolike/ |
| cyber | web/cyber/ | miyoo/cyber/ |
| neon | web/neon/ | miyoo/neon/ |
