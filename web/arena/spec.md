# Arena Blitz - Technical Specification

## Overview
Arena Blitz is a top-down twin-stick arena shooter inspired by Smash TV and Robotron: 2084. The player fights through 10 waves of increasingly difficult enemies in a single-screen arena, collecting weapon pickups and power-ups to survive. The game features a retro pixel art aesthetic with neon-on-dark visuals.

## Game Canvas
- Resolution: 640x480 pixels
- Target frame rate: 60fps (fixed timestep via requestAnimationFrame)
- Single HTML file with embedded CSS and JavaScript
- Retro CRT scanline overlay effect
- Font: Press Start 2P (Google Fonts)

## Arena Design
- Single-screen arena with visible electric fence boundaries
- Arena interior is a dark floor with subtle grid pattern
- Electric fence borders pulse with neon energy and damage the player on contact
- Occasional laser sweep hazards cross the arena horizontally or vertically (telegraph 1 second before activation)
- Arena base size: ~560x400 playable area centered in the 640x480 canvas
- The arena does not physically expand, but later waves unlock additional spawning zones and hazard patterns

## Player
- Top-down sprite, 16x16 pixels, procedurally drawn
- 8-directional movement
- Base speed: 3 pixels/frame
- Health: 100 HP
- Invincibility frames on hit: 60 frames (1 second) with blinking effect
- Collision hitbox: 12x12 centered

### Movement Controls
- **Keyboard:** WASD or Arrow Keys for 8-directional movement
- **Mobile:** Left virtual joystick (touch)
- Diagonal movement normalized to prevent faster diagonal speed

### Aiming Controls
- **Mouse:** Cursor position determines aim direction, click to fire
- **Mobile:** Right virtual joystick for aim direction, auto-fires when joystick is active
- Crosshair rendered at cursor/aim position

### Weapon Switching
- **Keyboard:** Number keys 1-4 to switch weapons
- **Mobile:** Tap weapon icon in HUD

## Weapons

### 1. Pistol (Default)
- Infinite ammo
- Fire rate: 6 rounds/second
- Damage: 10 per bullet
- Bullet speed: 8 px/frame
- Single projectile, small yellow dot
- Slight spread: +/- 2 degrees random

### 2. Shotgun
- Ammo: 20 shells per pickup
- Fire rate: 2 rounds/second
- Damage: 8 per pellet
- Bullet speed: 7 px/frame
- 5 pellets in a 30-degree spread cone
- Pellets have short range (fade after 150px)

### 3. Laser
- Ammo: 50 units per pickup (drains continuously while firing)
- Fire rate: Continuous beam
- Damage: 3 per frame (180 DPS)
- Instant hit (raycast)
- Rendered as a bright cyan beam with glow effect
- Beam length: full arena diagonal
- Pierces all enemies in line

### 4. Rocket
- Ammo: 8 rockets per pickup
- Fire rate: 1 round/second
- Direct hit damage: 30
- Splash damage: 20 (within 40px radius)
- Projectile speed: 5 px/frame
- Explodes on contact with enemy or after traveling 300px
- Area of Effect explosion with screen shake

## Enemy Types

### Swarmer
- HP: 15
- Speed: 2.5 px/frame
- Size: 10x10
- Behavior: Moves directly toward player, slight weaving
- Color: Green
- Score: 10 points
- Spawns in groups of 3-6
- First appears: Wave 1

### Tank
- HP: 80
- Speed: 1 px/frame
- Size: 20x20
- Behavior: Slow advance toward player, fires a slow bullet every 3 seconds
- Color: Red/orange
- Score: 50 points
- Tank bullets: 3 px/frame, 15 damage
- First appears: Wave 2

### Teleporter
- HP: 30
- Speed: 2 px/frame (when visible)
- Size: 14x14
- Behavior: Moves toward player, teleports to random position every 2-4 seconds (telegraph with flash before vanishing)
- Color: Purple/magenta
- Score: 30 points
- Brief invulnerability during teleport (0.3s)
- First appears: Wave 3

### Splitter
- HP: 50 (large), 20 (small)
- Speed: 1.5 px/frame (large), 3 px/frame (small)
- Size: 18x18 (large), 10x10 (small)
- Behavior: Moves toward player; on death, splits into 2-3 smaller versions
- Color: Teal/cyan
- Score: 40 (large), 15 (small)
- First appears: Wave 4

## Wave System

### Wave Structure
- 10 waves total
- Each wave has a target enemy count to clear
- Brief intermission between waves (3 seconds) with "WAVE X" announcement
- Enemy count and difficulty scale per wave
- Boss waves: Wave 5 and Wave 10

