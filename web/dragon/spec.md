# Dragon Fury - Technical Specification

## Overview

**Dragon Fury** is a side-scrolling belt-scrolling beat 'em up in the style of Streets of Rage and Final Fight. The player controls a martial artist fighting through waves of street thugs across three urban stages, using punches, kicks, grabs, throws, weapons, and special moves.

The game is implemented as a single-file HTML5 Canvas application with embedded CSS and JavaScript. It targets 640x480 resolution at 60fps with a retro pixel art aesthetic and gritty urban neon visuals.

---

## Core Mechanics

### Movement
- 8-directional movement on a pseudo-3D plane
- X-axis: horizontal movement (left/right along the stage)
- Y-axis: depth movement (up/down lanes, simulating moving toward/away from the screen)
- The playable depth band is constrained to roughly the lower 60% of the screen (Y range ~200-420 in game coordinates)
- Player speed: 3 px/frame horizontal, 2 px/frame vertical (depth movement is slower to preserve perspective illusion)

### Pseudo-3D Rendering
- All entities (player, enemies, pickups, weapons) are depth-sorted by their Y (foot) position each frame
- Entities with higher Y values (closer to the viewer) are drawn on top
- Shadow ellipses rendered beneath each entity, scaled by Y position
- Slight vertical scaling: entities higher on screen (further away) are drawn ~5% smaller

### Scrolling
- The camera follows the player horizontally with a dead zone (player stays within center 40% of screen)
- **Screen lock**: When enemies are present on screen, the camera stops scrolling and invisible walls appear at screen edges. The player must defeat all enemies in the current wave before scrolling resumes.
- Stage length: each stage is approximately 3000-4000 pixels wide
- Scroll triggers: predefined X positions along the stage trigger enemy wave spawns

---

## Combat System

### Punch Combo (Z key)
- 3-hit combo: jab → cross → hook
- Each hit in the combo has increasing damage (10, 15, 25)
- Combo window: 400ms between hits; if exceeded, combo resets
- Each hit produces a brief hitstun on the enemy (freeze frames: 2-3 frames)
- Hit sparks / impact particles spawn at the contact point
- Attack hitbox is a rectangle extending ~40px in front of the player

### Jump Kick (X key)
- Player enters a jump arc (parabolic, ~30 frames duration)
- Pressing Z during a jump performs a jump kick
- Jump kick deals 30 damage and knocks the enemy back ~60px
- The player is invulnerable during the first 10 frames of a jump
- If no attack input during jump, the player simply lands (can be used for evasion)

### Grab and Throw (walk into stunned enemy)
- When an enemy is in a "stunned" state (after taking a full combo or certain hits), walking into them initiates a grab
- During grab: the player holds the enemy for up to 1 second
- Press Z during grab: knee strike (20 damage), can do up to 3 knee strikes
- Press X or a direction + Z during grab: throw the enemy in that direction
- Thrown enemies deal 30 damage to any other enemy they collide with
- Thrown enemies travel ~200px before landing

### Special Move (C key)
- Costs 15% of the player's max health to use
- Cannot be used if health is at or below 15%
- Wide-area spinning attack hitting all enemies within ~80px radius
- Deals 40 damage
- Grants invincibility for the duration of the animation (~20 frames)
- Screen flash effect on activation

### Weapons
Weapons are picked up by walking over them on the ground.

| Weapon | Damage | Durability | Range | Speed |
|--------|--------|-----------|-------|-------|
| Pipe   | 35     | 8 hits    | 50px  | Normal |
| Knife  | 25     | 12 hits   | 35px  | Fast |
| Bottle | 40     | 3 hits    | 40px  | Slow |

- Weapons replace the normal punch combo with a single-hit attack
- When durability reaches 0, the weapon breaks (shatter particle effect) and is removed
- Player can only hold one weapon at a time
- Pressing down + Z drops the current weapon

---

## Enemies

### Thug (Basic)
- Health: 60
- Damage: 10 per hit
- Behavior: walks toward player, attacks when in range
- Attack pattern: single punch with ~1 second cooldown
- Movement speed: 1.5 px/frame
- Color palette: red/brown jacket

