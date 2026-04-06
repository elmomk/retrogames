-- title: Nano Wizards
-- author: retrogames
-- desc: The Obsidian Spire
-- script: lua

-- ============================================================
-- CONSTANTS
-- ============================================================
local SW,SH=240,136
local TS=8          -- tile size
local COLS=30       -- map columns (30*8=240)
local GRAVITY=0.15
local MAX_FALL=3.0
local WALL_SLIDE=0.7
local MOVE_SPEED=1.5
local JUMP_FORCE=-3.0
local WALL_JUMP_Y=-2.8
local WALL_JUMP_X=2.5
local BOUNCE_FORCE=-2.5
local BULLET_SPEED=4.0
local EBULLET_SPEED=1.5
local ANCHOR_SPEED=6.0
local CLIMB_SPEED=1.2
local MAX_BULLETS=12
local MAX_EBULLETS=16
local MAX_ENEMIES=24
local MAX_GEMS=24
local MAX_PARTICLES=40
local MAX_POPUPS=12

-- ============================================================
-- PALETTE (Sweetie 16)
-- 0=dark navy, 1=dark purple, 2=red, 3=light orange
-- 4=yellow, 5=light green, 6=green, 7=dark teal
-- 8=white, 9=orange, 10=green2, 11=light blue
-- 12=magenta, 13=teal, 14=dark blue-gray, 15=med gray
-- ============================================================

-- ============================================================
-- SPRITE DRAWING HELPERS
-- ============================================================
local function spr_pixel(x,y,c)
  if c~=0 then pix(x,y,c) end
end

-- Draw player (6x8) wizard sprite
local function draw_player(x,y,flip)
  x=math.floor(x) y=math.floor(y)
  -- hat
  local hc=11 -- light blue hat
  local bc=8  -- white body
  local rc=2  -- red gem
  if flip then
    -- mirrored
    spr_pixel(x+2,y,hc) spr_pixel(x+3,y,hc) spr_pixel(x+4,y,hc) spr_pixel(x+5,y,hc)
    spr_pixel(x+1,y+1,hc) spr_pixel(x+2,y+1,hc) spr_pixel(x+3,y+1,hc) spr_pixel(x+4,y+1,hc) spr_pixel(x+5,y+1,hc)
    -- body
    spr_pixel(x+1,y+2,bc) spr_pixel(x+2,y+2,rc) spr_pixel(x+3,y+2,bc) spr_pixel(x+4,y+2,bc) spr_pixel(x+5,y+2,bc)
    spr_pixel(x+1,y+3,bc) spr_pixel(x+2,y+3,bc) spr_pixel(x+3,y+3,bc) spr_pixel(x+4,y+3,bc) spr_pixel(x+5,y+3,bc)
    spr_pixel(x+1,y+4,bc) spr_pixel(x+2,y+4,bc) spr_pixel(x+3,y+4,9) spr_pixel(x+4,y+4,bc) spr_pixel(x+5,y+4,bc)
    -- legs
    spr_pixel(x+1,y+5,14) spr_pixel(x+2,y+5,14) spr_pixel(x+4,y+5,14) spr_pixel(x+5,y+5,14)
    spr_pixel(x+1,y+6,14) spr_pixel(x+5,y+6,14)
    spr_pixel(x+1,y+7,bc) spr_pixel(x+2,y+7,bc) spr_pixel(x+4,y+7,bc) spr_pixel(x+5,y+7,bc)
  else
    spr_pixel(x,y,hc) spr_pixel(x+1,y,hc) spr_pixel(x+2,y,hc) spr_pixel(x+3,y,hc)
    spr_pixel(x,y+1,hc) spr_pixel(x+1,y+1,hc) spr_pixel(x+2,y+1,hc) spr_pixel(x+3,y+1,hc) spr_pixel(x+4,y+1,hc)
    spr_pixel(x,y+2,bc) spr_pixel(x+1,y+2,bc) spr_pixel(x+2,y+2,bc) spr_pixel(x+3,y+2,rc) spr_pixel(x+4,y+2,bc)
    spr_pixel(x,y+3,bc) spr_pixel(x+1,y+3,bc) spr_pixel(x+2,y+3,bc) spr_pixel(x+3,y+3,bc) spr_pixel(x+4,y+3,bc)
    spr_pixel(x,y+4,bc) spr_pixel(x+1,y+4,bc) spr_pixel(x+2,y+4,9) spr_pixel(x+3,y+4,bc) spr_pixel(x+4,y+4,bc)
    spr_pixel(x,y+5,14) spr_pixel(x+1,y+5,14) spr_pixel(x+3,y+5,14) spr_pixel(x+4,y+5,14)
    spr_pixel(x,y+6,14) spr_pixel(x+4,y+6,14)
    spr_pixel(x,y+7,bc) spr_pixel(x+1,y+7,bc) spr_pixel(x+3,y+7,bc) spr_pixel(x+4,y+7,bc)
  end
end

-- Draw patrol enemy (5x6)
local function draw_patrol(x,y,flip)
  x=math.floor(x) y=math.floor(y)
  local c=12 -- magenta
  if flip then
    spr_pixel(x+1,y,c) spr_pixel(x+2,y,c) spr_pixel(x+3,y,c)
    spr_pixel(x,y+1,c) spr_pixel(x+1,y+1,8) spr_pixel(x+2,y+1,8) spr_pixel(x+3,y+1,c) spr_pixel(x+4,y+1,c)
    spr_pixel(x,y+2,c) spr_pixel(x+1,y+2,c) spr_pixel(x+2,y+2,8) spr_pixel(x+3,y+2,c) spr_pixel(x+4,y+2,c)
    spr_pixel(x,y+3,c) spr_pixel(x+1,y+3,c) spr_pixel(x+2,y+3,c) spr_pixel(x+3,y+3,c) spr_pixel(x+4,y+3,c)
    spr_pixel(x,y+4,c) spr_pixel(x+2,y+4,c) spr_pixel(x+4,y+4,c)
    spr_pixel(x,y+5,14) spr_pixel(x+2,y+5,14) spr_pixel(x+4,y+5,14)
  else
    spr_pixel(x+1,y,c) spr_pixel(x+2,y,c) spr_pixel(x+3,y,c)
    spr_pixel(x,y+1,c) spr_pixel(x+1,y+1,c) spr_pixel(x+2,y+1,8) spr_pixel(x+3,y+1,8) spr_pixel(x+4,y+1,c)
    spr_pixel(x,y+2,c) spr_pixel(x+1,y+2,c) spr_pixel(x+2,y+2,8) spr_pixel(x+3,y+2,c) spr_pixel(x+4,y+2,c)
    spr_pixel(x,y+3,c) spr_pixel(x+1,y+3,c) spr_pixel(x+2,y+3,c) spr_pixel(x+3,y+3,c) spr_pixel(x+4,y+3,c)
    spr_pixel(x,y+4,c) spr_pixel(x+2,y+4,c) spr_pixel(x+4,y+4,c)
    spr_pixel(x,y+5,14) spr_pixel(x+2,y+5,14) spr_pixel(x+4,y+5,14)
  end
