---
name: polish
description: Add visual effects and game juice to a specified game
user_invocable: true
args: "<game-name> [web|miyoo|both]"
---

Add visual polish and "game juice" to the specified game. Default target is "both" (web + miyoo).

Parse the game name from arguments. If not provided, ask the user which game to polish.

## What to add (pick what's missing):

### Visual Effects
- Screen shake on hits/deaths (small 2px, medium 5px, large 8px with 0.85 decay)
- Hit stop/freeze frames on big impacts (3-5 frames)
- CRT scanline overlay (dark lines every 4px, alpha 0.12)
- Vignette darkening at screen edges
- Particle explosions with varied colors and sizes
- Dash/movement afterimage trails
- Muzzle flash on weapon fire
- Damage red border flash

### Feedback
- Floating damage/score number popups that rise and fade
- Combo/kill streak text ("KILLING SPREE!", "GODLIKE!")
- POW/BAM impact text on melee hits
- Enemy death fade-out animations

### Ambient Polish
- Ambient dust/ember particles drifting across screen
- Star twinkle with sinusoidal alpha
- Background rain/weather effects where thematic
- Pulsing glow on collectibles

### For web games, edit: `web/<game>/index.html`
### For Miyoo ports, edit: `miyoo/<game>/src/main.rs`

After changes, run `cargo check` on the Miyoo port if modified.
