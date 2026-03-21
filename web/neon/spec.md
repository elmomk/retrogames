# Neon Runner - Technical Specification

## Overview
**Title**: Neon Runner
**Genre**: Cyberpunk Platformer
**Story**: "Ghost Protocol"
**Resolution**: 640x480 logical, scaled to fullscreen
**Target**: 60 FPS fixed timestep with death spiral prevention

## Story: Ghost Protocol
In Neo-Kyoto, Kira-7 is a rogue hacker with cybernetic legs. After stealing classified data exposing a corpo mind-control network, a kill order forces her to run across neon rooftops, hack security systems, and fight through corpo enforcers to reach an underground broadcast tower and expose the truth.

## Controls

### Keyboard
| Key | Action |
|-----|--------|
| Arrow Left/Right or A/D | Move |
| Arrow Up / W / Space | Jump (variable height) |
| X | Cyber blade attack / Hack terminal |
| Z | Dash |
| C | Throw EMP grenade |

### Touch (Mobile)
- Virtual joystick (120px outer, 50px knob) bottom-left with 20px deadzone
- B button (60px) bottom-right: Jump
- A button (60px) above B: Attack/Hack
- Dash button (50px) left of B: Dash

## Visual Style

### Color Palette
- Hot Magenta: #ff0080
- Electric Blue: #00d4ff
- Acid Green: #00ff41
- Amber: #ffb800
- Dark City: #0d0d1a
- Deep Purple: #1a0033

### CRT Effects
- Scanline overlay (2px repeating gradient)
- Vignette (radial gradient)
- Chromatic aberration on damage

### Visual Effects
- Rain particles (vertical lines, varying speed/alpha)
- 3-layer parallax city background with lit windows
- Neon glow on platforms (shadowBlur)
- Dash afterimage trail (3 fading copies)
- Wall slide sparks (particle emitter)
- Screen shake on damage/explosions
- Digital glitch on taking damage
- Flickering neon signs
- Particle burst on enemy death

## Sprite System
Procedural 8x8 or 16x16 pixel art via `createSprite(art, colors)`:
- Character arrays define pixel layout
- Color indices map to palette
- Rendered to offscreen canvas, cached

### Sprites
- **Kira-7**: Hooded runner, cybernetic legs (magenta/blue)
- **Patrol Drone**: Hovering orb with antenna
- **Corpo Guard**: Armored humanoid
- **Sentry Turret**: Wall-mounted rotating barrel
- **Cyber-Hound**: Quadruped robot (Level 3 boss)
- **Hackable Terminal**: Glowing console
- **Data Chip**: Rotating hexagon
- **Health Pack**: Cross symbol
- **EMP Grenade**: Sphere with lightning

## Player Mechanics

### Movement
- Run speed: 3.5 px/frame
- Gravity: 0.45 px/frame^2
- Max fall speed: 8.0 px/frame
- Coyote time: 6 frames
- Jump buffer: 6 frames

### Jump
- Jump force: -8.0
- Variable height: releasing jump early halves upward velocity
- Wall slide: 1.5 px/frame max fall
- Wall jump: -7.0 Y, 5.0 X away from wall

### Dash
- Duration: 8 frames
- Speed: 8.0 px/frame horizontal
- Cooldown: 30 frames
- Leaves 3 afterimage copies
- Invulnerable during dash

### Combat
- **Cyber Blade**: Melee slash, 20px range, 10-frame cooldown
- **EMP Grenade**: Arc throw, 60px blast radius, stuns enemies 120 frames
- Grenades are limited (start with 3, collectible)

### Hack Ability
- Press attack near hackable terminal (within 30px)
- Disables connected laser grids/turrets for 300 frames
- Terminal shows "HACKED" state with green glow

### Health
- 3 hit points per life
- 3 lives total
- Invulnerability frames: 60 after hit
- Health packs restore 1 HP

## Enemies

