-- title: Pixel Knight
-- author: retrogames
-- desc: The Crystal Kingdom - platformer with story
-- script: lua

-- ============================================================
-- CONSTANTS
-- ============================================================
local SW,SH=240,136
local TILE=7
local GRAV=0.28
local MAX_FALL=6
local JUMP_F=-5.8
local JUMP_HOLD_G=0.14
local MSPD=2.0
local MACCEL=0.22
local MDECEL=0.18
local STOMP_B=-4.2
local COYOTE=6
local JBUF=6
local FB_SPD=3.2

-- Sweetie16 palette indices (0-based)
local C_NAVY=0
local C_DPUR=1
local C_RED=2
local C_ORG=3
local C_YEL=4
local C_LGRN=5
local C_GRN=6
local C_TEAL=7
local C_WHT=8
local C_ORG2=9
local C_GRN2=10
local C_LBLU=11
local C_MAG=12
local C_TEAL2=13
local C_DGRAY=14
local C_GRAY=15

-- ============================================================
-- SPRITE DATA (pixel art as color index arrays)
-- each sprite: {w, h, rows of pixel data (0=transparent)}
-- ============================================================

-- Small knight (6x8)
local SPR_KNIGHT_S={6,8,{
 {0,2,2,2,2,0},
 {2,2,2,2,2,2},
 {2,2,8,8,2,2},
 {2,8,4,4,8,2},
 {2,2,8,8,2,2},
 {0,2,2,2,2,0},
 {0,11,11,11,11,0},
 {0,11,0,0,11,0},
}}

-- Big knight (10x14)
local SPR_KNIGHT_B={10,14,{
 {0,0,2,2,2,2,0,0,0,0},
 {0,2,2,2,2,2,2,0,0,0},
 {0,2,2,8,8,2,2,0,0,0},
 {0,2,8,4,4,8,2,0,0,0},
 {0,2,8,4,4,8,2,0,0,0},
 {0,2,2,8,8,2,2,0,0,0},
 {0,2,2,2,2,2,2,0,0,0},
 {0,11,11,11,11,11,11,0,0,0},
 {0,11,11,2,2,11,11,0,0,0},
 {0,11,11,2,2,11,11,0,0,0},
 {0,11,11,11,11,11,11,0,0,0},
 {0,0,11,0,0,11,0,0,0,0},
 {0,0,5,5,5,5,0,0,0,0},
 {0,0,5,5,5,5,0,0,0,0},
}}

-- Slime (7x6)
local SPR_SLIME={7,6,{
 {0,0,6,6,6,0,0},
 {0,6,6,6,6,6,0},
 {6,6,8,6,8,6,6},
 {6,6,6,6,6,6,6},
 {6,6,6,6,6,6,6},
 {0,6,0,0,0,6,0},
}}

-- Bat (8x5)
local SPR_BAT={8,5,{
 {1,1,0,0,0,0,1,1},
 {1,1,1,0,0,1,1,1},
 {1,1,1,1,1,1,1,1},
 {0,1,8,1,8,1,0,0},
 {0,0,1,0,1,0,0,0},
}}
local SPR_BAT2={8,5,{
 {0,1,0,0,0,0,1,0},
 {1,1,1,0,0,1,1,1},
 {1,1,1,1,1,1,1,1},
 {0,1,8,1,8,1,0,0},
 {0,0,1,0,1,0,0,0},
}}

-- Boss dark knight (14x14)
local SPR_BOSS={14,14,{
 {0,0,2,2,2,2,2,0,0,0,0,0,0,0},
 {0,2,2,2,2,2,2,2,0,0,0,0,0,0},
 {0,2,2,15,15,2,2,2,0,0,0,0,0,0},
 {0,2,15,8,8,15,2,2,0,0,0,0,0,0},
 {0,2,15,8,8,15,2,2,0,0,0,0,0,0},
 {0,2,2,15,15,2,2,2,0,0,0,0,0,0},
 {0,2,2,2,2,2,2,2,0,0,0,0,0,0},
 {0,0,14,14,14,14,14,0,0,0,0,0,0,0},
 {0,14,14,14,14,14,14,14,0,0,0,0,0,0},
 {0,14,14,2,14,14,14,14,0,0,0,0,0,0},
 {14,14,14,14,14,14,14,14,0,0,0,0,0,0},
 {0,0,14,0,0,14,0,0,0,0,0,0,0,0},
 {0,0,15,0,0,15,0,0,0,0,0,0,0,0},
 {0,0,15,0,0,15,0,0,0,0,0,0,0,0},
}}

-- Crystal (5x5)
local SPR_CRYSTAL={5,5,{
 {0,0,11,0,0},
 {0,11,8,11,0},
 {11,8,8,8,11},
 {0,11,8,11,0},
 {0,0,11,0,0},
}}

-- Power shard (5x5)
local SPR_SHARD={5,5,{
 {0,0,4,0,0},
 {0,4,8,4,0},
 {4,8,8,8,4},
 {0,4,8,4,0},
 {0,0,4,0,0},
}}

-- Fire crystal (5x5)
local SPR_FIRE={5,5,{
 {0,0,2,0,0},
 {0,2,9,2,0},
 {2,9,4,9,2},
 {0,2,9,2,0},
 {0,0,2,0,0},
}}

-- Fireball (4x4)
local SPR_FIREBALL={4,4,{
 {0,2,2,0},
 {2,9,4,2},
 {2,9,4,2},
 {0,2,2,0},
}}

-- Sword slash (6x5)
local SPR_SLASH={6,5,{
 {0,0,0,15,15,8},
 {0,0,15,15,8,0},
 {0,15,15,8,0,0},
 {15,15,8,0,0,0},
 {15,8,0,0,0,0},
}}

-- ============================================================
-- SPRITE DRAW HELPER
-- ============================================================
function drawSpr(spr, x, y, flipx)
 local w,h,rows=spr[1],spr[2],spr[3]
 for row=1,h do
  local r=rows[row]
  for col=1,w do
   local c=r[col]
   if c~=0 then
    local px=flipx and (x+w-col) or (x+col-1)
    pix(px, y+row-1, c)
   end
  end
 end
end

