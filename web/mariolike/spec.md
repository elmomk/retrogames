# Pixel Knight - The Crystal Kingdom

## Genre
Side-scrolling action platformer (Mario-like)

## Story
An evil sorcerer named Malachar has shattered the realm's Crystal Heart into fragments, plunging the kingdom into eternal twilight. The player is the last Pixel Knight, armed with a magic sword, who must journey through corrupted lands to reclaim the crystal shards and restore the light.

## Technical Spec

### Engine
- Single-file HTML5 Canvas game (640x480)
- 60 FPS fixed timestep with death spiral prevention (dt capped at 250ms)
- Press Start 2P Google Font
- CRT scanline overlay + vignette

### Sprite System
- Procedural pixel-art sprites as character arrays (8x8 and 16x16)
- `createSprite()` renders to offscreen canvas
- Hex digit indexing into color palette

### Controls
- **Keyboard**: Arrow keys (move), Space/Z (jump), X (attack/fireball)
- **Touch**: Virtual joystick (left half screen) + B button (jump) + A button (attack)
- Touch controls hidden on desktop via `@media (hover: hover)`

### Physics
- Gravity: 0.42 (0.22 when holding jump ascending)
- Jump force: -8.2
- Move speed: 3.0 (with acceleration 0.35 / deceleration 0.25)
- Max fall speed: 9
- Coyote time: 6 frames
- Jump buffer: 6 frames
- Stomp bounce: -6.5

### Player States
- Normal (small, 14x16)
- Big (powered up, takes extra hit, 14x24 drawn at 2x)
- Fire (can shoot fireballs, lost on hit before big)
- Invincibility frames: 90 frames after hit (flicker effect)

### Enemies
- **Slime**: Walks back and forth on platforms, reverses at walls/edges
- **Bat**: Flies in sine wave pattern, reverses at walls
- **Dark Knight Boss** (Level 3): 12 HP, paces and charges at player

### Power-ups
- **Crystal Shard**: Grow to big form (extra hit point)
- **Fire Crystal**: Shoot fireballs (X key)
- **Goal Crystal**: Level completion trigger

### Levels
1. **Crystal Meadows** - Open outdoor, basic platforms, slimes, introductory
2. **Shadow Caverns** - Underground, bats, spikes, breakable blocks, fire crystal
3. **The Dark Tower** - Boss level, Dark Knight boss fight

### Level Design
- Tile-based (32px tiles), character map parsed at load
- `normalizeMaps()` pads all rows to consistent width
- Tiles: # (solid), K (breakable), ^ (spike), @ (player start), G (goal), X (boss), C (crystal), S (slime), B (bat), P (power-up shard), F (fire crystal)

### Camera
- Horizontal follow with smooth lerp (0.1 factor)
- Clamped to level boundaries

### Sound Effects (Web Audio API)
- jump: square wave pitch up
- stomp: square wave crunch + noise burst
- coin: 3-note ascending chord
- powerup: 4-note ascending arpeggio
- hit: low thud
- death: dramatic pitch drop
- fireball: sawtooth swoosh
- break: noise burst
- boss_hit: sawtooth low hit
- victory: 7-note fanfare

### Visual Effects
- Screen shake (0.85 decay) on stomp, damage, boss hits
- Particle explosions on enemy death
- Floating score popups (+100, +200, +2000)
- Damage flash (red overlay)
- Invincibility flicker
- Crystal sparkle and float animation
- Running trail particles
- Ambient floating dust
- Parallax background mountains

### Screen States
- `START`: Title screen with animated crystals, blinking "PRESS START"
- `LEVEL_STORY`: Typewriter text reveal (1 char every 2 frames), skippable
- `PLAYING`: Gameplay with HUD (lives, score, coins, level name, power-up indicators)
- `GAMEOVER`: Red overlay, final score, retry prompt
- `WIN`: Victory screen with celebratory particles

### Lives System
- 3 lives at start
- Death on: spike contact, falling in pit, enemy contact (when small/no powerup)
- Game over when all lives lost