### Knife Wielder
- Health: 40
- Damage: 20 per hit (knife slash)
- Behavior: cautious - circles the player, dashes in to attack, retreats
- Attack pattern: quick dash-slash with 1.5s cooldown
- Movement speed: 2 px/frame
- Color palette: purple/dark outfit
- Drops: knife weapon on death (50% chance)

### Fat Brawler
- Health: 150
- Damage: 25 per hit (heavy punch), 35 (charge attack)
- Behavior: slow approach, has a charge attack (telegraphed with 1s windup)
- Cannot be grabbed or thrown
- Super armor: does not flinch from single hits (requires 3+ hits to stagger)
- Movement speed: 0.8 px/frame
- Color palette: green tank top, large sprite

### Boss (one per stage)
- **Stage 1 Boss - "Blade"**: Knife wielder captain. 300 HP. Fast combos, throws knives as projectiles.
- **Stage 2 Boss - "Crusher"**: Giant brawler. 500 HP. Ground pound AoE attack, can grab the player.
- **Stage 3 Boss - "Dragon King"**: Martial artist. 400 HP. Uses the player's own moveset (combos, jump kicks, specials). Final showdown.

---

## Stages

### Stage 1: Back Alley
- Length: 3000px
- Background: dark urban alley, neon signs, dumpsters, brick walls, flickering streetlights
- Parallax layers: far buildings (0.2x), mid buildings (0.5x), foreground trash/crates (1.0x)
- Enemy waves: 3 waves of thugs, 1 mixed wave, boss fight
- Pickups: pipe weapon, food (chicken), 1 extra life token
- Ambient: rain particles, puddle reflections

### Stage 2: Warehouse
- Length: 3500px
- Background: industrial warehouse interior, conveyor belts, crates, overhead lights
- Parallax layers: far wall (0.3x), mid shelving (0.6x), foreground crates (1.0x)
- Enemy waves: 2 thug waves, 2 mixed waves with knife wielders, 1 brawler wave, boss
- Pickups: bottle weapon, knife weapon, food (pizza), extra life token
- Ambient: sparking electrical effects, swinging overhead lights

### Stage 3: Rooftop Showdown
- Length: 4000px
- Background: city rooftop at night, skyline with neon, water tower, AC units
- Parallax layers: distant skyline (0.1x), mid buildings (0.4x), rooftop surface (1.0x)
- Enemy waves: all enemy types mixed, increasing density, final boss
- Pickups: all weapon types, food, extra life
- Ambient: wind particles, helicopter searchlight sweeping

---

## Pickups

| Item | Effect | Sprite |
|------|--------|--------|
| Chicken | Restore 30% health | Drumstick icon |
| Pizza | Restore 60% health | Pizza slice icon |
| Extra Life | +1 life | Star token |

- Pickups appear at predefined stage positions or drop from certain enemies (10% chance)
- Picked up by walking over them (within 20px radius)
- Brief flash effect and score bonus (+500) on pickup

---

## Player Stats

- Max Health: 100
- Lives: 3 (default)
- Walk Speed: 3 px/frame
- Jump Height: ~80px arc
- Invincibility on respawn: 120 frames (2 seconds)
- Respawn: player reappears at current screen position with brief invincibility flash

---

## HUD and UI

### In-Game HUD
- Top-left: Player name "DRAGON" + health bar (colored gradient: green → yellow → red)
- Top-right: Score display
- Top-center: Stage name (appears briefly at stage start, then fades)
- Bottom-left: Lives remaining (small character icons)
- Bottom-right: Weapon indicator (icon + durability count) when holding a weapon

### Title Screen
- Game logo "DRAGON FURY" with fire/neon effect
- Procedural character sprite art
- "PRESS START" blinking text
- Controls hint text

### Game Over Screen
- "GAME OVER" text
- Final score
- "CONTINUE?" countdown (9 seconds)
- Press start to continue or let timer expire for true game over

### Stage Transition
- Screen fades to black
- Stage name displayed in large text ("STAGE 2: WAREHOUSE")
- Brief loading pause, then fade in to new stage

---

## Controls

