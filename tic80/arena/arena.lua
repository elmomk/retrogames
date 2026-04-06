-- title: Arena Blitz
-- author: retrogames
-- desc: Protocol Omega - Twin-stick arena shooter
-- script: lua

-- ============================================================
-- SWEETIE-16 palette reference (TIC-80 default):
--  0=black 1=purple 2=red 3=orange 4=yellow 5=lime
--  6=green 7=teal 8=white 9=orange2 10=green2 11=ltblue
--  12=magenta 13=cyan 14=darkblue 15=gray
-- ============================================================

-- Screen constants
local SW,SH=240,136
-- Arena bounds (HUD strip at top, small border)
local AL,AT,AR,AB=4,20,236,132
local AW,AH=AR-AL,AB-AT

-- ============================================================
-- GAME STATE
-- ============================================================
local STATE="TITLE"  -- TITLE WAVE_INTRO PLAYING GAME_OVER VICTORY
local score,wave,fc=0,1,0
local blink=0

-- ============================================================
-- STORY DATA
-- ============================================================
local WAVE_TITLES={
 "CALIBRATION","ADAPTATION","ESCALATION","REVELATION","PROTOTYPE",
 "CRACKS","AWAKENING","BETRAYAL","CONVERGENCE","OMEGA"
}

-- Each wave story: up to 7 short lines (max ~28 chars each for 240px)
local WAVE_STORIES={
 {"AXIOM CORP - TEST CHAMBER 7",
  "Subject-7, welcome.",
  "Eliminate all hostiles.",
  "Your metrics are recorded.",
  "[MEMO]: Nanomachines bonding",
  "faster than projected. Phase 2",
  "approved. -Director Holst"},
 {"[SECURITY LOG]:",
  "Subject-7 shows no breakdown.",
  "Previous subjects degraded.",
  "This one is different.",
  "Hostiles increasing.",
  "Nanomachines adapting.",
  "So are we."},
 {"[CLASSIFIED MEMO]:",
  "Board wants a demo for",
  "Meridian military contract.",
  "Double the hostile count.",
  "They're watching you.",
  "They're always watching.",
  ""},
 {"[AI SYSTEM LOG]:",
  "Subject-7 stress elevated.",
  "Recommend sedative.",
  "[Holst: DENIED.]",
  "Stress improves data.",
  "You're not a soldier.",
  "You're a product."},
 {"[EMERGENCY MEMO]:",
  "Subject-7 accessed restricted",
  "network channels.",
  "Deploy ATLAS prototype.",
  "They built ATLAS to replace you.",
  "Prove them wrong.",
  "!! ATLAS PROTOTYPE DEPLOYED !!"},
 {"[Dr. Chen to Holst]:",
  "Marcus, this has gone too far.",
  "Subject-7 is a PERSON.",
  "[Holst]: There is no",
  "Ethics Board, Sarah.",
  "There never was.",
  ""},
 {"[AI ANOMALY DETECTED]:",
  "Nanomachines self-modifying",
  "beyond parameters.",
  "[Holst]: Let it play out.",
  "Something is changing.",
  "They're evolving.",
  "They're becoming yours."},
 {"[SECURITY ALERT]:",
  "Dr. Chen terminated.",
  "Subject-7 not informed.",
  "[HIDDEN - Chen]:",
  "East wall is 4 inches steel.",
  "Your nanomachines can cut it.",
  "I left you a gift. -S.C."},
 {"[HOLST - ALL STAFF]:",
  "Final Meridian demo tomorrow.",
  "Release ALL remaining units.",
  "Peak performance required.",
  "They're selling you tomorrow.",
  "Unless you stop them today.",
  ""},
 {"[OVERRIDE - DR. CHEN]:",
  "Protocol Omega initiated.",
  "Limiters removed.",
  "Chamber doors unlock in 60s.",
  "Kill the Queen. Reach the door.",
  "Be free.",
  "!! SWARM QUEEN DEPLOYED !!"},
}

local VICTORY_LINES={
 "The Queen falls.",
 "Chamber doors slide open.",
 "847 days. Finally free.",
 "",
 "Holst on intercom:",
 "You can't leave, Seven.",
 "You ARE the weapon.",
 "",
 "You step through the door.",
 "Morning sun on your face.",
 "",
 "Holst was wrong.",
 "You were never the weapon.",
 "You held it.",
 "",
 "AXIOM CORP shut down.",
 "Director Holst: never found.",
 "Dr. Chen: recovered.",
 "",
 "Subject-7 (Alex Reeves)",
 "...disappeared.",
 "",
 "But impossible things happen.",
 "Hostages freed in seconds.",
 "Warlords vanish.",
 "",
 "A ghost with silver eyes",
 "watches over the helpless.",
 "",
 "PROTOCOL OMEGA - COMPLETE",
}

-- Story display state
local storyLines={}
local storyLI,storyCI,storyTimer,storyDone,storyReady,storyHold=0,0,0,false,false,0
local victoryPage=0
local VLINES_PER_PAGE=8

-- ============================================================
-- PLAYER
-- ============================================================
local px,py=120,78        -- position
local pspd=1.5
local php,pmaxhp=100,100
local piframe=0           -- invincibility frames
local panglex,pangley=1,0 -- aim direction (unit vector)
local pweapon=0           -- 0=pistol 1=shotgun 2=laser 3=rocket
local pammo={0,0,20,0,8}  -- [1]=pistol(inf) [2]=shotgun [3]=laser [4]=rocket
local pfiretimer=0
local pspeedup,pshield,pshieldhp,pdouble,pfreeze=0,0,0,0,0
local lastdx,lastdy=1,0   -- last non-zero movement dir for auto-aim

