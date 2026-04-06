-- title: Dragon Fury
-- author: retrogames
-- script: lua
-- desc: Streets of Vengeance beat em up

-- TIC-80: 240x136, Sweetie 16 palette
-- 0=dark navy,1=dark purple,2=red,3=orange,4=yellow,5=light green
-- 6=teal,7=dark teal,8=white,9=orange,10=green,11=light blue
-- 12=magenta,13=teal,14=dark blue-gray,15=gray

-- Colors shorthand
local BG=0 local PUR=1 local RED=2 local ORG=3 local YEL=4
local LGR=5 local TEA=6 local DTL=7 local WHT=8 local ORG2=9
local GRN=10 local LBL=11 local MAG=12 local TEA2=13 local DBG=14 local GRY=15

-- Screen
local SW=240 local SH=136
local GMIN=96 local GMAX=120 -- ground depth band

-- Game state
local STATE="TITLE"
local frame=0
local score=0
local camX=0
local stageIdx=1
local waveIdx=0
local scrollLocked=false
local waveClearTimer=0
local stageIntroTimer=0
local gameOverTimer=0
local continueTimer=0
local titleBlink=0
local screenShake=0
local shakeTimer=0

-- Input prev state
local prevA=false local prevB=false local prevSt=false

-- Story system
local storyLines={}
local storyLineIdx=1
local storyCharIdx=0
local storyCharTimer=0
local storyPhase=""
local storyCb=nil
local midDlg=nil
local midDlgTimer=0
local midShown={}
local bossData=nil
local bossIntroTimer=0

-- Particles (flat arrays)
local pX={} local pY={} local pVX={} local pVY={} local pLife={} local pMaxL={} local pCol={} local pN=0

-- Hit numbers
local hnX={} local hnY={} local hnV={} local hnL={} local hnC={} local hnN=0

-- Player
local pl={}
-- Enemies flat arrays
local eX={} local eY={} local eType={} local eHP={} local eMaxHP={}
local eFace={} local eState={} local eStun={} local eHurt={} local eKBX={} local eKBY={}
local eAtkCD={} local eAtkT={} local eDead={} local eDeadT={} local eScore={}
local eIsBoss={} local eBossIdx={}
local eN=0

-- Ground weapons
local gwX={} local gwY={} local gwType={} local gwDur={} local gwN=0
-- Pickups
local puX={} local puY={} local puType={} local puN=0

-- Stage data
local STAGES={
  {name="BACK ALLEY",len=800,
   waves={
    {x=60,  e={{t="thug",n=2}}},
    {x=180, e={{t="thug",n=2},{t="knife",n=1}}},
    {x=340, e={{t="thug",n=2},{t="knife",n=1}}},
    {x=500, e={{t="thug",n=2},{t="brawler",n=1}}},
    {x=650, e={{t="boss",n=1,bi=1}}}
   },
   picks={
    {x=140,t="food"},{x=280,t="pipe",w=true},{x=420,t="food"},{x=560,t="pizza"}
   }
  },
  {name="WAREHOUSE",len=960,
   waves={
    {x=60,  e={{t="thug",n=2},{t="knife",n=1}}},
    {x=220, e={{t="brawler",n=1},{t="thug",n=2}}},
    {x=380, e={{t="knife",n=2},{t="thug",n=2}}},
    {x=540, e={{t="brawler",n=1},{t="knife",n=2}}},
    {x=700, e={{t="thug",n=2},{t="brawler",n=1}}},
    {x=830, e={{t="boss",n=1,bi=2}}}
   },
   picks={
    {x=120,t="pipe",w=true},{x=240,t="food"},{x=420,t="knife",w=true},
    {x=560,t="pizza"},{x=680,t="food"},{x=760,t="life"}
   }
  },
  {name="ROOFTOP",len=1100,
   waves={
    {x=60,  e={{t="thug",n=2},{t="knife",n=1}}},
    {x=220, e={{t="knife",n=2},{t="brawler",n=1}}},
    {x=400, e={{t="brawler",n=2},{t="thug",n=1}}},
    {x=580, e={{t="knife",n=2},{t="brawler",n=1},{t="thug",n=1}}},
    {x=760, e={{t="thug",n=2},{t="knife",n=1},{t="brawler",n=1}}},
    {x=940, e={{t="boss",n=1,bi=3}}}
   },
   picks={
    {x=110,t="pipe",w=true},{x=240,t="food"},{x=360,t="knife",w=true},
    {x=480,t="pizza"},{x=620,t="food"},{x=720,t="life"},{x=840,t="pizza"}
   }
  }
}

-- Boss intro data
local BOSS_DATA={
  {name="BLADE",title="Iron Serpent Lt.",quote="Nothing personal, Dragon boy."},
  {name="CRUSHER",title="Iron Serpent Enforcer",quote="I always follow orders."},
  {name="JIN TAKEDA",title="Former Disciple",quote="Sato chose you. Now I take everything."}
}

-- Story text (shortened to fit)
local STORY_INTRO={
  "NEO-OSAKA, 3:47 AM",
  "The Dragon Fist dojo burns.",
  "Grandmaster Sato is gone.",
  "Iron Serpents. Ten blocks east."
}
local PRE_STAGE={
  {"Back alleys. Someone here knows where Sato is."},
  {"The Iron Serpent warehouse. What are they doing to Sato?"},
  {"The rooftop. Sato kneels beside Jin. Alive but broken."}
}
local MID_STAGE={
  "You think you're tough? Wait till you see what's at the warehouse...",
  "A room. Restraints. Footage of Sato refusing to teach Jin. They hurt him. He still refuses.",
  'Jin\'s voice: "He saw what I really am. He taught YOU. His chosen son. While I got NOTHING."'
}
local POST_STAGE={
  {"Blade falls. Through his radio: \"Let him come.\" It's Jin."},
  {"A note in Sato's hand: \"Jin doesn't want the technique to sell. He wants it to destroy. Stop him.\""},
  {"Jin falls. Sato: \"I refused because the Dragon's Breath amplifies your heart. Your heart was angry.\"",
   "Jin looks tired. For the first time, not furious."}
}
local STORY_VICTORY={
  "The sun rises over Neo-Osaka.",
  "The Iron Serpents scatter.",
  "Jin Takeda turns himself in.",
  "You visit him. \"We saved you a spot.\"",
  "THE DRAGON FIST ENDURES."
}

-- Enemy stats
local EDEFS={
  thug=   {hp=60, spd=1,   dmg=8,  sc=100, armor=0},
  knife=  {hp=40, spd=1.5, dmg=12, sc=200, armor=0},
  brawler={hp=120,spd=0.6, dmg=18, sc=500, armor=3},
  boss=   {hp=280,spd=0.9, dmg=22, sc=2000,armor=2}
}

