-- title: Nova Evader
-- author: retrogames
-- script: lua

-- ============================================================
-- NOVA EVADER - Bullet Hell Dodge
-- 240x136, Sweetie 16 palette
-- D-Pad to move, A(Z) to start/restart
-- ============================================================

-- Palette indices (Sweetie 16)
local C_BG      = 0   -- dark navy
local C_PURPLE  = 1   -- dark purple
local C_RED     = 2   -- red
local C_ORANGE  = 3   -- orange
local C_YELLOW  = 4   -- yellow
local C_LGREEN  = 5   -- light green
local C_GREEN   = 6   -- green
local C_TEAL    = 7   -- dark teal
local C_WHITE   = 8   -- white / near-white
local C_ORANGE2 = 9   -- bright orange
local C_GREEN2  = 10  -- bright green
local C_LBLUE   = 11  -- light blue (cyan-ish)
local C_MAGENTA = 12  -- magenta
local C_CYAN    = 13  -- teal/cyan
local C_DBLUE   = 14  -- dark blue-gray
local C_GRAY    = 15  -- light gray

-- Screen dimensions
local SW = 240
local SH = 136

-- ============================================================
-- GAME STATE
-- ============================================================
local STATE_TITLE    = 0
local STATE_STORY    = 1
local STATE_PLAY     = 2
local STATE_GAMEOVER = 3
local STATE_VICTORY  = 4

local state       = STATE_TITLE
local score       = 0
local level       = 1
local MAX_LEVEL   = 7
local t           = 0   -- global frame counter
local spawnTimer  = 0
local levelTimer  = 0
local storyTimer  = 0
local storyIdx    = 0
local blinkTimer  = 0
local flashTimer  = 0  -- screen flash on hit

-- ============================================================
-- STORY LINES
-- ============================================================
local storyLines = {
  { level=1, text="NOVA-7 REPORTING.\nANOMALY DETECTED." },
  { level=2, text="SECTOR COMPROMISED.\nEVADE THE SWARM." },
  { level=3, text="PATTERN INTENSIFIES.\nSTAY FOCUSED." },
  { level=4, text="ENEMY ADAPTS.\nPREDICT THE SPIRAL." },
  { level=5, text="CRITICAL MASS.\nDO NOT FALTER." },
  { level=6, text="FINAL WAVE INCOMING.\nSURVIVE OR PERISH." },
  { level=7, text="SYSTEM OVERLOAD.\nGIVE EVERYTHING." },
}

-- Current story text (for typewriter)
local storyText   = ""
local storyTarget = ""
local storyChar   = 0
local storyDelay  = 0

-- ============================================================
-- PLAYER
-- ============================================================
local px      = SW / 2
local py      = SH * 0.7
local PSPEED  = 1.8
local PRADIUS = 3
local trail   = {}  -- {x,y} ring buffer, max 8
local TRAIL_MAX = 8

-- ============================================================
-- BULLETS
-- Flat arrays for performance: bx[], by[], bvx[], bvy[],
-- brad[], bcol[], bactive[]
-- ============================================================
local MAX_BULLETS = 300
local bx      = {}
local by      = {}
local bvx     = {}
local bvy     = {}
local brad    = {}
local bcol    = {}
local bactive = {}
local bcount  = 0

-- Delayed bullet queue: {timer, x, y, vx, vy, col, r}
local delayed = {}

-- ============================================================
-- STARS (background parallax)
-- ============================================================
local STAR_COUNT = 40
local stars = {}
for i = 1, STAR_COUNT do
  stars[i] = {
    x  = math.random(0, SW-1),
    y  = math.random(0, SH-1),
    sp = (math.random(1,3)) * 0.3,
    c  = (i % 3 == 0) and C_LBLUE or (i % 3 == 1 and C_DBLUE or C_PURPLE),
  }
end

-- ============================================================
-- HELPERS
-- ============================================================
local function clamp(v, lo, hi)
  if v < lo then return lo end
  if v > hi then return hi end
  return v
end

local function addBullet(x, y, vx, vy, col, r)
  if bcount >= MAX_BULLETS then return end
  bcount = bcount + 1
  bx[bcount]      = x
  by[bcount]      = y
  bvx[bcount]     = vx
  bvy[bcount]     = vy
  brad[bcount]    = r or 1.5
  bcol[bcount]    = col or C_RED
  bactive[bcount] = true
end

local function clearBullets()
  for i = 1, bcount do
    bactive[i] = false
  end
  bcount = 0
  delayed = {}
end

