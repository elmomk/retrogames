---
name: polish
description: Add visual effects and game juice to a specified game
user_invocable: true
args: "<game-name> [web|miyoo|both]"
---

Add visual polish to the specified game. Default target: both.

## Effects to add (pick what's missing)

**Impact**: screen shake (2-8px, 0.85 decay), hit stop (3-5 frames), damage red flash, muzzle flash
**Particles**: explosions, dash afterimages, ambient dust/embers, pulsing collectible glow
**Feedback**: floating damage numbers, combo text ("KILLING SPREE!"), POW/BAM impact text, death fade-outs
**Ambient**: star twinkle, rain/weather, CRT scanlines (4px, alpha 0.12), vignette

Edit `web/<game>/index.html` and/or `miyoo/<game>/src/main.rs`.
Run `cargo check` on Miyoo port if modified.