function initPlayer()
  pl={
    x=25,y=108,z=0,vz=0,
    hp=100,maxhp=100,lives=3,
    face=1,state="IDLE",
    animT=0,animF=0,
    combo=0,comboT=0,
    atkCD=0,
    invT=0,hurtT=0,
    kbx=0,kby=0,
    wpType=nil,wpDur=0,
    grabIdx=0,grabT=0,kneeN=0,
    jumpKick=false
  }
end

function spawnEnemy(t,x,y,bi)
  local d=EDEFS[t]
  eN=eN+1
  eX[eN]=x eY[eN]=y
  eType[eN]=t
  eHP[eN]=d.hp eMaxHP[eN]=d.hp
  eFace[eN]=-1 eState[eN]="IDLE"
  eStun[eN]=0 eHurt[eN]=0
  eKBX[eN]=0 eKBY[eN]=0
  eAtkCD[eN]=60+math.random(60)
  eAtkT[eN]=0
  eDead[eN]=false eDeadT[eN]=0
  eScore[eN]=d.sc
  eIsBoss[eN]=(t=="boss")
  eBossIdx[eN]=bi or 0
  return eN
end

function removeEnemy(i)
  eX[i]=eX[eN] eY[i]=eY[eN] eType[i]=eType[eN]
  eHP[i]=eHP[eN] eMaxHP[i]=eMaxHP[eN]
  eFace[i]=eFace[eN] eState[i]=eState[eN]
  eStun[i]=eStun[eN] eHurt[i]=eHurt[eN]
  eKBX[i]=eKBX[eN] eKBY[i]=eKBY[eN]
  eAtkCD[i]=eAtkCD[eN] eAtkT[i]=eAtkT[eN]
  eDead[i]=eDead[eN] eDeadT[i]=eDeadT[eN]
  eScore[i]=eScore[eN]
  eIsBoss[i]=eIsBoss[eN] eBossIdx[i]=eBossIdx[eN]
  eN=eN-1
end

function initStage(idx)
  stageIdx=idx
  camX=0 waveIdx=0 waveClearTimer=0 scrollLocked=false
  eN=0 gwN=0 puN=0 pN=0 hnN=0
  midDlg=nil midDlgTimer=0 midShown={}
  screenShake=0 shakeTimer=0
  -- reset wave boss shown flags
  local st=STAGES[idx]
  for _,w in ipairs(st.waves) do w.shown=false end
  -- spawn pickups
  for _,p in ipairs(st.picks) do
    local gy=GMIN+math.random(GMAX-GMIN)
    if p.w then
      gwN=gwN+1
      gwX[gwN]=p.x gwY[gwN]=gy gwType[gwN]=p.t
      gwDur[gwN]=(p.t=="pipe") and 8 or 10
    else
      puN=puN+1
      puX[puN]=p.x puY[puN]=gy puType[puN]=p.t
    end
  end
  -- reset player position
  pl.x=25 pl.y=108 pl.z=0 pl.vz=0
  pl.state="IDLE" pl.wpType=nil pl.wpDur=0
  pl.grabIdx=0 pl.face=1
  pl.hurtT=0 pl.kbx=0 pl.kby=0
  pl.combo=0 pl.comboT=0 pl.atkCD=0
  pl.jumpKick=false
end

function startStory(lines,phase,cb)
  storyLines=lines storyLineIdx=1 storyCharIdx=0
  storyCharTimer=0 storyPhase=phase storyCb=cb
  STATE="STORY"
end

function startGame()
  initPlayer() score=0
  initStage(1)
  startStory(STORY_INTRO,"INTRO",function()
    startStory(PRE_STAGE[1],"PRE",function()
      STATE="STAGE_INTRO" stageIntroTimer=150
    end)
  end)
end

-- Particle helpers
function spawnParticle(x,y,c,n,spd,life)
  for _=1,n do
    if pN>=60 then break end
    pN=pN+1
    local a=math.random()*6.28
    local s=spd*(0.5+math.random()*0.8)
    pX[pN]=x pY[pN]=y
    pVX[pN]=math.cos(a)*s pVY[pN]=math.sin(a)*s-0.5
    pLife[pN]=life pMaxL[pN]=life pCol[pN]=c
  end
end

function spawnHitNum(x,y,v,c)
  if hnN>=20 then return end
  hnN=hnN+1
  hnX[hnN]=x hnY[hnN]=y hnV[hnN]=v hnL[hnN]=30 hnC[hnN]=c or YEL
end

function spawnSpark(x,y)
  spawnParticle(x,y,YEL,4,2,10)
  spawnParticle(x,y,ORG,2,1.5,8)
end

-- Input helpers
function isLeft()  return btn(2) end
function isRight() return btn(3) end
function isUp()    return btn(0) end
function isDown()  return btn(1) end
function isA()     return btn(4) end
function isB()     return btn(5) end
function isSt()    return btn(6) end
function pressA()  local v=btn(4) and not prevA; return v end
function pressB()  local v=btn(5) and not prevB; return v end
function pressSt() local v=(btn(6) or btn(4)) and not prevSt; return v end

-- Wave spawning
function checkWave()
  local st=STAGES[stageIdx]
  if waveIdx>=# st.waves then return end
  local w=st.waves[waveIdx+1]
  if camX+SW*0.4>=w.x and not scrollLocked then
    -- boss wave: show intro first
    local isBoss=false
    for _,eg in ipairs(w.e) do if eg.t=="boss" then isBoss=true end end
    if isBoss and not w.shown then
      w.shown=true
      bossData=BOSS_DATA[stageIdx]
      bossIntroTimer=0 STATE="BOSS_INTRO"
      return
    end
    scrollLocked=true
    for _,eg in ipairs(w.e) do
      for _=1,eg.n do
        local side=math.random()<0.5 and -1 or 1
        local ex= side<0 and camX-20-math.random(30) or camX+SW+10+math.random(30)
        local ey=GMIN+math.random(GMAX-GMIN)
        local bi=eg.bi or 0
        local idx=spawnEnemy(eg.t,ex,ey,bi)
        eFace[idx]= side<0 and 1 or -1
      end
    end
    waveIdx=waveIdx+1
    -- mid-stage dialogue after wave 3
    local mk=stageIdx.."_"..waveIdx
    if waveIdx==3 and not midShown[mk] then
      midShown[mk]=true
      midDlg=MID_STAGE[stageIdx]
      midDlgTimer=240
    end
  end
end