-- ============================================================
-- LEVEL DATA
-- Map chars: #=solid, K=breakable, ^=spike, @=player start
--            C=crystal, S=slime, B=bat, P=shard powerup
--            F=fire powerup, G=goal, X=boss
-- All rows must be the same length within a level.
-- ============================================================
local levelDefs={
 -- Level 1: Crystal Meadows (38 wide x 14 high)
 {
  name="Crystal Meadows",
  bg=C_NAVY,
  tileCol1=C_GRN,tileCol2=C_LGRN,
  map={
   "......................................",
   "......................................",
   "......................................",
   ".......C.C..........C.C.........G.....",
   "......####......C...####.C.C...###....",
   "..........C.C.C...............####....",
   "............##.KKK.....C......####....",
   "......................................",
   "........C........C...S.....C..####....",
   "...C...#####...#####..S..#####.###....",
   ".@....................................",
   "####.......####.......................##",
   "####.^^.S..####.###.###.####.#########",
   "##########################################",
  }
 },
 -- Level 2: Shadow Caverns (36 wide x 13 high)
 {
  name="Shadow Caverns",
  bg=C_DPUR,
  tileCol1=C_DGRAY,tileCol2=C_GRAY,
  map={
   "####################################",
   "#..................................#",
   "#.C.C...B.....C.C...B...........G.#",
   "#.#####......####.KKKK.........####",
   "#..........###.......C.C.B.....####",
   "#.....B.S...........#####......####",
   "#.@...#####...S.....####.......####",
   "####.S.....^^^.####..KKKK.####.####",
   "####.###.......####.S......####.####",
   "####.......####.S..####.####.######",
   "####.####......####.....S..########",
   "##########################.#########",
   "####################################",
  }
 },
 -- Level 3: The Dark Tower (36 wide x 13 high)
 {
  name="The Dark Tower",
  bg=C_NAVY,
  tileCol1=C_DGRAY,tileCol2=C_DPUR,
  map={
   "####################################",
   "#.................................X#",
   "#..................................#",
   "#..............................C.C.#",
   "#..B.......B.C.C.B...........######",
   "#.C.C.....#####.....KKKK.......####",
   "#.#####.......S....C.........######",
   "#.@......S.F.......#####.......####",
   "####.#####.^^.###..........########",
   "####........######.##...###.S.#####",
   "####.^^..#####.###.....S.......####",
   "####.##############################.",
   "####################################",
  }
 },
}

-- ============================================================
-- GAME STATE
-- ============================================================
local state="START"
local score,lives,coins=0,3,0
local lvlIdx=0
local TILE_W,TILE_H=0,0
local tiles={}
local enemies={}
local crystals={}
local powerups={}
local fireballs={}
local particles={}
local popups={}
local boss=nil
local bossDefeated=false
local lvlComplTimer=0
local camX=0
local shakeX,shakeY,shakeMag=0,0,0
local damFlash,invTimer=0,0
local deathTimer=0
local titleFrame=0
local isVictory=false

-- story
local storyLines={}
local storyLine=1
local storyChar=1
local storyText=""
local storyTimer=0
local storyDone=false

local STORIES={
 intro={
  "Long ago, the Crystal Heart",
  "sustained all life in the",
  "kingdom with its light.",
  "",
  "The sorcerer Malachar shattered it,",
  "scattering fragments across the land.",
  "Eternal twilight fell upon the realm.",
  "",
  "You are the last Pixel Knight.",
  "Take up your sword and journey forth.",
  "Reclaim the crystal shards.",
 },
 after1={
  "The first shard glows warm",
  "in your hand. Its power",
  "pushes back the twilight.",
  "",
  "The caverns below writhe",
  "with dark magic.",
  "",
  "You descend into",
  "the Shadow Caverns.",
 },
 after2={
  "Two shards recovered.",
  "The darkness recoils.",
  "",
  "Malachar's Dark Tower rises",
  "before you - a spire of",
  "obsidian and malice.",
  "",
  "The final push begins.",
 },
 victory={
  "The Dark Knight falls.",
  "The final shard flies free.",
  "",
  "You hold all three fragments.",
  "They merge in radiant light.",
  "",
  "The Crystal Heart is whole.",
  "",
  "The eternal twilight shatters.",
  "Dawn breaks for the first",
  "time in a hundred years.",
  "",
  "The Crystal Kingdom is saved.",
 },
}

-- ============================================================
-- PLAYER
-- ============================================================
local pl={
 x=20,y=20,vx=0,vy=0,
 w=6,h=8,
 onGround=false,
 facing=1,
 coyote=0,jbuf=0,jumpHeld=false,
 big=false,fire=false,
 atkCool=0,
 animFrame=0,animTimer=0,
 dead=false,
 trailTimer=0,
}

local slash=nil -- {x,y,facing,timer,w,h}

-- ============================================================
-- INPUT HELPERS (prev frame state)
-- ============================================================
local prevBtns={}
for i=0,7 do prevBtns[i]=false end

function btnJust(id)
 return btn(id) and not prevBtns[id]
end

function updatePrevBtns()
 for i=0,7 do prevBtns[i]=btn(i) end
end

-- Aliases: 0=up,1=down,2=left,3=right,4=A(jump),5=B(attack)
function inputLeft()  return btn(2) end
function inputRight() return btn(3) end
function inputJump()  return btn(4) end
function inputAtk()   return btn(5) end
function inputStart() return btn(4) or btn(5) or btn(6) end
function inputStartJust() return btnJust(4) or btnJust(5) or btnJust(6) end

-- ============================================================
-- TILE HELPERS
-- ============================================================
function getTile(tx,ty)
 if tx<0 or tx>=TILE_W or ty<0 or ty>=TILE_H then return "#" end
 return tiles[ty*TILE_W+tx+1]
end
function setTile(tx,ty,v)
 if tx>=0 and tx<TILE_W and ty>=0 and ty<TILE_H then
  tiles[ty*TILE_W+tx+1]=v
 end
end
function isSolid(tx,ty)
 local t=getTile(tx,ty)
 return t=="#" or t=="K"
end

