# Shadow Blade - Technical Specification

## Overview

**Shadow Blade** is a fast-paced side-scrolling ninja action platformer rendered on an HTML5 Canvas. The player controls a ninja warrior navigating through dangerous environments, defeating enemies with sword combos, shurikens, and acrobatic movement. The game features retro 16-bit pixel art aesthetics with procedural sprite generation, Web Audio API sound effects, and both keyboard and touch controls.

## Canvas & Rendering

- **Resolution:** 640x480 internal canvas, scaled to fit the viewport while maintaining aspect ratio
- **Frame Rate:** 60fps via `requestAnimationFrame` with fixed timestep (16.67ms)
- **Art Style:** Procedural pixel art sprites defined as character arrays, 16-bit retro aesthetic
- **Post-Processing:** Scanline overlay effect via CSS, CRT-style glow using canvas `shadowBlur`
- **Camera:** Horizontal scrolling camera that follows the player, with slight vertical tracking
- **Parallax:** 3-layer parallax scrolling background (far mountains, mid trees/buildings, near details)

## Player Character

### Attributes

| Attribute       | Value              |
|-----------------|--------------------|
| Sprite Size     | 24x32 pixels       |
| Max Health      | 5 HP               |
| Run Speed       | 4 px/frame         |
| Jump Velocity   | -10 px/frame       |
| Gravity         | 0.5 px/frame^2     |
| Max Fall Speed  | 8 px/frame         |
| Shuriken Start  | 10 ammo            |
| Shuriken Max    | 30 ammo            |

### Movement Abilities

1. **Run** - Horizontal movement at base speed. Acceleration-based for smooth start/stop.
2. **Jump** - Variable height based on button hold duration. Max hold: 12 frames.
3. **Wall Slide** - When pressing toward a wall while airborne, the ninja slides down slowly (gravity * 0.3).
4. **Wall Jump** - Press jump while wall-sliding to leap away from the wall at a 45-degree angle. Applies horizontal velocity away from wall + upward velocity.
5. **Air Dash** - Press dash while airborne for a quick horizontal burst (16px/frame for 6 frames). One use per airborne period, resets on ground touch. Brief invincibility frames (6 frames). Leaves afterimage trail particles.
6. **Slide** - Press dash while grounded and moving to perform a low slide (10 frames duration). Reduces hitbox height by 50%. Passes under low obstacles and certain enemy attacks.

### Combat

1. **Sword Combo (Z key / Attack button)**
   - 3-hit melee combo: slash right -> slash left -> overhead slam
   - Each hit has a 10-frame window; combo resets if no input within 20 frames
   - Hit 1: 1 damage, short range horizontal arc
   - Hit 2: 1 damage, slightly wider arc
   - Hit 3: 2 damage, downward slam with small shockwave
   - Each swing produces slash particle effects (white/cyan arcs)

2. **Charged Slash**
   - Hold attack button for 30+ frames, release for a wide charged slash
   - 3 damage, large hitbox (1.5x normal range)
   - Visual: large cyan energy slash arc with screen flash

3. **Shuriken (V key / Shuriken button)**
   - Fires a projectile in the direction the player faces
   - Speed: 8 px/frame
   - Damage: 1
   - Limited ammo, displayed in HUD
   - Consumes 1 ammo per throw
   - 15-frame cooldown between throws

### Animation States

- `idle` - Breathing animation, 4 frames, 10 ticks/frame
- `run` - Run cycle, 6 frames, 5 ticks/frame
- `jump` - Rising pose, 2 frames
- `fall` - Falling pose, 2 frames
- `wall_slide` - Clinging to wall, 2 frames
- `air_dash` - Horizontal blur pose, 3 frames
- `slide` - Low crouching slide, 2 frames
- `attack_1` - Horizontal slash, 4 frames, 3 ticks/frame
- `attack_2` - Reverse slash, 4 frames, 3 ticks/frame
- `attack_3` - Overhead slam, 5 frames, 3 ticks/frame
- `charged_slash` - Wide energy slash, 6 frames, 3 ticks/frame
- `throw` - Shuriken throw pose, 4 frames, 3 ticks/frame
- `hurt` - Knockback flash, 3 frames
- `death` - Collapse animation, 8 frames

## Enemies

### 1. Patrol Guard

| Attribute     | Value            |
|---------------|------------------|
| HP            | 2                |
| Damage        | 1 (contact)      |
| Speed         | 1.5 px/frame     |
| Sprite Size   | 20x28 pixels     |
| Score Value   | 100              |

- Walks back and forth on a platform between two patrol points
- Turns around at edges or walls
- On detecting the player within 120px horizontal range, charges at 2.5x speed
- Wears basic armor (grey/brown palette)
- Death: falls backward, bursts into particles

### 2. Archer