-- Player update
function updatePlayer()
  if pl.state=="DEAD" then return end

  -- Invincibility
  if pl.invT>0 then pl.invT=pl.invT-1 end

  -- Hurt knockback
  if pl.hurtT>0 then
    pl.hurtT=pl.hurtT-1
    pl.x=pl.x+pl.kbx pl.y=pl.y+pl.kby
    pl.kbx=pl.kbx*0.85 pl.kby=pl.kby*0.85
    pl.y=math.max(GMIN,math.min(GMAX,pl.y))
    if pl.hurtT<=0 then pl.state="IDLE" end
    return
  end

  -- Combo timer
  if pl.comboT>0 then pl.comboT=pl.comboT-1
    else pl.combo=0 end

  if pl.atkCD>0 then pl.atkCD=pl.atkCD-1 end

  -- Grab state
  if pl.state=="GRAB" then
    pl.grabT=pl.grabT-1
    local gi=pl.grabIdx
    if gi>0 and (eDead[gi] or eHP[gi]<=0) then
      pl.state="IDLE" pl.grabIdx=0 return
    end
    if pl.grabT<=0 or gi==0 then
      if gi>0 then eState[gi]="STUN"; eStun[gi]=60 end
      pl.state="IDLE" pl.grabIdx=0 return
    end
    -- knee strikes
    if pressA() and pl.kneeN<3 then
      pl.kneeN=pl.kneeN+1
      if gi>0 then
        eHP[gi]=eHP[gi]-15
        spawnSpark(eX[gi],eY[gi]-8)
        spawnHitNum(eX[gi],eY[gi]-12,15,YEL)
        score=score+30
      end
      return
    end
    -- throw
    if pressB() and gi>0 then
      local td=pl.face
      eState[gi]="HURT" eHurt[gi]=25
      eKBX[gi]=td*7 eKBY[gi]=0
      eHP[gi]=eHP[gi]-25
      spawnHitNum(eX[gi],eY[gi]-12,25,YEL)
      score=score+80
      pl.state="IDLE" pl.grabIdx=0 return
    end
    -- keep enemy locked
    if gi>0 then
      eX[gi]=pl.x+pl.face*18 eY[gi]=pl.y
    end
    return
  end

  -- Punch animation
  if pl.state=="PUNCH" then
    pl.animT=pl.animT-1
    if pl.animT<=0 then pl.state="IDLE" pl.animF=0 end
    return
  end

  -- Special animation
  if pl.state=="SPECIAL" then
    pl.animT=pl.animT-1
    if pl.animT<=0 then pl.state="IDLE" pl.invT=0 end
    return
  end

  -- Jump physics
  if pl.z>0 or pl.vz>0 then
    pl.vz=pl.vz-0.5
    pl.z=pl.z+pl.vz
    -- jump kick
    if pl.state=="JUMP" and pressA() and not pl.jumpKick then
      pl.jumpKick=true pl.state="JKICK"
    end
    if pl.z<=0 then
      pl.z=0 pl.vz=0 pl.state="IDLE" pl.jumpKick=false
    end
    if isLeft()  then pl.x=pl.x-2; pl.face=-1 end
    if isRight() then pl.x=pl.x+2; pl.face=1 end
    clampPlayer()
    return
  end

  -- Special move (A+B or btn5 held)
  if btn(5) and btn(4) and pl.atkCD<=0 and pl.hp>12 then
    pl.state="SPECIAL" pl.animT=22
    pl.hp=pl.hp-12 pl.invT=22
    pl.atkCD=30
    spawnParticle(pl.x+4,pl.y-10,MAG,8,3,12)
    screenShake=3 shakeTimer=8
    return
  end

  -- Jump
  if pressB() then
    pl.state="JUMP" pl.vz=7 pl.jumpKick=false
    return
  end

  -- Punch
  if pressA() and pl.atkCD<=0 then
    if pl.comboT>0 and pl.combo<3 then
      pl.combo=pl.combo+1
    else
      pl.combo=1
    end
    pl.state="PUNCH" pl.animT=14
    pl.animF=pl.combo pl.comboT=20
    pl.atkCD=7
    return
  end

  -- Movement
  local mx=0 local my=0
  if isLeft()  then mx=mx-2; pl.face=-1 end
  if isRight() then mx=mx+2; pl.face=1 end
  if isUp()    then my=my-1.5 end
  if isDown()  then my=my+1.5 end

  if mx~=0 or my~=0 then pl.state="WALK" else pl.state="IDLE" end

  pl.x=pl.x+mx pl.y=pl.y+my
  pl.y=math.max(GMIN,math.min(GMAX,pl.y))
  clampPlayer()

  -- Pick up weapons
  for i=gwN,1,-1 do
    if math.abs(pl.x+4-gwX[i])<18 and math.abs(pl.y-gwY[i])<14 then
      if not pl.wpType then
        pl.wpType=gwType[i] pl.wpDur=gwDur[i]
        gwX[i]=gwX[gwN] gwY[i]=gwY[gwN]
        gwType[i]=gwType[gwN] gwDur[i]=gwDur[gwN]
        gwN=gwN-1
        sfx(0,30,5,0,15)
      end
    end
  end

  -- Pick up items
  for i=puN,1,-1 do
    if math.abs(pl.x+4-puX[i])<18 and math.abs(pl.y-puY[i])<14 then
      local pt=puType[i]
      if pt=="food" then
        pl.hp=math.min(pl.maxhp,pl.hp+25)
        score=score+300
      elseif pt=="pizza" then
        pl.hp=math.min(pl.maxhp,pl.hp+50)
        score=score+300
      elseif pt=="life" then
        pl.lives=pl.lives+1
        score=score+1000
      end
      puX[i]=puX[puN] puY[i]=puY[puN]
      puType[i]=puType[puN]
      puN=puN-1
      spawnParticle(pl.x+4,pl.y-8,WHT,5,2,12)
      sfx(0,40,5,0,15)
    end
  end

  -- Try grab stunned enemy
  for i=1,eN do
    if eState[i]=="STUN" and not eDead[i] then
      if math.abs(pl.x+4-(eX[i]+4))<22 and math.abs(pl.y-eY[i])<14 then
        pl.state="GRAB" pl.grabIdx=i
        pl.grabT=80 pl.kneeN=0
        eState[i]="GRAB"
        pl.face=eX[i]>pl.x and 1 or -1
        break
      end
    end
  end
end

function clampPlayer()
  if scrollLocked then
    pl.x=math.max(camX,math.min(camX+SW-10,pl.x))
  else
    pl.x=math.max(camX-5,math.min(STAGES[stageIdx].len-10,pl.x))
  end
end