-- ============================================================
-- PARTICLES & POPUPS
-- ============================================================
function spawnParticles(x,y,col,n,spd)
 for i=1,n do
  local a=math.random()*6.28
  local s=math.random()*(spd or 2)+0.5
  table.insert(particles,{
   x=x,y=y,
   vx=math.cos(a)*s,
   vy=math.sin(a)*s-1,
   life=20+math.random(10),
   maxLife=30,
   col=col,
  })
 end
end

function spawnPopup(x,y,txt,col)
 table.insert(popups,{x=x,y=y,txt=txt,col=col or C_WHT,life=50,vy=-0.8})
end

-- ============================================================
-- MOVE ENTITY (returns "spike","pit" or nil)
-- ============================================================
function moveEnt(e, gravity)
 local pw=e.w or 6
 local ph=e.h or 6

 -- horizontal
 e.x=e.x+e.vx
 local lt=math.floor(e.x/TILE)
 local rt=math.floor((e.x+pw-1)/TILE)
 local tt=math.floor(e.y/TILE)
 local bt=math.floor((e.y+ph-1)/TILE)

 if e.vx<0 then
  for ty=tt,bt do
   if isSolid(lt,ty) then
    e.x=(lt+1)*TILE; e.vx=0
    lt=math.floor(e.x/TILE)
    break
   end
  end
 elseif e.vx>0 then
  for ty=tt,bt do
   if isSolid(rt,ty) then
    e.x=rt*TILE-pw; e.vx=0
    rt=math.floor((e.x+pw-1)/TILE)
    break
   end
  end
 end

 -- vertical
 if gravity~=false then
  local addG=(e.jumpHeld and e.vy<0) and JUMP_HOLD_G or GRAV
  e.vy=e.vy+addG
  if e.vy>MAX_FALL then e.vy=MAX_FALL end
 end
 e.y=e.y+e.vy
 e.onGround=false

 local lt2=math.floor(e.x/TILE)
 local rt2=math.floor((e.x+pw-1)/TILE)
 local bt2=math.floor((e.y+ph-1)/TILE)
 local tt2=math.floor(e.y/TILE)

 -- floor
 if e.vy>=0 then
  for tx=lt2,rt2 do
   if isSolid(tx,bt2) then
    e.y=bt2*TILE-ph; e.vy=0; e.onGround=true
    break
   end
  end
 end
 -- ceiling
 if e.vy<=0 then
  local ct=math.floor(e.y/TILE)
  for tx=lt2,rt2 do
   if isSolid(tx,ct) then
    e.y=(ct+1)*TILE; e.vy=0
    -- breakable
    if getTile(tx,ct)=="K" then
     setTile(tx,ct,".")
     spawnParticles(tx*TILE+3,ct*TILE+3,C_ORG2,5,2)
     shakeMag=math.max(shakeMag,2)
     if math.random()<0.4 then
      table.insert(crystals,{x=tx*TILE+2,y=ct*TILE-5,alive=true,t=math.random()*100})
     end
    end
    break
   end
  end
 end

 -- spikes
 local sl=math.floor(e.x/TILE)
 local sr=math.floor((e.x+pw-1)/TILE)
 local st_=math.floor(e.y/TILE)
 local sb=math.floor((e.y+ph-1)/TILE)
 for tx=sl,sr do
  for ty=st_,sb do
   if getTile(tx,ty)=="^" then return "spike" end
  end
 end

 if e.y>TILE_H*TILE+20 then return "pit" end
 return nil
end

-- ============================================================
-- OVERLAP
-- ============================================================
function overlap(ax,ay,aw,ah,bx,by,bw,bh)
 return ax<bx+bw and ax+aw>bx and ay<by+bh and ay+ah>by
end

-- ============================================================
-- LOAD LEVEL
-- ============================================================
function loadLevel(idx)
 local ld=levelDefs[idx]
 local map=ld.map
 TILE_H=#map
 TILE_W=0
 for _,row in ipairs(map) do
  if #row>TILE_W then TILE_W=#row end
 end
 tiles={}
 enemies={}
 crystals={}
 powerups={}
 fireballs={}
 particles={}
 popups={}
 boss=nil
 bossDefeated=false
 lvlComplTimer=0
 camX=0
 shakeX,shakeY,shakeMag=0,0,0
 damFlash,invTimer=0,0
 slash=nil

 for y=1,TILE_H do
  local row=map[y]
  for x=1,#row do
   local c=row:sub(x,x)
   if c=="@" then
    pl.x=(x-1)*TILE+2; pl.y=(y-1)*TILE
    table.insert(tiles,".")
   elseif c=="C" then
    table.insert(crystals,{x=(x-1)*TILE+2,y=(y-1)*TILE+2,alive=true,t=math.random()*100})
    table.insert(tiles,".")
   elseif c=="S" then
    table.insert(enemies,{type="slime",x=(x-1)*TILE,y=(y-1)*TILE,w=7,h=6,vx=0.6,vy=0,alive=true,frame=0,hp=1})
    table.insert(tiles,".")
   elseif c=="B" then
    table.insert(enemies,{type="bat",x=(x-1)*TILE,y=(y-1)*TILE,w=8,h=5,vx=-1,vy=0,alive=true,frame=0,hp=1,baseY=(y-1)*TILE,t=math.random()*100})
    table.insert(tiles,".")
   elseif c=="P" then
    table.insert(powerups,{type="shard",x=(x-1)*TILE+2,y=(y-1)*TILE+2,alive=true})
    table.insert(tiles,".")
   elseif c=="F" then
    table.insert(powerups,{type="fire",x=(x-1)*TILE+2,y=(y-1)*TILE+2,alive=true})
    table.insert(tiles,".")
   elseif c=="G" then
    table.insert(powerups,{type="goal",x=(x-1)*TILE+2,y=(y-1)*TILE+2,alive=true})
    table.insert(tiles,".")
   elseif c=="X" then
    boss={
     x=(x-1)*TILE-7,y=(y-1)*TILE-7,
     w=14,h=14,hp=8,maxHp=8,
     vx=1,vy=0,alive=true,frame=0,
     atkTimer=0,flashTimer=0,
     charging=false,chargeTimer=0,
    }
    table.insert(tiles,".")
   elseif c=="^" then
    table.insert(tiles,"^")
   else
    table.insert(tiles,c)
   end
  end
  -- pad row if needed
  while #tiles < y*TILE_W do table.insert(tiles,".") end
 end