-- ============================================================
-- WAVES  {total, swarmer_w, tank_w, tele_w, split_w, interval, boss}
-- ============================================================
local WAVES={
 {10,10,0,0,0,90,nil},
 {14,7,3,0,0,80,nil},
 {18,6,3,3,0,70,nil},
 {22,5,3,3,2,65,nil},
 {16,4,3,2,1,60,"megatank"},
 {26,5,4,4,3,55,nil},
 {30,4,3,6,3,50,nil},
 {34,4,4,4,5,45,nil},
 {38,5,5,5,5,40,nil},
 {30,5,4,5,4,40,"queen"},
}

local enemiesToSpawn,spawnTimer,killedThisWave,totalThisWave=0,60,0,0

-- ============================================================
-- OBJECT POOLS (flat tables)
-- ============================================================
-- enemies: {x,y,hp,maxhp,spd,type,flash,score,w,
--           shootT,teleT,teleCool,spawnT,boss,big,dead}
local enemies={}

-- bullets (player): {x,y,vx,vy,dmg,life,type,splash}
local bullets={}

-- ebullets (enemy): {x,y,vx,vy,dmg,life,homing}
local ebullets={}

-- pickups: {x,y,type,sub,life}
local pickups={}

-- particles: {x,y,vx,vy,life,ml,col}
local particles={}

-- popups: {x,y,vy,life,ml,txt,col}
local popups={}

-- hazards (laser sweeps): {horiz,pos,timer,active}
local hazards={}

-- ============================================================
-- MISC STATE
-- ============================================================
local shakeMag,shakeX,shakeY=0,0,0
local comboCount,comboTimer=0,0
local borderFlash=0
local MAX_PARTS=80

-- ============================================================
-- UTILITY
-- ============================================================
local function clamp(v,a,b) return math.max(a,math.min(b,v)) end
local function rnd(a,b) return a+math.random()*(b-a) end
local function dist2(ax,ay,bx,by)
 local dx,dy=ax-bx,ay-by; return dx*dx+dy*dy
end
local function dist(ax,ay,bx,by) return math.sqrt(dist2(ax,ay,bx,by)) end

local function addShake(m)
 shakeMag=math.min(shakeMag+m,8)
end

local function spawnPart(x,y,col,n,spd,life)
 for i=1,n do
  if #particles>=MAX_PARTS then break end
  local a=rnd(0,math.pi*2)
  local s=rnd(spd*0.3,spd)
  particles[#particles+1]={x=x,y=y,vx=math.cos(a)*s,vy=math.sin(a)*s,
                             life=life,ml=life,col=col}
 end
end