function hurtPlayer(dmg,fromX)
  if pl.invT>0 or pl.hurtT>0 or pl.state=="DEAD" then return end
  pl.hp=pl.hp-dmg
  pl.state="HURT" pl.hurtT=18
  pl.kbx= fromX<pl.x and 3 or -3
  pl.kby=0
  if pl.grabIdx>0 then
    eState[pl.grabIdx]="IDLE" pl.grabIdx=0
  end
  screenShake=2 shakeTimer=6
  spawnParticle(pl.x+4,pl.y-8,RED,3,2,8)
  sfx(1,20,4,0,15)
  if pl.hp<=0 then
    pl.hp=0 pl.lives=pl.lives-1
    if pl.lives>=0 then
      pl.hp=pl.maxhp pl.invT=100
      pl.state="IDLE" pl.hurtT=0
      pl.wpType=nil pl.z=0 pl.vz=0
    else
      pl.state="DEAD" STATE="GAME_OVER"
      gameOverTimer=0 continueTimer=480
      sfx(2,10,8,0,30)
    end
  end
end

-- Enemy AI update
function updateEnemy(i)
  if eDead[i] then
    eDeadT[i]=eDeadT[i]+1
    if eDeadT[i]>50 then removeEnemy(i) end
    return false
  end

  -- Hurt knockback
  if eHurt[i]>0 then
    eHurt[i]=eHurt[i]-1
    eX[i]=eX[i]+eKBX[i] eY[i]=eY[i]+eKBY[i]
    eKBX[i]=eKBX[i]*0.82 eKBY[i]=eKBY[i]*0.82
    eY[i]=math.max(GMIN,math.min(GMAX,eY[i]))
    if eHurt[i]<=0 then
      if eHP[i]<=0 then
        killEnemy(i)
      else
        eState[i]="IDLE"
      end
    end
    return false
  end

  -- Stun
  if eState[i]=="STUN" then
    eStun[i]=eStun[i]-1
    if eStun[i]<=0 then eState[i]="IDLE" end
    return false
  end
  if eState[i]=="GRAB" then return false end

  -- Attack cooldown
  if eAtkCD[i]>0 then eAtkCD[i]=eAtkCD[i]-1 end

  -- Attacking animation
  if eState[i]=="ATK" then
    eAtkT[i]=eAtkT[i]-1
    if eAtkT[i]<=0 then
      eState[i]="IDLE"
      eAtkCD[i]=40+math.random(40)
    end
    return false
  end

  -- AI
  if pl.state=="DEAD" then return false end
  local dx=pl.x-eX[i]
  local dy=pl.y-eY[i]
  local dist=math.sqrt(dx*dx+dy*dy)
  eFace[i]= dx>0 and 1 or -1

  local d=EDEFS[eType[i]]
  local atkRange= eType[i]=="brawler" and 14 or eType[i]=="boss" and 13 or 12

  if dist<atkRange and eAtkCD[i]<=0 and math.abs(dy)<12 then
    eState[i]="ATK"
    eAtkT[i]= eType[i]=="brawler" and 18 or 14
    eAtkCD[i]=40+math.random(40)
    return false
  end

  eState[i]="WALK"
  local spd=d.spd
  if eType[i]=="boss" and dist>80 then spd=spd*1.8 end
  eX[i]=eX[i]+(dx>0 and 1 or -1)*spd
  eY[i]=eY[i]+(dy>0 and 1 or -1)*math.min(math.abs(dy),spd*0.6)
  eY[i]=math.max(GMIN,math.min(GMAX,eY[i]))

  if scrollLocked then
    eX[i]=math.max(camX-10,math.min(camX+SW+10,eX[i]))
  end
  return false
end

function killEnemy(i)
  if eDead[i] then return end
  eDead[i]=true eDeadT[i]=0 eState[i]="DEAD"
  score=score+eScore[i]
  spawnParticle(eX[i]+4,eY[i]-8,ORG,6,2.5,15)
  spawnParticle(eX[i]+4,eY[i]-8,YEL,3,1.5,12)
  spawnHitNum(eX[i]+4,eY[i]-14,eScore[i],LBL)
  sfx(2,12,6,0,25)
  -- chance to drop food
  if math.random()<0.15 then
    puN=puN+1
    puX[puN]=eX[i]+4 puY[puN]=eY[i] puType[puN]="food"
  end
  -- knife drop
  if eType[i]=="knife" and math.random()<0.5 then
    gwN=gwN+1
    gwX[gwN]=eX[i]+4 gwY[gwN]=eY[i]
    gwType[gwN]="knife" gwDur[gwN]=8
  end
end

-- Combat checks
function checkCombat()
  -- Player hitbox
  local phx,phw,phy,phh=0,0,0,0
  local active=false
  local pdmg=0

  if pl.state=="PUNCH" and pl.animT>2 and pl.animT<10 then
    active=true
    local dmgArr={10,15,25}
    pdmg=dmgArr[math.min(pl.combo,3)]
    if pl.wpType then
      pdmg= pl.wpType=="pipe" and 28 or pl.wpType=="knife" and 22 or 20
    end
    phw=22 phh=18
    phx= pl.face>0 and pl.x+10 or pl.x-22
    phy= pl.y-18
  elseif pl.state=="SPECIAL" and pl.animT>3 and pl.animT<18 then
    active=true pdmg=35
    phx=pl.x-28 phw=pl.x+40-pl.x+28 phw=60
    phy=pl.y-20 phh=22
  elseif pl.state=="JKICK" then
    active=true pdmg=25
    phw=20 phh=16
    phx= pl.face>0 and pl.x+10 or pl.x-20
    phy= pl.y-pl.z-18
  end

  if active then
    for i=1,eN do
      if not eDead[i] and eHurt[i]==0 and eState[i]~="GRAB" then
        if math.abs(pl.y-eY[i])<18 then
          local ex1=eX[i] local ex2=eX[i]+10
          local ey1=eY[i]-18 local ey2=eY[i]
          if phx<ex2 and phx+phw>ex1 and phy<ey2 and phy+phh>ey1 then
            -- Apply weapon durability
            if pl.wpType then
              pl.wpDur=pl.wpDur-1
              if pl.wpDur<=0 then
                spawnParticle(pl.x+4,pl.y-10,GRY,5,3,10)
                pl.wpType=nil
                sfx(3,15,5,0,20)
              end
            end
            eHP[i]=eHP[i]-pdmg
            eHurt[i]=10
            eKBX[i]=pl.face*(pl.state=="JKICK" and 5 or 2.5)
            eKBY[i]=0
            eState[i]="HURT"
            spawnSpark(eX[i]+5,eY[i]-9)
            spawnHitNum(eX[i]+5,eY[i]-14,pdmg,YEL)
            -- Combo/stun
            local d=EDEFS[eType[i]]
            if d.armor>0 then
              eStun[i]=eStun[i]+1
              if eStun[i]>=d.armor then
                eState[i]="STUN" eStun[i]=80
              end
            else
              if pl.combo>=3 or pl.state=="JKICK" then
                eState[i]="STUN" eStun[i]=60
                eKBX[i]=pl.face*4
              end
            end
            if eHP[i]<=0 then killEnemy(i) end
            score=score+8
            screenShake=1 shakeTimer=3
            if pl.state~="SPECIAL" then break end
          end
        end
      end
    end
  end

  -- Enemy attacks hitting player
  for i=1,eN do
    if not eDead[i] and eState[i]=="ATK" and eAtkT[i]>2 and eAtkT[i]<9 then
      if math.abs(pl.y-eY[i])<18 then
        local ahx= eFace[i]>0 and eX[i]+10 or eX[i]-20
        if pl.x<ahx+20 and pl.x+10>ahx and pl.y-18<eY[i] and pl.y>eY[i]-18 then
          hurtPlayer(EDEFS[eType[i]].dmg,eX[i])
          eAtkT[i]=0
        end
      end
    end
  end