end

function resetPlayer()
 pl.vx=0; pl.vy=0
 pl.w=6; pl.h=8
 pl.onGround=false
 pl.facing=1
 pl.coyote=0; pl.jbuf=0; pl.jumpHeld=false
 pl.big=false; pl.fire=false
 pl.atkCool=0
 pl.animFrame=0; pl.animTimer=0
 pl.dead=false; pl.trailTimer=0
 slash=nil
 invTimer=0; damFlash=0; shakeMag=0
end

-- ============================================================
-- STORY SYSTEM
-- ============================================================
function startStory(lines, isVic)
 state="STORY"
 storyLines=lines
 storyLine=1
 storyChar=0
 storyText=""
 storyTimer=0
 storyDone=false
 isVictory=isVic or false
end

function updateStory()
 storyTimer=storyTimer+1
 if not storyDone then
  if storyTimer%3==0 then
   local line=storyLines[storyLine] or ""
   if storyChar<#line then
    storyChar=storyChar+1
    storyText=storyText..line:sub(storyChar,storyChar)
   else
    storyText=storyText.."\n"
    storyLine=storyLine+1
    storyChar=0
    if storyLine>#storyLines then storyDone=true end
   end
  end
 end

 if inputStartJust() then
  if not storyDone then
   -- skip to end
   storyText=""
   for i=1,#storyLines do
    storyText=storyText..storyLines[i].."\n"
   end
   storyDone=true
  else
   if isVictory then
    state="WIN"
   else
    state="PLAYING"
   end
  end
 end
end

-- ============================================================
-- PLAYER HIT / DEATH
-- ============================================================
function playerHit()
 if invTimer>0 then return end
 if pl.big then
  pl.big=false; pl.fire=false
  pl.w=6; pl.h=8
  invTimer=80; damFlash=25
  shakeMag=math.max(shakeMag,3)
 else
  playerDie()
 end
end

function playerDie()
 if pl.dead then return end
 pl.dead=true; pl.vy=-5; pl.vx=0
 deathTimer=70
 shakeMag=math.max(shakeMag,5)
 spawnParticles(pl.x+pl.w/2,pl.y+pl.h/2,C_RED,8,3)
end

function bossDie()
 boss.alive=false; bossDefeated=true
 score=score+2000
 shakeMag=math.max(shakeMag,8)
 spawnParticles(boss.x+7,boss.y+7,C_RED,15,4)
 spawnPopup(boss.x+7,boss.y-8,"+2000",C_YEL)
 lvlComplTimer=90
end

-- ============================================================
-- UPDATE BOSS
-- ============================================================
function updateBoss()
 local b=boss
 b.atkTimer=b.atkTimer+1
 b.frame=b.frame+1
 if b.flashTimer>0 then b.flashTimer=b.flashTimer-1 end

 if not b.charging then
  b.x=b.x+b.vx
  b.vy=b.vy+GRAV
  if b.vy>MAX_FALL then b.vy=MAX_FALL end
  b.y=b.y+b.vy
  local bt=math.floor((b.y+b.h-1)/TILE)
  for tx=math.floor(b.x/TILE),math.floor((b.x+b.w-1)/TILE) do
   if isSolid(tx,bt) then b.y=bt*TILE-b.h; b.vy=0 end
  end
  local fTx=math.floor((b.x+(b.vx>0 and b.w or 0))/TILE)
  local fTy=math.floor((b.y+b.h/2)/TILE)
  if isSolid(fTx,fTy) then b.vx=-b.vx end

  if b.atkTimer>90 and math.abs(pl.x-b.x)<180 then
   b.charging=true; b.chargeTimer=30
   b.vx=(pl.x>b.x) and 3 or -3
   b.atkTimer=0
  end
 else
  b.x=b.x+b.vx
  b.vy=b.vy+GRAV
  if b.vy>MAX_FALL then b.vy=MAX_FALL end
  b.y=b.y+b.vy
  local bt=math.floor((b.y+b.h-1)/TILE)
  for tx=math.floor(b.x/TILE),math.floor((b.x+b.w-1)/TILE) do
   if isSolid(tx,bt) then b.y=bt*TILE-b.h; b.vy=0 end
  end
  local fTx=math.floor((b.x+(b.vx>0 and b.w or 0))/TILE)
  local fTy=math.floor((b.y+b.h/2)/TILE)
  if isSolid(fTx,fTy) then b.vx=-b.vx; b.charging=false end
  b.chargeTimer=b.chargeTimer-1
  if b.chargeTimer<=0 then b.charging=false; b.vx=(pl.x>b.x) and 1 or -1 end
 end

 -- boss-player collision
 if invTimer<=0 and not pl.dead then
  local pw=pl.w; local ph=pl.h
  if overlap(pl.x,pl.y,pw,ph,b.x,b.y,b.w,b.h) then
   if pl.vy>0 and pl.y+ph-3<b.y+b.h/2 then
    b.hp=b.hp-1; b.flashTimer=8
    pl.vy=STOMP_B
    shakeMag=math.max(shakeMag,4)
    spawnParticles(b.x+7,b.y,C_RED,6,2)
    if b.hp<=0 then bossDie() end
   else
    playerHit()
   end
  end
 end
end