### Wave Progression
| Wave | Enemies | Types | Special |
|------|---------|-------|---------|
| 1 | 8 | Swarmers | Tutorial wave |
| 2 | 12 | Swarmers, Tanks | Tanks introduced |
| 3 | 16 | Swarmers, Tanks, Teleporters | Teleporters introduced |
| 4 | 20 | All types + Splitters | Splitters introduced |
| 5 | 15 + Boss | All types | Boss: Mega Tank (300 HP) |
| 6 | 25 | All types | Laser sweeps begin |
| 7 | 30 | All types (more Teleporters) | Faster spawns |
| 8 | 35 | All types (more Splitters) | Double hazards |
| 9 | 40 | All types | Maximum intensity |
| 10 | 30 + Boss | All types | Final Boss: Swarm Queen (500 HP) |

### Boss Mechanics
- **Mega Tank (Wave 5):** Large sprite (32x32), fires 3-way spread shots, spawns 2 Swarmers every 5 seconds, 300 HP
- **Swarm Queen (Wave 10):** Medium sprite (24x24), teleports frequently, spawns all enemy types, fires homing projectiles, 500 HP

## Power-Ups
Power-ups drop from enemies with a 15% chance. They appear as rotating pickup sprites and despawn after 10 seconds.

### Speed Boost
- Color: Blue
- Duration: 8 seconds
- Effect: Player speed x1.5
- Visual: Blue trail particles behind player

### Shield
- Color: White/silver
- Duration: 6 seconds
- Effect: Absorbs next 50 damage
- Visual: Glowing circle around player

### Double Damage
- Color: Red
- Duration: 8 seconds
- Effect: All weapon damage x2
- Visual: Player sprite tints red, bullets glow brighter

### Freeze
- Color: Cyan
- Duration: 5 seconds
- Effect: All enemies move at 30% speed
- Visual: Ice crystal particles, enemies tinted blue

## Weapon Pickups
Weapon pickups drop from enemies with a 10% chance (separate roll from power-ups). They appear as weapon-shaped icons.

- Picking up a weapon you already have adds ammo
- Picking up a new weapon switches to it
- Each pickup grants the ammo amount listed in the weapon specs

## HUD
- **Top-left:** Health bar (red/green gradient, numeric value)
- **Top-center:** Wave indicator ("WAVE 3/10")
- **Top-right:** Score (numeric, increments with pop animation)
- **Bottom-left:** Current weapon name + ammo count (or "INF" for pistol)
- **Bottom-center:** Weapon slots 1-4 with highlight on active weapon
- **Active power-ups:** Icons with remaining duration timers below HUD

## Visual Effects
- **Bullets:** Small glowing dots with short trails
- **Laser beam:** Bright line with additive-blend glow
- **Explosions:** Expanding circle + 8-16 particles in random directions
- **Enemy death:** Flash white, then burst into 6-10 colored particles
- **Screen shake:** On explosions and player damage (intensity varies)
- **Player damage flash:** Screen border flashes red briefly
- **Electric fence:** Animated pulsing border with spark particles
- **Wave announcement:** Large text fades in center, scales up then fades out
- **Muzzle flash:** Brief bright circle at weapon barrel

## Audio (Web Audio API)
All sounds generated procedurally using oscillators and noise:
- **Pistol shot:** Short high-frequency blip
- **Shotgun blast:** White noise burst, medium duration
- **Laser hum:** Continuous low-frequency oscillation while firing
- **Rocket launch:** Rising pitch sweep
- **Explosion:** Low-frequency boom with noise decay
- **Enemy hit:** Short mid-frequency tick
- **Enemy death:** Descending pitch sweep
- **Pickup collected:** Rising arpeggio (3 quick ascending tones)
- **Player hit:** Low thud with brief distortion
- **Wave start:** Ascending fanfare (3 notes)
- **Game over:** Descending minor chord

## Game States
1. **TITLE** - Title screen with logo, "PRESS START" blinking, controls hint
2. **PLAYING** - Main gameplay
3. **WAVE_INTRO** - Brief wave announcement overlay (3 seconds)
4. **GAME_OVER** - Final score display, restart option
5. **VICTORY** - All 10 waves cleared, final score + congratulations

## Technical Architecture
- Single `requestAnimationFrame` loop with fixed 60fps timestep
- Entity arrays: `players`, `bullets`, `enemies`, `particles`, `powerUps`, `pickups`, `hazards`
- Collision detection: AABB (axis-aligned bounding box) for all entities
- All sprites drawn procedurally on canvas (no external assets)
- Sprite definitions as pixel color arrays
- Screen shake implemented as canvas translate offset
- Particle system with velocity, lifetime, color, size properties
- Web Audio API context created on first user interaction
- Touch controls: dual virtual joysticks overlaid on canvas area

## Mobile Touch Controls
- **Left joystick:** Bottom-left quadrant, controls movement
- **Right joystick:** Bottom-right quadrant, controls aim direction; auto-fires when deflected
- Joystick visual: outer ring (static) + inner knob (follows touch)
- Dead zone: 10% of joystick radius
- Joysticks only appear on touch-capable devices

## Performance Targets
- 60fps on modern browsers (Chrome, Firefox, Safari, Edge)
- Maximum simultaneous entities: ~200 (enemies + bullets + particles)
- Particle pool with recycling to minimize garbage collection
- Object pooling for bullets and particles