### Keyboard
| Key | Action |
|-----|--------|
| Arrow Left/Right | Move horizontally |
| Arrow Up/Down | Move in depth (up = further, down = closer) |
| Z | Punch / Attack / Confirm |
| X | Jump |
| C | Special move |
| Enter | Start / Pause |

### Mobile Touch
- **Left side**: Virtual joystick (8-directional analog stick)
- **Right side**: Three action buttons arranged in a triangle
  - "A" button (green): Punch/Attack (maps to Z)
  - "B" button (blue): Jump (maps to X)
  - "C" button (red): Special (maps to C)

---

## Technical Details

### Rendering
- Canvas size: 640x480 (logical), scaled to fit viewport
- 60fps target via `requestAnimationFrame` with delta time
- Depth sorting: all game entities collected into a single array, sorted by Y position each frame before rendering
- Sprite system: procedural pixel art defined as character arrays (color maps)
- Particle system: lightweight particle emitter for hit sparks, blood/sweat effects, weapon break effects
- Screen shake on heavy impacts (2-4px offset for 6-10 frames)
- Scanline overlay CSS effect for retro CRT feel

### Audio
- Web Audio API for all sound effects
- Procedurally generated sounds:
  - Punch impacts (noise burst + low thump)
  - Kick impacts (higher pitch thwack)
  - Grunt/pain sounds (short noise with pitch envelope)
  - Weapon hits (metallic clang for pipe, sharp slice for knife, glass shatter for bottle)
  - Special move activation (rising sweep)
  - Pickup chime (ascending tones)
  - KO sound (descending tone)
  - Stage clear fanfare

### Collision Detection
- Axis-aligned bounding box (AABB) for entity-entity overlap
- Attack hitboxes are separate rectangles activated during attack frames
- Depth check: entities must be within ~20px Y difference to interact (pseudo-3D lane matching)

### State Machine
- Game states: `TITLE`, `PLAYING`, `PAUSED`, `STAGE_TRANSITION`, `GAME_OVER`, `VICTORY`
- Enemy states: `IDLE`, `WALKING`, `ATTACKING`, `STUNNED`, `GRABBED`, `THROWN`, `KNOCKED_DOWN`, `DEAD`
- Player states: `IDLE`, `WALKING`, `PUNCHING`, `JUMPING`, `JUMP_KICKING`, `GRABBING`, `THROWING`, `SPECIAL`, `HURT`, `DOWN`, `DEAD`

### Performance Targets
- Maintain 60fps with up to 8 active enemies on screen
- Particle pool: max 100 particles
- Object pooling for enemies and particles to minimize garbage collection

---

## Asset Style Guide

### Color Palette (Neon Urban)
- Background darks: #0a0a1a, #1a1a2e, #16213e
- Neon accents: #ff0066 (pink), #00ffff (cyan), #ff6600 (orange), #ffff00 (yellow)
- Character outlines: #000000
- Player primary: #0088ff (blue jacket), #ffcc00 (blonde hair)
- Enemy thugs: #cc3333 (red), #884422 (brown)
- Enemy knife: #8833aa (purple), #333333 (dark)
- Enemy brawler: #33aa33 (green), #ffaa66 (skin)
- Health bar: gradient #00ff00 → #ffff00 → #ff0000
- UI text: #ffffff with #00ffff glow

### Sprite Dimensions
- Player: 32x48 px (logical), rendered at 2x
- Enemies: 32x48 px (thugs, knife), 40x52 px (brawler), 48x56 px (bosses)
- Weapons on ground: 24x12 px
- Pickups: 16x16 px
- Hit sparks: 8-16 px particles

---

## Game Flow

1. Title screen → Press Start
2. Stage 1: Back Alley intro text → gameplay → boss → victory
3. Stage transition screen
4. Stage 2: Warehouse intro text → gameplay → boss → victory
5. Stage transition screen
6. Stage 3: Rooftop intro text → gameplay → final boss → ending
7. Victory screen with final score
8. Return to title

If the player loses all lives:
- Game Over screen with continue countdown
- If continue: restart current stage with score preserved
- If timeout: return to title screen, score reset