-- ============================================================
-- MAIN UPDATE (PLAYING)
-- ============================================================
function updatePlaying()
 if pl.dead then
  deathTimer=deathTimer-1
  -- let player fall visually
  pl.vy=pl.vy+GRAV
  pl.y=pl.y+pl.vy
  if deathTimer<=0 then
   lives=lives-1
   if lives<=0 then
    state="GAMEOVER"
   else
    loadLevel(lvlIdx)
    resetPlayer()
   end
  end
  updateParticles()
  return
 end

 -- Movement
 local moveDir=0
 if inputLeft() then moveDir=-1 end
 if inputRight() then moveDir=1 end

 if moveDir~=0 then
  pl.vx=pl.vx+moveDir*MACCEL
  if pl.vx>MSPD then pl.vx=MSPD end
  if pl.vx<-MSPD then pl.vx=-MSPD end
  pl.facing=moveDir
  pl.animTimer=pl.animTimer+1
  if pl.animTimer>6 then pl.animTimer=0; pl.animFrame=(pl.animFrame+1)%4 end
 else
  if pl.vx>0 then pl.vx=math.max(0,pl.vx-MDECEL)
  elseif pl.vx<0 then pl.vx=math.min(0,pl.vx+MDECEL) end
  pl.animFrame=0
 end

 -- Coyote
 if pl.onGround then pl.coyote=COYOTE
 elseif pl.coyote>0 then pl.coyote=pl.coyote-1 end

 -- Jump buffer
 if btnJust(4) then pl.jbuf=JBUF
 elseif pl.jbuf>0 then pl.jbuf=pl.jbuf-1 end

 -- Jump
 if pl.jbuf>0 and pl.coyote>0 then
  pl.vy=JUMP_F; pl.onGround=false
  pl.coyote=0; pl.jbuf=0; pl.jumpHeld=true
 end
 if not inputJump() then pl.jumpHeld=false end

 -- Attack
 if pl.atkCool>0 then pl.atkCool=pl.atkCool-1 end
 if btnJust(5) and pl.atkCool<=0 then
  pl.atkCool=12
  if pl.fire then
   -- fireball
   local activeFb=0
   for _,f in ipairs(fireballs) do if f.alive then activeFb=activeFb+1 end end
   if activeFb<3 then
    local ox=pl.facing>0 and (pl.w+1) or -5
    table.insert(fireballs,{
     x=pl.x+ox,y=pl.y+pl.h/2-2,
     vx=pl.facing*FB_SPD,vy=0,
     alive=true,bounces=0,
    })
   end
  else
   -- sword slash
   local ox=pl.facing>0 and pl.w or -8
   slash={
    x=pl.x+ox,y=pl.y,
    facing=pl.facing,
    timer=8,w=8,h=8,
   }
  end
 end

 -- Update slash
 if slash then
  slash.timer=slash.timer-1
  local ox=pl.facing>0 and pl.w or -8
  slash.x=pl.x+ox; slash.y=pl.y
  if slash.timer<=0 then
   slash=nil
  else
   -- hit enemies
   for i=#enemies,1,-1 do
    local e=enemies[i]
    if e.alive and overlap(slash.x,slash.y,slash.w,slash.h,e.x,e.y,e.w,e.h) then
     e.alive=false; score=score+200
     spawnParticles(e.x+e.w/2,e.y+e.h/2,C_WHT,5,2)
     spawnPopup(e.x+e.w/2,e.y-6,"+200",C_WHT)
    end
   end
   -- hit boss
   if boss and boss.alive and slash.timer==7 then
    if overlap(slash.x,slash.y,slash.w,slash.h,boss.x,boss.y,boss.w,boss.h) then
     boss.hp=boss.hp-1; boss.flashTimer=8
     shakeMag=math.max(shakeMag,2)
     spawnParticles(boss.x+7,boss.y+7,C_WHT,4,2)
     if boss.hp<=0 then bossDie() end
    end
   end
  end
 end

 -- Move player
 pl.w = pl.big and 10 or 6
 pl.h = pl.big and 12 or 8
 local hz=moveEnt(pl, true)
 if hz=="spike" or hz=="pit" then playerDie(); return end

 -- Invincibility
 if invTimer>0 then invTimer=invTimer-1 end
 if damFlash>0 then damFlash=damFlash-1 end

 -- Camera
 local targetCam=pl.x-SW/2+pl.w/2
 camX=camX+(targetCam-camX)*0.12
 if camX<0 then camX=0 end
 local maxCam=TILE_W*TILE-SW
 if maxCam>0 and camX>maxCam then camX=maxCam end

 -- Collect crystals
 for i=#crystals,1,-1 do
  local c=crystals[i]
  if c.alive then
   if overlap(pl.x,pl.y,pl.w,pl.h,c.x-2,c.y-2,6,6) then
    c.alive=false; coins=coins+1; score=score+100
    spawnParticles(c.x,c.y,C_LBLU,4,2)
    spawnPopup(c.x,c.y-6,"+100",C_LBLU)
   end
   c.t=c.t+1
  end
 end

 -- Collect powerups
 for i=#powerups,1,-1 do
  local p=powerups[i]
  if p.alive then
   if overlap(pl.x,pl.y,pl.w,pl.h,p.x-3,p.y-3,9,9) then
    if p.type=="shard" then
     p.alive=false
     if not pl.big then
      pl.big=true; pl.y=pl.y-4
      invTimer=math.max(invTimer,25)
      spawnParticles(p.x,p.y,C_YEL,8,3)
      spawnPopup(p.x,p.y-8,"POWER UP!",C_YEL)
     else
      score=score+500
      spawnPopup(p.x,p.y-8,"+500",C_YEL)
     end
    elseif p.type=="fire" then
     p.alive=false; pl.fire=true
     spawnParticles(p.x,p.y,C_ORG,8,3)
     spawnPopup(p.x,p.y-8,"FIRE!",C_ORG)
    elseif p.type=="goal" then
     p.alive=false
     if lvlIdx<3 then
      score=score+1000
      spawnPopup(p.x,p.y-8,"+1000",C_YEL)
      lvlComplTimer=80
     elseif bossDefeated then
      score=score+2000
      lvlComplTimer=80
     end
    end
   end
  end
 end

 -- Level complete
 if lvlComplTimer>0 then
  lvlComplTimer=lvlComplTimer-1
  if lvlComplTimer<=0 then
   if lvlIdx<3 then
    local nextStory=(lvlIdx==1) and STORIES.after1 or STORIES.after2
    lvlIdx=lvlIdx+1
    loadLevel(lvlIdx)
    resetPlayer()
    startStory(nextStory, false)
   else
    startStory(STORIES.victory, true)
   end
  end
 end

 -- Update enemies
 for i=#enemies,1,-1 do
  local e=enemies[i]
  if e.alive then
   if e.type=="slime" then
    e.x=e.x+e.vx
    e.vy=e.vy+GRAV
    if e.vy>MAX_FALL then e.vy=MAX_FALL end
    e.y=e.y+e.vy
    local bt=math.floor((e.y+e.h-1)/TILE)
    for tx=math.floor(e.x/TILE),math.floor((e.x+e.w-1)/TILE) do
     if isSolid(tx,bt) then e.y=bt*TILE-e.h; e.vy=0 end
    end
    local fTx=math.floor((e.x+(e.vx>0 and e.w or 0))/TILE)
    local fTy=math.floor((e.y+e.h/2)/TILE)
    local fFloor=math.floor((e.y+e.h+1)/TILE)
    local eTx=math.floor((e.x+(e.vx>0 and e.w+1 or -1))/TILE)
    if isSolid(fTx,fTy) or not isSolid(eTx,fFloor) then
     e.vx=-e.vx
    end
    e.frame=e.frame+1
   elseif e.type=="bat" then
    e.t=e.t+0.05
    e.x=e.x+e.vx
    e.y=e.baseY+math.sin(e.t)*18
    local fTx=math.floor((e.x+(e.vx>0 and e.w or 0))/TILE)
    local fTy=math.floor((e.y+e.h/2)/TILE)
    if isSolid(fTx,fTy) or e.x<0 or e.x>TILE_W*TILE then
     e.vx=-e.vx
    end
    e.frame=e.frame+1
   end

   -- player-enemy collision
   if invTimer<=0 and not pl.dead then
    if overlap(pl.x,pl.y,pl.w,pl.h,e.x,e.y,e.w,e.h) then
     if pl.vy>0 and pl.y+pl.h-3<e.y+e.h/2 then
      e.alive=false; pl.vy=STOMP_B
      score=score+200
      shakeMag=math.max(shakeMag,3)
      spawnParticles(e.x+e.w/2,e.y+e.h/2,e.type=="slime" and C_GRN or C_DPUR,8,2)
      spawnPopup(e.x+e.w/2,e.y-6,"+200",C_YEL)
     else
      playerHit()
     end
    end
   end
  end
 end

 -- Boss
 if boss and boss.alive then updateBoss() end

 -- Fireballs
 for i=#fireballs,1,-1 do
  local f=fireballs[i]
  if f.alive then
   f.vy=f.vy+0.1
   f.x=f.x+f.vx; f.y=f.y+f.vy
   local fTy=math.floor((f.y+3)/TILE)
   local fTx=math.floor((f.x+2)/TILE)
   if isSolid(fTx,fTy) and f.vy>0 then
    f.vy=-2.5; f.bounces=f.bounces+1
   end
   if isSolid(fTx,math.floor(f.y/TILE)) then
    f.alive=false
    spawnParticles(f.x,f.y,C_ORG,3,2)
   end
   if f.bounces>3 or f.y>TILE_H*TILE or f.x<0 or f.x>TILE_W*TILE then
    f.alive=false
   end
   if f.alive then
    for j=#enemies,1,-1 do
     local e=enemies[j]
     if e.alive and overlap(f.x-2,f.y-2,7,7,e.x,e.y,e.w,e.h) then
      e.alive=false; f.alive=false
      score=score+200
      spawnParticles(e.x+e.w/2,e.y+e.h/2,C_ORG,6,2)
      spawnPopup(e.x,e.y-6,"+200",C_ORG)
     end
    end
    if f.alive and boss and boss.alive and overlap(f.x-2,f.y-2,7,7,boss.x,boss.y,boss.w,boss.h) then
     boss.hp=boss.hp-1; boss.flashTimer=8
     f.alive=false
     spawnParticles(f.x,f.y,C_RED,4,2)
     if boss.hp<=0 then bossDie() end
    end
   end
  end
 end

 -- Clean up dead fireballs
 for i=#fireballs,1,-1 do
  if not fireballs[i].alive then table.remove(fireballs,i) end
 end

 updateParticles()

 -- Screen shake
 if shakeMag>0.1 then
  shakeX=(math.random()-0.5)*shakeMag*2
  shakeY=(math.random()-0.5)*shakeMag*2
  shakeMag=shakeMag*0.82
 else
  shakeX=0; shakeY=0; shakeMag=0
 end