end

-- Camera
function updateCamera()
  if scrollLocked then return end
  local st=STAGES[stageIdx]
  local tx=pl.x-SW*0.35
  local maxC=st.len-SW
  camX=camX+(tx-camX)*0.1
  camX=math.max(0,math.min(math.max(0,maxC),camX))
end

-- Drawing helpers
function drawEntity(i,isPlayer)
  local ex,ey,ef,est,ez
  local isboss=false local isdead=false
  if isPlayer then
    ex=pl.x ey=pl.y ef=pl.face est=pl.state ez=pl.z
  else
    ex=eX[i] ey=eY[i] ef=eFace[i] est=eState[i] ez=0
    isboss=eIsBoss[i] isdead=eDead[i]
  end

  local sx=math.floor(ex-camX)
  local sy=math.floor(ey-ez)
  if sx<-20 or sx>SW+20 then return end

  -- Shadow
  circ(sx+5,ey+1,4,DTL)

  -- Blink on invincibility or hurt
  local skip=false
  if isPlayer then
    if pl.invT>0 and frame%4<2 then skip=true end
    if pl.hurtT>0 and frame%3==0 then skip=true end
  end
  if isdead and eDeadT[i]<30 and frame%4<2 then skip=true end
  if isdead and eDeadT[i]>=30 then return end
  if skip then return end

  if isPlayer then
    drawPlayer(sx,sy,ef,est,pl.animF)
  else
    drawEnemy(sx,sy,ef,est,eType[i],isboss)
  end
end

function drawPlayer(x,y,face,state,af)
  -- mirror
  local ox=0
  if face<0 then
    -- draw mirrored: shift x by width
    ox=10
  end
  local f=face>0 and 1 or -1

  -- hair
  rect(x+2,y-14,9,3,YEL)
  -- head
  rect(x+2,y-12,9,8,9)  -- skin=orange9
  -- eyes
  pix(x+3+(face>0 and 1 or 5),y-10,0)
  pix(x+3+(face>0 and 4 or 2),y-10,0)
  -- body
  rect(x+1,y-6,10,6,LBL)
  -- belt
  rect(x+1,y-1,10,2,ORG)
  -- arms
  if state=="PUNCH" then
    local ext=af<3 and 5 or 8
    rect(face>0 and x+11 or x-ext+1,y-5,ext,3,9)
    rect(face>0 and x-2 or x+10,y-5,3,5,9)
  elseif state=="SPECIAL" then
    rect(x+11,y-6,7,3,9)
    rect(x-7,y-6,7,3,9)
    rect(x+17,y-7,3,5,WHT)
    rect(x-8,y-7,3,5,WHT)
  else
    rect(face>0 and x+10 or x-2,y-5,3,5,9)
    rect(face>0 and x-2 or x+10,y-5,3,5,9)
  end
  -- legs
  local legA= (state=="WALK") and math.sin(frame*0.3)*3 or 0
  rect(x+2, y+1, 4, 5+legA, DBG)
  rect(x+6, y+1, 4, 5-legA, DBG)
  -- boots
  rect(x+1,  y+5+legA+1, 5, 2, ORG)
  rect(x+5,  y+5-legA+1, 5, 2, ORG)
end

function drawEnemy(x,y,face,state,et,isboss)
  local bc= et=="thug" and RED or et=="knife" and PUR or et=="brawler" and GRN or ORG2
  local big=(et=="brawler" or et=="boss")

  if big then
    -- bigger enemy
    rect(x+2,y-16,10,4,7)  -- hair/top
    rect(x+2,y-13,10,9,9)  -- head skin
    pix(x+3+(face>0 and 1 or 5),y-11,0)
    pix(x+3+(face>0 and 4 or 2),y-11,0)
    if isboss then -- scar
      line(x+2+(face>0 and 1 or 7),y-13,x+2+(face>0 and 1 or 7),y-7,RED)
    end
    rect(x,y-5,13,8,bc)
    -- arms
    if state=="ATK" then
      rect(face>0 and x+13 or x-8,y-4,9,4,9)
      rect(face>0 and x-3 or x+12,y-4,4,6,9)
    else
      rect(face>0 and x+12 or x-3,y-4,4,6,9)
      rect(face>0 and x-3 or x+12,y-4,4,6,9)
    end
    local lg=(state=="WALK") and math.sin(frame*0.25)*2 or 0
    rect(x+1,y+2,5,6+lg,DBG)
    rect(x+6,y+2,5,6-lg,DBG)
    rect(x,  y+7+lg+1,6,2,ORG)
    rect(x+5,y+7-lg+1,6,2,ORG)
    -- stun stars
    if state=="STUN" then
      for si=0,2 do
        local ang=frame*0.15+si*2.1
        local sx2=math.floor(x+6+math.sin(ang)*8)
        local sy2=math.floor(y-18+math.cos(ang)*4)
        rect(sx2,sy2,2,2,YEL)
      end
    end
  else
    -- normal enemy
    rect(x+2,y-14,8,3, et=="knife" and GRY or DTL)
    rect(x+2,y-12,8,7,9)
    pix(x+3+(face>0 and 1 or 4),y-10,0)
    pix(x+3+(face>0 and 3 or 2),y-10,0)
    rect(x+1,y-5,9,6,bc)
    -- weapon visual for knife
    if et=="knife" and state=="ATK" then
      rect(face>0 and x+11 or x-7,y-4,7,2,GRY)
    end
    if state=="ATK" then
      rect(face>0 and x+10 or x-7,y-4,8,3,9)
    else
      rect(face>0 and x+9 or x-2,y-4,3,5,9)
      rect(face>0 and x-2 or x+9,y-4,3,5,9)
    end
    local lg=(state=="WALK") and math.sin(frame*0.25)*2 or 0
    rect(x+2,y+1,4,5+lg,DBG)
    rect(x+5,y+1,4,5-lg,DBG)
    rect(x+1,y+5+lg+1,5,2,ORG)
    rect(x+5,y+5-lg+1,5,2,ORG)
    if state=="STUN" then
      for si=0,2 do
        local ang=frame*0.15+si*2.1
        local sx2=math.floor(x+5+math.sin(ang)*7)
        local sy2=math.floor(y-16+math.cos(ang)*3)
        rect(sx2,sy2,2,2,YEL)
      end
    end
  end
