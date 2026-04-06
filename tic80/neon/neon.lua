-- title: Neon Runner
-- author: retrogames
-- desc: Ghost Protocol - Cyberpunk Platformer
-- script: lua

-- ============================================================
-- NEON RUNNER: GHOST PROTOCOL  (TIC-80 port, 240x136)
-- Sweetie-16 palette indices used throughout:
--   0=dark navy  1=dark purple  2=red/crimson  3=orange
--   4=yellow     5=light green  6=green        7=dark teal
--   8=white      9=orange(alt) 10=mint green  11=light blue
--  12=magenta   13=teal       14=dark blue-gray 15=gray
-- ============================================================

-- ---- CONSTANTS ----
local TW,TH=240,136
local TS=4          -- tile size
local GRAV=0.35
local MAXFALL=6
local MSPD=1.8
local JFORCE=-5.5
local WSLIDE=0.9
local WJUMPY=-5.0
local WJUMPX=2.8
local DASHSPD=4.5
local DASHDUR=7
local DASHCD=28
local COYOTE=6
local JBUF=6
local IFRAMES=50
local SLASHRANGE=10
local SLASHCD=12
local EMPRAD=28
local EMPSTUN=100

-- ---- PALETTE color indices ----
local C_BG=0
local C_PUR=1
local C_RED=2
local C_ORG=3
local C_YEL=4
local C_LGN=5
local C_GRN=6
local C_TEA=7
local C_WHT=8
local C_MAG=12
local C_BLU=11
local C_AMB=9

-- ---- STATE ----
local gstate="START"
local gframe=0
local score=0
local lives=3
local clevel=0

-- ---- INPUT edges ----
local prev={}
local pressed={}
for i=0,7 do prev[i]=false pressed[i]=false end

local function updateInput()
  for i=0,7 do
    local cur=btn(i)
    pressed[i]=cur and not prev[i]
    prev[i]=cur
  end
end

local function anyPressed()
  for i=0,7 do if pressed[i] then return true end end
  return false
end

-- ---- LEVEL MAPS ----
-- 60 wide x 34 tall, scaled for 240x136 at TS=4
-- chars: #=solid P=player X=exit D=drone G=guard T=turret
--        C=chip H=health M=emp K=terminal L=laser E=electric
--        F=falling A=acid
local levelMaps={
-- LEVEL 1: ROOFTOP CHASE
{
"............................................................",
"............................................................",
"............................................................",
"....C.......C.......C.............C..........................",
"....#.......###.....###...........###......C................X",
"..D.....C.....D.....D.....C.........D.....###.....D.......##",
"......###.....###...###...###.....###...............###......",
"##........G...............................G..........G.......",
"..##....####..####...................F.F....####.............",
"....##...............................####..............G.....",
"......#.....G......G.......G......#..........G...............",
"......#####..######..######.......#####..###############....",
".P.....................................................#.....",
"####.........C..C..C...............................####.####.",
"...........###..###.###............................#.........",
"...........#.....#..#.....G......................#.#.........",
"...........#.....#..#...####.....................#..####.....",
"...........#####.####.#.....#.......C.C.C.......#...........",
"...............#......#.....#.......###.###......#...........",
"...............########.....#...........#........#...........",
"...........................#.............#.......#...........",
"...........................#.............#......############.",
"...........................###############....................",
"............................................................",
"###########################################################.",
},
-- LEVEL 2: CORPO TOWER
{
"............................................................",
"..C.....C..C.................................................",
"..###...##.##...................C..C.........................",
"...........#....................###.###...C...................",
"..........###.....K.............#...#...###...........X.....",
"..........#.....#######.........#.D.#..........D.....####...",
"...D......#.....#.....#.........#...#.................#.....",
"..........#.....#.....#.G.......#####.G................#....",
"..P.......#.....#.....#.........#...........F..F......#.....",
"######....#.G...#.....#########.#..........####......#......",
".....###..###...####.......#....###....G.............#......",
"..L.......L.L.......#.T....#.......####.........G..#........",
"..L.L.K.L.L.L.......#.......#...#.....#..........#.........",
"..L.....L...#.......#.......###..#.....##........#..........",
"..LLLLLLL...#.......###........###.....###......##..........",
"............#..#.....#...........#.......########..........",
"............####.....#####.......#..T.......................",
"................#.....#.G.#......####......................",
"................#.....#...#........#.G....................",
"................#.....#...#########.......................",
"................#######....................................",
"............................................................",
"###########################################################.",
},
-- LEVEL 3: THE UNDERGROUND
{
"............................................................",
"..C..C......................................................",
"..##.##......M......C..C.......C..C.........................",
"...#.#.....####.....##.##......##.##......G..........K......",
"....#.#....#..#....#...#.#....#...#.#....####....#######...",
".P...#.#..#...#..G.#...#..#..#...#..#...#....#..#.......#.",
"####.#.###.G..#....#...#...##.G..#..##.#......###.......###",
"....##.....####....##..#.......###..#..#.G...#.....C.C.....",
"......##.....#.....##.##.......#....#..####.#.....###.###..",
"......E.E.E..#.....#...#.E.E.E.#...#.......#.H......#..#..",
"......E.E.E..####.####.#.E.E.E.####.........#......####...",
"..............#....#....#.....#...#..........#...............",
"..............#....#....#.....#...#..........#..........X..",
"..............######....#######...###########.......######.",
"............................................................",
"###########################################################.",
},
}

local levelNames={"ROOFTOP CHASE","CORPO TOWER","THE UNDERGROUND"}
local levelColours={C_MAG,C_BLU,C_GRN}