| Attribute     | Value            |
|---------------|------------------|
| HP            | 1                |
| Damage        | 1 (arrow)        |
| Arrow Speed   | 5 px/frame       |
| Sprite Size   | 20x28 pixels     |
| Score Value   | 150              |
| Range         | 200px            |

- Stands on elevated platforms
- Fires arrows at the player every 90 frames when player is in line of sight
- Arrows travel horizontally in the direction the archer faces
- Low HP but dangerous at range
- Purple/dark red palette
- Death: slumps and fades

### 3. Shield Brute

| Attribute     | Value            |
|---------------|------------------|
| HP            | 4                |
| Damage        | 2 (contact)      |
| Speed         | 1 px/frame       |
| Sprite Size   | 24x32 pixels     |
| Score Value   | 250              |

- Carries a shield that blocks frontal attacks
- Must be attacked from behind or above (air attack / overhead slam)
- Periodically lowers shield to charge (vulnerability window: 30 frames)
- Heavy palette (dark grey/gold)
- Death: shield drops, enemy staggers and explodes into particles

### 4. Ninja Boss (Level 3 Boss)

| Attribute     | Value            |
|---------------|------------------|
| HP            | 20               |
| Damage        | 2 (melee), 1 (shuriken) |
| Speed         | 3 px/frame       |
| Sprite Size   | 28x36 pixels     |
| Score Value   | 2000             |

- Three attack phases based on remaining HP:
  - **Phase 1 (20-14 HP):** Dash attacks and single shuriken throws
  - **Phase 2 (13-7 HP):** Teleport behind player, 3-hit combo, fan of 3 shurikens
  - **Phase 3 (6-1 HP):** Rapid teleports, shadow clone decoys (1 HP each), desperation attacks
- Brief invulnerability during teleport (12 frames)
- Red/black palette with glowing eyes
- Arena fight: locked screen, no scrolling

## Level Design

### Level 1: Bamboo Forest

- **Theme:** Dense bamboo grove at twilight, purple-orange sky gradient
- **Length:** ~4000px scrolling width
- **Platforms:** Natural terrain, bamboo logs, stone ledges, wooden bridges
- **Hazards:** Spike pits, falling bamboo logs (triggered)
- **Enemies:** Patrol Guards (6), Archers (3)
- **Collectibles:** 5 scrolls, 2 health pickups, 1 shuriken ammo pack
- **End Trigger:** Reach the torii gate at the right edge

### Level 2: Castle Rooftops

- **Theme:** Moonlit castle rooftops, dark blue sky with stars
- **Length:** ~5000px scrolling width
- **Platforms:** Roof tiles (sloped), watchtowers, wooden beams, hanging lanterns
- **Hazards:** Crumbling tiles (fall after 30 frames of standing), torch fire (periodic)
- **Enemies:** Patrol Guards (5), Archers (5), Shield Brutes (3)
- **Collectibles:** 7 scrolls, 3 health pickups, 2 shuriken ammo packs
- **End Trigger:** Enter the castle gate

### Level 3: Demon Shrine

- **Theme:** Crimson shrine interior with flickering torchlight, fog effects
- **Length:** ~3000px scrolling width + boss arena
- **Platforms:** Stone pillars, floating shrine platforms, demon statues
- **Hazards:** Fire jets (periodic), collapsing floor segments, poison fog zones
- **Enemies:** Patrol Guards (4), Archers (4), Shield Brutes (4), Ninja Boss (1)
- **Collectibles:** 8 scrolls, 4 health pickups, 3 shuriken ammo packs
- **End Trigger:** Defeat the Ninja Boss

## HUD & UI

### In-Game HUD

- **Top-Left:** Health bar (5 red hearts or segmented bar)
- **Top-Center:** Level name
- **Top-Right:** Score display (yellow text)
- **Bottom-Left:** Shuriken ammo count with icon
- **Boss HP:** Large bar centered below top HUD, only visible during boss fight

### Screens

1. **Title Screen**
   - Game logo "SHADOW BLADE" in stylized pixel font
   - Ninja silhouette background with particle effects
   - "PRESS START" text blinking at 1Hz
   - Background: dark gradient with floating embers
   - Controls hint text at bottom

2. **Level Intro**
   - Level name fades in center-screen for 2 seconds
   - Brief subtitle (e.g., "The Bamboo Forest Awaits...")

3. **Pause Screen**
   - Semi-transparent dark overlay
   - "PAUSED" text centered
   - Resume / Quit options

4. **Death Screen**
   - Screen flashes red
   - "YOU DIED" text
   - Remaining lives display
   - Auto-respawn at last checkpoint after 2 seconds

5. **Game Over Screen**
   - "GAME OVER" in large red text
   - Final score display
   - "PRESS START TO CONTINUE" blinking
   - High score tracking (localStorage)

6. **Victory Screen**
   - "MISSION COMPLETE" text
   - Final score with bonus calculations
   - Time bonus, collectible bonus