end

function updateParticles()
 for i=#particles,1,-1 do
  local p=particles[i]
  p.x=p.x+p.vx; p.y=p.y+p.vy
  p.vy=p.vy+0.07
  p.life=p.life-1
  if p.life<=0 then table.remove(particles,i) end
 end
 for i=#popups,1,-1 do
  local p=popups[i]
  p.y=p.y+p.vy; p.life=p.life-1
  if p.life<=0 then table.remove(popups,i) end
 end
end

-- ============================================================
-- DRAW HELPERS
-- ============================================================
function drawTiles()
 local ld=levelDefs[lvlIdx]
 local startX=math.max(0,math.floor(camX/TILE)-1)
 local endX=math.min(TILE_W-1,math.floor((camX+SW)/TILE)+1)

 for ty=0,TILE_H-1 do
  for tx=startX,endX do
   local t=getTile(tx,ty)
   local sx=math.floor(tx*TILE-camX)
   local sy=ty*TILE
   if t=="#" then
    rect(sx,sy,TILE,TILE,ld.tileCol1)
    rect(sx+1,sy+1,TILE-2,TILE-2,ld.tileCol2)
    -- mortar lines
    line(sx,sy,sx+TILE-1,sy,ld.tileCol1)
    line(sx,sy,sx,sy+TILE-1,ld.tileCol1)
   elseif t=="K" then
    rect(sx,sy,TILE,TILE,C_ORG2)
    rect(sx+1,sy+1,TILE-2,TILE-2,9)
    rectb(sx,sy,TILE,TILE,C_ORG)
   elseif t=="^" then
    -- spikes
    for si=0,1 do
     local bx=sx+si*3+1
     line(bx+1,sy,bx,sy+TILE-1,C_GRAY)
     line(bx+1,sy,bx+2,sy+TILE-1,C_GRAY)
    end
   end
  end
 end
end

