---
name: port-to-tic80
description: Port a web game to TIC-80 (Lua) for Miyoo Mini Plus
user_invocable: true
args: "<game-name>"
---

Port a web game from `web/<game>/index.html` to TIC-80 Lua at `tic80/<game>/<game>.lua`.

Use a `game-builder` agent to:
1. Read the web version at `web/$ARGUMENTS/index.html` thoroughly
2. Read an existing TIC-80 port (e.g., `tic80/micro/micro.lua`) for patterns
3. Write `tic80/$ARGUMENTS/$ARGUMENTS.lua` with the complete game
4. Build the .tic cartridge and optionally deploy

## TIC-80 Constraints
- Resolution: 240×136, 16 colors (Sweetie 16 palette)
- Code limit: 65536 chars
- Tile/sprite size: 8×8

## TIC-80 API quick reference
```lua
-- File header (required):
-- title: Game Name
-- author: retrogames
-- script: lua

function TIC() end           -- main loop, 60fps
function BOOT() end          -- called once at start

cls(color)                   -- clear screen
rect(x,y,w,h,color)         -- filled rect
rectb(x,y,w,h,color)        -- rect outline
circ(x,y,r,color)           -- filled circle
circb(x,y,r,color)          -- circle outline
line(x1,y1,x2,y2,color)     -- line
pix(x,y,color)              -- pixel
print(text,x,y,color,fixed,scale)

btn(0-7)                     -- held: 0=up 1=down 2=left 3=right 4=A 5=B 6=X 7=Y
btnp(0-7)                    -- just pressed
```

## Scale factors (800×600 web → 240×136 TIC-80)
- Tile: 20px → 8px (0.4×)
- Player: 16px → 6px
- Speed/gravity: multiply by ~0.4
- Map width: 32 cols → 30 cols

## After porting
Build and deploy with `/deploy-tic80 $ARGUMENTS --run`