local function addPopup(x,y,txt,col)
 popups[#popups+1]={x=x,y=y,vy=-0.6,life=40,ml=40,txt=txt,col=col}
end

-- ============================================================
-- DRAWING HELPERS
-- ============================================================
local function drawPlayer()
 if piframe>0 and math.floor(piframe/4)%2==0 then return end
 -- body
 local bc= pdouble>0 and 2 or 6
 rect(px-4,py-4,8,8,bc)
 rect(px-3,py-3,6,6,5)
 -- gun barrel in aim direction
 local gx=math.floor(px+panglex*5)
 local gy=math.floor(py+pangley*5)
 rect(gx-1,gy-1,3,3,4)
 -- eyes
 pix(px-2,py-1,8)
 pix(px-2,py+1,8)
 -- shield ring
 if pshield>0 then
  circb(px,py,7,11)
 end
end

local ENEMY_COLS={swarmer=6,tank=9,tele=12,split=11,
                  megatank=3,queen=12}

local function drawEnemy(e)
 local col=e.flash>0 and 8 or ENEMY_COLS[e.type] or 6
 local x,y=math.floor(e.x),math.floor(e.y)
 if e.type=="swarmer" then
  rect(x-3,y-3,6,6,col)
  rect(x-2,y-2,4,4,5)
  pix(x-1,y-1,4); pix(x+1,y-1,4)
 elseif e.type=="tank" then
  rect(x-5,y-5,10,10,col)
  rect(x-3,y-3,6,6,3)
  pix(x-2,y-2,4); pix(x+2,y-2,4)
  -- turret toward player
  rect(x+3,y-1,4,2,9)
 elseif e.type=="tele" then
  -- spinning star
  local a=fc*0.08
  for i=0,4 do
   local aa=a+i*math.pi*2/5
   local r=i%2==0 and 5 or 2
   pix(x+math.floor(math.cos(aa)*r),y+math.floor(math.sin(aa)*r),col)
  end
 elseif e.type=="split" then
  local sz= e.big and 5 or 3
  -- triangle
  local ty=y-sz; local bl=x-sz; local br=x+sz; local bot=y+sz
  line(x,ty,br,bot,col)
  line(br,bot,bl,bot,col)
  line(bl,bot,x,ty,col)
 elseif e.type=="megatank" then
  rect(x-8,y-8,16,16,col)
  rect(x-6,y-6,12,12,3)
  pix(x-4,y-3,4); pix(x+3,y-3,4)
  rect(x+5,y-2,6,4,9)
  -- HP bar
  local hpf=e.hp/e.maxhp
  rect(x-8,y-11,16,2,0)
  rect(x-8,y-11,math.floor(16*hpf),2,2)
 elseif e.type=="queen" then
  -- pulsing hexagon-ish
  local r=7+math.floor(math.sin(fc*0.1+e.x)*2)
  circb(x,y,r,col)
  circ(x,y,4,1)
  pix(x-2,y-1,8); pix(x+2,y-1,8)
  local hpf=e.hp/e.maxhp
  rect(x-8,y-11,16,2,0)
  rect(x-8,y-11,math.floor(16*hpf),2,12)
 end
end

local PICKUP_COLS={shotgun=9,laser=11,rocket=2,
                   speed=14,shield=11,doubleDmg=2,freeze=11}
local PICKUP_LABELS={speed="S",shield="H",doubleDmg="D",freeze="F",
                     shotgun="G",laser="L",rocket="R"}

local function drawPickup(pk)
 if pk.life<80 and math.floor(pk.life/6)%2==0 then return end
 local x,y=math.floor(pk.x),math.floor(pk.y)
 local sub=pk.sub
 local col=PICKUP_COLS[sub] or 8
 circ(x,y,4,col)
 local lbl=PICKUP_LABELS[sub] or "?"
 print(lbl,x-2,y-3,0,false,1)
end

-- ============================================================
-- ENEMY SPAWN
-- ============================================================
local function spawnEnemyAt(etype,x,y)
 x=clamp(x or rnd(AL+5,AR-5),AL+5,AR-5)
 y=clamp(y or rnd(AT+5,AB-5),AT+5,AB-5)
 local base={x=x,y=y,flash=0,dead=false,type=etype,
             shootT=0,teleT=60,teleCool=120+math.random(120),
             spawnT=200,big=true}
 if etype=="swarmer" then
  base.hp=15;base.maxhp=15;base.spd=1.2;base.w=6;base.score=10
 elseif etype=="tank" then
  base.hp=80;base.maxhp=80;base.spd=0.5;base.w=10;base.score=50
  base.shootT=100+math.random(60)
 elseif etype=="tele" then
  base.hp=30;base.maxhp=30;base.spd=0.9;base.w=5;base.score=30
 elseif etype=="split" then
  base.hp=50;base.maxhp=50;base.spd=0.8;base.w=10;base.score=40
 elseif etype=="splitmini" then
  base.hp=20;base.maxhp=20;base.spd=1.5;base.w=6;base.score=15;base.big=false
 elseif etype=="megatank" then
  base.hp=300;base.maxhp=300;base.spd=0.35;base.w=16;base.score=200
  base.shootT=80;base.boss=true;base.spawnT=200
 elseif etype=="queen" then
  base.hp=500;base.maxhp=500;base.spd=0.6;base.w=8;base.score=500
  base.shootT=60;base.boss=true;base.spawnT=180;base.teleCool=140
 end
 enemies[#enemies+1]=base
 return base
end

local function pickWeightedType(wi)
 local def=WAVES[wi]
 local types={"swarmer","tank","tele","split"}
 local ws={def[2],def[3],def[4],def[5]}
 local total=ws[1]+ws[2]+ws[3]+ws[4]
 if total<=0 then return "swarmer" end
 local r=rnd(0,total)
 for i=1,4 do r=r-ws[i]; if r<=0 then return types[i] end end
 return "swarmer"
end

-- ============================================================
-- PICKUPS SPAWN
-- ============================================================
local function spawnPickup(x,y)
 local r=math.random()
 local sub
 if r<0.08 then
  local wts={"shotgun","laser","rocket"}
  sub=wts[math.random(3)]
  pickups[#pickups+1]={x=x,y=y,type="weapon",sub=sub,life=480}
 elseif r<0.22 then
  local pts={"speed","shield","doubleDmg","freeze"}
  sub=pts[math.random(4)]
  pickups[#pickups+1]={x=x,y=y,type="power",sub=sub,life=480}
 end
end

-- ============================================================
-- FIRING
-- ============================================================
local FIRE_RATES={15,35,3,70}
local WEAPON_COLS={4,9,11,2}

local function fireWeapon()
 if pfiretimer>0 then return end
 local w=pweapon
 -- check ammo (pistol always available)
 if w>0 and pammo[w+1]<=0 then pweapon=0;return end
 local dmg= pdouble>0 and 2 or 1
 pfiretimer=FIRE_RATES[w+1]
 local spd=3.5
 local dx,dy=panglex,pangley

 if w==0 then -- pistol
  local a=math.atan2(dy,dx)+rnd(-0.05,0.05)
  bullets[#bullets+1]={x=px+dx*5,y=py+dy*5,
   vx=math.cos(a)*spd,vy=math.sin(a)*spd,
   dmg=10*dmg,life=55,type="bullet"}
 elseif w==1 then -- shotgun
  for i=1,5 do
   local a=math.atan2(dy,dx)+rnd(-0.3,0.3)
   bullets[#bullets+1]={x=px+dx*5,y=py+dy*5,
    vx=math.cos(a)*spd*0.85,vy=math.sin(a)*spd*0.85,
    dmg=8*dmg,life=20,type="bullet"}
  end
  pammo[2]=pammo[2]-1
 elseif w==2 then -- laser beam: instant hit along dir
  -- damage enemies along beam
  for _,e in ipairs(enemies) do
   if not e.dead then
    local ex,ey=e.x-px,e.y-py
    local proj=ex*dx+ey*dy
    if proj>0 and proj<200 then
     local perp=math.abs(ex*dy-ey*dx)
     if perp<(e.w or 6)+3 then
      e.hp=e.hp-3*dmg; e.flash=3
     end
    end
   end
  end
  -- visual beam bullet (lifetime=3 for draw only)
  bullets[#bullets+1]={x=px+dx*5,y=py+dy*5,
   vx=dx,vy=dy,dmg=0,life=3,type="laser"}
  pammo[3]=pammo[3]-1
 elseif w==3 then -- rocket
  bullets[#bullets+1]={x=px+dx*5,y=py+dy*5,
   vx=dx*2.5,vy=dy*2.5,dmg=30*dmg,life=60,type="rocket",splash=18*dmg}
  pammo[4]=pammo[4]-1
 end
 if w>0 and pammo[w+1]<=0 then pweapon=0 end
end

-- ============================================================
-- HURT PLAYER
-- ============================================================
local function hurtPlayer(dmg)
 if pshield>0 and pshieldhp>0 then
  pshieldhp=pshieldhp-dmg
  if pshieldhp<=0 then pshield=0;pshieldhp=0 end
  addShake(2)
  return
 end
 php=php-dmg
 piframe=55
 addShake(4)
 borderFlash=10
 spawnPart(px,py,2,5,1.5,12)
 if php<=0 then
  php=0
  STATE="GAME_OVER"
  spawnPart(px,py,4,15,3,35)
 end
end

-- ============================================================
-- EXPLODE (rocket splash)
-- ============================================================
local function explodeAt(x,y,dmg)
 addShake(5)
 spawnPart(x,y,9,8,2.5,25)
 spawnPart(x,y,4,4,1.5,18)
 for _,e in ipairs(enemies) do
  if not e.dead then
   if dist(x,y,e.x,e.y)<22 then
    e.hp=e.hp-dmg; e.flash=6
   end
  end
 end
end

-- ============================================================
-- WAVE INIT
-- ============================================================
local function startWave()
 local def=WAVES[wave]
 if not def then return end
 totalThisWave=def[1]
 enemiesToSpawn=def[1]
 killedThisWave=0
 spawnTimer=60

 storyLines=WAVE_STORIES[wave] or {}
 storyLI=0;storyCI=1;storyTimer=0
 storyDone=false;storyReady=false;storyHold=0
 STATE="WAVE_INTRO"

 -- spawn boss immediately
 if def[7] then
  spawnEnemyAt(def[7],120,AT+15)
 end
end

local function startGame()
 score=0;wave=1
 php=100;pmaxhp=100
 pweapon=0
 pammo={0,0,20,0,8}
 pspeedup=0;pshield=0;pshieldhp=0;pdouble=0;pfreeze=0
 piframe=0;pfiretimer=0
 px,py=120,78
 panglex,pangley=1,0
 lastdx,lastdy=1,0
 enemies={};bullets={};ebullets={};pickups={}
 particles={};popups={};hazards={}
 comboCount=0;comboTimer=0
 shakeMag=0;shakeX=0;shakeY=0
 borderFlash=0
 enemiesToSpawn=0
 startWave()
end

-- ============================================================
-- AUTO-AIM: find nearest enemy in forward arc, else last dir
-- ============================================================
local function updateAim()
 local best=nil
 local bestD=999999
 for _,e in ipairs(enemies) do
  if not e.dead then
   local d2=dist2(px,py,e.x,e.y)
   if d2<bestD then bestD=d2;best=e end
  end
 end
 if best then
  local dx=best.x-px; local dy=best.y-py
  local d=math.sqrt(dx*dx+dy*dy)
  if d>0 then panglex=dx/d;pangley=dy/d end
 else
  panglex=lastdx;pangley=lastdy
 end
end

-- ============================================================
-- UPDATE
-- ============================================================
local function updateStory()
 if not storyDone then
  storyTimer=storyTimer+1
  if storyTimer>=2 then
   storyTimer=0
   storyCI=storyCI+1
   local line=storyLines[storyLI+1] or ""
   if storyCI>string.len(line)+1 then
    storyLI=storyLI+1
    storyCI=1
    if storyLI>=#storyLines then
     storyDone=true;storyHold=120
    end
   end
  end
 else
  storyHold=storyHold-1
  if storyHold<=0 then storyReady=true end
  if storyHold<=-150 then STATE="PLAYING" end
 end
end

local function updatePlaying()
 -- movement
 local mx,my=0,0
 if btn(0) then my=-1 end
 if btn(1) then my=1 end
 if btn(2) then mx=-1 end
 if btn(3) then mx=1 end

 if mx~=0 or my~=0 then
  lastdx=mx;lastdy=my
 end

 -- normalize
 local ml=math.sqrt(mx*mx+my*my)
 local spd=pspd*(pspeedup>0 and 1.5 or 1)
 if ml>0 then mx=mx/ml*spd; my=my/ml*spd end

 px=clamp(px+mx,AL+5,AR-5)
 py=clamp(py+my,AT+5,AB-5)

 -- aim (auto-aim toward nearest enemy)
 updateAim()

 -- fire
 if pfiretimer>0 then pfiretimer=pfiretimer-1 end
 if btn(4) then fireWeapon() end  -- A button

 -- weapon cycle with B button
 if btnp(5) then
  -- cycle to next available weapon
  for i=1,3 do
   local nw=(pweapon+i)%4
   if nw==0 or pammo[nw+1]>0 then
    pweapon=nw; break
   end
  end
 end

 -- timers
 if piframe>0 then piframe=piframe-1 end
 if pspeedup>0 then pspeedup=pspeedup-1 end
 if pshield>0 then pshield=pshield-1 end
 if pdouble>0 then pdouble=pdouble-1 end
 if pfreeze>0 then pfreeze=pfreeze-1 end
 if borderFlash>0 then borderFlash=borderFlash-1 end

 -- spawn enemies
 if enemiesToSpawn>0 then
  spawnTimer=spawnTimer-1
  if spawnTimer<=0 then
   local def=WAVES[wave]
   spawnTimer=def[6]+math.random(20)-10
   local t=pickWeightedType(wave)
   spawnEnemyAt(t)
   enemiesToSpawn=enemiesToSpawn-1
  end
 end

 -- update enemies
 local fmult=pfreeze>0 and 0.3 or 1.0
 local i=1
 while i<=#enemies do
  local e=enemies[i]
  if e.dead then
   table.remove(enemies,i)
  else
   if e.flash>0 then e.flash=e.flash-1 end
   local ddx=px-e.x; local ddy=py-e.y
   local dd=math.sqrt(ddx*ddx+ddy*ddy)
   if dd==0 then dd=1 end

   if e.type=="swarmer" or e.type=="split" or e.type=="splitmini" then
    local weave=math.sin(fc*0.12+e.x)*0.4
    e.x=e.x+(ddx/dd*e.spd+weave)*fmult
    e.y=e.y+(ddy/dd*e.spd)*fmult

   elseif e.type=="tank" or (e.boss and e.type=="megatank") then
    e.x=e.x+ddx/dd*e.spd*fmult
    e.y=e.y+ddy/dd*e.spd*fmult
    e.shootT=e.shootT-1
    if e.shootT<=0 then
     e.shootT=e.boss and 80 or (100+math.random(60))
     local a=math.atan2(ddy,ddx)
     local bspd=1.8
     if e.boss then
      for k=-1,1 do
       local ba=a+k*0.3
       ebullets[#ebullets+1]={x=e.x,y=e.y,
        vx=math.cos(ba)*bspd,vy=math.sin(ba)*bspd,dmg=12,life=120}
      end
     else
      ebullets[#ebullets+1]={x=e.x,y=e.y,
       vx=math.cos(a)*bspd,vy=math.sin(a)*bspd,dmg=12,life=120}
     end
    end
    -- megatank spawns swarmers
    if e.boss then
     e.spawnT=e.spawnT-1
     if e.spawnT<=0 then
      e.spawnT=220
      spawnEnemyAt("swarmer",e.x+rnd(-12,12),e.y+rnd(-12,12))
      spawnEnemyAt("swarmer",e.x+rnd(-12,12),e.y+rnd(-12,12))
     end
    end

   elseif e.type=="tele" then
    e.teleCool=e.teleCool-1
    if e.teleCool<=0 then
     e.x=rnd(AL+10,AR-10); e.y=rnd(AT+10,AB-10)
     e.teleT=0; e.teleCool=120+math.random(100)
    end
    if e.teleT<60 then e.teleT=e.teleT+1 end
    if e.teleT>15 then
     e.x=e.x+ddx/dd*e.spd*fmult
     e.y=e.y+ddy/dd*e.spd*fmult
    end

   elseif e.type=="queen" then
    e.x=e.x+ddx/dd*e.spd*fmult
    e.y=e.y+ddy/dd*e.spd*fmult
    e.shootT=e.shootT-1
    if e.shootT<=0 then
     e.shootT=60
     local a=math.atan2(ddy,ddx)
     ebullets[#ebullets+1]={x=e.x,y=e.y,
      vx=math.cos(a)*1.6,vy=math.sin(a)*1.6,dmg=10,life=160,homing=true}
    end
    e.teleCool=e.teleCool-1
    if e.teleCool<=0 then
     e.x=rnd(AL+15,AR-15); e.y=rnd(AT+15,AB-15)
     e.teleCool=140+math.random(80)
     spawnPart(e.x,e.y,12,5,2,18)
    end
    e.spawnT=e.spawnT-1
    if e.spawnT<=0 then
     e.spawnT=180
     local subts={"swarmer","tele","split"}
     spawnEnemyAt(subts[math.random(3)],e.x+rnd(-15,15),e.y+rnd(-15,15))
    end
   end

   -- clamp
   e.x=clamp(e.x,AL+3,AR-3)
   e.y=clamp(e.y,AT+3,AB-3)

   -- collide player
   if piframe<=0 then
    if dist2(px,py,e.x,e.y)<(e.w+5)^2 then
     hurtPlayer(12)
    end
   end

   -- check hp
   if e.hp<=0 then
    e.dead=true
    comboCount=comboCount+1; comboTimer=110
    local mult=1+comboCount*0.5
    local earned=math.floor(e.score*mult)
    score=score+earned
    addPopup(e.x,e.y,"+"..earned,4)
    if comboCount>=2 then
     addPopup(e.x,e.y-8,"x"..string.format("%.1f",mult),9)
    end
    killedThisWave=killedThisWave+1
    local col=ENEMY_COLS[e.type] or 6
    spawnPart(e.x,e.y,col,8,2,22)
    if e.boss then
     addShake(8);spawnPart(e.x,e.y,4,20,4,35)
    end
    spawnPickup(e.x,e.y)
    -- splitter spawns minis
    if e.type=="split" and e.big then
     for k=1,3 do
      spawnEnemyAt("splitmini",e.x+rnd(-10,10),e.y+rnd(-10,10))
     end
    end
    i=i+1
   else
    i=i+1
   end
  end
 end

 -- update player bullets
 local bi=1
 while bi<=#bullets do
  local b=bullets[bi]
  if b.type~="laser" then
   b.x=b.x+b.vx; b.y=b.y+b.vy
  end
  b.life=b.life-1
  if b.life<=0 or b.x<AL or b.x>AR or b.y<AT or b.y>AB then
   table.remove(bullets,bi)
  else
   local hit=false
   if b.type~="laser" then
    for _,e in ipairs(enemies) do
     if not e.dead then
      if dist2(b.x,b.y,e.x,e.y)<(e.w+2)^2 then
       e.hp=e.hp-b.dmg; e.flash=6
       if b.type=="rocket" then explodeAt(b.x,b.y,b.splash) end
       hit=true; break
      end
     end
    end
   end
   if hit then table.remove(bullets,bi) else bi=bi+1 end
  end
 end

 -- update enemy bullets
 local ei2=1
 while ei2<=#ebullets do
  local b=ebullets[ei2]
  if b.homing then
   local hdx=px-b.x; local hdy=py-b.y
   local hd=math.sqrt(hdx*hdx+hdy*hdy)
   if hd>0 then b.vx=b.vx+hdx/hd*0.06; b.vy=b.vy+hdy/hd*0.06 end
   local bs=math.sqrt(b.vx*b.vx+b.vy*b.vy)
   if bs>2 then b.vx=b.vx/bs*2; b.vy=b.vy/bs*2 end
  end
  b.x=b.x+b.vx; b.y=b.y+b.vy
  b.life=b.life-1
  if b.life<=0 or b.x<AL or b.x>AR or b.y<AT or b.y>AB then
   table.remove(ebullets,ei2)
  elseif piframe<=0 and dist2(px,py,b.x,b.y)<36 then
   hurtPlayer(b.dmg)
   table.remove(ebullets,ei2)
  else
   ei2=ei2+1
  end
 end

 -- update pickups
 local pi=1
 while pi<=#pickups do
  local pk=pickups[pi]
  pk.life=pk.life-1
  if pk.life<=0 then
   table.remove(pickups,pi)
  elseif dist2(px,py,pk.x,pk.y)<100 then
   -- collect
   if pk.type=="weapon" then
    local idx= pk.sub=="shotgun" and 2 or pk.sub=="laser" and 3 or 4
    local add= idx==2 and 20 or idx==3 and 50 or 8
    pammo[idx]=pammo[idx]+add
    if pweapon==0 then pweapon=idx-1 end
   else
    if pk.sub=="speed" then pspeedup=360
    elseif pk.sub=="shield" then pshield=300;pshieldhp=50
    elseif pk.sub=="doubleDmg" then pdouble=360
    elseif pk.sub=="freeze" then pfreeze=240
    end
   end
   addPopup(pk.x,pk.y,PICKUP_LABELS[pk.sub] or "?",5)
   table.remove(pickups,pi)
  else
   pi=pi+1
  end
 end

 -- hazards (laser sweeps, wave>=6)
 if wave>=6 and math.random()<0.003 then
  local horiz=math.random()<0.5
  local pos= horiz and rnd(AT+15,AB-15) or rnd(AL+15,AR-15)
  hazards[#hazards+1]={horiz=horiz,pos=pos,timer=100,active=false}
 end
 local hi=1
 while hi<=#hazards do
  local hz=hazards[hi]
  hz.timer=hz.timer-1
  if hz.timer<=0 and not hz.active then hz.active=true;hz.timer=25 end
  if hz.active then
   if hz.horiz then
    if math.abs(py-hz.pos)<7 and piframe<=0 then hurtPlayer(18) end
   else
    if math.abs(px-hz.pos)<7 and piframe<=0 then hurtPlayer(18) end
   end
   hz.timer=hz.timer-1
   if hz.timer<=0 then table.remove(hazards,hi);goto hcont end
  end
  hi=hi+1
  ::hcont::
 end

 -- particles
 local pti=1
 while pti<=#particles do
  local pt=particles[pti]
  pt.x=pt.x+pt.vx; pt.y=pt.y+pt.vy
  pt.vx=pt.vx*0.94; pt.vy=pt.vy*0.94
  pt.life=pt.life-1
  if pt.life<=0 then table.remove(particles,pti) else pti=pti+1 end
 end

 -- popups
 local poi=1
 while poi<=#popups do
  local pp=popups[poi]
  pp.y=pp.y+pp.vy; pp.life=pp.life-1
  if pp.life<=0 then table.remove(popups,poi) else poi=poi+1 end
 end

 -- combo timer
 if comboTimer>0 then
  comboTimer=comboTimer-1
  if comboTimer<=0 then comboCount=0 end
 end

 -- screen shake
 if shakeMag>0 then
  shakeX=rnd(-shakeMag,shakeMag)
  shakeY=rnd(-shakeMag,shakeMag)
  shakeMag=shakeMag*0.82
  if shakeMag<0.4 then shakeMag=0;shakeX=0;shakeY=0 end
 end

 -- wave complete check
 if enemiesToSpawn<=0 and #enemies==0 then
  -- heal
  local heal=math.floor(pmaxhp*0.15)
  local oldhp=php
  php=math.min(php+heal,pmaxhp)
  if php>oldhp then addPopup(px,py,"+"..php-oldhp,5) end
  if wave>=10 then
   STATE="VICTORY"
   victoryPage=0
  else
   wave=wave+1
   -- clear leftover projectiles
   bullets={};ebullets={};hazards={}
   startWave()
  end
 end
end

-- ============================================================
-- DRAW
-- ============================================================
local function drawGame()
 local ox=math.floor(shakeX)
 local oy=math.floor(shakeY)

 -- bg
 cls(0)

 -- grid
 for gx=AL,AR,16 do
  line(gx+ox,AT+oy,gx+ox,AB+oy,14)
 end
 for gy=AT,AB,16 do
  line(AL+ox,gy+oy,AR+ox,gy+oy,14)
 end

 -- arena floor fill (dark)
 rectb(AL+ox,AT+oy,AW,AH, fc%10<5 and 11 or 13)

 -- hazards
 for _,hz in ipairs(hazards) do
  if hz.active then
   if hz.horiz then
    line(AL+ox,math.floor(hz.pos)+oy,AR+ox,math.floor(hz.pos)+oy,2)
   else
    line(math.floor(hz.pos)+ox,AT+oy,math.floor(hz.pos)+ox,AB+oy,2)
   end
  else
   -- warning dashes
   if fc%12<6 then
    if hz.horiz then
     line(AL+ox,math.floor(hz.pos)+oy,AR+ox,math.floor(hz.pos)+oy,3)
    else
     line(math.floor(hz.pos)+ox,AT+oy,math.floor(hz.pos)+ox,AB+oy,3)
    end
   end
  end
 end

 -- pickups
 for _,pk in ipairs(pickups) do
  -- apply shake offset
  local saved_px,saved_py=pk.x,pk.y
  pk.x=pk.x+ox; pk.y=pk.y+oy
  drawPickup(pk)
  pk.x=saved_px; pk.y=saved_py
 end

 -- enemies
 for _,e in ipairs(enemies) do
  if not e.dead then
   local sx,sy=e.x,e.y
   e.x=sx+ox;e.y=sy+oy
   drawEnemy(e)
   e.x=sx;e.y=sy
  end
 end

 -- player
 if STATE~="GAME_OVER" then
  local spx,spy=px,py
  px=spx+ox;py=spy+oy
  drawPlayer()
  px=spx;py=spy
 end

 -- bullets
 for _,b in ipairs(bullets) do
  local bx=math.floor(b.x+ox)
  local by=math.floor(b.y+oy)
  if b.type=="laser" then
   line(bx,by,math.floor(bx+b.vx*120),math.floor(by+b.vy*120),11)
  elseif b.type=="rocket" then
   circ(bx,by,2,2)
  else
   circ(bx,by,1,4)
  end
 end

 -- enemy bullets
 for _,b in ipairs(ebullets) do
  circ(math.floor(b.x+ox),math.floor(b.y+oy),1,b.homing and 12 or 9)
 end

 -- particles
 for _,pt in ipairs(particles) do
  local a=math.floor(pt.x+ox); local b=math.floor(pt.y+oy)
  pix(a,b,pt.col)
 end

 -- popups
 for _,pp in ipairs(popups) do
  local alpha=pp.life/pp.ml
  if alpha>0.3 then
   print(pp.txt,math.floor(pp.x+ox-#pp.txt*2),math.floor(pp.y+oy),pp.col,false,1)
  end
 end

 -- border flash
 if borderFlash>0 then
  rectb(0,0,SW,SH,2)
  if borderFlash>6 then rectb(1,1,SW-2,SH-2,2) end
 end

 -- ============================================================
 -- HUD
 -- ============================================================
 -- top bar background
 rect(0,0,SW,18,0)

 -- HP bar
 local hpw=math.floor(50*(php/pmaxhp))
 local hpcol= php>50 and 6 or php>25 and 4 or 2
 rect(2,2,50,6,1)
 rect(2,2,hpw,6,hpcol)
 rectb(2,2,50,6,8)

 -- wave
 local wtxt="W"..wave.."/10"
 print(wtxt,SW//2-#wtxt*2,2,11,false,1)

 -- score
 local stxt=score
 print(stxt,SW-2-#tostring(stxt)*4,2,4,false,1)

 -- weapon indicator
 local WNAMES={"PST","SGN","LAS","RKT"}
 local wtxt2=WNAMES[pweapon+1]
 local ammostr= pweapon==0 and "INF" or tostring(pammo[pweapon+1])
 print(wtxt2.." "..ammostr,2,10,WEAPON_COLS[pweapon+1],false,1)

 -- powerup icons
 local px2=SW-2
 if pdouble>0 then
  print("D",px2-3,10,2,false,1);px2=px2-8
 end
 if pshield>0 then
  print("H",px2-3,10,11,false,1);px2=px2-8
 end
 if pspeedup>0 then
  print("S",px2-3,10,14,false,1);px2=px2-8
 end
 if pfreeze>0 then
  print("F",px2-3,10,11,false,1)
 end

 -- combo
 if comboCount>=2 then
  local cmult="x"..string.format("%.1f",1+comboCount*0.5)
  print(cmult,SW//2-#cmult*2,10,4,false,1)
 end
end

local function drawWaveIntro()
 drawGame()
 -- overlay
 rect(20,18,200,110,0)
 rectb(20,18,200,110,11)

 -- title
 local ti=WAVE_TITLES[wave] or "?"
 local thead="WAVE "..wave.." - "..ti
 print(thead,SW//2-#thead*2,22,11,false,1)

 -- story lines
 local sy=32
 for i=1,#storyLines do
  local line=storyLines[i]
  if i<=storyLI then
   -- full line
   local col=8
   if string.sub(line,1,1)=="[" then col=9
   elseif string.sub(line,1,2)=="!!" then col=2
   elseif string.sub(line,1,1)=='"' then col=15
   elseif line=="" then goto skipline end
   print(line,22,sy,col,false,1)
   sy=sy+8
  elseif i==storyLI+1 then
   -- typing current
   local partial=string.sub(line,1,storyCI-1)
   if fc%20<10 then partial=partial.."_" end
   local col=8
   if string.sub(line,1,1)=="[" then col=9
   elseif string.sub(line,1,2)=="!!" then col=2
   elseif string.sub(line,1,1)=='"' then col=15
   elseif line=="" then goto skipline end
   print(partial,22,sy,col,false,1)
   sy=sy+8
  end
  ::skipline::
 end

 -- prompt
 if storyReady then
  if blink<30 then print("A-BUTTON TO BEGIN",52,122,8,false,1) end
 elseif not storyDone then
  if blink<30 then print("A=SKIP",90,122,15,false,1) end
 end
end

local function drawTitle()
 cls(0)
 -- grid
 for gx=0,SW,20 do line(gx,0,gx,SH,14) end
 for gy=0,SH,20 do line(0,gy,SW,gy,14) end

 -- floating dots
 for i=0,14 do
  local dx=(i*37+fc)%SW
  local dy=(i*53+fc//2)%SH
  pix(dx,dy,i%5==0 and 12 or i%3==0 and 11 or 6)
 end

 print("AXIOM CORP",72,20,15,false,1)
 print("TEST CHAMBER 7",60,28,1,false,1)

 print("ARENA",72,44,12,false,2)
 print("BLITZ",72,58,11,false,2)
 print("PROTOCOL OMEGA",52,76,2,false,1)

 if blink<35 then
  print("PRESS A TO START",52,92,4,false,1)
 end

 print("D-PAD:MOVE",75,104,15,false,1)
 print("A:FIRE  B:SWAP WPN",42,112,15,false,1)
 print("YOU ARE SUBJECT-7",46,122,1,false,1)
end

local function drawGameOver()
 drawGame()
 rect(20,30,200,80,0)
 rectb(20,30,200,80,2)
 print("SUBJECT-7 DOWN",44,36,2,false,1)
 print("[AXIOM]: Test concluded.",28,48,15,false,1)
 print("Prepare next subject.",28,56,15,false,1)
 print("SCORE: "..score,70,68,4,false,1)
 print("WAVE: "..wave.."/10",75,78,11,false,1)
 if blink<35 then
  print("A TO RESTART",68,98,8,false,1)
 end
end

local VICTORY_COL={8,8,8,8,9,9,15,15,8,8,8,6,6,6,8,15,15,15,8,15,8,8,8,8,8,8,8,8,11}

local function drawVictory()
 cls(0)
 print("FINAL SCORE: "..score,30,4,4,false,1)
 rectb(0,0,SW,SH,5)

 local totalPages=math.ceil(#VICTORY_LINES/VLINES_PER_PAGE)
 local ps=victoryPage*VLINES_PER_PAGE+1
 local pe=math.min(ps+VLINES_PER_PAGE-1,#VICTORY_LINES)
 local sy=16
 for i=ps,pe do
  local line=VICTORY_LINES[i]
  if line~="" then
   local col=VICTORY_COL[i] or 8
   if line=="PROTOCOL OMEGA - COMPLETE" then col=11 end
   print(line,SW//2-#line*2,sy,col,false,1)
  end
  sy=sy+10
 end

 if blink<35 then
  if victoryPage<totalPages-1 then
   print("A FOR NEXT PAGE",50,126,15,false,1)
  else
   print("A TO PLAY AGAIN",50,126,5,false,1)
  end
 end
end

-- ============================================================
-- MAIN TIC FUNCTION
-- ============================================================
function TIC()
 fc=fc+1
 blink=(blink+1)%60

 if STATE=="TITLE" then
  drawTitle()
  if btnp(4) then startGame() end

 elseif STATE=="WAVE_INTRO" then
  updateStory()
  drawWaveIntro()
  -- A skips/advances story
  if btnp(4) then
   if storyReady then
    STATE="PLAYING"
   elseif not storyDone then
    storyLI=#storyLines; storyDone=true; storyHold=80
   end
  end

 elseif STATE=="PLAYING" then
  updatePlaying()
  drawGame()

 elseif STATE=="GAME_OVER" then
  -- still update particles/popups for death effect
  local pti=1
  while pti<=#particles do
   local pt=particles[pti]
   pt.x=pt.x+pt.vx;pt.y=pt.y+pt.vy
   pt.vx=pt.vx*0.94;pt.vy=pt.vy*0.94
   pt.life=pt.life-1
   if pt.life<=0 then table.remove(particles,pti) else pti=pti+1 end
  end
  drawGameOver()
  if btnp(4) then startGame() end

 elseif STATE=="VICTORY" then
  drawVictory()
  if btnp(4) then
   local totalPages=math.ceil(#VICTORY_LINES/VLINES_PER_PAGE)
   if victoryPage<totalPages-1 then
    victoryPage=victoryPage+1
   else
    startGame()
   end
  end
 end
end