function drawBackground()
 local ld=levelDefs[lvlIdx]
 cls(ld.bg)
 -- simple mountain silhouettes (parallax)
 local par=math.floor(camX*0.15)
 for i=0,9 do
  local bx=(i*28-par%28)-14
  local bh=12+math.floor(math.sin(i*1.3)*8)+5
  for dy=0,bh do
   local w2=math.floor((bh-dy)*14/bh)
   if w2>0 then
    line(bx+14-w2,SH-bh+dy,bx+14+w2,SH-bh+dy,ld.tileCol1)
   end
  end
 end
end

function drawPlayer()
 if pl.dead then
  local px=math.floor(pl.x-camX)
  drawSpr(SPR_KNIGHT_S,px,math.floor(pl.y),pl.facing<0)
  return
 end
 if invTimer>0 and math.floor(invTimer/3)%2==0 then return end

 local px=math.floor(pl.x-camX)
 local py=math.floor(pl.y)

 if pl.big then
  drawSpr(SPR_KNIGHT_B,px,py,pl.facing<0)
 else
  drawSpr(SPR_KNIGHT_S,px,py,pl.facing<0)
 end

 -- flash red on damage
 if damFlash>0 and damFlash%4<2 then
  local w=pl.big and 10 or 6
  local h=pl.big and 12 or 8
  for dy=0,h-1 do
   for dx=0,w-1 do
    if pix and false then end -- no-op, just rect overlay
   end
  end
  -- simple red overlay as rect
  local rc=px; if pl.facing<0 then rc=px end
  rectb(rc,py,pl.big and 10 or 6,pl.big and 12 or 8,C_RED)
 end

 -- sword slash
 if slash and slash.timer>0 then
  local sx=math.floor(slash.x-camX)
  local sy=math.floor(slash.y)
  drawSpr(SPR_SLASH,sx,sy,slash.facing<0)
 end

 -- fire aura when fire power
 if pl.fire then
  local fx=math.floor(pl.x-camX)+pl.w/2
  local fy=math.floor(pl.y)+pl.h/2
  pix(fx,fy-pl.h/2-1,C_ORG)
  pix(fx-1,fy-pl.h/2,C_YEL)
  pix(fx+1,fy-pl.h/2,C_YEL)
 end
end

function drawEnemies()
 for _,e in ipairs(enemies) do
  if e.alive then
   local ex=math.floor(e.x-camX)
   local ey=math.floor(e.y)
   if ex>-16 and ex<SW+16 then
    if e.type=="slime" then
     local squish=(math.floor(e.frame/6)%2==0) and 0 or 1
     drawSpr(SPR_SLIME,ex,ey+squish,false)
    elseif e.type=="bat" then
     local spr=(math.floor(e.frame/8)%2==0) and SPR_BAT or SPR_BAT2
     drawSpr(spr,ex,ey,e.vx>0)
    end
   end
  end
 end
end

function drawBoss()
 if not boss or not boss.alive then return end
 local bx=math.floor(boss.x-camX)
 local by_=math.floor(boss.y)
 if boss.flashTimer>0 and boss.flashTimer%2==0 then
  -- draw white flash
  for dy=0,boss.h-1 do
   for dx=0,boss.w-1 do
    pix(bx+dx,by_+dy,C_WHT)
   end
  end
 else
  drawSpr(SPR_BOSS,bx,by_,false)
 end
 -- health bar
 local bw=40
 local barX=bx+boss.w/2-bw/2
 local barY=by_-6
 rect(barX,barY,bw,3,C_DGRAY)
 rect(barX,barY,math.floor(bw*boss.hp/boss.maxHp),3,C_RED)
 rectb(barX,barY,bw,3,C_GRAY)
end

function drawCrystals()
 local t=time()/1000
 for _,c in ipairs(crystals) do
  if c.alive then
   local cx=math.floor(c.x-camX)
   local cy=math.floor(c.y+math.sin(c.t*0.05)*2)
   if cx>-8 and cx<SW+8 then
    drawSpr(SPR_CRYSTAL,cx,cy,false)
   end
  end
 end
end

function drawPowerups()
 local t=time()/1000
 for _,p in ipairs(powerups) do
  if p.alive then
   local px=math.floor(p.x-camX)
   local py=math.floor(p.y+math.sin(t*3)*2)
   if px>-8 and px<SW+8 then
    if p.type=="shard" then
     drawSpr(SPR_SHARD,px,py,false)
    elseif p.type=="fire" then
     drawSpr(SPR_FIRE,px,py,false)
    elseif p.type=="goal" then
     drawSpr(SPR_SHARD,px,py,false)
     -- glow ring
     circ(px+2,py+2,5,C_YEL)
    end
   end
  end
 end
end

function drawFireballs()
 for _,f in ipairs(fireballs) do
  if f.alive then
   local fx=math.floor(f.x-camX)
   local fy=math.floor(f.y)
   drawSpr(SPR_FIREBALL,fx,fy,false)
  end
 end
end

function drawParticles()
 for _,p in ipairs(particles) do
  local px=math.floor(p.x-camX)
  local py=math.floor(p.y)
  if px>=0 and px<SW and py>=0 and py<SH then
   pix(px,py,p.col)
  end
 end
end

function drawPopups()
 for _,p in ipairs(popups) do
  local px=math.floor(p.x-camX)
  local py=math.floor(p.y)
  if px>0 and px<SW-20 then
   print(p.txt,px,py,p.col,true,1,true)
  end
 end
end