end

-- Draw bat enemy (5x5)
local function draw_bat(x,y,t)
  x=math.floor(x) y=math.floor(y)
  local c=2 -- red
  local wing=(math.floor(t/6)%2==0)
  -- body
  spr_pixel(x+2,y+1,c) spr_pixel(x+2,y+2,c) spr_pixel(x+1,y+2,c) spr_pixel(x+3,y+2,c)
  spr_pixel(x+2,y+3,c)
  -- wings
  if wing then
    spr_pixel(x,y+1,c) spr_pixel(x+1,y+1,c)
    spr_pixel(x+3,y+1,c) spr_pixel(x+4,y+1,c)
    spr_pixel(x,y+2,c) spr_pixel(x+4,y+2,c)
  else
    spr_pixel(x,y+2,c) spr_pixel(x+1,y+2,c)
    spr_pixel(x+3,y+2,c) spr_pixel(x+4,y+2,c)
    spr_pixel(x,y+3,c) spr_pixel(x+4,y+3,c)
  end
  -- eyes
  spr_pixel(x+1,y+2,8) spr_pixel(x+3,y+2,8)
end

-- Draw turret enemy (5x6)
local function draw_turret(x,y,t)
  x=math.floor(x) y=math.floor(y)
  local c=10 -- green
  local blink=(math.floor(t/15)%2==0)
  spr_pixel(x+1,y,c) spr_pixel(x+2,y,c) spr_pixel(x+3,y,c)
  spr_pixel(x,y+1,c) spr_pixel(x+1,y+1,c) spr_pixel(x+2,y+1,c) spr_pixel(x+3,y+1,c) spr_pixel(x+4,y+1,c)
  spr_pixel(x,y+2,c) spr_pixel(x+1,y+2,c) spr_pixel(x+2,y+2,blink and 2 or 8) spr_pixel(x+3,y+2,c) spr_pixel(x+4,y+2,c)
  spr_pixel(x,y+3,14) spr_pixel(x+1,y+3,c) spr_pixel(x+2,y+3,c) spr_pixel(x+3,y+3,c) spr_pixel(x+4,y+3,14)
  spr_pixel(x,y+4,14) spr_pixel(x+1,y+4,14) spr_pixel(x+2,y+4,14) spr_pixel(x+3,y+4,14) spr_pixel(x+4,y+4,14)
  spr_pixel(x,y+5,14) spr_pixel(x+2,y+5,14) spr_pixel(x+4,y+5,14)
end

-- Draw gem (4x4)
local function draw_gem(x,y,t)
  x=math.floor(x) y=math.floor(y)
  local c=(math.floor(t/8)%2==0) and 11 or 4
  spr_pixel(x+1,y,c) spr_pixel(x+2,y,c)
  spr_pixel(x,y+1,c) spr_pixel(x+1,y+1,8) spr_pixel(x+2,y+1,c) spr_pixel(x+3,y+1,c)
  spr_pixel(x,y+2,c) spr_pixel(x+1,y+2,c) spr_pixel(x+2,y+2,8) spr_pixel(x+3,y+2,c)
  spr_pixel(x+1,y+3,c) spr_pixel(x+2,y+3,c)
end

-- Draw goal portal (6x6)
local function draw_goal(x,y,t)
  x=math.floor(x) y=math.floor(y)
  local c=(math.floor(t/4)%3)
  local cols={4,9,8}
  local cc=cols[c+1]
  spr_pixel(x+2,y,cc) spr_pixel(x+3,y,cc)
  spr_pixel(x+1,y+1,cc) spr_pixel(x+2,y+1,cc) spr_pixel(x+3,y+1,cc) spr_pixel(x+4,y+1,cc)
  spr_pixel(x,y+2,cc) spr_pixel(x+1,y+2,cc) spr_pixel(x+2,y+2,8) spr_pixel(x+3,y+2,8) spr_pixel(x+4,y+2,cc) spr_pixel(x+5,y+2,cc)
  spr_pixel(x,y+3,cc) spr_pixel(x+1,y+3,cc) spr_pixel(x+2,y+3,8) spr_pixel(x+3,y+3,8) spr_pixel(x+4,y+3,cc) spr_pixel(x+5,y+3,cc)
  spr_pixel(x+1,y+4,cc) spr_pixel(x+2,y+4,cc) spr_pixel(x+3,y+4,cc) spr_pixel(x+4,y+4,cc)
  spr_pixel(x+2,y+5,cc) spr_pixel(x+3,y+5,cc)
end

-- Draw anchor head (3x3)
local function draw_anchor(x,y)
  x=math.floor(x) y=math.floor(y)
  spr_pixel(x+1,y,8)
  spr_pixel(x,y+1,8) spr_pixel(x+1,y+1,15) spr_pixel(x+2,y+1,8)
  spr_pixel(x+1,y+2,8)
end

-- Draw tile (8x8)
local function draw_tile(x,y,tp)
  x=math.floor(x) y=math.floor(y)
  if tp==1 then -- brick
    rect(x,y,8,8,14)
    -- mortar lines
    line(x,y+3,x+7,y+3,0)
    line(x,y+7,x+7,y+7,0)
    line(x+3,y,x+3,y+2,0)
    line(x+7,y,x+7,y+2,0)
    line(x+1,y+4,x+1,y+6,0)
    line(x+5,y+4,x+5,y+6,0)
  elseif tp==2 then -- breakable stone
    rect(x,y,8,8,15)
    pix(x,y,14) pix(x+7,y,14) pix(x,y+7,14) pix(x+7,y+7,14)
    pix(x+3,y+2,14) pix(x+4,y+5,14) pix(x+2,y+4,14)
  elseif tp==3 then -- chest
    rect(x,y,8,8,9)
    rect(x+1,y+3,6,1,4)
    pix(x+3,y+4,4) pix(x+4,y+4,4)
    rectb(x,y,8,8,3)
  end
end

