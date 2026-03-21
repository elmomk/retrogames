# Chrome Viper - Technical Specification

## 1. Overview

**Title:** Chrome Viper
**Genre:** Cyberpunk Horizontal Scrolling Shooter
**Story:** "Neon Abyss"
**Platform:** Browser (HTML5 Canvas, single-file)

In 2187, megacorp AXIOM has seized control of the neon-drenched orbital colonies. The player pilots the Chrome Viper, a stolen prototype stealth fighter, through AXIOM's defense networks across 3 levels, culminating in the destruction of the corporate flagship -- the Leviathan.

## 2. Tech Stack

- Single HTML file with embedded CSS and JS
- HTML5 Canvas (fullscreen, responsive)
- Web Audio API for procedural sound
- Google Font: Press Start 2P
- No external dependencies or assets

## 3. Visual Design

### Color Palette
| Name | Hex | Usage |
|------|-----|-------|
| Neon Pink | #ff2d78 | Spread shot, UI accents |
| Electric Cyan | #00f2ff | Default laser, shields, HUD |
| Toxic Green | #39ff14 | Homing missiles, story text |
| Deep Purple | #8b00ff | EMP blast, boss effects |
| Dark BG | #0a0a1a | Background base |

### CRT Effects
- Scanline overlay (repeating gradient, 4px period)
- Radial vignette (transparent center, dark edges)
- Chromatic aberration on damage (RGB channel offset)
- Screen shake on explosions

### Sprite System
- Procedural pixel-art via character arrays (8x8 and 16x16)
- `createSprite(data, palette, scale)` renders to offscreen canvas
- All sprites cached as Image objects

## 4. Game Architecture

### States
```
START -> LEVEL_STORY -> PLAYING -> LEVEL_STORY -> PLAYING -> ... -> WIN
                                    |
                                    v
                                 GAMEOVER
```

### Main Loop
- `requestAnimationFrame` with fixed timestep accumulator
- 60 FPS target (dt = 1/60)
- Death spiral prevention: cap accumulated time at 5 frames

### Coordinate System
- Logical resolution scales to fill viewport
- All game logic uses logical coordinates
- Canvas CSS fills 100vw x 100vh

## 5. Player Ship

- 16x16 sprite, angular cyberpunk design
- 8-directional movement, constrained to left 40% of screen
- Speed: 4 units/frame
- Shield: 3 hit points (visual shimmer effect)
- Invincibility frames on hit (90 frames / 1.5 seconds)
- Thruster animation (2-frame cycle)

### Weapons
| Weapon | Key | Behavior |
|--------|-----|----------|
| Dual Laser | Default | Two cyan beams, fast fire rate |
| Spread Shot | Power-up 1 | 3-way pink lasers, medium damage |
| Homing Missiles | Power-up 2 | Green projectiles, track nearest enemy |
| EMP Blast | Special (E/button) | Purple shockwave, screen clear, cooldown 600 frames |

## 6. Enemy Types

| Type | Size | HP | Speed | Behavior |
|------|------|----|-------|----------|
| Drone | 8x8 | 1 | 3 | Swarm patterns, sine wave |
| Gunship | 12x12 | 3 | 1.5 | Strafe and shoot |
| Turret | 10x10 | 5 | 0 (scroll) | Rotating fire pattern |
| Shield Gen | 12x12 | 8 | 0 (scroll) | Must destroy to damage boss |

## 7. Boss Design

### Level 1: Defense Satellite
- HP: 50
- Rotating arm with 4 turrets
- Fires radial bullet patterns
- Weak point: central core (glows when vulnerable)

### Level 2: Cyborg Carrier
- HP: 80
- Launches drone waves
- Missile barrage attack
- Shield phases (destroy generators first)

### Level 3: The Leviathan
- HP: 120
- Phase 1: Turret array
- Phase 2: Laser grid sweep
- Phase 3: Core exposed, desperate barrage

## 8. Level Structure

### Level 1: "Orbital Ring"
- Duration: ~90 seconds of waves
- Light drone swarms, introduce gunships
- Boss: Defense Satellite
- Story: Breaching outer defenses

### Level 2: "Neon Corridor"
- Duration: ~120 seconds
- Dense turret placement, gunship squads
- Hazards: Laser grids, asteroid debris
- Boss: Cyborg Carrier
- Story: Discovering AXIOM's weapon project

### Level 3: "The Abyss"
- Duration: ~150 seconds
- All enemy types, heavy assault
- Hazards: EMP zones, dense fire
- Boss: The Leviathan
- Story: Final confrontation

## 9. Power-up System

- Dropped by destroyed enemies (10% chance)
- Types: Spread (pink), Homing (green), Shield (cyan), EMP charge (purple)
- Float left-to-right, collected on contact
- Visual: Glowing rotating icons

## 10. Scoring

- Drone: 100 pts
- Gunship: 250 pts
- Turret: 300 pts
- Shield Gen: 500 pts
- Boss: 5000 pts
- Chain multiplier: kills within 2 seconds increase multiplier (max 8x)

## 11. Sound Design (Web Audio API)

All sounds are procedural:
- **laser**: High sawtooth burst, 800->1200Hz, 50ms
- **explosion**: White noise + sine drop 200->40Hz, 200ms
- **shield_hit**: Metallic band-pass noise, 100ms
- **power_up**: Ascending arpeggio (square wave), 300ms
- **boss_warning**: Deep pulsing sine 60Hz, 1s
- **emp**: Wide frequency sweep 100->2000Hz, 500ms
- **death**: Distorted noise + descending tone, 500ms

## 12. Touch Controls

- Virtual joystick: 120px outer ring, 50px knob, bottom-left
- Fire button: 60px circle, bottom-right (auto-fire when held)
- Special button: 50px circle, above fire button
- 20px deadzone on joystick
- Hidden on desktop via CSS media query

## 13. Visual Effects

- 3-layer parallax starfield
- Neon city silhouettes (procedural, scrolling)
- Particle explosions (20-40 particles per explosion)
- Neon glow trails on projectiles
- Digital rain on story screens
- Boss health bar at screen top
- Shield shimmer (sine-wave opacity on ship outline)