function drawHUD()
 -- semi-transparent bar
 rect(0,0,SW,10,C_NAVY)
 -- lives
 print("x"..lives,8,2,C_RED,true,1,true)
 drawSpr(SPR_KNIGHT_S,2,1,false)
 -- score
 local sc="SC:"..score
 print(sc,SW/2-#sc*2,2,C_WHT,true,1,true)
 -- coins
 print(coins,SW-20,2,C_LBLU,true,1,true)
 drawSpr(SPR_CRYSTAL,SW-26,1,false)
 -- power indicator
 if pl.fire then
  print("FIRE",50,2,C_ORG,true,1,true)
 elseif pl.big then
  print("BIG",50,2,C_YEL,true,1,true)
 end
end

function drawScanlines()
 for y=0,SH-1,3 do
  line(0,y,SW-1,y,C_NAVY)
 end
end

-- ============================================================
-- TITLE SCREEN
-- ============================================================
function drawTitle()
 titleFrame=titleFrame+1
 cls(C_NAVY)
 -- starfield
 for i=1,25 do
  local sx=(i*47+titleFrame/3)%SW
  local sy=(i*31+math.sin(titleFrame*0.02+i)*8)%SH
  pix(sx,sy,C_GRAY)
 end
 -- orbiting crystals
 for i=0,2 do
  local a=titleFrame*0.03+i*2.09
  local ox=math.floor(SW/2+math.cos(a)*22)
  local oy=math.floor(SH/2+math.sin(a)*12-10)
  drawSpr(SPR_CRYSTAL,ox,oy,false)
 end
 -- knight
 local kb=math.floor(math.sin(titleFrame*0.05)*3)
 drawSpr(SPR_KNIGHT_B,SW/2-5,SH/2-20+kb,false)
 -- title text (smallfont=true means 6px chars; scale=1)
 -- "PIXEL KNIGHT" 12 chars * 6px = 72px wide, center = 120-36=84
 print("PIXEL KNIGHT",SW/2-36,SH/2-42,C_LBLU,false,1,false)
 -- "The Crystal Kingdom" 19*6=114, center=120-57=63
 print("The Crystal Kingdom",SW/2-57,SH/2-30,C_GRAY,false,1,true)
 -- blink
 if math.floor(titleFrame/25)%2==0 then
  -- "PRESS A TO START" 16*6=96, center=120-48=72
  print("PRESS A TO START",SW/2-48,SH/2+20,C_YEL,false,1,true)
 end
 print("A:Jump B:Sword/Fire",2,SH-8,C_DGRAY,false,1,true)
 drawScanlines()
 if inputStartJust() then
  -- start game
  score=0; lives=3; coins=0
  lvlIdx=1
  loadLevel(1)
  resetPlayer()
  startStory(STORIES.intro, false)
 end
end

-- ============================================================
-- STORY SCREEN DRAW
-- ============================================================
function drawStory()
 cls(C_NAVY)
 -- title of story
 print("Pixel Knight",SW/2-36,4,C_DPUR,false,1,false)
 line(0,11,SW,11,C_DPUR)

 local lines={}
 local s=storyText
 local idx=1
 while true do
  local nl=s:find("\n",idx,true)
  if nl then
   table.insert(lines,s:sub(idx,nl-1))
   idx=nl+1
  else
   if idx<=#s then table.insert(lines,s:sub(idx)) end
   break
  end
 end

 local startY=15
 for i,line in ipairs(lines) do
  if startY+(i-1)*8<SH-12 then
   print(line,4,startY+(i-1)*8,C_LBLU,true,1,true)
  end
 end

 if storyDone then
  if math.floor(storyTimer/25)%2==0 then
   print("A: CONTINUE",SW/2-22,SH-10,C_YEL,true,1,true)
  end
 else
  -- cursor blink
  if math.floor(storyTimer/15)%2==0 then
   local lastLine=lines[#lines] or ""
   pix(4+#lastLine*6,startY+(#lines-1)*8,C_YEL)
  end
 end
 drawScanlines()
end

-- ============================================================
-- GAMEOVER / WIN
-- ============================================================
function drawGameOver()
 titleFrame=titleFrame+1
 cls(C_RED)
 rect(0,0,SW,SH,C_DPUR)
 print("GAME OVER",SW/2-18,SH/2-20,C_RED,true,1,true)
 print("SCORE: "..score,SW/2-24,SH/2-5,C_WHT,true,1,true)
 print("COINS: "..coins,SW/2-20,SH/2+5,C_LBLU,true,1,true)
 if math.floor(titleFrame/25)%2==0 then
  print("A: RETRY",SW/2-16,SH/2+20,C_YEL,true,1,true)
 end
 drawScanlines()
 if inputStartJust() then state="START"; titleFrame=0 end
end

function drawWin()
 titleFrame=titleFrame+1
 cls(C_NAVY)
 -- celebration particles
 for i=1,15 do
  local px=(i*61+titleFrame*2)%SW
  local py=(i*37+titleFrame)%SH
  local colors={C_RED,C_GRN,C_LBLU,C_YEL,C_MAG}
  pix(px,py,colors[i%5+1])
 end
 print("VICTORY!",SW/2-16,SH/2-32,C_YEL,true,1,true)
 print("Crystal Kingdom saved!",SW/2-44,SH/2-18,C_LBLU,true,1,true)
 print("SCORE: "..score,SW/2-24,SH/2-4,C_WHT,true,1,true)
 print("COINS: "..coins,SW/2-20,SH/2+8,C_LBLU,true,1,true)
 if math.floor(titleFrame/25)%2==0 then
  print("A: TITLE",SW/2-16,SH/2+24,C_YEL,true,1,true)
 end
 drawScanlines()
 if inputStartJust() then state="START"; titleFrame=0 end
end

-- ============================================================
-- MAIN DRAW (PLAYING)
-- ============================================================
function drawPlaying()
 -- apply shake offset (clamp drawing)
 local ox=math.floor(shakeX)
 local oy=math.floor(shakeY)

 drawBackground()
 drawTiles()
 drawCrystals()
 drawPowerups()
 drawEnemies()
 drawBoss()
 drawFireballs()
 drawPlayer()
 drawParticles()
 drawPopups()
 drawHUD()

 -- damage flash overlay
 if damFlash>15 then
  for y=0,SH-1,2 do
   for x=0,SW-1,2 do
    if (x+y)%4==0 then pix(x,y,C_RED) end
   end
  end
 end

 drawScanlines()
end

-- ============================================================
-- TIC MAIN
-- ============================================================
function TIC()
 if state=="START" then
  drawTitle()
 elseif state=="STORY" then
  updateStory()
  drawStory()
 elseif state=="PLAYING" then
  updatePlaying()
  drawPlaying()
 elseif state=="GAMEOVER" then
  drawGameOver()
 elseif state=="WIN" then
  drawWin()
 end
 updatePrevBtns()
end

-- Init
titleFrame=0
state="START"