-- ============================================================
-- LEVEL DATA (30 chars wide)
-- ============================================================
local MAPS={
  -- LEVEL 1: THE OVERGROWN DEPTHS
  {
    name="OVERGROWN DEPTHS",
    lava_speed=0.08,
    rows={
      "##############################",
      "#............................#",
      "#............X...............#",
      "#............####............#",
      "#............................#",
      "#............................#",
      "#................C...........#",
      "#....#####......######.......#",
      "#............................#",
      "#..........####..............#",
      "#............................#",
      "####################%%########",  -- note: 30 chars
      "####################%%########",
      "#............................#",
      "#............................#",
      "#................P...........#",
      "#..........#########.........#",
      "#............................#",
      "#............................#",
      "#............................#",
      "#.....######.................#",
      "#............................#",
      "#............................#",
      "#............................#",
      "#............................#",
      "#...####.....................#",
      "#............................#",
      "#............................#",
      "#............................#",
      "#............................#",
      "#............................#",
      "##############################",
    },
  },
  -- LEVEL 2: THE FROZEN ARCHIVE
  {
    name="FROZEN ARCHIVE",
    lava_speed=0.18,
    rows={
      "##############################",
      "#............................#",
      "#............X...............#",
      "#............####............#",
      "#............................#",
      "#....####..........####......#",
      "#%%...............B.......%%%#",
      "#%%......................%%%%#",
      "#............####............#",
      "#......##.............##.....#",
      "#........T.......C...........#",
      "#............####............#",
      "#.......G....................#",
      "#......####..........####....#",
      "##############################",
      "#%%......................%%%%#",
      "#%%.....C......B.........%%%#",
      "#%%......................%%%%#",
      "#..........######............#",
      "#.........P..........P.......#",
      "#............................#",
      "#....####..............####..#",
      "#........................G...#",
      "#%%......................%%%%#",
      "#%%..................C....%%%#",
      "#%%......................%%%%#",
      "#..........######............#",
      "#................T...........#",
      "#............................#",
      "#..........P.................#",
      "#............................#",
      "##############################",
    },
  },
  -- LEVEL 3: THE LIVING CORE
  {
    name="LIVING CORE",
    lava_speed=0.3,
    rows={
      "##############################",
      "#............................#",
      "#............X...............#",
      "#............####............#",
      "#............................#",
      "#........B............B......#",
      "#..####........C.......####..#",
      "#............................#",
      "#........T............T......#",
      "#......######.....######.....#",
      "#............................#",
      "#..G...C.......P.......C...G.#",
      "#...######..........######...#",
      "#............................#",
      "#%%......................%%%%#",
      "#%%.........B............%%%#",
      "#%%......................%%%%#",
      "#..........######............#",
      "#............................#",
      "#............................#",
      "#....#####............####...#",
      "#..........G......G..........#",
      "#...T......########......T...#",
      "#............................#",
      "#.........P..........P.......#",
      "#............................#",
      "##############################",
    },
  },
}

