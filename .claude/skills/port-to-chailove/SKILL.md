---
name: port-to-chailove
description: Port a web game to ChaiLove (ChaiScript) for Miyoo Mini Plus
user_invocable: true
args: "<game-name>"
---

Port a web game from `web/<game>/index.html` to ChaiLove at `chailove/<game>/main.chai`.

Use a `game-builder` agent to:
1. Read the web version at `web/$ARGUMENTS/index.html` thoroughly
2. Read an existing ChaiLove port (e.g., `chailove/micro/main.chai`) for patterns
3. Write `chailove/$ARGUMENTS/main.chai` with the complete game

## ChaiLove API quick reference
```chaiscript
// Loop: def conf(t), def load(), def update(dt), def draw()
// Graphics: love.graphics.setColor(r,g,b,a), rectangle("fill",x,y,w,h), circle("fill",x,y,r)
//           line(x1,y1,x2,y2), point(x,y), print(text,x,y), clear(r,g,b)
//           draw(image,x,y,r,sx,sy), newImage(path), newFont(size), setFont(f)
// Input: love.keyboard.isDown("left"), love.joystick.isDown(0,"dpleft")
// Math: love.math.random(min,max)
// Resolution: 640x480 (set in conf)
```

## Sprite approach
Draw procedurally with 1×1 or 2×2 rectangles per pixel from string art data.

## After porting
Test with `/test-chailove $ARGUMENTS`.