-- ---- STORY SCREENS ----
local stories={
 intro={
  "> SYSTEM BREACH DETECTED",
  "> OPERATOR: KIRA-7",
  "",
  "You cracked NEXUS-Corp's",
  "mainframe. Stole everything.",
  "",
  "Project MINDGATE: neural",
  "control in every implant.",
  "Total obedience.",
  "",
  "Kill order issued.",
  "Every enforcer hunting you.",
  "",
  "One chance: reach the",
  "broadcast tower.",
  "Send the truth out.",
  "",
  "> OBJECTIVE: BROADCAST TOWER",
  "> STATUS: RUNNING...",
 },
 level2={
  "> ROOFTOPS CLEARED",
  "> ENTERING CORPO TOWER",
  "",
  "The tower's laser grid hums.",
  "Inside: the encryption key.",
  "",
  "Without it the broadcast",
  "is gibberish.",
  "",
  "Turrets. Drones. Lasers.",
  "You flex your cyber legs.",
  "",
  "Time to hack their own",
  "systems against them.",
  "",
  "> WARNING: HEAVY SECURITY",
 },
 level3={
  "> ENCRYPTION KEY ACQUIRED",
  "> DESCENDING TO UNDERGROUND",
  "",
  "Below the city: the pirate",
  "broadcast tower. Forgotten.",
  "",
  "NEXUS-Corp sent a CY-HOUND.",
  "Military grade.",
  "It stands between you",
  "and the tower.",
  "",
  "> FINAL OBJECTIVE: BROADCAST",
  "> SURVIVE.",
 },
 victory={
  "> BROADCAST INITIATED...",
  "> SIGNAL: MAXIMUM",
  "> RECIPIENT: EVERYONE",
  "",
  "Data streams across every",
  "screen, every implant,",
  "every network in Neo-Kyoto.",
  "",
  "MINDGATE exposed.",
  "Neural shackles dissolving.",
  "Citizens wake up.",
  "",
  "NEXUS-Corp crashes.",
  "The corpo-police stand down.",
  "",
  "The truth runs free.",
  "",
  "> CONNECTION: LIBERATED",
 },
}

-- ---- GAME OBJECTS ----
local map={}
local mapW,mapH=0,0
local player={}
local enemies={}
local bullets={}
local particles={}
local chips={}
local pickups={}
local terminals={}
local lasers={}
local fallingPlats={}
local camX,camY=0,0
local exitX,exitY=0,0
local boss=nil
local storyLines={}
local storyIdx=0
local storyChr=0
local storyRevealed=""
local storyDone=false
local storyNext=""  -- state to go to after story

-- ---- TILE HELPERS ----
local function tileAt(px,py)
  local tx=math.floor(px/TS)+1
  local ty=math.floor(py/TS)+1
  if tx<1 or tx>mapW or ty<1 or ty>mapH then return 1 end
  return map[ty][tx] or 0
end

local function solidAt(px,py)
  if tileAt(px,py)==1 then return true end
  for _,fp in ipairs(fallingPlats) do
    if fp.state~="falling" and fp.state~="respawn" then
      if px>=fp.x and px<fp.x+TS and py>=fp.y and py<fp.y+TS then
        return true
      end
    end
  end
  return false
end

local function overlap(ax,ay,aw,ah,bx,by,bw,bh)
  return ax<bx+bw and ax+aw>bx and ay<by+bh and ay+ah>by
end

-- ---- PARTICLES ----
local function spawnParts(x,y,col,n,spd,life)
  for i=1,n do
    local a=math.random()*6.28
    local s=math.random()*spd
    particles[#particles+1]={
      x=x,y=y,
      vx=math.cos(a)*s,vy=math.sin(a)*s-0.5,
      life=life,maxLife=life,col=col
    }
  end
end