end

-- Background drawing
function drawBG()
  local st=STAGES[stageIdx]
  cls(0)

  if stageIdx==1 then drawAlley()
  elseif stageIdx==2 then drawWarehouse()
  else drawRooftop() end

  -- Ground
  rect(0,GMIN-5,SW,SH-GMIN+5, stageIdx==3 and DBG or 7)
  -- Ground lines
  for gy=GMIN,GMAX+4,8 do
    line(0,gy,SW,gy,0)
  end
end

function drawAlley()
  -- sky
  rect(0,0,SW,GMIN-5,0)
  -- far buildings
  for i=0,7 do
    local bx=math.floor(i*34-(camX*0.15)%34)
    local bh=30+(i*13%22)
    rect(bx,GMIN-5-bh,30,bh,PUR)
    -- windows
    for wy=4,bh-6,7 do
      for wx=3,24,9 do
        if math.sin(i*3+wx+wy)>0.3 then
          pix(bx+wx,GMIN-5-bh+wy,YEL)
          pix(bx+wx+1,GMIN-5-bh+wy,YEL)
        end
      end
    end
  end
  -- neon signs
  local ncs={RED,LBL,ORG,MAG}
  for i=0,3 do
    local nx=math.floor(i*62-(camX*0.4)%62)
    if nx>-20 and nx<SW+20 then
      rect(nx,GMIN-28-(i%2)*12,18,5,ncs[i%4+1])
    end
  end
  -- dumpsters
  for i=0,3 do
    local dx=math.floor(i*68+20-camX%68)
    if dx>-20 and dx<SW+20 then
      rect(dx,GMIN-12,22,12,GRN)
      rect(dx,GMIN-12,22,3,DTL)
    end
  end
end

function drawWarehouse()
  rect(0,0,SW,GMIN-5,DBG)
  -- shelves
  for i=0,5 do
    local sx=math.floor(i*48-(camX*0.3)%48)
    if sx>-20 and sx<SW+20 then
      rect(sx,20,18,GMIN-25,ORG)
      for sy=28,GMIN-20,14 do
        rect(sx-3,sy,24,2,YEL)
      end
    end
  end
  -- overhead lights
  for i=0,4 do
    local lx=math.floor(i*54+10-(camX*0.5)%54)
    if lx>-10 and lx<SW+10 then
      rect(lx,8,12,3,YEL)
      -- light cone (simple triangle)
      for j=1,20 do
        line(lx,8+j,lx+12,8+j,0)
      end
    end
  end
  -- crates
  for i=0,3 do
    local cx=math.floor(i*70+30-camX%70)
    if cx>-20 and cx<SW+20 then
      rect(cx,GMIN-14,14,14,ORG)
      rectb(cx,GMIN-14,14,14,YEL)
    end
  end
end

function drawRooftop()
  rect(0,0,SW,GMIN-5,0)
  -- stars
  for i=0,30 do
    local sx=(i*67+10)%SW
    local sy=(i*43+5)%(GMIN-20)
    if math.sin(frame*0.05+i)>0.7 then
      pix(sx,sy,WHT)
    end
  end
  -- distant skyline
  for i=0,10 do
    local bx=math.floor(i*26-(camX*0.1)%26)
    local bh=20+(i*17%30)
    rect(bx,GMIN-5-bh,22,bh,PUR)
    -- lit windows tiny
    for wy=3,bh-3,5 do
      for wx=2,18,6 do
        if math.sin(i*5+wx+wy)>0.2 then
          pix(bx+wx,GMIN-5-bh+wy,MAG)
        end
      end
    end
  end
  -- horizon glow
  rect(0,GMIN-10,SW,6,PUR)
  -- AC units
  for i=0,2 do
    local ax=math.floor(i*90+40-(camX*0.8)%90)
    if ax>-20 and ax<SW+20 then
      rect(ax,GMIN-18,16,13,DBG)
      rect(ax+2,GMIN-20,12,4,GRY)
    end
  end
end

