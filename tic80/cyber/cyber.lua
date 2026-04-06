-- title: Chrome Viper
-- author: retrogames
-- script: lua
-- desc: Cyberpunk horizontal scrolling shooter

-- Sweetie 16 palette indices:
-- 0=dark navy, 1=dark purple, 2=red, 3=orange, 4=yellow
-- 5=light green, 6=dark green, 7=dark teal, 8=white
-- 9=orange, 10=green, 11=light blue, 12=magenta, 13=teal
-- 14=dark blue-gray, 15=gray

local W,H=240,136
local t=0  -- frame counter

-- Game state
local state="START"  -- START, STORY, PLAYING, GAMEOVER, WIN
local score=0
local hi=0
local level=0  -- 0,1,2
local story_phase=0

-- Story
local STORIES={
[0]="AXIOM CORP -- 2187\n\nThe orbital colonies\nare ours.\n\nBut one asset has\ngone missing.\n\nPrototype CV-7\n'Chrome Viper'\nstolen from Hangar 9.\n\n> MISSION: Breach\n  the Orbital Ring.\n> LAUNCH READY",
[1]="FLIGHT LOG 2187\n\nOuter defense ring\nbreached. The Satellite\nis scrap metal.\n\nAXIOM is building\na weapon to glass\nevery city on Earth.\n\nProject LEVIATHAN.\n\n> Entering Sector 7-G\n> Neon Corridor ahead",
[2]="FLIGHT LOG 2187\n\nThe Carrier is down.\n\nThrough the debris\nfield -- The Leviathan.\n\nMassive. A dreadnought\nthe size of a colony.\nPowering its cannon\naimed at Earth.\n\nNo backup. No retreat.\n\n> FINAL APPROACH",
[3]="AXIOM EMERGENCY\n\n[SIGNAL LOST]\n[SIGNAL LOST]\n\nThe Leviathan is\ndestroyed. AXIOM's\nnetwork is collapsing.\n\nThe colonies are free.\n\nBut corporations\ndon't die.\nThey rebrand.\n\n> MISSION COMPLETE\n> PILOT: ALIVE"
}
local story_txt=""
local story_idx=0
local story_timer=0

-- Stars (3 layers)
local stars={}
local function init_stars()
  stars={}
  for i=1,60 do
    local layer=1
    if i>40 then layer=3
    elseif i>20 then layer=2 end
    stars[i]={
      x=math.random(0,W-1),
      y=math.random(0,H-1),
      spd=layer*0.5+math.random()*0.3,
      layer=layer
    }
  end
end

-- Player
local px,py=30,68  -- position
local pspd=1.2
local pshields=3
local pweapon=0  -- 0=dual, 1=spread, 2=homing
local pemp=1
local pemp_cd=0
local pinv=0  -- invincibility frames
local pfire=0
local palive=true
local pdead_timer=0

-- Bullets (player)
-- {x,y,vx,vy,typ,life}  typ: 0=laser,1=spread,2=homing
local bullets={}

-- Enemy bullets
-- {x,y,vx,vy,life}
local ebullets={}

-- Enemies
-- {x,y,w,h,hp,maxhp,spd,typ,stimer,srate,sinoff,alive,flash}
-- typ: 0=drone,1=gunship,2=turret,3=shieldgen
local enemies={}

-- Particles
-- {x,y,vx,vy,life,col}
local particles={}

-- Power-ups
-- {x,y,typ,angle}  typ: 0=spread,1=homing,2=shield,3=emp
local powerups={}

-- Floating texts
-- {x,y,txt,col,life}
local ftexts={}

-- EMP waves
-- {x,y,r,maxr}
local empwaves={}

-- Boss
local boss=nil
local boss_active=false

-- Chain
local chain=1
local chain_t=0

-- Shake
local shake_mag=0
local shake_x=0
local shake_y=0

-- Level timer & waves
local lvl_timer=0
local wave_idx=0
local level_waves={}

local LEVEL_NAMES={"ORBITAL RING","NEON CORRIDOR","THE ABYSS"}
local BOSS_NAMES={"DEF.SATELLITE","CYBORG CARRIER","LEVIATHAN"}