-- ---- LOAD LEVEL ----
local function loadLevel(idx)
  local data=levelMaps[idx]
  map={}
  enemies={}
  bullets={}
  particles={}
  chips={}
  pickups={}
  terminals={}
  lasers={}
  fallingPlats={}
  boss=nil
  mapH=#data
  mapW=#data[1]

  for r=1,mapH do
    map[r]={}
    for c=1,mapW do
      local ch=data[r]:sub(c,c)
      map[r][c]=0
      if ch=="#" then
        map[r][c]=1
      elseif ch=="P" then
        player.x=(c-1)*TS
        player.y=(r-1)*TS
      elseif ch=="X" then
        exitX=(c-1)*TS
        exitY=(r-1)*TS
      elseif ch=="D" then
        enemies[#enemies+1]={type="drone",x=(c-1)*TS,y=(r-1)*TS,
          sx=(c-1)*TS,sy=(r-1)*TS,vx=0.7,hp=1,timer=0,stun=0,alive=true}
      elseif ch=="G" then
        enemies[#enemies+1]={type="guard",x=(c-1)*TS,y=(r-1)*TS,
          vx=0.6,vy=0,hp=2,timer=0,stun=0,alive=true,dir=1}
      elseif ch=="T" then
        enemies[#enemies+1]={type="turret",x=(c-1)*TS,y=(r-1)*TS,
          hp=3,timer=0,angle=0,stun=0,alive=true,hacked=false,hackTimer=0}
      elseif ch=="C" then
        chips[#chips+1]={x=(c-1)*TS+1,y=(r-1)*TS+1,alive=true}
      elseif ch=="H" then
        pickups[#pickups+1]={type="health",x=(c-1)*TS,y=(r-1)*TS,alive=true}
      elseif ch=="M" then
        pickups[#pickups+1]={type="emp",x=(c-1)*TS,y=(r-1)*TS,alive=true}
      elseif ch=="K" then
        terminals[#terminals+1]={x=(c-1)*TS,y=(r-1)*TS,hacked=false,hackTimer=0}
      elseif ch=="L" then
        lasers[#lasers+1]={x=(c-1)*TS,y=(r-1)*TS,active=true,timer=0}
      elseif ch=="E" then
        map[r][c]=2
      elseif ch=="F" then
        fallingPlats[#fallingPlats+1]={x=(c-1)*TS,y=(r-1)*TS,
          origY=(r-1)*TS,state="solid",timer=0}
        map[r][c]=0
      end
    end
  end

  -- Boss on level 3
  if idx==3 then
    boss={
      x=math.floor(mapW*TS*0.72),
      y=(mapH-4)*TS,
      vx=0,vy=0,
      hp=12,maxHp=12,
      state="patrol",timer=0,dir=-1,
      enraged=false,alive=true,stun=0,flash=0
    }
  end

  player.vx=0;player.vy=0
  player.onGround=false
  player.wallSlide=0
  player.facing=1
  player.dashing=0
  player.dashCd=0
  player.iframes=0
  player.slashCd=0
  player.slashing=0
  player.animFrame=0
  player.animTimer=0
  player.coyote=0
  player.jumpBuf=0

  camX=player.x-TW/2
  camY=player.y-TH/2
end

local function resetPlayer()
  player.hp=3
  player.emps=2
  player.vx=0;player.vy=0
  player.dashing=0;player.dashCd=0
  player.iframes=50
  player.slashCd=0;player.slashing=0
end

local function initGame()
  score=0;lives=3;clevel=1
  player={hp=3,emps=2}
  loadLevel(1)
  resetPlayer()
end

-- ---- HURT PLAYER ----
local function hurtPlayer()
  if player.iframes>0 or player.dashing>0 then return end
  player.hp=player.hp-1
  player.iframes=IFRAMES
  if player.hp<=0 then
    lives=lives-1
    if lives<=0 then
      gstate="GAMEOVER"
    else
      loadLevel(clevel)
      resetPlayer()
    end
  end
end

-- ---- START STORY ----
local function startStory(lines,nextState)
  storyLines=lines
  storyIdx=1
  storyChr=0
  storyRevealed=""
  storyDone=false
  storyNext=nextState
  gstate="STORY"
end

-- ---- COMPLETE LEVEL ----
local function completeLevel()
  clevel=clevel+1
  if clevel>3 then
    startStory(stories.victory,"WIN")
  elseif clevel==2 then
    startStory(stories.level2,"PLAY")
  elseif clevel==3 then
    startStory(stories.level3,"PLAY")
  end
end

-- ---- UPDATE GAME ----
local function updateGame()
  local pw=3;local ph=4

  -- Horizontal
  local mx=0
  if btn(2) then mx=-1 end
  if btn(3) then mx=1 end

  if player.dashing<=0 then
    player.vx=mx*MSPD
    if mx~=0 then player.facing=mx end
  end

  -- Gravity
  if player.dashing<=0 then
    player.vy=player.vy+GRAV
    if player.vy>MAXFALL then player.vy=MAXFALL end
  end

  -- Wall slide
  player.wallSlide=0
  if not player.onGround and player.dashing<=0 then
    local wr=solidAt(player.x+pw+1,player.y+1) or solidAt(player.x+pw+1,player.y+ph-1)
    local wl=solidAt(player.x-1,player.y+1) or solidAt(player.x-1,player.y+ph-1)
    if wr and mx>0 and player.vy>0 then
      player.vy=math.min(player.vy,WSLIDE);player.wallSlide=1
    end
    if wl and mx<0 and player.vy>0 then
      player.vy=math.min(player.vy,WSLIDE);player.wallSlide=-1
    end
  end

  -- Coyote
  if player.onGround then player.coyote=COYOTE
  elseif player.coyote>0 then player.coyote=player.coyote-1 end

  -- Jump buffer
  if pressed[4] then player.jumpBuf=JBUF
  elseif player.jumpBuf>0 then player.jumpBuf=player.jumpBuf-1 end

  -- Jump
  if player.jumpBuf>0 and player.dashing<=0 then
    if player.coyote>0 then
      player.vy=JFORCE;player.coyote=0;player.jumpBuf=0
      player.onGround=false
      sfx(0,40,8,0,8)
    elseif player.wallSlide~=0 then
      player.vx=-player.wallSlide*WJUMPX
      player.vy=WJUMPY
      player.facing=-player.wallSlide
      player.wallSlide=0;player.jumpBuf=0
      sfx(0,44,8,0,8)
    end
  end

  -- Variable jump height
  if not btn(4) and player.vy<-1.5 and player.dashing<=0 then
    player.vy=player.vy*0.55
  end

  -- Dash
  if pressed[5] and player.dashCd<=0 and player.dashing<=0 then
    player.dashing=DASHDUR;player.dashCd=DASHCD
    player.vx=player.facing*DASHSPD;player.vy=0
    sfx(1,50,6,1,8)
  end
  if player.dashing>0 then
    player.dashing=player.dashing-1
    if player.dashing<=0 then player.vx=mx*MSPD end
  end
  if player.dashCd>0 then player.dashCd=player.dashCd-1 end

  -- Move X
  local nx=player.x+player.vx
  if player.vx>0 then
    if solidAt(nx+pw,player.y+1) or solidAt(nx+pw,player.y+ph-1) then
      nx=math.floor((nx+pw)/TS)*TS-pw;player.vx=0
    end
  elseif player.vx<0 then
    if solidAt(nx,player.y+1) or solidAt(nx,player.y+ph-1) then
      nx=math.floor(nx/TS)*TS+TS;player.vx=0
    end
  end
  player.x=nx

  -- Move Y
  local ny=player.y+player.vy
  player.onGround=false
  if player.vy>0 then
    if solidAt(player.x+1,ny+ph) or solidAt(player.x+pw-1,ny+ph) then
      ny=math.floor((ny+ph)/TS)*TS-ph;player.vy=0;player.onGround=true
    end
  elseif player.vy<0 then
    if solidAt(player.x+1,ny) or solidAt(player.x+pw-1,ny) then
      ny=math.floor(ny/TS)*TS+TS;player.vy=0
    end
  end
  player.y=ny

  -- Falling platforms trigger
  for _,fp in ipairs(fallingPlats) do
    if fp.state=="solid" and player.onGround then
      if player.x+pw>fp.x and player.x<fp.x+TS and
         math.abs(player.y+ph-fp.y)<2 then
        fp.state="shaking";fp.timer=20
      end
    end
    if fp.state=="shaking" then
      fp.timer=fp.timer-1
      if fp.timer<=0 then fp.state="falling" end
    elseif fp.state=="falling" then
      fp.y=fp.y+3
      if fp.y>mapH*TS+40 then fp.state="respawn";fp.timer=120 end
    elseif fp.state=="respawn" then
      fp.timer=fp.timer-1
      if fp.timer<=0 then fp.state="solid";fp.y=fp.origY end
    end
  end

  if player.iframes>0 then player.iframes=player.iframes-1 end

  -- Attack / hack
  if pressed[6] and player.slashCd<=0 then
    local hacked=false
    for _,t in ipairs(terminals) do
      if not t.hacked then
        local dx=math.abs((player.x+pw/2)-(t.x+TS/2))
        local dy=math.abs((player.y+ph/2)-(t.y+TS/2))
        if dx<10 and dy<10 then
          t.hacked=true;t.hackTimer=300
          hacked=true
          sfx(2,60,12,2,8)
          for _,l in ipairs(lasers) do
            if math.abs(l.x-t.x)+math.abs(l.y-t.y)<TS*14 then
              l.active=false;l.timer=300
            end
          end
          for _,e in ipairs(enemies) do
            if e.type=="turret" and e.alive then
              if math.abs(e.x-t.x)+math.abs(e.y-t.y)<TS*14 then
                e.hacked=true;e.hackTimer=300;e.stun=300
              end
            end
          end
          break
        end
      end
    end
    if not hacked then
      player.slashing=8;player.slashCd=SLASHCD
      sfx(3,70,6,0,8)
      local sx=player.x+(player.facing>0 and pw or -SLASHRANGE)
      local sy=player.y-2
      for i,e in ipairs(enemies) do
        if e.alive then
          local ew=TS;local eh=(e.type=="guard" and TS+1 or TS)
          if overlap(sx,sy,SLASHRANGE,ph+3,e.x,e.y,ew,eh) then
            e.hp=e.hp-1
            if e.hp<=0 then
              e.alive=false
              score=score+(e.type=="guard" and 200 or 150)
              spawnParts(e.x+2,e.y+2,e.type=="drone" and C_BLU or C_MAG,8,2,20)
              sfx(4,30,10,1,8)
            else
              spawnParts(e.x+2,e.y+2,C_WHT,4,1.5,10)
              sfx(4,40,6,0,8)
            end
          end
        end
      end
      -- Boss slash
      if boss and boss.alive then
        if overlap(sx,sy,SLASHRANGE,ph+3,boss.x,boss.y,8,6) then
          boss.hp=boss.hp-1;boss.flash=6
          if boss.hp<=boss.maxHp/2 and not boss.enraged then
            boss.enraged=true
          end
          if boss.hp<=0 then
            boss.alive=false;score=score+1000
            spawnParts(boss.x+4,boss.y+3,C_MAG,20,3,35)
            sfx(4,20,15,1,8)
          else
            spawnParts(boss.x+4,boss.y+3,C_WHT,6,2,12)
            sfx(4,40,6,0,8)
          end
        end
      end
    end
  end
  if player.slashing>0 then player.slashing=player.slashing-1 end
  if player.slashCd>0 then player.slashCd=player.slashCd-1 end

  -- EMP
  if pressed[7] and player.emps>0 then
    player.emps=player.emps-1
    sfx(5,50,14,2,8)
    local ex=player.x+pw/2+player.facing*22
    local ey=player.y
    spawnParts(ex,ey,C_BLU,16,3,22)
    for _,e in ipairs(enemies) do
      if e.alive then
        local dx=e.x-ex;local dy=e.y-ey
        if math.sqrt(dx*dx+dy*dy)<EMPRAD then
          e.stun=EMPSTUN
        end
      end
    end
    if boss and boss.alive then
      local dx=boss.x-ex;local dy=boss.y-ey
      if math.sqrt(dx*dx+dy*dy)<EMPRAD then
        boss.stun=EMPSTUN;boss.state="stunned";boss.timer=EMPSTUN
      end
    end
  end

  -- Animation
  player.animTimer=player.animTimer+1
  if player.animTimer>6 then
    player.animTimer=0;player.animFrame=1-player.animFrame
  end

  -- ---- ENEMIES ----
  for _,e in ipairs(enemies) do
    if e.alive then
      if e.stun>0 then
        e.stun=e.stun-1
      else
        if e.type=="drone" then
          e.x=e.x+e.vx
          e.y=e.sy+math.sin(gframe*0.06)*6
          if solidAt(e.x+TS+1,e.y+2) or solidAt(e.x-1,e.y+2) then e.vx=-e.vx end
          if math.abs(e.x-e.sx)>50 then e.vx=-e.vx end
          e.timer=e.timer+1
          if e.timer>=80 then
            e.timer=0
            local dx=player.x-e.x;local dy=player.y-e.y
            local d=math.sqrt(dx*dx+dy*dy)
            if d<120 and d>0 then
              bullets[#bullets+1]={x=e.x+2,y=e.y+2,
                vx=(dx/d)*2,vy=(dy/d)*2,life=90}
              sfx(6,80,4,0,8)
            end
          end

        elseif e.type=="guard" then
          -- simple gravity
          if not solidAt(e.x+2,e.y+TS+1) then
            e.vy=(e.vy or 0)+GRAV
            if e.vy>MAXFALL then e.vy=MAXFALL end
          else
            e.vy=0
          end
          e.y=e.y+(e.vy or 0)

          local spd=e.dir*0.7
          local nx2=e.x+spd
          local ahead=e.x+(e.dir>0 and TS+1 or -1)
          if solidAt(ahead,e.y+2) or not solidAt(ahead,e.y+TS+1) then
            e.dir=-e.dir
          else
            e.x=nx2
          end
          e.timer=e.timer+1
          if e.timer>=100 then
            e.timer=0
            local dx=player.x-e.x;local dy=player.y-e.y
            local d=math.sqrt(dx*dx+dy*dy)
            if d<100 and d>0 then
              bullets[#bullets+1]={x=e.x+2,y=e.y+2,
                vx=(dx/d)*1.8,vy=(dy/d)*1.8,life=80}
              sfx(6,80,4,0,8)
            end
          end

        elseif e.type=="turret" then
          if e.hacked then
            e.hackTimer=e.hackTimer-1
            if e.hackTimer<=0 then e.hacked=false;e.stun=0 end
          else
            e.angle=(e.angle or 0)+0.025
            e.timer=e.timer+1
            if e.timer>=50 then
              e.timer=0
              bullets[#bullets+1]={x=e.x+2,y=e.y+2,
                vx=math.cos(e.angle)*2.2,vy=math.sin(e.angle)*2.2,life=70}
              sfx(6,80,3,0,8)
            end
          end
        end

        -- Touch damage
        if player.iframes<=0 and player.dashing<=0 then
          if overlap(player.x,player.y,pw,ph,e.x,e.y,TS,TS) then
            hurtPlayer()
          end
        end
      end
    end
  end

  -- ---- BOSS ----
  if boss and boss.alive then
    if boss.stun>0 then
      boss.stun=boss.stun-1
      if boss.stun<=0 then boss.state="patrol" end
    else
      local spd=boss.enraged and 2.2 or 1.3
      boss.timer=boss.timer+1
      if boss.state=="patrol" then
        boss.x=boss.x+boss.dir*spd
        if solidAt(boss.x+9,boss.y+3) or solidAt(boss.x-1,boss.y+3) then
          boss.dir=-boss.dir
        end
        if not solidAt(boss.x+4,boss.y+7) then boss.y=boss.y+2 end
        if boss.timer>(boss.enraged and 50 or 100) then
          boss.timer=0
          local dx=player.x-boss.x
          if math.abs(dx)<100 then
            boss.state="charge";boss.dir=(dx>0 and 1 or -1)
          else
            boss.state="leap"
            boss.vy=-6
            boss.vx=(player.x>boss.x and 1 or -1)*2.5
          end
        end
      elseif boss.state=="charge" then
        boss.x=boss.x+boss.dir*spd*2
        if not solidAt(boss.x+4,boss.y+7) then boss.y=boss.y+2 end
        if solidAt(boss.x+9,boss.y+3) or solidAt(boss.x-1,boss.y+3) or boss.timer>30 then
          boss.state="patrol";boss.timer=0
        end
      elseif boss.state=="leap" then
        boss.x=boss.x+boss.vx;boss.y=boss.y+boss.vy
        boss.vy=boss.vy+0.4
        if boss.vy>0 and solidAt(boss.x+4,boss.y+7) then
          boss.y=math.floor((boss.y+7)/TS)*TS-7
          boss.vy=0;boss.state="patrol";boss.timer=0
          spawnParts(boss.x+4,boss.y+6,C_TEA,6,2,12)
        end
      end
    end
    if boss.flash>0 then boss.flash=boss.flash-1 end
    -- Boss touch
    if player.iframes<=0 and player.dashing<=0 then
      if overlap(player.x,player.y,pw,ph,boss.x,boss.y,8,6) then
        hurtPlayer()
      end
    end
  end

  -- ---- BULLETS ----
  local i=#bullets
  while i>0 do
    local b=bullets[i]
    b.x=b.x+b.vx;b.y=b.y+b.vy;b.life=b.life-1
    if b.life<=0 or solidAt(b.x,b.y) then
      table.remove(bullets,i)
    else
      if player.iframes<=0 and player.dashing<=0 then
        if overlap(player.x,player.y,pw,ph,b.x-1,b.y-1,3,3) then
          hurtPlayer()
          table.remove(bullets,i)
        end
      end
    end
    i=i-1
  end

  -- ---- LASERS ----
  for _,l in ipairs(lasers) do
    if l.active then
      l.timer=l.timer+1
      if l.timer>=80 then l.timer=0;l.active=false end
    else
      l.timer=l.timer+1
      if l.timer>=40 then l.timer=0;l.active=true end
    end
    if l.active and player.iframes<=0 and player.dashing<=0 then
      if overlap(player.x,player.y,pw,ph,l.x+1,l.y,2,TS) then
        hurtPlayer()
      end
    end
  end

  -- ---- TERMINALS ----
  for _,t in ipairs(terminals) do
    if t.hacked then
      t.hackTimer=t.hackTimer-1
      if t.hackTimer<=0 then
        t.hacked=false
        for _,l in ipairs(lasers) do
          if math.abs(l.x-t.x)+math.abs(l.y-t.y)<TS*14 then
            l.active=true
          end
        end
      end
    end
  end

  -- ---- PICKUPS ----
  for _,c in ipairs(chips) do
    if c.alive and overlap(player.x,player.y,pw,ph,c.x-2,c.y-2,4,4) then
      c.alive=false;score=score+50
      spawnParts(c.x,c.y,C_AMB,5,1.5,15)
      sfx(7,70,8,2,8)
    end
  end
  for _,p in ipairs(pickups) do
    if p.alive and overlap(player.x,player.y,pw,ph,p.x,p.y,TS,TS) then
      p.alive=false
      if p.type=="health" and player.hp<3 then
        player.hp=player.hp+1
        spawnParts(p.x+2,p.y+2,C_RED,6,1.5,15)
      elseif p.type=="emp" then
        player.emps=player.emps+1
        spawnParts(p.x+2,p.y+2,C_BLU,6,1.5,15)
      end
      sfx(7,70,8,2,8)
    end
  end

  -- ---- ELECTRIC FLOOR ----
  for ty=1,mapH do
    for tx=1,mapW do
      if map[ty][tx]==2 then
        if player.iframes<=0 and player.dashing<=0 then
          if overlap(player.x,player.y,pw,ph,(tx-1)*TS,(ty-1)*TS,TS,TS) then
            hurtPlayer()
          end
        end
      end
    end
  end

  -- ---- EXIT ----
  if overlap(player.x,player.y,pw,ph,exitX,exitY,TS,TS) then
    if not (boss and boss.alive) then
      completeLevel()
      return
    end
  end

  -- ---- PARTICLES ----
  local pi=#particles
  while pi>0 do
    local p=particles[pi]
    p.x=p.x+p.vx;p.y=p.y+p.vy;p.vy=p.vy+0.04;p.life=p.life-1
    if p.life<=0 then table.remove(particles,pi) end
    pi=pi-1
  end

  -- Fall death
  if player.y>mapH*TS+30 then
    player.hp=0;hurtPlayer()
  end

  -- Camera
  local tcx=player.x-TW/2+player.facing*16
  local tcy=player.y-TH/2+12
  camX=camX+(tcx-camX)*0.12
  camY=camY+(tcy-camY)*0.10
  camX=math.max(0,math.min(mapW*TS-TW,camX))
  camY=math.max(0,math.min(mapH*TS-TH,camY))
end

-- ---- DRAW HELPERS ----
local function drawPlayer()
  local pw=3;local ph=4
  local px=math.floor(player.x-camX)
  local py=math.floor(player.y-camY)
  if player.iframes>0 and gframe%4<2 then return end

  -- Body
  local bc=C_MAG
  if player.dashing>0 then bc=C_BLU end
  -- torso
  rect(px,py+1,pw,ph-1,bc)
  -- head
  rect(px,py,pw,1,C_WHT)
  -- legs
  if player.onGround and math.abs(player.vx)>0.4 then
    local lf=math.floor(gframe/6)%2
    pix(px+(lf==0 and 0 or 2),py+ph,C_BLU)
    pix(px+(lf==0 and 2 or 0),py+ph+1,C_BLU)
  else
    pix(px,py+ph,C_BLU)
    pix(px+2,py+ph,C_BLU)
  end

  -- Slash arc
  if player.slashing>0 then
    local alpha=player.slashing/8
    local sx=px+(player.facing>0 and pw+1 or -6)
    line(sx,py-1,sx+(player.facing>0 and 5 or -5),py+ph+1,C_BLU)
    line(sx+player.facing,py,sx+(player.facing>0 and 6 or -6),py+ph,C_WHT)
  end
end

local function drawHUD()
  -- HP pips
  for i=1,3 do
    local c=i<=player.hp and C_MAG or C_PUR
    rect(2+(i-1)*8,2,6,3,c)
  end
  -- EMP
  print("E:"..player.emps,2,8,C_BLU,false,1,false)
  -- Score
  local sc=string.format("%05d",score)
  print(sc,TW-30,2,C_AMB,false,1,false)
  -- Level name
  local lname=levelNames[clevel] or ""
  print(lname,math.floor((TW-#lname*4)/2),2,levelColours[clevel] or C_MAG,false,1,false)
  -- Dash bar
  if player.dashCd>0 then
    rect(2,13,math.floor(20*(1-player.dashCd/DASHCD)),1,C_GRN)
  else
    rect(2,13,20,1,C_GRN)
  end
  -- Boss bar
  if boss and boss.alive then
    rect(40,TH-6,TW-80,3,C_PUR)
    local bw=math.floor((TW-80)*boss.hp/boss.maxHp)
    rect(40,TH-6,bw,3,boss.enraged and C_RED or C_MAG)
    print("CY-HOUND",40,TH-13,C_MAG,false,1,false)
  end
end

local function drawMap()
  local lc=levelColours[clevel] or C_MAG
  local startC=math.max(1,math.floor(camX/TS)+1)
  local endC=math.min(mapW,math.ceil((camX+TW)/TS)+1)
  local startR=math.max(1,math.floor(camY/TS)+1)
  local endR=math.min(mapH,math.ceil((camY+TH)/TS)+1)

  for r=startR,endR do
    for c=startC,endC do
      local tx=(c-1)*TS-math.floor(camX)
      local ty=(r-1)*TS-math.floor(camY)
      local cell=map[r][c]
      if cell==1 then
        rect(tx,ty,TS,TS,C_PUR)
        rect(tx,ty,TS,1,lc) -- neon top edge
        pix(tx,ty,C_BLU)   -- corner accent
      elseif cell==2 then
        -- electric floor
        local spark=math.floor(gframe*0.3+c)%2==0
        rect(tx,ty,TS,TS,C_PUR)
        rect(tx+1,ty+1,TS-2,TS-2,spark and C_MAG or 1)
      end
    end
  end
end

local function drawObjects()
  -- Falling platforms
  for _,fp in ipairs(fallingPlats) do
    if fp.state~="respawn" then
      local fx=math.floor(fp.x-camX)
      local fy=math.floor(fp.y-camY)
      local off=fp.state=="shaking" and (gframe%2==0 and 1 or -1) or 0
      rect(fx+off,fy,TS,TS,C_ORG)
      rect(fx+off,fy,TS,1,C_AMB)
    end
  end

  -- Lasers
  for _,l in ipairs(lasers) do
    local lx=math.floor(l.x-camX)
    local ly=math.floor(l.y-camY)
    if l.active then
      rect(lx+1,ly,2,TS,C_RED)
    else
      rect(lx+1,ly,1,TS,C_PUR)
    end
  end

  -- Terminals
  for _,t in ipairs(terminals) do
    local tx2=math.floor(t.x-camX)
    local ty2=math.floor(t.y-camY)
    rect(tx2,ty2,TS,TS,C_TEA)
    rect(tx2+1,ty2+1,TS-2,TS-2,t.hacked and C_GRN or 7)
    pix(tx2+1,ty2+1,t.hacked and C_LGN or C_GRN)
  end

  -- Chips
  local bob=math.floor(gframe/8)%2
  for _,c in ipairs(chips) do
    if c.alive then
      local cx=math.floor(c.x-camX)
      local cy=math.floor(c.y-camY)-bob
      rect(cx-1,cy-1,3,3,C_AMB)
      pix(cx,cy,C_YEL)
    end
  end

  -- Pickups
  for _,p in ipairs(pickups) do
    if p.alive then
      local px2=math.floor(p.x-camX)
      local py2=math.floor(p.y-camY)-bob
      if p.type=="health" then
        rect(px2+1,py2,2,TS,C_RED)
        rect(px2,py2+1,TS,2,C_RED)
      else
        circ(px2+2,py2+2,2,C_BLU)
        pix(px2+2,py2+2,C_WHT)
      end
    end
  end

  -- Exit
  local ex2=math.floor(exitX-camX)
  local ey2=math.floor(exitY-camY)-bob
  local canExit=not (boss and boss.alive)
  rect(ex2,ey2,TS,TS,canExit and C_GRN or C_TEA)
  if canExit then
    pix(ex2+2,ey2+2,C_LGN)
    line(ex2,ey2,ex2+TS-1,ey2,C_LGN)
  end

  -- Enemies
  for _,e in ipairs(enemies) do
    if e.alive then
      if e.stun>0 and gframe%4<2 then -- flash when stunned
      else
        local ex3=math.floor(e.x-camX)
        local ey3=math.floor(e.y-camY)
        if e.type=="drone" then
          circ(ex3+2,ey3+2,2,C_BLU)
          rect(ex3+1,ey3+3,2,1,C_WHT)
          if e.hacked then rectb(ex3-1,ey3-1,TS+2,TS+2,C_GRN) end
        elseif e.type=="guard" then
          rect(ex3,ey3,TS,TS,C_TEA)
          rect(ex3+1,ey3,2,2,C_WHT)    -- head
          rect(ex3,ey3+2,TS,2,2)       -- body (red)
          pix(ex3,ey3+TS,C_TEA)
          pix(ex3+TS-1,ey3+TS,C_TEA)
        else -- turret
          rect(ex3,ey3,TS,TS,C_ORG)
          circ(ex3+2,ey3+2,2,C_RED)
          -- barrel direction
          local bx=ex3+2+math.floor(math.cos(e.angle or 0)*2)
          local by=ey3+2+math.floor(math.sin(e.angle or 0)*2)
          line(ex3+2,ey3+2,bx,by,C_AMB)
          if e.hacked then rectb(ex3-1,ey3-1,TS+2,TS+2,C_GRN) end
        end
      end
    end
  end

  -- Boss
  if boss and boss.alive then
    if not (boss.stun>0 and gframe%4<2) then
      if not (boss.flash>0 and gframe%2==0) then
        local bx=math.floor(boss.x-camX)
        local by=math.floor(boss.y-camY)
        -- body
        rect(bx,by,8,5,C_PUR)
        -- head
        rect(bx+1,by-2,6,2,1)
        pix(bx+2,by-2,C_MAG);pix(bx+5,by-2,C_MAG)
        -- legs
        rect(bx+1,by+5,2,2,C_PUR)
        rect(bx+5,by+5,2,2,C_PUR)
        -- eye glow
        if boss.enraged then
          pix(bx+2,by,C_RED);pix(bx+5,by,C_RED)
        else
          pix(bx+2,by,C_MAG);pix(bx+5,by,C_MAG)
        end
      end
    end
  end

  -- Bullets
  for _,b in ipairs(bullets) do
    local bx2=math.floor(b.x-camX)
    local by2=math.floor(b.y-camY)
    pix(bx2,by2,C_MAG)
    pix(bx2+1,by2,C_RED)
  end

  -- Particles
  for _,p in ipairs(particles) do
    if p.life>0 then
      local alpha=p.life/p.maxLife
      if alpha>0.5 then
        pix(math.floor(p.x-camX),math.floor(p.y-camY),p.col)
      end
    end
  end
end

-- ---- DRAW STORY ----
local function drawStory()
  cls(0)
  -- Border
  rectb(2,2,TW-4,TH-4,C_GRN)
  rectb(3,3,TW-6,TH-6,C_TEA)
  -- Header
  print("NEO-KYOTO TERMINAL",8,6,C_GRN,false,1,false)
  line(8,13,TW-8,13,C_TEA)

  -- Story text
  local lines={}
  for s in (storyRevealed.."\n"):gmatch("([^\n]*)\n") do
    lines[#lines+1]=s
  end
  local maxLines=math.floor((TH-30)/8)
  local startL=math.max(1,#lines-maxLines)
  for i=startL,#lines do
    print(lines[i],8,16+(i-startL)*8,C_GRN,false,1,false)
  end

  -- Cursor blink
  if gframe%30<15 and not storyDone then
    local last=lines[#lines] or ""
    print("_",8+#last*4,16+(#lines-startL)*8,C_GRN,false,1,false)
  end

  -- Continue prompt
  if storyDone then
    if gframe%40<20 then
      print("PRESS ANY KEY",math.floor(TW/2)-26,TH-12,C_AMB,false,1,false)
    end
  end
end

-- ---- DRAW TITLE ----
local function drawTitle()
  cls(0)
  -- Parallax city silhouette
  for i=0,14 do
    local h=20+math.floor((i*53)%30)
    local bx=(i*16+math.floor(gframe*0.1))%TW
    rect(bx,TH-h,14,h,C_PUR)
    -- windows
    for wy=TH-h+2,TH-4,5 do
      for wx=bx+2,bx+12,4 do
        if (wx*3+wy)%3~=0 then
          local on=math.floor(gframe*0.02+wx*0.2+wy*0.1)%4>0
          if on then pix(wx,wy,C_AMB) end
        end
      end
    end
  end

  -- Ground line
  line(0,TH-18,TW,TH-18,C_MAG)

  -- Title glitch
  local gloff=0
  if math.sin(gframe*0.15)>0.85 then
    gloff=math.random(-2,2)
  end
  if gloff~=0 then
    print("NEON RUNNER",math.floor(TW/2)-22+gloff-1,38,C_BLU,false,1,false)
    print("NEON RUNNER",math.floor(TW/2)-22+gloff+1,38,C_RED,false,1,false)
  end
  print("NEON RUNNER",math.floor(TW/2)-22,38,C_WHT,false,1,false)
  print("GHOST PROTOCOL",math.floor(TW/2)-28,50,C_GRN,false,1,false)

  -- Blink
  if gframe%60<30 then
    print("PRESS ANY KEY TO JACK IN",math.floor(TW/2)-48,80,C_AMB,false,1,false)
  end

  -- Controls
  print("ARROWS:MOVE A:JUMP B:DASH X:ATK Y:EMP",1,TH-10,C_TEA,false,1,false)
end

-- ---- DRAW GAMEOVER ----
local function drawGameOver()
  cls(0)
  -- Static noise
  for i=1,40 do
    local nx=math.random(0,TW-1)
    local ny=math.random(0,TH-1)
    pix(nx,ny,math.random(1,6))
  end

  local gx=0
  if math.sin(gframe*0.2)>0.8 then gx=math.random(-2,2) end
  print("CONNECTION",math.floor(TW/2)-20+gx-1,50,C_BLU,false,1,false)
  print("CONNECTION",math.floor(TW/2)-20+gx+1,50,C_RED,false,1,false)
  print("CONNECTION",math.floor(TW/2)-20+gx,50,C_WHT,false,1,false)
  print("LOST",math.floor(TW/2)-8,62,C_MAG,false,1,false)

  print("SCORE:"..string.format("%05d",score),math.floor(TW/2)-22,78,C_AMB,false,1,false)

  if gframe%60<30 then
    print("PRESS ANY KEY",math.floor(TW/2)-26,95,C_GRN,false,1,false)
  end
end

-- ---- DRAW WIN ----
local function drawWin()
  cls(0)
  -- Celebration particles
  for i=1,20 do
    local nx=math.floor(gframe*i*0.7)%TW
    local ny=math.floor(gframe*0.5+i*8)%TH
    pix(nx,ny,i%12+1)
  end

  print("TRUTH",math.floor(TW/2)-10,40,C_GRN,false,1,false)
  print("BROADCAST",math.floor(TW/2)-18,52,C_GRN,false,1,false)

  print("FINAL SCORE:"..string.format("%05d",score),
    math.floor(TW/2)-38,68,C_AMB,false,1,false)

  print("THE SIGNAL IS OUT.",math.floor(TW/2)-36,82,C_BLU,false,1,false)
  print("NEO-KYOTO IS FREE.",math.floor(TW/2)-36,90,C_BLU,false,1,false)

  if gframe%60<30 then
    print("JACK IN AGAIN?",math.floor(TW/2)-28,108,C_MAG,false,1,false)
  end
end

-- ---- UPDATE STORY ----
local function updateStory()
  if anyPressed() then
    if not storyDone then
      -- reveal all
      storyRevealed=""
      for _,ln in ipairs(storyLines) do
        storyRevealed=storyRevealed..ln.."\n"
      end
      storyDone=true
    else
      -- advance
      if storyNext=="PLAY" then
        loadLevel(clevel)
        resetPlayer()
        gstate="PLAYING"
      elseif storyNext=="WIN" then
        gstate="WIN"
      end
    end
    return
  end

  -- Typewriter: 2 chars per frame
  if not storyDone then
    if storyIdx<=#storyLines then
      local line=storyLines[storyIdx]
      storyChr=storyChr+2
      if storyChr>#line then
        -- advance to next line
        local consumed=line:sub(1)
        storyRevealed=storyRevealed..consumed.."\n"
        storyIdx=storyIdx+1
        storyChr=0
        if storyIdx>#storyLines then storyDone=true end
      else
        -- partial reveal: rebuild storyRevealed each frame with partial last line
        -- We keep a base and append current partial
      end
    end
  end
end

-- Improved typewriter: track base text + partial
local storyBase=""
local storyPartial=""

local function startStory2(lines,nextState)
  storyLines=lines
  storyIdx=1
  storyChr=0
  storyBase=""
  storyPartial=""
  storyRevealed=""
  storyDone=false
  storyNext=nextState
  gstate="STORY"
end

local function updateStory2()
  if anyPressed() then
    if not storyDone then
      storyBase=""
      for _,ln in ipairs(storyLines) do storyBase=storyBase..ln.."\n" end
      storyRevealed=storyBase
      storyDone=true
    else
      if storyNext=="PLAY" then
        loadLevel(clevel)
        resetPlayer()
        gstate="PLAYING"
      elseif storyNext=="WIN" then
        gstate="WIN"
      end
    end
    return
  end

  if not storyDone then
    if gframe%2==0 then
      if storyIdx<=#storyLines then
        local line=storyLines[storyIdx]
        storyChr=storyChr+1
        if storyChr>#line then
          storyBase=storyBase..line.."\n"
          storyIdx=storyIdx+1
          storyChr=0
          storyPartial=""
          if storyIdx>#storyLines then storyDone=true end
        else
          storyPartial=line:sub(1,storyChr)
        end
      end
    end
    storyRevealed=storyBase..storyPartial
  end
end

-- override with improved version
startStory=startStory2
local updateStoryFn=updateStory2

-- ---- MAIN TIC LOOP ----
function TIC()
  gframe=gframe+1
  updateInput()

  if gstate=="START" then
    drawTitle()
    if anyPressed() then
      startStory(stories.intro,"PLAY")
      initGame()
    end

  elseif gstate=="STORY" then
    updateStoryFn()
    drawStory()

  elseif gstate=="PLAYING" then
    updateGame()
    -- Draw
    cls(C_BG)
    -- Background buildings
    for i=0,11 do
      local bh=12+math.floor((i*47)%20)
      local bx=(i*22-math.floor(camX*0.08))%TW
      if bx>TW then bx=bx-TW end
      rect(bx,TH-bh-4,18,bh,C_PUR)
      -- windows
      for wy=TH-bh,TH-8,4 do
        for wx=bx+1,bx+16,4 do
          if (wx+wy)%3~=0 then
            local on=math.floor(gframe*0.015+wx*0.15+wy*0.08)%5>0
            if on then pix(wx,wy,C_AMB) end
          end
        end
      end
    end
    -- Rain
    for i=0,19 do
      local rx=(i*31+math.floor(gframe*0.3+i*7))%TW
      local ry=(math.floor(gframe*1.5+i*23))%(TH+10)-5
      line(rx,ry,rx-1,ry+3,C_BLU)
    end

    drawMap()
    drawObjects()
    drawPlayer()
    drawHUD()

  elseif gstate=="GAMEOVER" then
    drawGameOver()
    if anyPressed() then gstate="START" end

  elseif gstate=="WIN" then
    drawWin()
    if anyPressed() then gstate="START" end
  end
end