-- Fix map rows to exactly 30 chars (pad/trim)
for _,lv in ipairs(MAPS) do
  for i,row in ipairs(lv.rows) do
    if #row<30 then
      lv.rows[i]=row..string.rep(".",30-#row)
    elseif #row>30 then
      lv.rows[i]=row:sub(1,30)
    end
  end
end

-- ============================================================
-- STORY DATA
-- ============================================================
local STORIES={
  intro={
    "The Obsidian Spire awakes.",
    "Corruption spreads -- forests",
    "wither, rivers turn black.",
    "The Nano Wizards are gone.",
    "All but one.",
    "",
    "You are Vael, the last.",
    "Ascend the Spire.",
    "Destroy its heart.",
    "Before the end.",
  },
  after1={
    "The walls pulse darkly.",
    "You feel it in your chest --",
    "a heartbeat not your own.",
    "",
    "Something here",
    "recognizes you.",
  },
  after2={
    "The whispers grow louder.",
    "Memory flashes -- a child",
    "laughing in these halls.",
    "",
    "Your hands glow with",
    "the same dark energy.",
  },
  victory={
    "You reach the heart chamber.",
    "The crystal is... familiar.",
    "",
    "You place your hand on it.",
    "And remember everything.",
    "",
    "You ARE the heart.",
    "",
    "They sent you home,",
    "hoping you'd merge back.",
    "",
    "But you are Vael now.",
    "You shatter the crystal.",
    "The Spire crumbles.",
    "",
    "Free at last.",
  },
}

-- ============================================================
-- GAME STATE
-- ============================================================
local STATE="START"
local score=0
local lives=5
local cur_level=1
local tick=0

-- Player
local px,py,pvx,pvy
local p_on_ground,p_wall_dir,p_facing_right,p_jumps
local p_w,p_h=6,8
local coyote=0
local jump_buf=0

-- Anchor
local an_active,an_attached
local an_x,an_y,an_vx,an_vy,an_len
local an_w,an_h=3,3
local an_held=false  -- b button currently held
local an_fired=false -- anchor already fired this hold

-- Camera
local cam_y=0
local lava_y=0
local lava_speed=0.08

-- Goal position (world coords)
local goal_x,goal_y

-- Dynamic arrays (flat, fixed max)
-- Tiles: stored as flat arrays per type
local tiles_x={}   -- world x
local tiles_y={}   -- world y
local tiles_t={}   -- type: 1=brick,2=stone,3=chest
local tile_count=0

local gem_x={} local gem_y={} local gem_vx={} local gem_vy={}
local gem_alive={} local gem_count=0

local en_x={} local en_y={} local en_type={} -- 1=patrol,2=bat,3=turret
local en_vx={} local en_vy={} local en_sx={} local en_timer={}
local en_alive={} local en_count=0

local bul_x={} local bul_y={} local bul_vx={} local bul_vy={}
local bul_alive={} local bul_count=0

local ebul_x={} local ebul_y={} local ebul_vx={} local ebul_vy={}
local ebul_alive={} local ebul_count=0

local par_x={} local par_y={} local par_vx={} local par_vy={}
local par_c={} local par_life={} local par_count=0

local pop_x={} local pop_y={} local pop_txt={} local pop_life={}
local pop_count=0

-- Damage flash / death guard
local dmg_flash=0
local died_this_frame=false
local shake_mag=0
local shake_x=0
local shake_y=0

-- Story
local story_lines={}
local story_idx=1
local story_char=1
local story_timer=0
local story_text=""
local story_done=false
local story_next_state=""  -- what to do after story

-- ============================================================
-- UTILITY
-- ============================================================
local function rnd() return math.random() end
local function rndrange(a,b) return a+(b-a)*math.random() end
local function sign(v) if v>0 then return 1 elseif v<0 then return -1 else return 0 end end
local function clamp(v,lo,hi) if v<lo then return lo elseif v>hi then return hi else return v end end

local function overlaps(ax,ay,aw,ah,bx,by,bw,bh)
  return ax<bx+bw and ax+aw>bx and ay<by+bh and ay+ah>by
end

-- ============================================================
-- TILE COLLISION
-- ============================================================
local function tile_solid(i)
  -- stone and chest are destructible but solid for movement
  return tiles_t[i]==1 or tiles_t[i]==2 or tiles_t[i]==3
end

local function collide_tiles_horiz(rx,ry,rw,rh,vx)
  -- returns new rx after horizontal tile collision, and wall_dir
  local new_x=rx+vx
  local wall=0
  for i=1,tile_count do
    if overlaps(new_x,ry,rw,rh,tiles_x[i],tiles_y[i],TS,TS) then
      if vx>0 then new_x=tiles_x[i]-rw; wall=1
      elseif vx<0 then new_x=tiles_x[i]+TS; wall=-1 end
    end
  end
  return new_x,wall
end

local function collide_tiles_vert(rx,ry,rw,rh,vy)
  local new_y=ry+vy
  local on_ground=false
  local hit_ceil=false
  for i=1,tile_count do
    if overlaps(rx,new_y,rw,rh,tiles_x[i],tiles_y[i],TS,TS) then
      if vy>0 then new_y=tiles_y[i]-rh; vy=0; on_ground=true
      elseif vy<0 then new_y=tiles_y[i]+TS; vy=0; hit_ceil=true end
    end
  end
  return new_y,vy,on_ground
end

-- ============================================================
-- SPAWN HELPERS
-- ============================================================
local function spawn_particle(x,y,vx,vy,c,life)
  if par_count>=MAX_PARTICLES then return end
  par_count=par_count+1
  par_x[par_count]=x par_y[par_count]=y
  par_vx[par_count]=vx par_vy[par_count]=vy
  par_c[par_count]=c par_life[par_count]=life
end

local function spawn_popup(x,y,txt)
  if pop_count>=MAX_POPUPS then return end
  pop_count=pop_count+1
  pop_x[pop_count]=x pop_y[pop_count]=y
  pop_txt[pop_count]=txt pop_life[pop_count]=40
end

local function spawn_bullet(x,y,vx,vy)
  if bul_count>=MAX_BULLETS then return end
  bul_count=bul_count+1
  bul_x[bul_count]=x bul_y[bul_count]=y
  bul_vx[bul_count]=vx bul_vy[bul_count]=vy
  bul_alive[bul_count]=true
end

local function spawn_ebullet(x,y,vx,vy)
  if ebul_count>=MAX_EBULLETS then return end
  ebul_count=ebul_count+1
  ebul_x[ebul_count]=x ebul_y[ebul_count]=y
  ebul_vx[ebul_count]=vx ebul_vy[ebul_count]=vy
  ebul_alive[ebul_count]=true
end

-- ============================================================
-- LOAD LEVEL
-- ============================================================
local function load_level(keep_score)
  if not keep_score then score=0 end

  tile_count=0
  gem_count=0
  en_count=0
  bul_count=0
  ebul_count=0
  par_count=0
  pop_count=0

  an_active=false
  an_attached=false
  an_held=false
  an_fired=false
  shake_mag=0 shake_x=0 shake_y=0
  dmg_flash=0
  died_this_frame=false
  coyote=0
  jump_buf=0

  local lv=MAPS[cur_level]
  lava_speed=lv.lava_speed
  local map=lv.rows
  local nrows=#map
  local world_h=nrows*TS

  -- World starts at y=0 (top) going down
  -- Bottom of map = world_h
  -- Player spawns near bottom
  goal_x=nil goal_y=nil

  local temp_tiles_x={} local temp_tiles_y={} local temp_tiles_t={}
  local tc=0

  for row=1,nrows do
    for col=1,COLS do
      local ch=map[row]:sub(col,col)
      local wx=(col-1)*TS
      local wy=(row-1)*TS
      if ch=='#' then
        tc=tc+1 temp_tiles_x[tc]=wx temp_tiles_y[tc]=wy temp_tiles_t[tc]=1
      elseif ch=='%' then
        tc=tc+1 temp_tiles_x[tc]=wx temp_tiles_y[tc]=wy temp_tiles_t[tc]=2
      elseif ch=='C' then
        tc=tc+1 temp_tiles_x[tc]=wx temp_tiles_y[tc]=wy temp_tiles_t[tc]=3
      elseif ch=='G' then
        if gem_count<MAX_GEMS then
          gem_count=gem_count+1
          gem_x[gem_count]=wx+2 gem_y[gem_count]=wy+2
          gem_vx[gem_count]=0 gem_vy[gem_count]=0
          gem_alive[gem_count]=true
        end
      elseif ch=='P' then
        if en_count<MAX_ENEMIES then
          en_count=en_count+1
          en_x[en_count]=wx+1 en_y[en_count]=wy+1
          en_type[en_count]=1
          en_vx[en_count]=1.0 en_vy[en_count]=0
          en_sx[en_count]=wx+1
          en_timer[en_count]=0
          en_alive[en_count]=true
        end
      elseif ch=='B' then
        if en_count<MAX_ENEMIES then
          en_count=en_count+1
          en_x[en_count]=wx+1 en_y[en_count]=wy+1
          en_type[en_count]=2
          en_vx[en_count]=0 en_vy[en_count]=0
          en_sx[en_count]=wx+1
          en_timer[en_count]=0
          en_alive[en_count]=true
        end
      elseif ch=='T' then
        if en_count<MAX_ENEMIES then
          en_count=en_count+1
          en_x[en_count]=wx+1 en_y[en_count]=wy+1
          en_type[en_count]=3
          en_vx[en_count]=0 en_vy[en_count]=0
          en_sx[en_count]=wx+1
          en_timer[en_count]=math.random(0,60)
          en_alive[en_count]=true
        end
      elseif ch=='X' then
        goal_x=wx+1 goal_y=wy+1
      end
    end
  end

  -- Copy temp tiles to main arrays
  tile_count=tc
  tiles_x=temp_tiles_x tiles_y=temp_tiles_y tiles_t=temp_tiles_t

  -- Player spawns near bottom center
  local spawn_col=14
  local spawn_row=nrows-2
  px=(spawn_col)*TS
  py=(spawn_row-1)*TS
  pvx=0 pvy=0
  p_on_ground=false p_wall_dir=0 p_facing_right=true p_jumps=0

  -- Lava starts well below player
  lava_y=world_h+20  -- starts below map, rises up

  -- Camera: show bottom of map initially
  cam_y=world_h - SH
  if cam_y<0 then cam_y=0 end
end

-- ============================================================
-- STORY SYSTEM
-- ============================================================
local function start_story(lines, next_state)
  story_lines=lines
  story_idx=1
  story_char=1
  story_timer=0
  story_text=""
  story_done=false
  story_next_state=next_state
  STATE="STORY"
end

local function story_skip()
  -- Reveal all remaining text
  for i=story_idx,#story_lines do
    local line=story_lines[i]
    local start=(i==story_idx) and story_char or 1
    story_text=story_text..line:sub(start).."\n"
  end
  story_idx=#story_lines+1
  story_done=true
end

local function update_story()
  story_timer=story_timer+1
  if story_idx<=#story_lines then
    if story_timer%3==0 then
      local line=story_lines[story_idx]
      if story_char<=#line then
        story_text=story_text..line:sub(story_char,story_char)
        story_char=story_char+1
      else
        story_text=story_text.."\n"
        story_idx=story_idx+1
        story_char=1
        if story_idx>#story_lines then story_done=true end
      end
    end
  end

  -- Input: A or B to skip/continue
  if btnp(4) or btnp(5) or btnp(6) or btnp(7) then
    if not story_done then
      story_skip()
    else
      -- Advance to next state
      if story_next_state=="PLAY" then
        STATE="PLAY"
        load_level(true)
      elseif story_next_state=="WIN" then
        STATE="WIN"
      else
        STATE=story_next_state
      end
    end
  end
end

local function draw_story()
  cls(0)
  -- title bar
  rect(0,0,SW,10,1)
  print("THE OBSIDIAN SPIRE",4,2,11,false,1)

  local lines={}
  local s=story_text
  local start=1
  while true do
    local e=s:find("\n",start,true)
    if e then
      lines[#lines+1]=s:sub(start,e-1)
      start=e+1
    else
      if start<=#s then lines[#lines+1]=s:sub(start) end
      break
    end
  end

  local y0=16
  local lh=8
  for i,l in ipairs(lines) do
    local sy=y0+(i-1)*lh
    if sy<SH-16 then
      print(l,4,sy,8,false,1)
    end
  end

  -- Cursor blink
  if not story_done then
    if tick%30<15 then
      print("_",4,y0+(#lines-1)*lh,4,false,1)
    end
  else
    if tick%60<30 then
      print("A/B: continue",SW/2-30,SH-12,9,false,1)
    end
  end
end

-- ============================================================
-- ADD SCORE
-- ============================================================
local function add_score(amt,x,y)
  score=score+amt
  spawn_popup(x,y,"+"..amt)
end

-- ============================================================
-- LOSE LIFE / GAME OVER
-- ============================================================
local function lose_life()
  if died_this_frame then return end
  died_this_frame=true
  lives=lives-1
  dmg_flash=8
  shake_mag=4
  if lives<=0 then
    STATE="GAMEOVER"
  else
    load_level(true)
    STATE="PLAY"
  end
end

-- ============================================================
-- GET SHOT DIRECTION
-- ============================================================
local function get_traj(speed)
  local vx2,vy2=0,0
  local up=btn(0) local dn=btn(1)
  local lt=btn(2) local rt=btn(3)
  if up and rt then vx2=speed vy2=-speed
  elseif up and lt then vx2=-speed vy2=-speed
  elseif dn and rt then vx2=speed vy2=speed
  elseif dn and lt then vx2=-speed vy2=speed
  elseif up then vy2=-speed
  elseif dn then vy2=speed
  elseif rt then vx2=speed
  elseif lt then vx2=-speed
  else
    vx2=p_facing_right and speed or -speed
  end
  return vx2,vy2
end

-- ============================================================
-- PUSH PLAYER OUT OF TILES
-- ============================================================
local function pushout()
  for i=1,tile_count do
    if overlaps(px,py,p_w,p_h,tiles_x[i],tiles_y[i],TS,TS) then
      local ol=(px+p_w)-tiles_x[i]
      local or2=(tiles_x[i]+TS)-px
      local ot=(py+p_h)-tiles_y[i]
      local ob=(tiles_y[i]+TS)-py
      local m=math.min(ol,or2,ot,ob)
      if m==ol then px=tiles_x[i]-p_w; pvx=0
      elseif m==or2 then px=tiles_x[i]+TS; pvx=0
      elseif m==ot then py=tiles_y[i]-p_h; pvy=0; p_on_ground=true; p_jumps=0
      else py=tiles_y[i]+TS; pvy=0 end
    end
  end
end

-- ============================================================
-- UPDATE PLAY
-- ============================================================
local function update_play()
  died_this_frame=false
  -- Coyote & jump buffer decay
  if p_on_ground then coyote=6 else coyote=coyote-1 end
  if jump_buf>0 then jump_buf=jump_buf-1 end

  -- Input
  local left=btn(2) local right=btn(3)
  local b_held=btn(5)   -- jump / anchor
  local a_press=btnp(4) -- shoot

  -- Jump buffer
  if btnp(5) then jump_buf=6 end

  -- Anchor logic
  if b_held and not an_held then
    an_held=true
    an_fired=false
  end
  if not b_held then
    an_held=false
    an_fired=false
    if an_active then
      an_active=false
      if an_attached then
        pvy=JUMP_FORCE*0.8
        p_jumps=1
        an_attached=false
        coyote=0
      end
    end
  end

  -- Fire anchor after holding B for ~10 frames with no prior jump
  if an_held and not an_active and not an_fired and jump_buf==0 then
    -- hold detection: fire if b is still held after jump_buf expires
    -- Actually fire anchor if held and coyote==0 (in air)
    if coyote<=0 and not p_on_ground then
      local avx,avy=get_traj(ANCHOR_SPEED)
      an_active=true
      an_attached=false
      an_fired=true
      an_x=px+p_w/2-an_w/2
      an_y=py+p_h/2-an_h/2
      an_vx=avx an_vy=avy
      an_len=0
    end
  end

  -- Move anchor
  if an_active and not an_attached then
    an_x=an_x+an_vx an_y=an_y+an_vy
    local hit=false
    -- Check tile collision
    for i=tile_count,1,-1 do
      if overlaps(an_x,an_y,an_w,an_h,tiles_x[i],tiles_y[i],TS,TS) then
        if tiles_t[i]==2 or tiles_t[i]==3 then
          -- Destroy stone/chest, spawn gems from chest
          if tiles_t[i]==3 then
            if gem_count<MAX_GEMS then
              gem_count=gem_count+1
              gem_x[gem_count]=tiles_x[i]+2
              gem_y[gem_count]=tiles_y[i]
              gem_vx[gem_count]=rndrange(-1.5,1.5)
              gem_vy[gem_count]=-2
              gem_alive[gem_count]=true
            end
          end
          -- Remove tile (swap with last)
          if i<tile_count then
            tiles_x[i]=tiles_x[tile_count]
            tiles_y[i]=tiles_y[tile_count]
            tiles_t[i]=tiles_t[tile_count]
          end
          tile_count=tile_count-1
          -- Particles
          for _=1,4 do
            spawn_particle(an_x,an_y,rndrange(-2,2),rndrange(-2,2),15,12)
          end
          an_active=false
          hit=true
          break
        else
          -- Attach to brick
          an_attached=true
          an_active=true
          shake_mag=2
          local adx=(px+p_w/2)-an_x
          local ady=(py+p_h/2)-an_y
          an_len=math.sqrt(adx*adx+ady*ady)
          hit=true
          break
        end
      end
    end
    if not hit then
      -- Check if too far
      local adx2=(px+p_w/2)-an_x
      local ady2=(py+p_h/2)-an_y
      if math.sqrt(adx2*adx2+ady2*ady2)>160 then
        an_active=false
      end
    end
  end

  -- Anchor swing constraint
  if an_attached then
    local up2=btn(0) local dn2=btn(1)
    if up2 and an_len>12 then an_len=an_len-CLIMB_SPEED end
    if dn2 and an_len<160 then an_len=an_len+CLIMB_SPEED end
  end

  -- Player movement
  local target_vx=0
  if right then target_vx=MOVE_SPEED; p_facing_right=true end
  if left then target_vx=-MOVE_SPEED; p_facing_right=false end

  if an_attached then
    pvx=pvx+target_vx*0.05
  else
    if p_on_ground then pvx=target_vx
    else pvx=pvx*0.82+target_vx*0.18 end
  end

  -- Horizontal movement
  local new_px,wall=collide_tiles_horiz(px,py,p_w,p_h,pvx)
  px=new_px
  p_wall_dir=wall
  if wall~=0 then pvx=0 end

  -- Screen wrap
  if px>SW then px=-p_w elseif px<-p_w then px=SW end

  -- Gravity
  pvy=pvy+GRAVITY
  -- Wall slide
  if p_wall_dir~=0 and pvy>0 and not an_attached then
    if pvy>WALL_SLIDE then pvy=WALL_SLIDE end
  else
    if pvy>MAX_FALL then pvy=MAX_FALL end
  end

  -- Vertical movement
  local old_on=p_on_ground
  local new_vy
  py,new_vy,p_on_ground=collide_tiles_vert(px,py,p_w,p_h,pvy)
  pvy=new_vy
  if p_on_ground and not old_on then p_jumps=0 end

  -- Anchor rope constraint
  if an_attached then
    local dx=(px+p_w/2)-an_x
    local dy=(py+p_h/2)-an_y
    local dist=math.sqrt(dx*dx+dy*dy)
    if dist>0.001 and dist>an_len then
      local diff=dist-an_len
      local nx=dx/dist; local ny=dy/dist
      px=px-nx*diff; py=py-ny*diff
      local dot=pvx*nx+pvy*ny
      pvx=pvx-dot*nx; pvy=pvy-dot*ny
      pvx=pvx*0.99; pvy=pvy*0.99
    end
  end

  pushout()

  -- Jumping
  if jump_buf>0 then
    if an_attached then
      -- no jump while anchored
    elseif coyote>0 then
      pvy=JUMP_FORCE; p_jumps=1; coyote=0; jump_buf=0
    elseif p_wall_dir~=0 then
      pvy=WALL_JUMP_Y; pvx=-p_wall_dir*WALL_JUMP_X
      p_facing_right=(p_wall_dir==-1)
      p_jumps=1; jump_buf=0
    elseif p_jumps<2 then
      pvy=JUMP_FORCE; p_jumps=p_jumps+1; jump_buf=0
    end
  end

  -- Shooting
  if a_press then
    local bvx,bvy=get_traj(BULLET_SPEED)
    spawn_bullet(px+p_w/2-1,py+p_h/2-1,bvx,bvy)
  end

  -- Update bullets
  for i=1,bul_count do
    if bul_alive[i] then
      bul_x[i]=bul_x[i]+bul_vx[i]
      bul_y[i]=bul_y[i]+bul_vy[i]
      local hit=false
      -- Wall collision
      for j=1,tile_count do
        if tiles_t[j]==1 and overlaps(bul_x[i],bul_y[i],2,2,tiles_x[j],tiles_y[j],TS,TS) then
          hit=true; break
        end
      end
      -- Enemy collision
      if not hit then
        for j=1,en_count do
          if en_alive[j] and overlaps(bul_x[i],bul_y[i],2,2,en_x[j],en_y[j],5,6) then
            en_alive[j]=false
            add_score(100,en_x[j],en_y[j])
            shake_mag=math.max(shake_mag,2)
            for _=1,4 do
              spawn_particle(en_x[j],en_y[j],rndrange(-2,2),rndrange(-2,2),12,10)
            end
            hit=true; break
          end
        end
      end
      if hit or bul_x[i]>SW+10 or bul_x[i]<-10
         or bul_y[i]>lava_y+20 or bul_y[i]<cam_y-20 then
        bul_alive[i]=false
      end
    end
  end

  -- Update enemy bullets
  for i=1,ebul_count do
    if ebul_alive[i] then
      ebul_x[i]=ebul_x[i]+ebul_vx[i]
      ebul_y[i]=ebul_y[i]+ebul_vy[i]
      local hit=false
      for j=1,tile_count do
        if tiles_t[j]==1 and overlaps(ebul_x[i],ebul_y[i],2,2,tiles_x[j],tiles_y[j],TS,TS) then
          hit=true; break
        end
      end
      if overlaps(ebul_x[i],ebul_y[i],2,2,px,py,p_w,p_h) then
        lose_life()
        hit=true
      end
      if hit or ebul_x[i]>SW+10 or ebul_x[i]<-10
         or ebul_y[i]>lava_y+20 or ebul_y[i]<cam_y-20 then
        ebul_alive[i]=false
      end
    end
  end

  -- Update enemies
  for i=1,en_count do
    if en_alive[i] then
      local et=en_type[i]
      if et==1 then -- patrol
        en_x[i]=en_x[i]+en_vx[i]
        if en_x[i]>en_sx[i]+32 or en_x[i]<en_sx[i]-32 then en_vx[i]=-en_vx[i] end
      elseif et==2 then -- bat
        local dx=px-en_x[i]; local dy=py-en_y[i]
        local dist=math.sqrt(dx*dx+dy*dy)
        if dist>0.001 and dist<120 then
          en_x[i]=en_x[i]+(dx/dist)*0.8
          en_y[i]=en_y[i]+(dy/dist)*0.8
        end
      elseif et==3 then -- turret
        en_timer[i]=en_timer[i]+1
        if en_timer[i]>90 then
          local dx=px-en_x[i]; local dy=py-en_y[i]
          local dist=math.sqrt(dx*dx+dy*dy)
          if dist>0.001 and dist<160 then
            spawn_ebullet(en_x[i]+2,en_y[i]+2,
              (dx/dist)*EBULLET_SPEED,(dy/dist)*EBULLET_SPEED)
          end
          en_timer[i]=0
        end
      end

      -- Player collision
      if en_alive[i] and overlaps(px,py,p_w,p_h,en_x[i],en_y[i],5,6) then
        if pvy>0 and py+p_h < en_y[i]+3 and et~=3 then
          -- stomp
          pvy=BOUNCE_FORCE; p_jumps=1
          en_alive[i]=false
          add_score(100,en_x[i],en_y[i])
          shake_mag=math.max(shake_mag,2)
          for _=1,4 do
            spawn_particle(en_x[i],en_y[i],rndrange(-2,2),rndrange(-2,2),12,10)
          end
        else
          lose_life()
        end
      end
    end
  end

  -- Update gems
  for i=1,gem_count do
    if gem_alive[i] then
      gem_vy[i]=gem_vy[i]+GRAVITY
      gem_x[i]=gem_x[i]+gem_vx[i]
      gem_y[i]=gem_y[i]+gem_vy[i]
      -- Land on tiles
      for j=1,tile_count do
        if tiles_t[j]==1 and overlaps(gem_x[i],gem_y[i],4,4,tiles_x[j],tiles_y[j],TS,TS) then
          if gem_vy[i]>0 then
            gem_y[i]=tiles_y[j]-4; gem_vy[i]=-gem_vy[i]*0.4; gem_vx[i]=gem_vx[i]*0.8
          end
        end
      end
      -- Player collect
      if overlaps(px,py,p_w,p_h,gem_x[i],gem_y[i],4,4) then
        gem_alive[i]=false
        add_score(50,gem_x[i],gem_y[i])
      end
    end
  end

  -- Update particles
  for i=1,par_count do
    if par_life[i]>0 then
      par_x[i]=par_x[i]+par_vx[i]
      par_y[i]=par_y[i]+par_vy[i]
      par_vy[i]=par_vy[i]+GRAVITY*0.5
      par_life[i]=par_life[i]-1
    end
  end

  -- Update popups
  for i=1,pop_count do
    if pop_life[i]>0 then
      pop_y[i]=pop_y[i]-0.5
      pop_life[i]=pop_life[i]-1
    end
  end

  -- Lava rises
  lava_y=lava_y-lava_speed

  -- Lava kill
  if py+p_h>=lava_y then lose_life() end

  -- Goal check
  if goal_x and overlaps(px,py,p_w,p_h,goal_x,goal_y,6,6) then
    local just=cur_level
    cur_level=cur_level+1
    if cur_level>3 then
      start_story(STORIES.victory,"WIN")
    elseif just==1 then
      start_story(STORIES.after1,"PLAY")
    elseif just==2 then
      start_story(STORIES.after2,"PLAY")
    else
      load_level(true)
    end
  end

  -- Camera: smooth follow player upward
  local target_cam=py - SH*0.55
  local lv_rows=#MAPS[cur_level>3 and 3 or cur_level].rows
  local world_h=lv_rows*TS
  local cam_max=world_h-SH
  cam_y=cam_y+(target_cam-cam_y)*0.12
  -- Don't let camera go above lava
  if cam_y>lava_y-SH+16 then cam_y=lava_y-SH+16 end
  if cam_y<0 then cam_y=0 end
  if cam_y>cam_max then cam_y=cam_max end

  -- Shake decay
  if shake_mag>0.1 then
    shake_x=(math.random()-0.5)*2*shake_mag
    shake_y=(math.random()-0.5)*2*shake_mag
    shake_mag=shake_mag*0.8
  else shake_x=0; shake_y=0; shake_mag=0 end

  if dmg_flash>0 then dmg_flash=dmg_flash-1 end
end

-- ============================================================
-- DRAW PLAY
-- ============================================================
local function draw_play()
  cls(0)

  local ox=math.floor(shake_x)
  local oy=math.floor(shake_y)
  local cy=math.floor(cam_y)

  -- Background stars (simple procedural)
  math.randomseed(cur_level*999)
  for _=1,18 do
    local bx=math.random(0,SW-1)
    local by=math.random(0,#MAPS[cur_level>3 and 3 or cur_level].rows*TS-1)
    local sy=by-cy
    if sy>=0 and sy<SH then
      pix(bx,sy,14)
    end
  end
  math.randomseed(tick) -- restore randomness

  -- Tiles
  for i=1,tile_count do
    local sx=tiles_x[i]+ox
    local sy=tiles_y[i]-cy+oy
    if sy>-TS and sy<SH+TS then
      draw_tile(sx,sy,tiles_t[i])
    end
  end

  -- Gems
  for i=1,gem_count do
    if gem_alive[i] then
      local sx=math.floor(gem_x[i])+ox
      local sy=math.floor(gem_y[i])-cy+oy
      if sy>-8 and sy<SH+8 then
        draw_gem(sx,sy,tick)
      end
    end
  end

  -- Particles
  for i=1,par_count do
    if par_life[i]>0 then
      local sx=math.floor(par_x[i])+ox
      local sy=math.floor(par_y[i])-cy+oy
      if sy>=0 and sy<SH then
        pix(sx,sy,par_c[i])
      end
    end
  end

  -- Bullets
  for i=1,bul_count do
    if bul_alive[i] then
      local sx=math.floor(bul_x[i])+ox
      local sy=math.floor(bul_y[i])-cy+oy
      if sy>-4 and sy<SH+4 then
        rect(sx,sy,2,2,9)
        pix(sx,sy,8)
      end
    end
  end

  -- Enemy bullets
  for i=1,ebul_count do
    if ebul_alive[i] then
      local sx=math.floor(ebul_x[i])+ox
      local sy=math.floor(ebul_y[i])-cy+oy
      if sy>-4 and sy<SH+4 then
        rect(sx,sy,2,2,12)
        pix(sx,sy,8)
      end
    end
  end

  -- Enemies
  for i=1,en_count do
    if en_alive[i] then
      local sx=math.floor(en_x[i])+ox
      local sy=math.floor(en_y[i])-cy+oy
      if sy>-8 and sy<SH+8 then
        local et=en_type[i]
        if et==1 then draw_patrol(sx,sy,en_vx[i]>0)
        elseif et==2 then draw_bat(sx,sy,tick)
        elseif et==3 then draw_turret(sx,sy,tick)
        end
      end
    end
  end

  -- Anchor
  if an_active or an_attached then
    local px2=math.floor(px+p_w/2)+ox
    local py2=math.floor(py+p_h/2)-cy+oy
    local ax=math.floor(an_x+an_w/2)+ox
    local ay=math.floor(an_y+an_h/2)-cy+oy
    line(px2,py2,ax,ay,15)
    draw_anchor(math.floor(an_x)+ox, math.floor(an_y)-cy+oy)
  end

  -- Player
  local psx=math.floor(px)+ox
  local psy=math.floor(py)-cy+oy
  draw_player(psx,psy,not p_facing_right)

  -- Goal portal
  if goal_x then
    local gsx=math.floor(goal_x)+ox
    local gsy=math.floor(goal_y)-cy+oy
    if gsy>-8 and gsy<SH+8 then
      draw_goal(gsx,gsy,tick)
    end
  end

  -- Lava
  local lsy=math.floor(lava_y)-cy+oy
  if lsy<SH then
    local lava_top=math.max(0,lsy)
    if lava_top<SH then
      rect(0,lava_top,SW,SH-lava_top,9)
      -- Wave top
      for wx=0,SW-1,2 do
        local wave=math.floor(math.sin(tick*0.15+wx*0.3)*2)
        local wy=lava_top+wave
        if wy>=0 and wy<SH then
          pix(wx,wy,4)
          if wy+1<SH then pix(wx,wy+1,3) end
        end
      end
    end
  end

  -- Popups
  for i=1,pop_count do
    if pop_life[i]>0 then
      local sx=math.floor(pop_x[i])+ox
      local sy=math.floor(pop_y[i])-cy+oy
      if sy>0 and sy<SH then
        print(pop_txt[i],sx,sy,4,false,1)
      end
    end
  end

  -- Damage flash
  if dmg_flash>0 then
    local alpha=dmg_flash
    -- Draw red tint via line pattern
    for dy=0,SH-1,3 do
      line(0,dy,SW-1,dy,2)
    end
  end

  -- HUD
  print("SC:"..score,1,1,8,false,1)
  local lv_name=MAPS[cur_level>3 and 3 or cur_level].name
  print(lv_name,SW-#lv_name*6-1,1,11,false,1)
  -- Lives as hearts
  for lf=1,lives do
    local hx=1+(lf-1)*7
    pix(hx,9,2) pix(hx+1,8,2) pix(hx+2,9,2) pix(hx+3,8,2) pix(hx+4,9,2)
    pix(hx+1,9,2) pix(hx+2,10,2) pix(hx+3,9,2)
    pix(hx+2,11,2)
  end
end

-- ============================================================
-- DRAW TITLE SCREEN
-- ============================================================
local function draw_title()
  cls(0)

  -- Ember particles (reuse par arrays for title)
  if tick%4==0 and par_count<MAX_PARTICLES then
    spawn_particle(
      math.random(0,SW-1), SH+2,
      rndrange(-0.3,0.3), rndrange(-1.5,-0.5),
      math.random(2)>1 and 9 or 3,
      40+math.random(30)
    )
  end
  for i=1,par_count do
    if par_life[i]>0 then
      par_x[i]=par_x[i]+par_vx[i]
      par_y[i]=par_y[i]+par_vy[i]
      par_life[i]=par_life[i]-1
      if par_x[i]>=0 and par_x[i]<SW and par_y[i]>=0 and par_y[i]<SH then
        pix(math.floor(par_x[i]),math.floor(par_y[i]),par_c[i])
      end
    end
  end

  -- Title
  print("NANO WIZARDS",SW/2-36,SH/2-28,8,false,1)
  print("NANO WIZARDS",SW/2-35,SH/2-29,9,false,1)
  print("THE OBSIDIAN SPIRE",SW/2-53,SH/2-18,11,false,1)
  print("Last wizard vs the darkness",SW/2-78,SH/2-6,15,false,1)

  -- Blink prompt
  if tick%60<30 then
    print("PRESS A or B to START",SW/2-60,SH/2+16,4,false,1)
  end
  print("D-PAD:move  B:jump  A:shoot",2,SH-9,14,false,1)
end

-- ============================================================
-- DRAW GAMEOVER
-- ============================================================
local function draw_gameover()
  cls(0)
  for dy=0,SH-1,4 do line(0,dy,SW-1,dy,2) end
  print("THE SPIRE CLAIMS YOU",SW/2-60,SH/2-20,8,false,1)
  print("SCORE: "..score,SW/2-30,SH/2-4,4,false,1)
  if tick%60<30 then
    print("B to retry",SW/2-28,SH/2+14,9,false,1)
  end
end

-- ============================================================
-- DRAW WIN
-- ============================================================
local function draw_win()
  cls(1)
  print("FREE AT LAST",SW/2-36,SH/2-30,8,false,1)
  print("The Spire crumbles.",SW/2-56,SH/2-16,11,false,1)
  print("Vael walks into the dawn.",SW/2-72,SH/2-6,15,false,1)
  print("FINAL SCORE: "..score,SW/2-38,SH/2+10,4,false,1)
  if tick%60<30 then
    print("B to play again",SW/2-44,SH/2+26,9,false,1)
  end
end

-- ============================================================
-- BOOT
-- ============================================================
function BOOT()
  math.randomseed(42)
  tick=0
  STATE="START"
  score=0
  lives=5
  cur_level=1
  par_count=0
  -- init arrays
  for i=1,MAX_PARTICLES do
    par_x[i]=0 par_y[i]=0 par_vx[i]=0 par_vy[i]=0 par_c[i]=0 par_life[i]=0
  end
  for i=1,MAX_BULLETS do
    bul_x[i]=0 bul_y[i]=0 bul_vx[i]=0 bul_vy[i]=0 bul_alive[i]=false
  end
  for i=1,MAX_EBULLETS do
    ebul_x[i]=0 ebul_y[i]=0 ebul_vx[i]=0 ebul_vy[i]=0 ebul_alive[i]=false
  end
  for i=1,MAX_ENEMIES do
    en_x[i]=0 en_y[i]=0 en_type[i]=0 en_vx[i]=0 en_vy[i]=0
    en_sx[i]=0 en_timer[i]=0 en_alive[i]=false
  end
  for i=1,MAX_GEMS do
    gem_x[i]=0 gem_y[i]=0 gem_vx[i]=0 gem_vy[i]=0 gem_alive[i]=false
  end
  for i=1,MAX_POPUPS do
    pop_x[i]=0 pop_y[i]=0 pop_txt[i]="" pop_life[i]=0
  end
end

-- ============================================================
-- MAIN LOOP
-- ============================================================
function TIC()
  tick=tick+1

  if STATE=="START" then
    draw_title()
    if btnp(4) or btnp(5) then
      -- Start game with intro story
      lives=5 score=0 cur_level=1
      start_story(STORIES.intro,"PLAY")
    end

  elseif STATE=="STORY" then
    update_story()
    draw_story()

  elseif STATE=="PLAY" then
    update_play()
    -- STATE may have changed inside update_play (lose_life, goal)
    if STATE=="PLAY" then
      draw_play()
    elseif STATE=="GAMEOVER" then
      draw_gameover()
    elseif STATE=="WIN" then
      draw_win()
    elseif STATE=="STORY" then
      draw_story()
    end

  elseif STATE=="GAMEOVER" then
    draw_gameover()
    if btnp(4) or btnp(5) then
      lives=5 score=0 cur_level=1
      load_level(false)
      STATE="PLAY"
    end

  elseif STATE=="WIN" then
    draw_win()
    if btnp(4) or btnp(5) then
      lives=5 score=0 cur_level=1
      STATE="START"
    end
  end
end