-- =============================================
-- WAVE GENERATOR
-- =============================================
local function gen_waves(lvl)
  local w={}
  if lvl==0 then
    w[#w+1]={t=60, tp=0,  cnt=5, pat=0, y=25, sp=8}
    w[#w+1]={t=180,tp=0,  cnt=5, pat=0, y=88, sp=8}
    w[#w+1]={t=300,tp=0,  cnt=7, pat=2, y=60, sp=7}
    w[#w+1]={t=450,tp=1,  cnt=2, pat=0, y=38, sp=22}
    w[#w+1]={t=550,tp=0,  cnt=6, pat=1, y=50, sp=7}
    w[#w+1]={t=700,tp=1,  cnt=3, pat=0, y=75, sp=18}
    w[#w+1]={t=850,tp=0,  cnt=8, pat=2, y=60, sp=6}
    w[#w+1]={t=1000,tp=1, cnt=2, pat=3, y=0,  sp=0}
    w[#w+1]={t=1100,tp=0, cnt=8, pat=0, y=30, sp=7}
    w[#w+1]={t=1200,tp=0, cnt=8, pat=0, y=90, sp=7}
    w[#w+1]={t=1400,boss=0}
  elseif lvl==1 then
    w[#w+1]={t=60, tp=0,  cnt=8, pat=2, y=38, sp=6}
    w[#w+1]={t=150,tp=2,  cnt=3, pat=0, y=20, sp=28}
    w[#w+1]={t=250,tp=1,  cnt=3, pat=0, y=75, sp=14}
    w[#w+1]={t=350,tp=0,  cnt=9, pat=1, y=60, sp=6}
    w[#w+1]={t=450,tp=2,  cnt=3, pat=0, y=100,sp=22}
    w[#w+1]={t=550,tp=1,  cnt=4, pat=3, y=0,  sp=0}
    w[#w+1]={t=650,tp=0,  cnt=10,pat=2, y=50, sp=5}
    w[#w+1]={t=750,tp=2,  cnt=2, pat=0, y=30, sp=40}
    w[#w+1]={t=750,tp=2,  cnt=2, pat=0, y=90, sp=40}
    w[#w+1]={t=900,tp=1,  cnt=4, pat=0, y=60, sp=12}
    w[#w+1]={t=1050,tp=0, cnt=12,pat=2, y=60, sp=5}
    w[#w+1]={t=1200,tp=3, cnt=2, pat=0, y=38, sp=56}
    w[#w+1]={t=1350,boss=1}
  else
    w[#w+1]={t=60, tp=0,  cnt=9, pat=2, y=30, sp=5}
    w[#w+1]={t=60, tp=0,  cnt=9, pat=2, y=90, sp=5}
    w[#w+1]={t=200,tp=1,  cnt=4, pat=3, y=0,  sp=0}
    w[#w+1]={t=300,tp=2,  cnt=4, pat=0, y=20, sp=20}
    w[#w+1]={t=400,tp=0,  cnt=12,pat=1, y=60, sp=5}
    w[#w+1]={t=500,tp=1,  cnt=5, pat=0, y=50, sp=12}
    w[#w+1]={t=500,tp=2,  cnt=3, pat=0, y=100,sp=28}
    w[#w+1]={t=650,tp=0,  cnt=15,pat=2, y=60, sp=4}
    w[#w+1]={t=800,tp=3,  cnt=3, pat=0, y=30, sp=34}
    w[#w+1]={t=900,tp=1,  cnt=5, pat=3, y=0,  sp=0}
    w[#w+1]={t=1000,tp=0, cnt=10,pat=0, y=25, sp=6}
    w[#w+1]={t=1000,tp=0, cnt=10,pat=0, y=95, sp=6}
    w[#w+1]={t=1150,tp=2, cnt=5, pat=0, y=60, sp=14}
    w[#w+1]={t=1300,boss=2}
  end
  return w
end

-- =============================================
-- SPAWN WAVE
-- =============================================
local function spawn_wave(w)
  if w.boss then
    -- create boss
    boss_active=true
    local bx=W+20
    local bhp={50,80,120}
    local bw={20,22,26}
    local bh={18,20,22}
    boss={
      tp=w.boss, x=bx, y=H/2-10,
      tx=W-bw[w.boss+1]-8,
      w=bw[w.boss+1], h=bh[w.boss+1],
      hp=bhp[w.boss+1], mhp=bhp[w.boss+1],
      angle=0, ftimer=0, alive=true,
      entering=true, sweep=0,
      drone_t=0
    }
    return
  end
  -- pat: 0=line,1=vee,2=sine,3=random
  for i=1,w.cnt do
    local ex=W+5+(i-1)*(w.sp or 8)
    local ey=w.y
    if w.pat==3 then
      ey=10+math.random()*(H-20)
      ex=W+5+(i-1)*16
    elseif w.pat==1 then
      local mid=math.floor(w.cnt/2)
      ey=w.y+math.abs(i-1-mid)*7
    elseif w.pat==2 then
      ey=w.y+math.sin((i-1)*0.8)*16
    end
    ey=math.max(5,math.min(H-5,ey))
    local tp=w.tp
    local hp,spd,srate,ew,eh,pts
    if tp==0 then -- drone
      hp=1; spd=0.7+math.random()*0.4; srate=0; ew=5; eh=5; pts=100
    elseif tp==1 then -- gunship
      hp=3; spd=0.35; srate=90; ew=8; eh=8; pts=250
    elseif tp==2 then -- turret
      hp=5; spd=0.22; srate=60; ew=7; eh=7; pts=300
    else -- shieldgen
      hp=8; spd=0.18; srate=120; ew=8; eh=8; pts=500
    end
    enemies[#enemies+1]={
      x=ex,y=ey,w=ew,h=eh,
      hp=hp,mhp=hp,spd=spd,tp=tp,
      stimer=math.random()*srate,
      srate=srate, sinoff=math.random()*6.28,
      alive=true, flash=0, pts=pts
    }
  end
end

-- =============================================
-- SPAWNERS
-- =============================================
local function spawn_particle(x,y,col,cnt,spd)
  for _=1,cnt do
    local a=math.random()*6.28
    local s=(0.5+math.random())*(spd or 1.5)
    particles[#particles+1]={
      x=x,y=y,
      vx=math.cos(a)*s, vy=math.sin(a)*s,
      life=20+math.random()*20, col=col
    }
  end
end

local function add_ftext(x,y,txt,col)
  ftexts[#ftexts+1]={x=x,y=y,txt=txt,col=col,life=50}
end

local function spawn_powerup(x,y)
  local tp=math.random(0,3)
  powerups[#powerups+1]={x=x,y=y,tp=tp,angle=0}
end

-- =============================================
-- RESET / START
-- =============================================
local function reset_game()
  px=30; py=H/2-4
  pshields=3; pweapon=0
  pemp=1; pemp_cd=0
  pinv=0; pfire=0; palive=true; pdead_timer=0
  bullets={}; ebullets={}; enemies={}
  particles={}; powerups={}; ftexts={}; empwaves={}
  score=0; chain=1; chain_t=0
  shake_mag=0; shake_x=0; shake_y=0
  level=0; story_phase=0
  lvl_timer=0; wave_idx=1
  boss_active=false; boss=nil
  init_stars()
end

local function start_level(lvl)
  level=lvl
  level_waves=gen_waves(lvl)
  lvl_timer=0; wave_idx=1
  boss_active=false; boss=nil
  enemies={}; ebullets={}; powerups={}; empwaves={}
  px=30; py=H/2-4
  pfire=0
  state="PLAYING"
end

-- =============================================
-- HIT PLAYER
-- =============================================
local function hit_player()
  pshields=pshields-1
  pinv=90
  shake_mag=4
  spawn_particle(px+5,py+4,11,8,1.5)
  if pshields<=0 then
    palive=false
    pdead_timer=90
    spawn_particle(px+5,py+4,12,20,2.5)
    if score>hi then hi=score end
  end
end

-- =============================================
-- UPDATE
-- =============================================
local function update_stars()
  for _,s in ipairs(stars) do
    s.x=s.x-s.spd
    if s.x<0 then s.x=W; s.y=math.random(0,H-1) end
  end
end

local function update_playing()
  lvl_timer=lvl_timer+1

  -- spawn waves
  while wave_idx<=#level_waves and level_waves[wave_idx].t<=lvl_timer do
    spawn_wave(level_waves[wave_idx])
    wave_idx=wave_idx+1
  end

  -- player input
  local dx,dy=0,0
  if btn(2) then dx=-1 end
  if btn(3) then dx=1 end
  if btn(0) then dy=-1 end
  if btn(1) then dy=1 end

  if palive then
    px=px+dx*pspd
    py=py+dy*pspd
    -- constrain to left 40%
    px=math.max(2, math.min(W*0.42-8, px))
    py=math.max(2, math.min(H-10, py))
  end

  if pinv>0 then pinv=pinv-1 end
  if pemp_cd>0 then pemp_cd=pemp_cd-1 end

  -- fire
  local shooting=(btn(4) or btn(5))
  if shooting and palive then
    pfire=pfire-1
    if pfire<=0 then
      if pweapon==0 then
        bullets[#bullets+1]={x=px+8,y=py+1,vx=3.5,vy=0,tp=0,life=80}
        bullets[#bullets+1]={x=px+8,y=py+6,vx=3.5,vy=0,tp=0,life=80}
        pfire=8
      elseif pweapon==1 then
        bullets[#bullets+1]={x=px+8,y=py+3,vx=3.5,vy=-0.8,tp=1,life=70}
        bullets[#bullets+1]={x=px+8,y=py+3,vx=3.5,vy=0,   tp=1,life=70}
        bullets[#bullets+1]={x=px+8,y=py+3,vx=3.5,vy=0.8, tp=1,life=70}
        pfire=12
      else -- homing
        bullets[#bullets+1]={x=px+8,y=py+3,vx=2.5,vy=0,tp=2,life=120,tgt=nil}
        pfire=18
      end
    end
  else
    if pfire<0 then pfire=0 end
  end

  -- EMP  (btn 5=B in TIC-80, use separate check)
  if btnp(5) and pweapon~=2 then end  -- placeholder, actual EMP on btn index...
  -- EMP fires on button A long-hold or dedicated: use btn(4) with z-key for EMP via separate mapping
  -- In TIC-80: btn(4)=A, btn(5)=B. We'll use btn(5) for EMP when not shooting via btn(4)
  -- Remap: A=fire, B=EMP
  if btnp(5) and pemp>0 and pemp_cd<=0 and palive then
    empwaves[#empwaves+1]={x=px+5,y=py+4,r=0,maxr=100}
    pemp=pemp-1
    pemp_cd=600
    shake_mag=math.max(shake_mag,5)
  end

  -- update player bullets
  for i=#bullets,1,-1 do
    local b=bullets[i]
    -- homing
    if b.tp==2 and b.life>90 then
      local nt=nil; local md=9999
      for _,e in ipairs(enemies) do
        if e.alive then
          local d=math.sqrt((e.x-b.x)^2+(e.y-b.y)^2)
          if d<md then md=d; nt=e end
        end
      end
      if boss and boss.alive then
        local d=math.sqrt((boss.x-b.x)^2+(boss.y-b.y)^2)
        if d<md then nt=boss end
      end
      if nt then
        local ang=math.atan2((nt.y+(nt.h or 0)/2)-b.y, nt.x-b.x)
        b.vx=b.vx+math.cos(ang)*0.3
        b.vy=b.vy+math.sin(ang)*0.3
        local sp=math.sqrt(b.vx^2+b.vy^2)
        if sp>3.5 then b.vx=b.vx/sp*3.5; b.vy=b.vy/sp*3.5 end
      end
    end
    b.x=b.x+b.vx; b.y=b.y+b.vy; b.life=b.life-1
    if b.x>W+5 or b.x<-5 or b.y<-5 or b.y>H+5 or b.life<=0 then
      table.remove(bullets,i)
    end
  end

  -- update enemy bullets
  for i=#ebullets,1,-1 do
    local b=ebullets[i]
    b.x=b.x+b.vx; b.y=b.y+b.vy; b.life=b.life-1
    if b.x<-5 or b.x>W+5 or b.y<-5 or b.y>H+5 or b.life<=0 then
      table.remove(ebullets,i)
    elseif palive and pinv<=0 then
      if b.x>px and b.x<px+8 and b.y>py and b.y<py+8 then
        hit_player()
        table.remove(ebullets,i)
      end
    end
  end

  -- update enemies
  for i=#enemies,1,-1 do
    local e=enemies[i]
    if not e.alive then table.remove(enemies,i) goto continue_e end
    if e.flash>0 then e.flash=e.flash-1 end

    -- movement
    if e.tp==0 then -- drone
      e.x=e.x-e.spd
      e.y=e.y+math.sin(t*0.05+e.sinoff)*0.8
    elseif e.tp==1 then -- gunship
      e.x=e.x-e.spd
      e.y=e.y+math.sin(t*0.03+e.sinoff)*0.5
    else -- turret/shieldgen
      e.x=e.x-e.spd
    end

    -- shooting
    if e.srate>0 and e.x<W-8 then
      e.stimer=e.stimer+1
      if e.stimer>=e.srate then
        e.stimer=0
        if e.tp==2 then -- turret aimed
          local ang=math.atan2(py+4-e.y-e.h/2, px-e.x)
          ebullets[#ebullets+1]={x=e.x,y=e.y+e.h/2,vx=math.cos(ang)*1.8,vy=math.sin(ang)*1.8,life=140}
        elseif e.tp==1 then -- gunship straight
          ebullets[#ebullets+1]={x=e.x-2,y=e.y+e.h/2,vx=-2,vy=0,life=100}
        elseif e.tp==3 then -- shieldgen radial
          for a=0,3 do
            local ang=a*1.5708+t*0.02
            ebullets[#ebullets+1]={x=e.x+e.w/2,y=e.y+e.h/2,vx=math.cos(ang)*1.2,vy=math.sin(ang)*1.2,life=80}
          end
        end
      end
    end

    -- off left
    if e.x<-15 then table.remove(enemies,i) goto continue_e end

    -- bullet hit
    for j=#bullets,1,-1 do
      local b=bullets[j]
      if b.x+2>e.x and b.x<e.x+e.w and b.y+2>e.y and b.y<e.y+e.h then
        e.hp=e.hp-1
        e.flash=4
        table.remove(bullets,j)
        if e.hp<=0 then
          e.alive=false
          spawn_particle(e.x+e.w/2,e.y+e.h/2,12,10,1.8)
          shake_mag=math.max(shake_mag,2)
          local pts=e.pts*chain
          score=score+pts
          add_ftext(e.x,e.y,"+"..pts,4)
          chain_t=100
          chain=math.min(8,chain+1)
          if math.random()<0.12 then spawn_powerup(e.x,e.y) end
        end
        break
      end
    end

    -- collide player
    if palive and pinv<=0 and e.alive then
      if px+8>e.x and px<e.x+e.w and py+8>e.y and py<e.y+e.h then
        hit_player()
        e.hp=e.hp-2
        if e.hp<=0 then
          e.alive=false
          spawn_particle(e.x+e.w/2,e.y+e.h/2,12,8,1.8)
        end
      end
    end
    ::continue_e::
  end

  -- EMP waves
  for i=#empwaves,1,-1 do
    local emp=empwaves[i]
    emp.r=emp.r+5
    if emp.r>=emp.maxr then table.remove(empwaves,i) goto continue_emp end
    for _,e in ipairs(enemies) do
      if e.alive then
        local d=math.sqrt((e.x+e.w/2-emp.x)^2+(e.y+e.h/2-emp.y)^2)
        if d<emp.r+6 and d>emp.r-12 then
          e.hp=e.hp-3
          if e.hp<=0 then
            e.alive=false
            spawn_particle(e.x+e.w/2,e.y+e.h/2,13,8,2)
            score=score+e.pts
          end
        end
      end
    end
    for j=#ebullets,1,-1 do
      local b=ebullets[j]
      local d=math.sqrt((b.x-emp.x)^2+(b.y-emp.y)^2)
      if d<emp.r+3 then table.remove(ebullets,j) end
    end
    ::continue_emp::
  end

  -- boss update
  if boss and boss.alive then
    if boss.entering then
      boss.x=boss.x+(boss.tx-boss.x)*0.03
      if math.abs(boss.x-boss.tx)<1.5 then
        boss.entering=false; boss.x=boss.tx
      end
    else
      boss.y=H/2-boss.h/2+math.sin(t*0.015)*28
    end

    boss.ftimer=boss.ftimer+1
    if not boss.entering then
      if boss.tp==0 then
        boss.angle=boss.angle+0.02
        if boss.ftimer%30==0 then
          for a=0,3 do
            local ang=boss.angle+a*1.5708
            ebullets[#ebullets+1]={x=boss.x+boss.w/2,y=boss.y+boss.h/2,vx=math.cos(ang)*1.5,vy=math.sin(ang)*1.5,life=160}
          end
        end
      elseif boss.tp==1 then
        boss.drone_t=boss.drone_t+1
        if boss.drone_t%180==0 and #enemies<12 then
          spawn_wave({t=0,tp=0,cnt=3,pat=0,y=boss.y+boss.h/2,sp=12})
        end
        if boss.ftimer%45==0 then
          local ang=math.atan2(py+4-boss.y-boss.h/2,px-boss.x)
          ebullets[#ebullets+1]={x=boss.x,y=boss.y+boss.h/2,vx=math.cos(ang)*2,vy=math.sin(ang)*2,life=140}
        end
      else -- leviathan
        local hpct=boss.hp/boss.mhp
        if hpct>0.66 then
          if boss.ftimer%20==0 then
            for k=-2,2 do
              ebullets[#ebullets+1]={x=boss.x,y=boss.y+boss.h/2+k*5,vx=-2,vy=k*0.2,life=160}
            end
          end
        elseif hpct>0.33 then
          boss.sweep=boss.sweep+0.03
          if boss.ftimer%10==0 then
            local ang=math.sin(boss.sweep)*1.2
            ebullets[#ebullets+1]={x=boss.x,y=boss.y+boss.h/2,vx=math.cos(math.pi+ang)*2.5,vy=math.sin(math.pi+ang)*2.5,life=120}
          end
          if boss.ftimer%60==0 and #enemies<8 then
            spawn_wave({t=0,tp=0,cnt=4,pat=3,y=0,sp=0})
          end
        else
          if boss.ftimer%6==0 then
            local ang=math.atan2(py+4-boss.y-boss.h/2,px-boss.x)+(math.random()-0.5)*0.5
            ebullets[#ebullets+1]={x=boss.x,y=boss.y+boss.h/2,vx=math.cos(ang)*2.2,vy=math.sin(ang)*2.2,life=150}
          end
          if boss.ftimer%90==0 then
            for a=0,7 do
              local ang=a*0.785398
              ebullets[#ebullets+1]={x=boss.x+boss.w/2,y=boss.y+boss.h/2,vx=math.cos(ang)*1.5,vy=math.sin(ang)*1.5,life=160}
            end
          end
        end
      end
    end

    -- bullet hit boss
    for j=#bullets,1,-1 do
      local b=bullets[j]
      if b.x+2>boss.x and b.x<boss.x+boss.w and b.y+2>boss.y and b.y<boss.y+boss.h then
        boss.hp=boss.hp-1
        spawn_particle(b.x,b.y,11,3,1)
        table.remove(bullets,j)
        if boss.hp<=0 then
          boss.alive=false
          spawn_particle(boss.x+boss.w/2,boss.y+boss.h/2,12,30,3)
          spawn_particle(boss.x+boss.w/2,boss.y+boss.h/2,4,20,2.5)
          shake_mag=8
          score=score+5000*chain
          add_ftext(boss.x,boss.y,"+"..(5000*chain),4)
          if score>hi then hi=score end
          -- transition
          boss_active=false
          if level<2 then
            story_phase=level+1
            story_txt=STORIES[story_phase]
            story_idx=0; story_timer=0
            state="STORY"
          else
            story_txt=STORIES[3]
            story_idx=0; story_timer=0
            state="WIN"
          end
        end
        break
      end
    end

    -- EMP vs boss
    for _,emp in ipairs(empwaves) do
      local d=math.sqrt((boss.x+boss.w/2-emp.x)^2+(boss.y+boss.h/2-emp.y)^2)
      if d<emp.r+12 and d>emp.r-18 then
        boss.hp=boss.hp-3
        spawn_particle(boss.x+boss.w/2,boss.y+boss.h/2,13,6,2)
      end
    end

    -- boss collide player
    if palive and pinv<=0 and not boss.entering then
      if px+8>boss.x and px<boss.x+boss.w and py+8>boss.y and py<boss.y+boss.h then
        hit_player()
      end
    end
  end

  -- update particles
  for i=#particles,1,-1 do
    local p=particles[i]
    p.x=p.x+p.vx; p.y=p.y+p.vy
    p.vx=p.vx*0.96; p.vy=p.vy*0.96
    p.life=p.life-1
    if p.life<=0 then table.remove(particles,i) end
  end

  -- update powerups
  for i=#powerups,1,-1 do
    local p=powerups[i]
    p.x=p.x-0.5
    p.angle=p.angle+0.05
    if p.x<-10 then table.remove(powerups,i) goto continue_pu end
    if palive and px+8>p.x and px<p.x+6 and py+8>p.y and py<p.y+6 then
      if p.tp==0 then pweapon=1; add_ftext(p.x,p.y,"SPREAD",12)
      elseif p.tp==1 then pweapon=2; add_ftext(p.x,p.y,"HOMING",10)
      elseif p.tp==2 then pshields=math.min(3,pshields+1); add_ftext(p.x,p.y,"SHIELD+",11)
      else pemp=math.min(3,pemp+1); add_ftext(p.x,p.y,"EMP+",13) end
      table.remove(powerups,i)
    end
    ::continue_pu::
  end

  -- floating texts
  for i=#ftexts,1,-1 do
    local f=ftexts[i]
    f.y=f.y-0.5
    f.life=f.life-1
    if f.life<=0 then table.remove(ftexts,i) end
  end

  -- chain decay
  if chain_t>0 then
    chain_t=chain_t-1
    if chain_t<=0 then chain=1 end
  end

  -- shake
  if shake_mag>0 then
    shake_x=(math.random()-0.5)*shake_mag*2
    shake_y=(math.random()-0.5)*shake_mag*2
    shake_mag=shake_mag*0.85
    if shake_mag<0.5 then shake_mag=0; shake_x=0; shake_y=0 end
  end

  -- dead timer
  if not palive then
    pdead_timer=pdead_timer-1
    if pdead_timer<=0 then
      state="GAMEOVER"
      t=0
    end
  end
end

-- =============================================
-- DRAW HELPERS
-- =============================================
local function draw_player()
  if not palive then return end
  if pinv>0 and math.floor(pinv/4)%2==0 then return end

  local x=math.floor(px+shake_x)
  local y=math.floor(py+shake_y)

  -- thruster flame
  local fc=(t%8<4) and 9 or 3
  rect(x-3,y+2,3,4,fc)
  rect(x-2,y+3,2,2,4)

  -- body (cyan/teal angular ship)
  -- nose tip
  pix(x+8,y+3,11)
  pix(x+8,y+4,11)
  -- upper hull
  rect(x+3,y+1,5,2,14)
  rect(x+5,y+0,2,1,11)
  -- center hull
  rect(x+1,y+3,7,2,11)
  rect(x+0,y+3,1,2,13)
  -- lower hull (mirror)
  rect(x+3,y+5,5,2,14)
  rect(x+5,y+7,2,1,11)
  rect(x+1,y+5,1,2,11)

  -- cockpit glow
  pix(x+6,y+3,8)
  pix(x+6,y+4,8)
  pix(x+7,y+3,11)
  pix(x+7,y+4,11)

  -- weapon pylons pink
  pix(x+4,y+2,12)
  pix(x+4,y+5,12)

  -- shield ring
  if pshields>0 then
    if t%20<10 then
      rectb(x-1,y-1,12,10,13)
    end
  end
end

local function get_bullet_col(tp)
  if tp==0 then return 11 end -- laser cyan
  if tp==1 then return 12 end -- spread pink
  return 10  -- homing green
end

local function draw_bullets()
  for _,b in ipairs(bullets) do
    local c=get_bullet_col(b.tp)
    if b.tp==0 then
      rect(math.floor(b.x),math.floor(b.y),4,1,c)
    elseif b.tp==1 then
      rect(math.floor(b.x),math.floor(b.y),3,1,c)
    else
      circ(math.floor(b.x),math.floor(b.y),2,c)
    end
  end
end

local function draw_enemy_bullets()
  for _,b in ipairs(ebullets) do
    pix(math.floor(b.x),math.floor(b.y),2)
    pix(math.floor(b.x)+1,math.floor(b.y),3)
  end
end

local ECOLS={
  [0]={2,2,12},   -- drone: dark red / red / pink
  [1]={14,1,11},  -- gunship: dark blue / purple / cyan
  [2]={15,7,9},   -- turret: gray / teal / orange
  [3]={6,10,8}    -- shieldgen: dark green / green / white
}

local function draw_enemy(e)
  if not e.alive then return end
  local x=math.floor(e.x+shake_x)
  local y=math.floor(e.y+shake_y)
  local c=ECOLS[e.tp]
  local flash=(e.flash>0)

  if e.tp==0 then -- drone: small circle-ish
    local cc= flash and 8 or c[2]
    rect(x+1,y,e.w-2,e.h,cc)
    rect(x,y+1,e.w,e.h-2,cc)
    pix(x+2,y+2,flash and 8 or c[3])
  elseif e.tp==1 then -- gunship
    local cc= flash and 8 or c[1]
    rect(x,y+2,e.w,e.h-4,cc)
    rect(x+2,y,e.w-4,e.h,flash and 8 or c[2])
    pix(x+3,y+3,flash and 8 or c[3])
  elseif e.tp==2 then -- turret
    local cc= flash and 8 or c[1]
    rect(x+1,y+1,e.w-2,e.h-2,cc)
    rect(x,y+2,e.w,e.h-4,flash and 8 or c[2])
    -- barrel
    rect(x-2,y+e.h/2-1,3,2,flash and 8 or c[3])
  else -- shieldgen
    local cc= flash and 8 or c[1]
    circ(math.floor(x+e.w/2),math.floor(y+e.h/2),math.floor(e.w/2),cc)
    if t%20<10 then
      rectb(x-1,y-1,e.w+2,e.h+2,c[3])
    end
  end

  -- HP bar for multi-hp enemies
  if e.mhp>1 then
    local bw=e.w
    local hp_pct=e.hp/e.mhp
    rect(x,y-3,bw,2,0)
    local hc= hp_pct>0.5 and 10 or (hp_pct>0.25 and 4 or 2)
    rect(x,y-3,math.max(1,math.floor(bw*hp_pct)),2,hc)
  end
end

local function draw_boss()
  if not boss or not boss.alive then return end
  local x=math.floor(boss.x+shake_x)
  local y=math.floor(boss.y+shake_y)
  local cx=x+math.floor(boss.w/2)
  local cy=y+math.floor(boss.h/2)

  -- entering warning
  if boss.entering then
    if t%20<10 then
      local bname=BOSS_NAMES[boss.tp+1]
      print("WARNING",86,45,2,true,1)
      print(bname,math.floor(120-#bname*2),55,12,true,1)
    end
  end

  -- boss body
  if boss.tp==0 then -- Defense Satellite
    -- core
    rect(x+4,y+4,boss.w-8,boss.h-8,1)
    circ(cx,cy,math.floor(boss.h/2)-2,2)
    pix(cx,cy,12)
    -- rotating arms
    local ang=boss.angle
    for a=0,3 do
      local ra=ang+a*1.5708
      local ex2=math.floor(cx+math.cos(ra)*10)
      local ey2=math.floor(cy+math.sin(ra)*10)
      line(cx,cy,ex2,ey2,12)
      rect(ex2-1,ey2-1,3,3,3)
    end
  elseif boss.tp==1 then -- Cyborg Carrier
    rect(x,y+4,boss.w,boss.h-8,14)
    rect(x+3,y,boss.w-6,boss.h,1)
    rect(x+5,y+2,boss.w-10,boss.h-4,13)
    -- cannons
    rect(x-2,y+4,3,4,7)
    rect(x-2,y+boss.h-8,3,4,7)
    pix(cx,cy,8)
    pix(cx-1,cy,11)
  else -- Leviathan
    -- bulk
    rect(x,y,boss.w,boss.h,1)
    rect(x+2,y+2,boss.w-4,boss.h-4,14)
    -- glowing core
    circ(cx,cy,4,2)
    pix(cx,cy,12)
    if t%10<5 then pix(cx-1,cy-2,8); pix(cx+1,cy+2,8) end
    -- extra wings
    rect(x-3,y+4,3,boss.h-8,0)
    rect(x+boss.w,y+4,3,boss.h-8,0)
  end

  -- boss HP bar
  local bw=80
  local bx=math.floor((W-bw)/2)
  local hp_pct=math.max(0,boss.hp/boss.mhp)
  rect(bx-1,2,bw+2,5,0)
  rect(bx,3,math.floor(bw*hp_pct),3,12)
  rectb(bx-1,2,bw+2,5,2)
  local bname=BOSS_NAMES[boss.tp+1]
  print(bname,math.floor(W/2-#bname*2),1,8,true,1)
end

local function draw_powerup(p)
  local x=math.floor(p.x)
  local y=math.floor(p.y)
  local cols={12,10,11,13}
  local c=cols[p.tp+1]
  -- spinning gem
  if t%20<10 then
    rect(x+1,y,4,6,c)
    rect(x,y+1,6,4,c)
  else
    rect(x,y+2,6,2,c)
    rect(x+2,y,2,6,c)
  end
end

local function draw_emp_waves()
  for _,emp in ipairs(empwaves) do
    local alpha=1-emp.r/emp.maxr
    if alpha>0 then
      local r=math.floor(emp.r)
      local cx=math.floor(emp.x+shake_x)
      local cy=math.floor(emp.y+shake_y)
      -- ring (use circb if r big enough, else skip)
      if r>1 then
        circb(cx,cy,r,13)
        if r>3 then circb(cx,cy,r-1,13) end
      end
    end
  end
end

local function draw_particles()
  for _,p in ipairs(particles) do
    pix(math.floor(p.x+shake_x),math.floor(p.y+shake_y),p.col)
  end
end

local function draw_ftexts()
  for _,f in ipairs(ftexts) do
    if f.life>0 then
      print(f.txt,math.floor(f.x),math.floor(f.y),f.col,true,1)
    end
  end
end

local function draw_hud()
  -- score
  print("SCORE "..score,2,2,11,true,1)
  -- chain
  if chain>1 then
    print("x"..chain,2,10,4,true,1)
  end
  -- shields
  for i=1,3 do
    local c=i<=pshields and 11 or 7
    rect(W-4-i*6,2,5,3,c)
  end
  -- weapon
  local wnames={"DUAL","SPRD","HOME"}
  local wcols={11,12,10}
  print(wnames[pweapon+1],W-28,8,wcols[pweapon+1],true,1)
  -- EMP
  print("E:"..pemp,W-16,14,13,true,1)
  -- level
  local lname=LEVEL_NAMES[level+1]
  print("LV"..(level+1).." "..lname,math.floor(W/2-#lname*2-4),H-7,14,true,1)
  -- hi
  if hi>0 then
    print("HI "..hi,2,H-7,7,true,1)
  end
end

local function draw_stars()
  local star_cols={14,7,15,8}
  for _,s in ipairs(stars) do
    local c=star_cols[s.layer+1] or 15
    pix(math.floor(s.x),math.floor(s.y),c)
  end
end

-- =============================================
-- DRAW STORY/TITLE SCREENS
-- =============================================
local function draw_story_screen()
  cls(0)
  draw_stars()
  -- terminal border
  rect(8,8,W-16,H-16,0)
  rectb(8,8,W-16,H-16,6)
  -- header
  print("CHROME VIPER TERMINAL v2.187",12,11,10,true,1)
  line(8,18,W-9,18,6)
  -- story text
  local visible=string.sub(story_txt,1,story_idx)
  local lines={}
  local cur=""
  for c in visible:gmatch(".") do
    if c=="\n" then lines[#lines+1]=cur; cur=""
    else cur=cur..c end
  end
  lines[#lines+1]=cur
  -- wrap and print lines
  local ly2=22
  local max_chars=28
  for _,ln in ipairs(lines) do
    if #ln<=max_chars then
      print(ln,12,ly2,10,true,1)
      ly2=ly2+7
    else
      -- simple word wrap
      local words={}
      for w in ln:gmatch("%S+") do words[#words+1]=w end
      local cl=""
      for _,wrd in ipairs(words) do
        local test=cl=="" and wrd or (cl.." "..wrd)
        if #test>max_chars and cl~="" then
          print(cl,12,ly2,10,true,1)
          ly2=ly2+7; cl=wrd
        else cl=test end
      end
      if cl~="" then print(cl,12,ly2,10,true,1); ly2=ly2+7 end
    end
    if ly2>H-18 then break end
  end
  -- cursor blink
  if story_idx<#story_txt and t%20<10 then
    rect(12,ly2,3,5,10)
  end
  -- continue prompt
  if story_idx>=#story_txt and t%30<15 then
    print("[A TO CONTINUE]",math.floor(W/2-30),H-12,11,true,1)
  end
end

local function draw_title()
  cls(0)
  draw_stars()
  -- digital rain effect (simplified)
  for i=1,8 do
    local rx=(i*29)%W
    local rc=math.floor(t*0.1+i)%8
    local ch=string.char(0x41+rc)
    print(ch,rx,(t*2+i*17)%H,6,true,1)
  end
  -- title
  print("CHROME",math.floor(W/2-24),38,11,true,2)
  print("VIPER",math.floor(W/2-20),52,12,true,2)
  print("NEON ABYSS",math.floor(W/2-20),68,15,true,1)
  -- story blurb
  print("Year 2187. Megacorp AXIOM",math.floor(W/2-49),80,7,true,1)
  print("controls the colonies.",math.floor(W/2-43),87,7,true,1)
  print("You pilot the Chrome Viper.",math.floor(W/2-52),94,7,true,1)
  print("End their reign.",math.floor(W/2-32),101,7,true,1)
  -- controls
  print("ARROWS:MOVE A:FIRE B:EMP",math.floor(W/2-47),111,14,true,1)
  -- press start blink
  if t%30<15 then
    print("PRESS A TO START",math.floor(W/2-31),122,11,true,1)
  end
  -- hi score
  if hi>0 then
    print("HI:"..hi,math.floor(W/2-#("HI:"..hi)*2),129,4,true,1)
  end
end

local function draw_gameover()
  cls(0)
  draw_stars()
  -- glitch bars
  if t%7<2 then
    rect(0,math.random(0,H-3),W,2,12)
  end
  print("SYSTEM",math.floor(W/2-24),40,12,true,2)
  print("FAILURE",math.floor(W/2-28),58,12,true,2)
  print("CHROME VIPER DESTROYED",math.floor(W/2-43),80,8,true,1)
  print("SCORE: "..score,math.floor(W/2-#("SCORE: "..score)*2),90,11,true,1)
  if hi>0 then
    print("HI: "..hi,math.floor(W/2-#("HI: "..hi)*2),100,4,true,1)
  end
  print("AXIOM prevails. Colonies",math.floor(W/2-47),112,7,true,1)
  print("under corporate control.",math.floor(W/2-47),119,7,true,1)
  if t>60 and t%30<15 then
    print("PRESS A TO RETRY",math.floor(W/2-31),129,11,true,1)
  end
end

-- =============================================
-- MAIN TIC FUNCTION
-- =============================================
function TIC()
  t=t+1

  -- ---- UPDATE ----
  if state=="START" then
    update_stars()
    if btnp(4) or btnp(5) then
      reset_game()
      story_txt=STORIES[0]
      story_idx=0; story_timer=0
      state="STORY"
    end

  elseif state=="STORY" then
    update_stars()
    story_timer=story_timer+1
    if story_timer%2==0 and story_idx<#story_txt then
      story_idx=story_idx+1
    end
    if (btnp(4) or btnp(5)) and story_timer>30 then
      if story_idx<#story_txt then
        story_idx=#story_txt
      else
        if story_phase<=2 then
          start_level(story_phase)
        end
      end
    end

  elseif state=="WIN" then
    update_stars()
    story_timer=story_timer+1
    if story_timer%2==0 and story_idx<#story_txt then
      story_idx=story_idx+1
    end
    if (btnp(4) or btnp(5)) and story_timer>30 then
      if story_idx<#story_txt then
        story_idx=#story_txt
      else
        state="START"; t=0
      end
    end

  elseif state=="GAMEOVER" then
    update_stars()
    if t>90 and (btnp(4) or btnp(5)) then
      state="START"; t=0
    end

  elseif state=="PLAYING" then
    update_stars()
    update_playing()
  end

  -- ---- DRAW ----
  if state=="START" then
    draw_title()

  elseif state=="STORY" or state=="WIN" then
    draw_story_screen()

  elseif state=="GAMEOVER" then
    draw_gameover()

  elseif state=="PLAYING" then
    cls(0)
    -- bg stars
    draw_stars()
    -- game objects
    for _,p in ipairs(powerups) do draw_powerup(p) end
    for _,e in ipairs(enemies) do draw_enemy(e) end
    draw_boss()
    draw_bullets()
    draw_enemy_bullets()
    draw_player()
    draw_emp_waves()
    draw_particles()
    draw_ftexts()
    draw_hud()
    -- scanline overlay
    for sy=0,H-1,2 do
      line(0,sy,W-1,sy,0)
    end
  end
end

-- init
init_stars()