-- HUD
function drawHUD()
  -- health bar bg
  rect(2,2,52,5,7)
  local hp=math.max(0,pl.hp/pl.maxhp)
  local hc= hp>0.5 and GRN or hp>0.25 and YEL or RED
  rect(2,2,math.floor(52*hp),5,hc)
  rectb(2,2,52,5,WHT)
  -- lives
  for i=0,pl.lives-1 do
    rect(2+i*7,9,5,5,LBL)
    rect(3+i*7,9,3,2,YEL)
  end
  -- score
  local sc=tostring(score)
  print("SC:"..sc,SW-3-#sc*5,2,YEL,false,1,true)
  -- stage name
  print(STAGES[stageIdx].name,SW/2-#STAGES[stageIdx].name*2,2,GRY,false,1,true)
  -- weapon indicator
  if pl.wpType then
    print(pl.wpType.." "..pl.wpDur,2,16,WHT,false,1,true)
  end
  -- boss health bar
  for i=1,eN do
    if eIsBoss[i] and not eDead[i] then
      local bx=SW/2-40
      rect(bx,SH-12,80,5,7)
      local bp=math.max(0,eHP[i]/eMaxHP[i])
      rect(bx,SH-12,math.floor(80*bp),5,MAG)
      rectb(bx,SH-12,80,5,WHT)
      print(BOSS_DATA[eBossIdx[i]].name,bx,SH-19,WHT,false,1,true)
      break
    end
  end
  -- GO arrow
  if not scrollLocked and eN==0 and waveIdx<#STAGES[stageIdx].waves then
    if frame%40<25 then
      print("GO>>>",SW-28,SH/2,YEL,false,1,true)
    end
  end
end

-- Story screen
function drawStory()
  cls(0)
  -- title bar
  local ph=storyPhase
  local title= ph=="INTRO" and "STREETS OF VENGEANCE"
             or ph=="PRE" and "STAGE "..stageIdx..": "..STAGES[stageIdx].name
             or ph=="POST" and "STAGE "..stageIdx.." CLEAR"
             or "EPILOGUE"
  print(title,SW/2-#title*2,5,RED,false,1,true)
  line(20,12,SW-20,12,PUR)

  -- current lines
  local y=20
  for li=1,#storyLines do
    local txt
    if li<storyLineIdx then
      txt=storyLines[li]
    elseif li==storyLineIdx then
      txt=string.sub(storyLines[li],1,storyCharIdx)
    else
      break
    end
    -- simple word wrap at ~36 chars
    y=printWrap(txt,4,y,WHT)
    y=y+3
    if y>SH-20 then break end
  end

  -- prompt blink
  if storyLineIdx>#storyLines then
    if frame%30<15 then
      print("A:NEXT",SW/2-15,SH-8,GRY,false,1,true)
    end
  elseif storyCharIdx>=#(storyLines[storyLineIdx] or "") then
    if frame%30<15 then
      print("A:NEXT",SW/2-15,SH-8,GRY,false,1,true)
    end
  end
end

function printWrap(txt,x,y,c)
  if not txt or txt=="" then return y end
  local maxW=SW-x-4
  local line="" local lw=0
  for word in (txt.." "):gmatch("([^ ]*) ") do
    local wlen=#word+1
    if lw+wlen>30 and lw>0 then
      print(line,x,y,c,false,1,true)
      y=y+7 line="" lw=0
    end
    line=line..word.." " lw=lw+wlen
  end
  if lw>0 then print(line,x,y,c,false,1,true); y=y+7 end
  return y
end

-- Boss intro screen
function drawBossIntro()
  drawBG()
  -- darken
  for dy=0,SH,2 do
    line(0,dy,SW,dy,0)
  end
  if not bossData then return end
  local t=math.min(bossIntroTimer/20,1)
  -- warning flash
  if bossIntroTimer<40 and math.floor(bossIntroTimer/4)%2==0 then
    print("!! WARNING !!",SW/2-26,SH/2-30,RED,false,1,true)
  end
  -- boss name
  print(bossData.name,SW/2-#bossData.name*3,SH/2-12,MAG,false,1,true)
  -- title
  print(bossData.title,SW/2-#bossData.title*2,SH/2,YEL,false,1,true)
  line(30,SH/2+8,SW-30,SH/2+8,PUR)
  -- quote
  printWrap('"'..bossData.quote..'"',10,SH/2+14,WHT)
  if bossIntroTimer>25 and frame%30<15 then
    print("A:FIGHT",SW/2-14,SH-8,GRY,false,1,true)
  end
end

-- Title screen
function drawTitle()
  cls(0)
  -- skyline
  for i=0,14 do
    local bx=i*18
    local bh=22+(i*13%30)
    rect(bx,SH-50-bh,16,bh+50,PUR)
  end
  rect(0,SH-50,SW,50,DBG)

  -- title
  print("DRAGON",SW/2-18,28,RED,false,1,true)
  print("FURY",SW/2-12,36,ORG,false,1,true)
  print("STREETS OF VENGEANCE",SW/2-40,46,LBL,false,1,true)

  -- player preview (small)
  local px=SW/2-5 local py=70
  rect(px+2,py-10,6,3,YEL)
  rect(px+2,py-8,6,6,9)
  rect(px+1,py-3,8,5,LBL)
  rect(px+2,py+1,3,5,DBG)
  rect(px+5,py+1,3,5,DBG)

  -- press start blink
  if frame%60<30 then
    print("PRESS A START",SW/2-26,90,WHT,false,1,true)
  end
  print("A=PUNCH B=JUMP A+B=SPECIAL",3,SH-8,GRY,false,1,true)
end

-- Stage intro
function drawStageIntro()
  cls(0)
  local a= stageIntroTimer>120 and (150-stageIntroTimer)/30
         or stageIntroTimer<30 and stageIntroTimer/30 or 1
  a=math.max(0,math.min(1,a))
  local n="STAGE "..stageIdx
  local nm=STAGES[stageIdx].name
  print(n,SW/2-#n*3,SH/2-10,RED,false,1,true)
  print(nm,SW/2-#nm*3,SH/2+2,WHT,false,1,true)
end

-- Game over screen
function drawGameOver()
  -- semi-transparent overlay (alternating lines)
  for dy=0,SH,2 do line(0,dy,SW,dy,0) end
  print("GAME OVER",SW/2-18,SH/2-16,RED,false,1,true)
  print("CONTINUE?",SW/2-18,SH/2-4,WHT,false,1,true)
  local secs=math.max(0,math.ceil(continueTimer/60))
  print(tostring(secs),SW/2-3,SH/2+8,YEL,false,1,true)
  if frame%30<15 then
    print("A:YES",SW/2-10,SH/2+20,GRY,false,1,true)
  end
end

-- Victory screen
function drawVictory()
  cls(0)
  -- sunrise
  rect(0,0,SW,30,PUR)
  for i=1,8 do
    rect(0,i*3,SW,3,i<4 and RED or i<7 and ORG or YEL)
  end
  print("VICTORY!",SW/2-16,35,YEL,false,1,true)
  print("STREETS OF VENGEANCE",SW/2-40,45,LBL,false,1,true)
  local sc="SCORE:"..score
  print(sc,SW/2-#sc*3,55,LBL,false,1,true)
  print("THE DRAGON FIST ENDURES",SW/2-46,65,MAG,false,1,true)
  -- player
  local px=SW/2-5 local py=90
  rect(px+2,py-10,6,3,YEL)
  rect(px+2,py-8,6,6,9)
  rect(px+1,py-3,8,5,LBL)
  rect(px+2,py+1,3,5,DBG)
  rect(px+5,py+1,3,5,DBG)
  if frame%60<30 then
    print("A:MENU",SW/2-12,SH-8,WHT,false,1,true)
  end
end

-- Mid-stage dialogue
function drawMidDlg()
  if not midDlg then return end
  -- box at bottom
  rect(4,SH-30,SW-8,26,0)
  rectb(4,SH-30,SW-8,26,MAG)
  printWrap(midDlg,7,SH-27,WHT)
end

-- Main TIC function
function TIC()
  frame=frame+1

  -- Update
  if STATE=="TITLE" then
    titleBlink=titleBlink+1
    if pressSt() or pressA() then startGame() end

  elseif STATE=="STORY" then
    storyCharTimer=storyCharTimer+1
    if storyCharTimer>=2 then
      storyCharTimer=0
      if storyLineIdx<=#storyLines and storyCharIdx<#(storyLines[storyLineIdx] or "") then
        storyCharIdx=storyCharIdx+1
      end
    end
    if pressA() or pressSt() then
      if storyLineIdx<=#storyLines and storyCharIdx<#(storyLines[storyLineIdx] or "") then
        storyCharIdx=#storyLines[storyLineIdx]
      else
        storyLineIdx=storyLineIdx+1
        storyCharIdx=0 storyCharTimer=0
        if storyLineIdx>#storyLines then
          if storyCb then storyCb() end
        end
      end
    end

  elseif STATE=="BOSS_INTRO" then
    bossIntroTimer=bossIntroTimer+1
    if (pressA() or pressSt()) and bossIntroTimer>20 then
      STATE="PLAYING"
      checkWave()
    end

  elseif STATE=="STAGE_INTRO" then
    stageIntroTimer=stageIntroTimer-1
    if stageIntroTimer<=0 then STATE="PLAYING" end

  elseif STATE=="PLAYING" then
    updatePlayer()
    local i=1
    while i<=eN do
      local wasN=eN
      updateEnemy(i)
      if eN<wasN then
        -- enemy was removed, don't increment
      else
        i=i+1
      end
    end
    checkCombat()
    checkWave()

    if midDlg then
      midDlgTimer=midDlgTimer-1
      if midDlgTimer<=0 or pressA() then midDlg=nil end
    end

    -- Wave clear check
    if scrollLocked then
      local alive=0
      for j=1,eN do if not eDead[j] then alive=alive+1 end end
      if alive==0 then
        waveClearTimer=waveClearTimer+1
        if waveClearTimer>25 then
          scrollLocked=false waveClearTimer=0
          local st=STAGES[stageIdx]
          if waveIdx>=#st.waves then
            -- stage complete
            sfx(4,30,8,0,60)
            if stageIdx<#STAGES then
              local ci=stageIdx
              startStory(POST_STAGE[ci],"POST",function()
                stageIdx=ci+1
                initStage(stageIdx)
                startStory(PRE_STAGE[stageIdx],"PRE",function()
                  STATE="STAGE_INTRO" stageIntroTimer=150
                end)
              end)
            else
              startStory(POST_STAGE[stageIdx],"POST",function()
                startStory(STORY_VICTORY,"VICTORY_STORY",function()
                  STATE="VICTORY"
                end)
              end)
            end
          end
        end
      else
        waveClearTimer=0
      end
    end

    updateCamera()

    -- Update particles
    local pi=1
    while pi<=pN do
      pX[pi]=pX[pi]+pVX[pi]
      pY[pi]=pY[pi]+pVY[pi]
      pVY[pi]=pVY[pi]+0.1
      pLife[pi]=pLife[pi]-1
      if pLife[pi]<=0 then
        pX[pi]=pX[pN] pY[pi]=pY[pN]
        pVX[pi]=pVX[pN] pVY[pi]=pVY[pN]
        pLife[pi]=pLife[pN] pMaxL[pi]=pMaxL[pN] pCol[pi]=pCol[pN]
        pN=pN-1
      else
        pi=pi+1
      end
    end

    -- Update hit numbers
    local hi=1
    while hi<=hnN do
      hnY[hi]=hnY[hi]-0.8
      hnL[hi]=hnL[hi]-1
      if hnL[hi]<=0 then
        hnX[hi]=hnX[hnN] hnY[hi]=hnY[hnN]
        hnV[hi]=hnV[hnN] hnL[hi]=hnL[hnN] hnC[hi]=hnC[hnN]
        hnN=hnN-1
      else
        hi=hi+1
      end
    end

    -- Screen shake
    if shakeTimer>0 then shakeTimer=shakeTimer-1
    else screenShake=0 end

  elseif STATE=="GAME_OVER" then
    gameOverTimer=gameOverTimer+1
    continueTimer=continueTimer-1
    if (pressA() or pressSt()) and gameOverTimer>30 then
      pl.lives=2 pl.hp=pl.maxhp
      pl.state="IDLE" pl.invT=80
      pl.hurtT=0 pl.z=0 pl.vz=0
      pl.wpType=nil pl.grabIdx=0
      pl.kbx=0 pl.kby=0
      STATE="PLAYING"
    end
    if continueTimer<=0 then STATE="TITLE" end

  elseif STATE=="VICTORY" then
    if pressA() or pressSt() then STATE="TITLE" end
  end

  -- Draw
  if STATE=="TITLE" then
    drawTitle()
  elseif STATE=="STORY" then
    drawStory()
  elseif STATE=="BOSS_INTRO" then
    drawBossIntro()
  elseif STATE=="STAGE_INTRO" then
    drawStageIntro()
  elseif STATE=="PLAYING" or STATE=="GAME_OVER" then
    -- screen shake offset (draw slightly shifted)
    local shx=0 local shy=0
    if screenShake>0 and shakeTimer>0 then
      shx=math.random(-screenShake,screenShake)
      shy=math.random(-1,1)
    end

    drawBG()

    -- Collect entities sorted by Y for depth
    local drawOrder={}
    table.insert(drawOrder,{y=pl.y,isP=true})
    for i=1,eN do
      table.insert(drawOrder,{y=eY[i],idx=i,isP=false})
    end
    table.sort(drawOrder,function(a,b) return a.y<b.y end)

    for _,d in ipairs(drawOrder) do
      if d.isP then
        if pl.state~="DEAD" then drawEntity(0,true) end
      else
        drawEntity(d.idx,false)
      end
    end

    -- Ground weapons
    for i=1,gwN do
      local gsx=math.floor(gwX[i]-camX)
      if gsx>-10 and gsx<SW+10 then
        local gc= gwType[i]=="pipe" and GRY or LBL
        rect(gsx,gwY[i]-2,12,2,gc)
        if frame%60<10 then pix(gsx+5,gwY[i]-3,WHT) end
      end
    end

    -- Pickups
    for i=1,puN do
      local psx=math.floor(puX[i]-camX)
      if psx>-10 and psx<SW+10 then
        local bob=math.floor(math.sin(frame*0.1)*2)
        if puType[i]=="food" then
          rect(psx,puY[i]-5+bob,6,4,ORG2)
          rect(psx+2,puY[i]-4+bob,4,3,YEL)
        elseif puType[i]=="pizza" then
          rect(psx,puY[i]-5+bob,8,6,YEL)
          pix(psx+2,puY[i]-3+bob,RED)
          pix(psx+5,puY[i]-2+bob,RED)
        elseif puType[i]=="life" then
          -- star shape
          rect(psx+2,puY[i]-6+bob,3,7,YEL)
          rect(psx,puY[i]-4+bob,7,3,YEL)
        end
      end
    end

    -- Particles
    for i=1,pN do
      local psx=math.floor(pX[i]-camX)
      local psy=math.floor(pY[i])
      if psx>=0 and psx<SW and psy>=0 and psy<SH then
        pix(psx,psy,pCol[i])
      end
    end

    -- Hit numbers
    for i=1,hnN do
      local hsx=math.floor(hnX[i]-camX)
      if hsx>0 and hsx<SW then
        print(tostring(hnV[i]),hsx,math.floor(hnY[i]),hnC[i],false,1,true)
      end
    end

    drawHUD()

    if midDlg then drawMidDlg() end
    if STATE=="GAME_OVER" then drawGameOver() end

  elseif STATE=="VICTORY" then
    drawVictory()
  end

  -- Store prev input
  prevA=btn(4) prevB=btn(5) prevSt=btn(6) or btn(4)
end