-- ============================================================
-- BULLET PATTERNS
-- ============================================================
local function patternRain()
  local count = math.min(4 + level, 14)
  for i = 1, count do
    addBullet(
      math.random(4, SW-4), -6,
      (math.random() - 0.5) * 0.8,
      1.4 + math.random() * 1.2,
      C_RED, 1.5
    )
  end
end

local function patternRadial()
  local cx = math.random(20, SW-20)
  local count = math.min(8 + level * 2, 20)
  for i = 1, count do
    local a = (i / count) * math.pi * 2
    addBullet(cx, -6,
      math.cos(a) * 1.8,
      math.sin(a) * 1.8 + 0.8,
      C_ORANGE2, 1.5
    )
  end
end

local function patternAimed()
  local tx = math.random(10, SW-10)
  local ang = math.atan2(py - (-6), px - tx)
  local ca = math.cos(ang)
  local sa = math.sin(ang)
  -- 3 bullets staggered
  for i = 0, 2 do
    delayed[#delayed+1] = {
      timer = i * 18,  -- 18 frames = ~0.3s
      x=tx, y=-6, vx=ca*3.0, vy=sa*3.0,
      col=C_GREEN2, r=2.0
    }
  end
end

local function patternSideSwipe()
  -- Waves of bullets from both sides
  local waves = math.min(2 + math.floor(level/2), 5)
  for w = 0, waves-1 do
    delayed[#delayed+1] = {
      timer = w * 12,
      x=-4, y=math.random(10, SH-20), vx=2.5, vy=0,
      col=C_MAGENTA, r=2.0
    }
    delayed[#delayed+1] = {
      timer = w * 12,
      x=SW+4, y=math.random(10, SH-20), vx=-2.5, vy=0,
      col=C_MAGENTA, r=2.0
    }
  end
end

local function patternSpiral()
  local sx = SW / 2
  local shots = math.min(10 + level, 18)
  for i = 0, shots-1 do
    local a = i * 0.45
    delayed[#delayed+1] = {
      timer = i * 5,
      x=sx, y=-4,
      vx=math.cos(a) * 2.8,
      vy=math.sin(a) * 1.0 + 1.8,
      col=C_WHITE, r=1.5
    }
  end
end

local function patternCross()
  -- 4-way burst from center top
  local cx = SW / 2
  local count = math.min(6 + level, 16)
  for i = 1, count do
    local a = (i/count) * math.pi  -- half circle downward
    addBullet(cx, -4, math.cos(a)*2.5, math.sin(a)*2.5+0.5, C_CYAN, 1.5)
  end
end

local function patternWall()
  -- Vertical wall of bullets with a gap
  local gap   = math.random(30, SW-30)
  local gsize = math.max(22 - level*2, 10)
  for x = 0, SW, 8 do
    if x < gap - gsize or x > gap + gsize then
      addBullet(x, -4, 0, 1.6 + level * 0.1, C_YELLOW, 1.5)
    end
  end
end

local patterns = {
  patternRain, patternRadial, patternAimed,
  patternSideSwipe, patternSpiral, patternCross, patternWall
}