## Controls

### Keyboard

| Key           | Action              |
|---------------|---------------------|
| Arrow Left/A  | Move left           |
| Arrow Right/D | Move right          |
| Arrow Up/W    | Look up / climb     |
| Arrow Down/S  | Look down / crouch  |
| Z             | Attack (sword)      |
| X             | Jump                |
| C             | Dash / Slide        |
| V             | Throw shuriken      |
| Enter         | Start / Pause       |
| Escape        | Pause               |

### Mobile / Touch

- **Left side:** Virtual joystick for movement (same pattern as space game)
- **Right side:** 4 action buttons arranged in diamond:
  - Top: Jump (X)
  - Right: Attack (Z)
  - Bottom: Dash (C)
  - Left: Shuriken (V)
- Touch controls panel fixed at bottom of screen (160px height)

## Audio (Web Audio API)

All sounds generated procedurally using Web Audio API oscillators and noise:

| Sound           | Description                                        |
|-----------------|----------------------------------------------------|
| `sword_slash`   | Short noise burst with high-frequency sweep down   |
| `charged_slash` | Longer sweep with reverb and bass impact           |
| `shuriken`      | Quick high-pitched whistle (sine wave glide)       |
| `jump`          | Short ascending tone (square wave)                 |
| `wall_jump`     | Double ascending tone                              |
| `air_dash`      | Whoosh noise burst                                 |
| `hit_enemy`     | Mid-pitch impact thud (noise + square)             |
| `enemy_death`   | Descending tone with noise burst                   |
| `player_hurt`   | Low harsh buzz                                     |
| `player_death`  | Long descending wail                               |
| `pickup_scroll` | Ascending arpeggio (3 quick notes)                 |
| `pickup_health` | Gentle ascending chime                             |
| `pickup_ammo`   | Metallic click                                     |
| `boss_music`    | Looping arpeggio pattern (bass + lead)             |

## Technical Implementation

### Architecture

```
Single HTML file:
├── <style> - CSS layout, font import, scanlines, touch controls
├── <body>
│   ├── #app-wrapper
│   │   ├── #game-container
│   │   │   ├── <canvas> - Main game canvas (640x480)
│   │   │   ├── .scanlines - CSS overlay
│   │   │   └── #ui-layer - HUD and screen overlays
│   │   └── #control-panel - Touch controls (joystick + buttons)
│   └── <script> - All game logic
│       ├── Canvas setup & resize handling
│       ├── Sprite definitions (character arrays)
│       ├── Input handling (keyboard + touch)
│       ├── Game state machine
│       ├── Physics engine (gravity, collision)
│       ├── Entity classes (Player, Enemy, Projectile, Particle, Pickup)
│       ├── Level data & camera system
│       ├── Procedural audio functions
│       └── Main game loop (fixed timestep)
```

### Sprite System

Sprites are defined as 2D character arrays where each character maps to a color:

```javascript
const PALETTE = {
    '.': null,           // transparent
    'K': '#111111',      // black (outline)
    'G': '#555555',      // dark grey
    'W': '#ffffff',      // white
    'R': '#ff0000',      // red
    'B': '#0066ff',      // blue
    'C': '#00ffff',      // cyan
    'Y': '#ffcc00',      // yellow
    'P': '#9933ff',      // purple
    'O': '#ff6600',      // orange
    'N': '#8B4513',      // brown
    'D': '#333333',      // dark
    'S': '#C0C0C0',      // silver
    'M': '#ff00ff',      // magenta
};
```

Each sprite frame is a string array where each string is a row of pixels:

```javascript
const NINJA_IDLE = [
    "....KKK.....",
    "...KDDDK....",
    "...KDWDK....",
    "...KKRKK....",
    "....KKK.....",
    // ... etc
];
```

### Collision Detection

- **AABB** (Axis-Aligned Bounding Box) for entity-entity collisions
- **Tile-based** collision for level geometry (each tile is 16x16 pixels)
- **Sweep test** for fast-moving projectiles to prevent tunneling
- **Separate hitboxes** for attack arcs (positioned relative to player facing direction)

### Performance Targets

- Maintain 60fps on mid-range mobile devices
- Maximum active entities: 20 enemies, 30 projectiles, 100 particles
- Off-screen entities deactivated (culling at camera bounds + 64px margin)
- Particle pooling to minimize garbage collection

### State Machine

```
TITLE -> PLAYING -> PAUSED -> PLAYING
                 -> DEATH -> PLAYING (respawn)
                 -> GAME_OVER -> TITLE
                 -> LEVEL_COMPLETE -> NEXT_LEVEL -> PLAYING
                                   -> VICTORY -> TITLE
```

### Data Persistence

- High scores saved to `localStorage` under key `shadowBlade_highScore`
- Current level progress not persisted (arcade-style, play from start)