### Patrol Drone
- Hovers in sine wave pattern
- Fires laser every 90 frames
- HP: 1 (1 blade hit)
- Score: 100

### Corpo Guard
- Walks platform edge-to-edge
- Fires pistol every 120 frames
- HP: 2
- Score: 200

### Sentry Turret
- Wall-mounted, stationary
- Rotating beam (90-degree sweep)
- Can be hacked to disable
- HP: 3 (or hack to disable)
- Score: 150

### Cyber-Hound (Boss - Level 3)
- Fast movement, charges at player
- Leap attack (parabolic arc)
- HP: 15
- Phases: Normal -> Enraged (50% HP, faster)
- Score: 1000

## Hazards
- **Laser Grids**: Toggle on/off every 90 frames, 1 damage
- **Electric Floors**: Constant damage zone, spark particles
- **Acid Pools**: Instant kill, bubbling animation
- **Falling Platforms**: Shake 30 frames after stepped on, then fall

## Level Design

### Tile System
- Tile size: 20x20 pixels
- Map width: ~120 tiles (2400px)
- Map height: ~20 tiles (400px)

### Tile Legend
| Char | Element |
|------|---------|
| `#` | Solid platform (neon-edged) |
| `.` | Empty space |
| `D` | Patrol drone spawn |
| `G` | Corpo guard spawn |
| `T` | Sentry turret spawn |
| `L` | Laser grid |
| `E` | Electric floor |
| `C` | Data chip |
| `H` | Health pack |
| `M` | EMP ammo |
| `K` | Hackable terminal |
| `F` | Falling platform |
| `A` | Acid pool |
| `P` | Player start |
| `X` | Level exit |

### Level 1: "Rooftop Chase"
- Outdoor neon rooftops
- Basic platforming, introduces dash and wall-jump
- Patrol drones and corpo guards
- ~120 tiles wide

### Level 2: "Corpo Tower"
- Interior corporate building
- Laser grids, turrets, hacking puzzles
- More complex layouts with vertical sections
- ~120 tiles wide

### Level 3: "The Underground"
- Sewer/tunnel aesthetic
- Electric floors, acid pools
- Boss fight: Cyber-Hound at broadcast tower
- ~120 tiles wide + boss arena

## Screen States

### START
- Parallax neon city skyline with rain
- Glitch title text "NEON RUNNER" with chromatic split
- Subtitle "GHOST PROTOCOL"
- "JACK IN" blinking prompt
- Any key/tap to start

### LEVEL_STORY
- Terminal aesthetic (green-on-black)
- Blinking cursor
- Typewriter text reveal (2 chars/frame)
- Press key to skip/advance

### PLAYING
- Gameplay with scrolling camera
- HUD: Health bar, lives, score, EMP count, level name

### GAMEOVER
- Glitch static effect
- "CONNECTION LOST" text
- Final score display
- "RECONNECT?" prompt

### WIN
- "TRUTH BROADCAST" header
- Victory narrative typewriter
- Final score

## Audio (Web Audio API)
All procedural, no audio files:
- **jump**: Square wave, 300->600Hz, 80ms
- **dash**: Noise sweep, 100ms
- **slash**: Sawtooth, 800->200Hz, 60ms
- **emp**: Noise burst + sine sweep, 200ms
- **hack**: 3-note ascending beep sequence
- **hit**: Square wave, 200->80Hz + noise, 100ms
- **death**: Sawtooth descend 400->30Hz, 800ms
- **pickup**: Ascending chime (3 notes), 180ms
- **laser**: Sine pulse, 1200Hz, 50ms

## Camera
- Horizontal follow with look-ahead (lerp 0.1)
- Vertical follow (lerp 0.08)
- Clamped to level bounds
- Screen shake offset applied after camera calc

## Performance
- Fixed 60 FPS timestep with accumulator
- Death spiral prevention: max 3 updates per frame
- Object pooling for particles and bullets
- Canvas state save/restore for effects