local function spawnPattern()
  -- Unlock patterns as level rises; always allow first 2
  local maxPat = math.min(2 + math.floor(level * 0.7), #patterns)
  local pick = math.random(1, maxPat)
  patterns[pick]()
end

-- ============================================================
-- INIT / RESET
-- ============================================================
local function initGame()
  score      = 0
  level      = 1
  spawnTimer = 0
  levelTimer = 0
  flashTimer = 0
  t          = 0
  px         = SW / 2
  py         = SH * 0.7
  trail      = {}
  clearBullets()
  state      = STATE_PLAY
end

local function showStory(lvl)
  for _, s in ipairs(storyLines) do
    if s.level == lvl then
      storyTarget = s.text
      storyChar   = 0
      storyDelay  = 0
      storyIdx    = lvl
      state       = STATE_STORY
      storyTimer  = 0
      return
    end
  end
  -- No story for this level, go directly to play
  state = STATE_PLAY
end

-- ============================================================
-- UPDATE FUNCTIONS
-- ============================================================
local function updateStars()
  for i = 1, STAR_COUNT do
    stars[i].y = stars[i].y + stars[i].sp
    if stars[i].y >= SH then
      stars[i].y = 0
      stars[i].x = math.random(0, SW-1)
    end
  end
end

local function updatePlayer()
  local dx, dy = 0, 0
  if btn(0) then dy = dy - 1 end
  if btn(1) then dy = dy + 1 end
  if btn(2) then dx = dx - 1 end
  if btn(3) then dx = dx + 1 end

  -- Diagonal normalization
  if dx ~= 0 and dy ~= 0 then
    dx = dx * 0.707
    dy = dy * 0.707
  end

  px = px + dx * PSPEED
  py = py + dy * PSPEED
  px = clamp(px, PRADIUS + 1, SW - PRADIUS - 1)
  py = clamp(py, PRADIUS + 1, SH - PRADIUS - 1)

  -- Trail
  table.insert(trail, 1, {x=px, y=py})
  if #trail > TRAIL_MAX then
    table.remove(trail)
  end
end

local function updateDelayed()
  local i = 1
  while i <= #delayed do
    local d = delayed[i]
    d.timer = d.timer - 1
    if d.timer <= 0 then
      addBullet(d.x, d.y, d.vx, d.vy, d.col, d.r)
      table.remove(delayed, i)
    else
      i = i + 1
    end
  end
end

local function updateBullets()
  local OOB = 12
  local writeIdx = 0
  for i = 1, bcount do
    if bactive[i] then
      bx[i] = bx[i] + bvx[i]
      by[i] = by[i] + bvy[i]

      -- OOB cull
      if bx[i] < -OOB or bx[i] > SW+OOB or
         by[i] < -OOB or by[i] > SH+OOB then
        bactive[i] = false
      else
        -- Collision with player (circle vs hitbox radius 2)
        local ddx = bx[i] - px
        local ddy = by[i] - py
        local dist2 = ddx*ddx + ddy*ddy
        local cr = (brad[i] + 2)
        if dist2 < cr * cr then
          -- Hit!
          state     = STATE_GAMEOVER
          flashTimer = 20
          return
        end
      end
    end
  end

  -- Compact active bullets
  writeIdx = 0
  for i = 1, bcount do
    if bactive[i] then
      writeIdx = writeIdx + 1
      if writeIdx ~= i then
        bx[writeIdx]      = bx[i]
        by[writeIdx]      = by[i]
        bvx[writeIdx]     = bvx[i]
        bvy[writeIdx]     = bvy[i]
        brad[writeIdx]    = brad[i]
        bcol[writeIdx]    = bcol[i]
        bactive[writeIdx] = true
      end
    end
  end
  bcount = writeIdx
end

-- ============================================================
-- DRAW FUNCTIONS
-- ============================================================
local function drawStars()
  for i = 1, STAR_COUNT do
    pix(math.floor(stars[i].x), math.floor(stars[i].y), stars[i].c)
  end
end

local function drawPlayer()
  -- Trail
  for i = 2, #trail do
    local alpha_c = (i <= 4) and C_LBLUE or C_DBLUE
    pix(math.floor(trail[i].x), math.floor(trail[i].y), alpha_c)
  end

  -- Ship triangle (pointing up)
  local x, y = math.floor(px), math.floor(py)
  -- Body: 5 wide, 6 tall triangle
  -- Use lines for triangle
  line(x,     y-5,  x+4,  y+3,  C_LBLUE)  -- right edge
  line(x,     y-5,  x-4,  y+3,  C_LBLUE)  -- left edge
  line(x-4,   y+3,  x+4,  y+3,  C_LBLUE)  -- base
  -- Fill center column
  pix(x,   y-4, C_WHITE)
  pix(x,   y-3, C_WHITE)
  pix(x-1, y-2, C_LBLUE)
  pix(x,   y-2, C_WHITE)
  pix(x+1, y-2, C_LBLUE)
  pix(x-1, y-1, C_LBLUE)
  pix(x,   y-1, C_CYAN)
  pix(x+1, y-1, C_LBLUE)
  pix(x-2, y,   C_LBLUE)
  pix(x-1, y,   C_CYAN)
  pix(x,   y,   C_WHITE)
  pix(x+1, y,   C_CYAN)
  pix(x+2, y,   C_LBLUE)
  -- Engine glow (flicker)
  if t % 4 < 2 then
    pix(x-1, y+2, C_ORANGE2)
    pix(x,   y+2, C_YELLOW)
    pix(x+1, y+2, C_ORANGE2)
  else
    pix(x,   y+2, C_ORANGE)
  end
end

local function drawBullets()
  for i = 1, bcount do
    local bxi = math.floor(bx[i])
    local byi = math.floor(by[i])
    local r   = brad[i]
    local c   = bcol[i]
    if r <= 1.5 then
      pix(bxi, byi, c)
      -- Cross highlight
      pix(bxi-1, byi,   c)
      pix(bxi+1, byi,   c)
      pix(bxi,   byi-1, c)
      pix(bxi,   byi+1, c)
    else
      -- Larger bullet: small filled circle via rect
      circ(bxi, byi, math.floor(r), c)
    end
  end
end

local function drawHUD()
  -- Top bar background
  rect(0, 0, SW, 8, C_BG)
  -- Score
  print("SC:"..math.floor(score), 2, 1, C_LBLUE, false, 1)
  -- Level
  local lvlStr = "LV:"..level
  print(lvlStr, SW - #lvlStr*6 - 2, 1, C_YELLOW, false, 1)
  -- Bullet count (debug feel)
  local bcStr = "B:"..bcount
  print(bcStr, SW/2 - #bcStr*3, 1, C_GRAY, false, 1)
end

-- ============================================================
-- TITLE SCREEN
-- ============================================================
local function drawTitle()
  cls(C_BG)
  drawStars()

  -- Animated star bursts
  local cx, cy = SW/2, SH/2 - 20
  for i = 1, 8 do
    local a = (i/8)*math.pi*2 + t*0.02
    local r = 18 + math.sin(t*0.05 + i)*4
    local ex = cx + math.cos(a)*r
    local ey = cy + math.sin(a)*r
    line(cx, cy, math.floor(ex), math.floor(ey), (i%3==0) and C_YELLOW or C_ORANGE2)
  end
  circ(cx, cy, 6, C_LBLUE)
  circ(cx, cy, 4, C_CYAN)
  circ(cx, cy, 2, C_WHITE)

  -- Title
  print("NOVA EVADER", 62, SH/2, C_LBLUE, false, 2)
  print("NOVA EVADER", 61, SH/2-1, C_WHITE, false, 2)

  -- Subtitle
  print("BULLET HELL SURVIVAL", 40, SH/2 + 17, C_CYAN, false, 1)

  -- Blink prompt
  blinkTimer = blinkTimer + 1
  if (blinkTimer // 30) % 2 == 0 then
    print("PRESS A OR Z TO START", 39, SH/2 + 28, C_YELLOW, false, 1)
  end

  -- Controls
  print("D-PAD: MOVE", 2, SH-10, C_GRAY, false, 1)
  print("DODGE ALL BULLETS", 90, SH-10, C_GRAY, false, 1)
end

-- ============================================================
-- STORY SCREEN
-- ============================================================
local function updateStory()
  storyTimer = storyTimer + 1
  -- Typewriter effect
  storyDelay = storyDelay + 1
  if storyDelay >= 3 and storyChar < #storyTarget then
    storyChar  = storyChar + 1
    storyDelay = 0
    storyText  = string.sub(storyTarget, 1, storyChar)
  end

  -- Auto-advance or skip with A
  local done = storyChar >= #storyTarget
  if done and storyTimer > 120 then
    state      = STATE_PLAY
    spawnTimer = 0
    levelTimer = 0
  end
  if btnp(4) or btnp(5) then
    if not done then
      storyChar = #storyTarget
      storyText = storyTarget
    else
      state      = STATE_PLAY
      spawnTimer = 0
      levelTimer = 0
    end
  end
end

local function drawStory()
  cls(C_BG)
  drawStars()

  -- Panel
  local pw, ph = 180, 60
  local px2 = (SW - pw) // 2
  local py2 = (SH - ph) // 2
  rect(px2, py2, pw, ph, C_PURPLE)
  rectb(px2, py2, pw, ph, C_LBLUE)
  rectb(px2+1, py2+1, pw-2, ph-2, C_DBLUE)

  -- Level badge
  local badge = "LEVEL "..storyIdx
  print(badge, SW/2 - #badge*3, py2 + 4, C_YELLOW, false, 1)

  -- Story text with newline handling
  local line1 = storyText
  local line2 = ""
  local nl = string.find(storyText, "\n")
  if nl then
    line1 = string.sub(storyText, 1, nl-1)
    line2 = string.sub(storyText, nl+1)
  end
  print(line1, px2 + 8, py2 + 18, C_WHITE, false, 1)
  if line2 ~= "" then
    print(line2, px2 + 8, py2 + 28, C_LBLUE, false, 1)
  end

  -- Continue hint
  if storyChar >= #storyTarget then
    if (t // 20) % 2 == 0 then
      print("PRESS A TO CONTINUE", px2 + 16, py2 + ph - 12, C_CYAN, false, 1)
    end
  end
end

-- ============================================================
-- GAMEPLAY
-- ============================================================
local function updatePlay()
  -- Score increments over time
  score = score + 0.6

  -- Level up timer
  levelTimer = levelTimer + 1
  if levelTimer >= 600 then  -- 10 seconds at 60fps
    levelTimer = 0
    if level < MAX_LEVEL then
      level = level + 1
      showStory(level)
      return
    else
      -- Max level reached: victory
      state = STATE_VICTORY
      return
    end
  end

  -- Spawn bullets
  spawnTimer = spawnTimer + 1
  local spawnInterval = math.max(90 - level * 8, 28)
  if spawnTimer >= spawnInterval then
    spawnTimer = 0
    spawnPattern()
  end

  updatePlayer()
  updateDelayed()
  updateBullets()

  -- Bonus score for survival time
  if levelTimer % 60 == 0 then
    score = score + level * 10
  end
end

local function drawPlay()
  -- Background with motion blur trail feel
  cls(C_BG)
  drawStars()

  -- Play area border (subtle)
  rectb(0, 8, SW, SH-8, C_DBLUE)

  drawBullets()
  drawPlayer()
  drawHUD()

  -- Flash on hit
  if flashTimer > 0 then
    flashTimer = flashTimer - 1
    if flashTimer % 4 < 2 then
      rectb(1, 9, SW-2, SH-10, C_RED)
    end
  end
end

-- ============================================================
-- GAME OVER
-- ============================================================
local function drawGameOver()
  cls(C_BG)
  drawStars()

  -- Remaining bullets still visible
  drawBullets()

  -- Panel
  local pw, ph = 160, 70
  local gx = (SW - pw) // 2
  local gy = (SH - ph) // 2
  rect(gx, gy, pw, ph, C_PURPLE)
  rectb(gx, gy, pw, ph, C_RED)

  print("SYSTEM CRITICAL", gx + 12, gy + 6, C_RED, false, 1)
  print("SHIP DESTROYED", gx + 14, gy + 16, C_ORANGE2, false, 1)

  local sc = "SCORE: "..math.floor(score)
  print(sc, gx + (pw - #sc*6)//2, gy + 30, C_WHITE, false, 1)

  local lv = "LEVEL: "..level
  print(lv, gx + (pw - #lv*6)//2, gy + 40, C_CYAN, false, 1)

  blinkTimer = blinkTimer + 1
  if (blinkTimer // 25) % 2 == 0 then
    print("A/Z: REBOOT SHIP", gx + 16, gy + 54, C_YELLOW, false, 1)
  end
end

-- ============================================================
-- VICTORY SCREEN
-- ============================================================
local function drawVictory()
  cls(C_BG)

  -- Celebration particles using stars
  for i = 1, STAR_COUNT do
    local sx = (stars[i].x + t * stars[i].sp * 2) % SW
    local sy = (stars[i].y + t * 0.5) % SH
    local cc = (i % 5 == 0) and C_YELLOW or (i % 5 == 1 and C_ORANGE2 or
               (i % 5 == 2 and C_GREEN2 or (i % 5 == 3 and C_CYAN or C_MAGENTA)))
    pix(math.floor(sx), math.floor(sy), cc)
  end

  -- Title
  print("MISSION COMPLETE", 44, 20, C_YELLOW, false, 2)

  print("ALL WAVES SURVIVED", 40, 56, C_WHITE, false, 1)

  local sc = "FINAL SCORE: "..math.floor(score)
  print(sc, SW/2 - #sc*3, 70, C_CYAN, false, 1)

  blinkTimer = blinkTimer + 1
  if (blinkTimer // 25) % 2 == 0 then
    print("A/Z: PLAY AGAIN", 64, 90, C_GREEN2, false, 1)
  end

  print("NOVA-7 RETURNS HOME.", 38, SH-12, C_GRAY, false, 1)
end

-- ============================================================
-- MAIN TIC LOOP
-- ============================================================
function TIC()
  t = t + 1
  updateStars()

  if state == STATE_TITLE then
    drawTitle()
    if btnp(4) or btnp(5) then
      showStory(1)
    end

  elseif state == STATE_STORY then
    updateStory()
    drawStory()

  elseif state == STATE_PLAY then
    updatePlay()
    if state == STATE_PLAY then
      drawPlay()
    end

  elseif state == STATE_GAMEOVER then
    drawGameOver()
    if btnp(4) or btnp(5) then
      initGame()
      showStory(1)
    end

  elseif state == STATE_VICTORY then
    drawVictory()
    if btnp(4) or btnp(5) then
      initGame()
      showStory(1)
    end
  end
end
