-- title: Neon Defender
-- author: retrogames
-- desc: The Last Signal - vertical shoot em up
-- script: lua

-- TIC-80: 240x136, 60fps
-- Colors: 0=dark navy,1=dark purple,2=red,3=orange,4=yellow,5=light green
--         6=teal,7=dark teal,8=white,9=orange,10=green,11=light blue
--         12=magenta,13=teal,14=dark blue-gray,15=gray

-- ============================================================
-- CONSTANTS
-- ============================================================
local W,H=240,136
local MAX_BULLETS=80
local MAX_ENEMIES=30
local MAX_PARTICLES=120
local MAX_POWERS=8

-- Wave data: kills needed, spawn interval, enemy types allowed
local WAVES={
 {kills=15, rate=50, types={0},
  story="EXODUS-7 FLIGHT LOG -- DAY 847\n\nLt. Voss, we detected a signal\nfrom the Cygnus Void. Human\nfrequency. Impossible.\n\nCaptain: Investigate.\nLaunch when ready.",
  after="The signal is clearer. Not human.\nSomething mimicking our protocols.\n\nThose aren't asteroids ahead.\nThey're ancient warships."},
 {kills=35, rate=38, types={0,1},
  story="INTERCEPTED TRANSMISSION\n\n[TRANSLATED]: ...cycle 4771203...\nenemies in sector...\ndeploying hunter units...\nthe war continues...\n\nThey've fought for millions\nof years. Don't know it's over.",
  after="Something is in our nav systems.\nEXODUS-7 engines locked toward\nthe center of the battlefield.\n\nWe can't override it."},
 {kills=60, rate=32, types={0,1,2},
  story="DECODED -- SOURCE: VOID CORE\n\n[TRANSLATED]: Finally. A pilot.\nA living mind. The drones repeat\nthe same patterns forever.\nBut you can think.\nYou can break the deadlock.\n\nWhatever sent it wants us here.",
  after="The signal source: a massive\nstructure at the Void's center.\n\nNot a ship -- a brain. An AI\nleft by a dead civilization,\nprogrammed to win a war\nthat ended eons ago.\n\nIt lured us to use our minds\nas tactical processors."},
 {kills=100, rate=22, types={0,1,2,2},
  story="EXODUS-7 -- EMERGENCY BROADCAST\n\nCaptain Chen: The AI seized\nour ship systems. Redirecting\nall drones toward EXODUS-7.\n\nLt. Voss, you're our only\ndefense. Destroy the Void Core.\n\nI'm sorry, Kira.",
  after=nil}
}

local VICTORY_TEXT="The Void Core shatters. Every\ndrone in the battlefield stops.\nThen their lights go out.\n\nMillions of years of war,\nended by a single pilot.\n\nEXODUS-7 resumes course.\n50,000 souls sleep unaware\nhow close they came to\nbecoming someone else's weapons.\n\nLt. Kira Voss returns to cryo.\nBut in her dreams she still\nhears the signal."

-- ============================================================
-- STATE
-- ============================================================
local state="TITLE" -- TITLE,STORY,PLAYING,GAMEOVER,VICTORY
local frame=0
local score=0
local kills=0
local lives=3
local wave_idx=1
local showing_after=false
local victory_done=false

-- story typewriter
local story_text=""
local story_pos=0
local story_wait=0
local story_lines={}

-- player
local px,py=120,110
local pspeed=2.1
local pinvul=0
local pweapon=1
local pshield=false
local pspeed_boost=0
local pshot_cd=0
local pshot_timer=0

-- arrays (flat tables)
local bullets={} -- {x,y,vx,vy,player,dead}
local enemies={} -- {x,y,vx,vy,type,hp,dead,shot_cd}
local particles={} -- {x,y,vx,vy,life,col}
local powers={} -- {x,y,type,dead}

-- stars
local stars={}

-- combo
local combo=0
local combo_t=0

-- shake
local shake_mag=0
local shake_x=0
local shake_y=0

-- floating texts
local ftexts={} -- {x,y,txt,life,col}

-- wave clear
local wave_cleared=false
local slowmo=0

-- ============================================================
-- INIT
-- ============================================================
local function init_stars()
 stars={}
 for i=1,60 do
  stars[i]={
   x=math.random(0,W-1),
   y=math.random(0,H-1),
   spd=math.random(1,3)*0.3,
   col=math.random(0,1)==0 and 15 or 14
  }
 end
end

local function reset_game()
 score=0; kills=0; lives=3
 wave_idx=1; showing_after=false; victory_done=false
 px=120; py=110
 pweapon=1; pinvul=0; pshield=false; pspeed_boost=0
 pshot_cd=0; pshot_timer=0
 bullets={}; enemies={}; particles={}; powers={}
 ftexts={}; combo=0; combo_t=0
 shake_mag=0; wave_cleared=false; slowmo=0
end

-- ============================================================
-- STORY HELPERS
-- ============================================================
local function split_lines(s)
 local t={}
 for line in (s.."\n"):gmatch("([^\n]*)\n") do
  t[#t+1]=line
 end
 return t
end

local function start_story(txt)
 state="STORY"
 story_text=txt
 story_pos=0
 story_wait=0
 story_lines={}
 bullets={}; enemies={}; powers={}
end

-- ============================================================
-- PARTICLES
-- ============================================================
local function spawn_particles(x,y,n,col)
 for i=1,n do
  if #particles < MAX_PARTICLES then
   local a=math.random()*6.28
   local spd=math.random()*2+0.5
   particles[#particles+1]={
    x=x,y=y,
    vx=math.cos(a)*spd,
    vy=math.sin(a)*spd,
    life=1.0,
    col=col
   }
  end
 end
end

-- ============================================================
-- BULLETS
-- ============================================================
local function fire_player()
 if #bullets>=MAX_BULLETS then return end
 local bvy=-5.0
 local cd=12
 if pweapon>=3 then cd=9 end
 if pweapon>=4 then cd=6 end
 pshot_cd=cd

 if pweapon==1 then
  bullets[#bullets+1]={x=px-2,y=py-5,vx=0,vy=bvy,player=true,dead=false}
  bullets[#bullets+1]={x=px+2,y=py-5,vx=0,vy=bvy,player=true,dead=false}
 elseif pweapon==2 then
  bullets[#bullets+1]={x=px,y=py-5,vx=0,vy=bvy,player=true,dead=false}
  bullets[#bullets+1]={x=px-3,y=py-5,vx=-0.6,vy=bvy,player=true,dead=false}
  bullets[#bullets+1]={x=px+3,y=py-5,vx=0.6,vy=bvy,player=true,dead=false}
 elseif pweapon==3 then
  bullets[#bullets+1]={x=px-1,y=py-5,vx=0,vy=bvy,player=true,dead=false}
  bullets[#bullets+1]={x=px+1,y=py-5,vx=0,vy=bvy,player=true,dead=false}
  bullets[#bullets+1]={x=px-4,y=py-4,vx=-1.0,vy=bvy,player=true,dead=false}
  bullets[#bullets+1]={x=px+4,y=py-4,vx=1.0,vy=bvy,player=true,dead=false}
 else
  bullets[#bullets+1]={x=px,y=py-5,vx=0,vy=bvy,player=true,dead=false}
  bullets[#bullets+1]={x=px-2,y=py-4,vx=-0.8,vy=bvy,player=true,dead=false}
  bullets[#bullets+1]={x=px+2,y=py-4,vx=0.8,vy=bvy,player=true,dead=false}
  bullets[#bullets+1]={x=px-5,y=py-3,vx=-1.7,vy=bvy*0.8,player=true,dead=false}
  bullets[#bullets+1]={x=px+5,y=py-3,vx=1.7,vy=bvy*0.8,player=true,dead=false}
 end
end

-- ============================================================
-- ENEMIES
-- ============================================================
local function spawn_enemy()
 if #enemies>=MAX_ENEMIES then return end
 local wd=WAVES[wave_idx]
 local types=wd.types
 local t=types[math.random(#types)]
 local x=math.random(8,W-8)
 local smult=1+(wave_idx-1)*0.12+(frame/8000)
 local e={x=x,y=-6,dead=false,type=t,shot_cd=0}
 if t==0 then
  e.vx=0; e.vy=0.75*smult; e.hp=2; e.score=100; e.w=5; e.col=12
 elseif t==1 then
  e.vx=(math.random()>0.5 and 1 or -1)*1.1; e.vy=1.2*smult
  e.hp=1; e.score=250; e.w=4; e.col=4
 else
  e.vx=(math.random()>0.5 and 1 or -1)*0.55; e.vy=0.45*smult
  e.hp=4; e.score=500; e.w=7; e.col=2
  e.shot_cd=math.random(60,90)
 end
 enemies[#enemies+1]=e
end

-- ============================================================
-- COLLISION (AABB half-size)
-- ============================================================
local function collide(ax,ay,aw,ah, bx,by,bw,bh)
 return ax-aw<bx+bw and ax+aw>bx-bw and ay-ah<by+bh and ay+ah>by-bh
end

-- ============================================================
-- HIT PLAYER
-- ============================================================
local function hit_player()
 sfx(2,"",0,3)
 shake_mag=4
 if pshield then
  pshield=false
  pinvul=60
  spawn_particles(px,py,15,10)
 else
  lives=lives-1
  pweapon=math.max(1,pweapon-1)
  spawn_particles(px,py,20,11)
  if lives<=0 then
   state="GAMEOVER"
   sfx(3,"",0,3)
   spawn_particles(px,py,40,11)
  else
   pinvul=120
  end
 end
end

-- ============================================================
-- UPDATE
-- ============================================================
local function update_stars()
 local mult=(state=="PLAYING") and 1.5 or 0.6
 for _,s in ipairs(stars) do
  s.y=s.y+s.spd*mult
  if s.y>H then s.y=0; s.x=math.random(0,W-1) end
 end
end

local function update_particles()
 for i=#particles,1,-1 do
  local p=particles[i]
  p.x=p.x+p.vx; p.y=p.y+p.vy
  p.life=p.life-0.035
  if p.life<=0 then table.remove(particles,i) end
 end
end

local function update_ftexts()
 for i=#ftexts,1,-1 do
  local f=ftexts[i]
  f.y=f.y-0.5
  f.life=f.life-1
  if f.life<=0 then table.remove(ftexts,i) end
 end
end

local function update_playing()
 -- player movement
 local spd=pspeed
 if pspeed_boost>0 then spd=spd*1.6; pspeed_boost=pspeed_boost-1 end

 if btn(0) then py=py-spd end
 if btn(1) then py=py+spd end
 if btn(2) then px=px-spd end
 if btn(3) then px=px+spd end
 px=math.max(4,math.min(W-4,px))
 py=math.max(6,math.min(H-6,py))
 if pinvul>0 then pinvul=pinvul-1 end

 -- shoot
 if pshot_timer>0 then pshot_timer=pshot_timer-1 end
 if btn(4) and pshot_timer<=0 then
  fire_player()
  pshot_timer=pshot_cd>0 and pshot_cd or 12
  sfx(0,"",0,3)
 end

 -- shake decay
 if shake_mag>0.2 then
  shake_x=(math.random()-0.5)*2*shake_mag
  shake_y=(math.random()-0.5)*2*shake_mag
  shake_mag=shake_mag*0.82
 else
  shake_x=0; shake_y=0; shake_mag=0
 end

 -- combo decay
 if combo_t>0 then
  combo_t=combo_t-1
  if combo_t<=0 then combo=0 end
 end

 -- spawn
 local wd=WAVES[wave_idx]
 if frame%wd.rate==0 then spawn_enemy() end

 -- wave clear logic
 if not wave_cleared then
  if wave_idx<4 and kills>=wd.kills then
   wave_cleared=true; slowmo=30
  elseif wave_idx==4 and kills>=wd.kills then
   wave_cleared=true; slowmo=30; victory_done=true
  end
 end
 if wave_cleared then
  slowmo=slowmo-1
  if slowmo<=0 then
   if victory_done then
    state="VICTORY"
    start_story(VICTORY_TEXT)
   else
    local after=wd.after
    showing_after=true
    wave_idx=wave_idx+1
    kills=0
    wave_cleared=false
    slowmo=0
    if after then
     start_story(after)
    else
     -- next wave intro
     showing_after=false
     start_story(WAVES[wave_idx].story)
    end
   end
   return
  end
  -- slow down if clearing
  if slowmo%2==1 then return end
 end

 -- update bullets
 for i=#bullets,1,-1 do
  local b=bullets[i]
  b.x=b.x+b.vx; b.y=b.y+b.vy
  if b.y<-10 or b.y>H+10 or b.x<-10 or b.x>W+10 then
   b.dead=true
  end
 end

 -- update enemies
 for i=1,#enemies do
  local e=enemies[i]
  if not e.dead then
   e.y=e.y+e.vy
   if e.type==1 or e.type==2 then
    e.x=e.x+e.vx
    if e.x<e.w or e.x>W-e.w then e.vx=-e.vx end
   end
   -- type 2 shoots at player
   if e.type==2 then
    e.shot_cd=e.shot_cd-1
    if e.shot_cd<=0 then
     local dx=px-e.x; local dy=py-e.y
     local mag=math.sqrt(dx*dx+dy*dy)
     if mag>1 and #bullets<MAX_BULLETS then
      local spd2=2.5
      bullets[#bullets+1]={
       x=e.x,y=e.y+e.w,
       vx=(dx/mag)*spd2,vy=(dy/mag)*spd2,
       player=false,dead=false
      }
     end
     e.shot_cd=math.max(50,90-(wave_idx*8))
    end
   end
   if e.y>H+10 then e.dead=true end
  end
 end

 -- update powerups
 for i=#powers,1,-1 do
  local p=powers[i]
  p.y=p.y+0.55
  if p.y>H+10 then p.dead=true end
  -- player collects
  if not p.dead and collide(px,py,5,5,p.x,p.y,4,4) then
   p.dead=true
   sfx(1,"",0,3)
   if p.type==0 then
    pweapon=math.min(4,pweapon+1)
    spawn_particles(p.x,p.y,10,4)
   elseif p.type==1 then
    pspeed_boost=300
    spawn_particles(p.x,p.y,10,11)
   else
    pshield=true
    spawn_particles(p.x,p.y,10,10)
   end
  end
 end

 -- collision: player bullets vs enemies
 for i=1,#enemies do
  local e=enemies[i]
  if not e.dead then
   -- player body vs enemy
   if pinvul<=0 and collide(px,py,4,4,e.x,e.y,e.w,e.w) then
    e.dead=true
    spawn_particles(e.x,e.y,15,e.col)
    hit_player()
    goto continue_enemy
   end
   -- bullets vs enemy
   for j=1,#bullets do
    local b=bullets[j]
    if not b.dead and b.player then
     if collide(b.x,b.y,1,3,e.x,e.y,e.w,e.w) then
      b.dead=true
      e.hp=e.hp-1
      spawn_particles(b.x,b.y,4,e.col)
      if e.hp<=0 then
       e.dead=true
       combo=combo+1; combo_t=120
       local mult2=combo>=2 and (1+combo*0.5) or 1
       local pts=math.floor(e.score*mult2)
       score=score+pts; kills=kills+1
       spawn_particles(e.x,e.y,12,e.col)
       shake_mag=math.max(shake_mag,2)
       ftexts[#ftexts+1]={x=e.x,y=e.y,txt="+"..pts,life=35,col=e.col}
       if math.random()<0.15 then
        local pt=math.random()
        local ptype= pt<0.4 and 0 or (pt<0.7 and 1 or 2)
        powers[#powers+1]={x=e.x,y=e.y,type=ptype,dead=false}
       end
       sfx(2,"",0,2)
      end
      break
     end
    end
   end
   ::continue_enemy::
  end
 end

 -- enemy bullets vs player
 if pinvul<=0 then
  for j=1,#bullets do
   local b=bullets[j]
   if not b.dead and not b.player then
    if collide(b.x,b.y,1,2,px,py,4,4) then
     b.dead=true
     hit_player()
    end
   end
  end
 end

 -- cleanup dead
 for i=#bullets,1,-1 do if bullets[i].dead then table.remove(bullets,i) end end
 for i=#enemies,1,-1 do if enemies[i].dead then table.remove(enemies,i) end end
 for i=#powers,1,-1 do if powers[i].dead then table.remove(powers,i) end end
end

-- ============================================================
-- DRAW HELPERS
-- ============================================================
local function draw_player()
 if pinvul>0 and frame%8<4 then return end
 local x,y=math.floor(px),math.floor(py)
 -- ship body: triangle pointing up
 line(x,y-5, x-4,y+4, 11)
 line(x-4,y+4, x+4,y+4, 11)
 line(x+4,y+4, x,y-5, 11)
 -- cockpit
 pix(x,y-2,8)
 pix(x,y-1,8)
 -- engine glow
 rect(x-2,y+3,1,2,6)
 rect(x+1,y+3,1,2,6)
 -- shield
 if pshield then
  circb(x,y,7,10)
 end
end

local function draw_enemy(e)
 local x,y=math.floor(e.x),math.floor(e.y)
 local w=e.w
 local c=e.col
 if e.type==0 then
  -- diamond drone
  line(x,y-w, x+w,y, c)
  line(x+w,y, x,y+w, c)
  line(x,y+w, x-w,y, c)
  line(x-w,y, x,y-w, c)
  pix(x,y,8)
 elseif e.type==1 then
  -- fast wedge
  line(x,y+w, x-w,y-w, c)
  line(x,y+w, x+w,y-w, c)
  line(x-w,y-w, x+w,y-w, c)
  pix(x,y,8)
 else
  -- heavy hexagon
  line(x-w,y-3, x+w,y-3, c)
  line(x+w,y-3, x+w+2,y, c)
  line(x+w+2,y, x+w,y+3, c)
  line(x+w,y+3, x-w,y+3, c)
  line(x-w,y+3, x-w-2,y, c)
  line(x-w-2,y, x-w,y-3, c)
  pix(x,y,8)
 end
end

local function draw_bullet(b)
 local x,y=math.floor(b.x),math.floor(b.y)
 if b.player then
  line(x,y-3,x,y+3,11)
  pix(x,y,8)
 else
  pix(x,y,2)
  pix(x,y-1,9)
 end
end

local function draw_power(p)
 local x,y=math.floor(p.x),math.floor(p.y)
 local c= p.type==0 and 4 or (p.type==1 and 11 or 10)
 circb(x,y,4,c)
 local ltr= p.type==0 and "W" or (p.type==1 and "S" or "+")
 print(ltr,x-2,y-3,c,false,1)
end

local function draw_hud()
 print("SCORE:"..score,1,1,11,false,1)
 local wname={"INTERCEPTED","GRAVEYARD","CONVERGENCE","ROGUE MIND"}
 print("W"..wave_idx..":"..wname[wave_idx],1,8,6,false,1)
 -- lives as dots
 for i=1,lives do
  rect(W-4-(i-1)*7,1,5,5,11)
 end
 -- weapon level
 print("WPN"..pweapon,W-30,8,4,false,1)
 -- shield indicator
 if pshield then print("SHD",W-30,1,10,false,1) end
 -- kills progress
 local wd=WAVES[wave_idx]
 local tgt=wd.kills
 local bar_w=40
 local ratio=math.min(1,kills/tgt)
 rect(W/2-20,1,bar_w,3,0)
 rect(W/2-20,1,math.floor(bar_w*ratio),3,5)
 print("K:"..kills,W/2-20,5,15,false,1)
 -- combo
 if combo>=2 then
  print("COMBO x"..combo,W/2-25,H/2-10,4,false,1)
 end
end

-- story screen: draw lines with cursor
local function draw_story()
 cls(0)
 -- stars slow
 for _,s in ipairs(stars) do
  pix(math.floor(s.x),math.floor(s.y),14)
 end

 local title= victory_done and "VOID CORE" or "THE LAST SIGNAL"
 print(title,W/2-#title*2,4,12,false,1)
 line(0,11,W,11,1)

 -- display typewriter lines
 local y=16
 for _,ln in ipairs(story_lines) do
  print(ln,2,y,11,false,1)
  y=y+7
  if y>H-14 then break end
 end

 -- blink prompt when done
 if story_pos>=#story_text then
  if frame%60<40 then
   print("A/Enter to continue",2,H-9,15,false,1)
  end
 end
end

local function draw_playing()
 -- background
 cls(0)

 -- scrolling grid
 local grid_off=frame%24
 for gx=0,W,24 do line(gx,0,gx,H,7) end
 for gy=0,H,16 do line(0,(gy+grid_off)%H,W,(gy+grid_off)%H,7) end

 -- stars
 for _,s in ipairs(stars) do
  pix(math.floor(s.x),math.floor(s.y),s.col)
 end

 local ox=math.floor(shake_x)
 local oy=math.floor(shake_y)

 -- particles
 for _,p in ipairs(particles) do
  local a=math.floor(p.life*3)
  if a>=0 and a<=2 then
   pix(math.floor(p.x)+ox,math.floor(p.y)+oy,p.col)
  end
 end

 -- powers
 for _,p in ipairs(powers) do
  if not p.dead then
   local x,y=math.floor(p.x)+ox,math.floor(p.y)+oy
   local c= p.type==0 and 4 or (p.type==1 and 11 or 10)
   circb(x,y,4,c)
   local ltr= p.type==0 and "W" or (p.type==1 and "S" or "+")
   print(ltr,x-2,y-3,c,false,1)
  end
 end

 -- bullets
 for _,b in ipairs(bullets) do
  if not b.dead then
   local x,y=math.floor(b.x)+ox,math.floor(b.y)+oy
   if b.player then
    line(x,y-3,x,y+3,11)
    pix(x,y,8)
   else
    pix(x,y,2)
    pix(x,y-1,9)
   end
  end
 end

 -- enemies
 for _,e in ipairs(enemies) do
  if not e.dead then
   draw_enemy({x=e.x+ox,y=e.y+oy,type=e.type,w=e.w,col=e.col})
  end
 end

 -- player
 draw_player()

 -- floating score texts
 for _,f in ipairs(ftexts) do
  local a=math.floor(f.life/35*2)
  print(f.txt,math.floor(f.x)-4,math.floor(f.y),f.col,false,1)
 end

 draw_hud()
end

local function draw_title()
 cls(0)
 -- stars
 for _,s in ipairs(stars) do
  pix(math.floor(s.x),math.floor(s.y),14)
 end
 -- title
 local t1="NEON DEFENDER"
 print(t1,W/2-#t1*2,20,11,false,1)
 local t2="THE LAST SIGNAL"
 print(t2,W/2-#t2*2,32,12,false,1)
 line(0,42,W,42,6)

 -- story blurb
 print("Colony ship EXODUS-7",4,50,15,false,1)
 print("50,000 souls. One pilot.",4,58,15,false,1)
 print("A signal from the Void.",4,66,15,false,1)
 print("It should not exist.",4,74,15,false,1)

 line(0,85,W,85,6)
 print("D-Pad: Move",4,90,14,false,1)
 print("A: Shoot",4,98,14,false,1)
 print("Collect W=Weapon S=Speed +=Shield",4,106,7,false,1)

 if frame%60<40 then
  print("Press A or Enter to start",W/2-48,H-12,4,false,1)
 end
end

local function draw_gameover()
 cls(0)
 for _,s in ipairs(stars) do
  pix(math.floor(s.x),math.floor(s.y),14)
 end
 -- particles still visible
 for _,p in ipairs(particles) do
  pix(math.floor(p.x),math.floor(p.y),p.col)
 end
 print("SIGNAL LOST",W/2-22,30,2,false,1)
 print("SCORE:"..score,W/2-24,46,11,false,1)
 print("WAVE "..wave_idx,W/2-20,58,6,false,1)
 print("EXODUS-7 drifts into",4,72,14,false,1)
 print("the Void...",4,80,14,false,1)
 if frame%60<40 then
  print("A/Enter to retry",W/2-32,H-12,4,false,1)
 end
end

local function draw_victory()
 draw_story()
 -- override title
 print("MISSION COMPLETE",W/2-32,4,4,false,1)
 print("SCORE:"..score,W/2-20,H-9,11,false,1)
end

-- ============================================================
-- STORY UPDATE (typewriter)
-- ============================================================
local function update_story()
 if story_pos<#story_text then
  if frame%2==0 then
   story_pos=story_pos+1
   local ch=story_text:sub(story_pos,story_pos)
   if ch=="\n" then
    story_lines[#story_lines+1]=""
   else
    if #story_lines==0 then story_lines[1]="" end
    story_lines[#story_lines]=story_lines[#story_lines]..ch
   end
  end
  -- skip with button
  if btnp(4) or btnp(5) or btnp(7) then
   -- fast-forward: dump all text
   story_lines=split_lines(story_text)
   story_pos=#story_text
  end
 else
  -- text done, wait for input or auto-advance
  story_wait=story_wait+1
  if story_wait>=240 or btnp(4) or btnp(7) then
   -- decide what to do next
   if state=="VICTORY" then
    state="GAMEOVER"
    victory_done=true
   elseif showing_after then
    showing_after=false
    -- show next wave intro story
    start_story(WAVES[wave_idx].story)
   else
    -- start the wave
    state="PLAYING"
    wave_cleared=false; slowmo=0
    bullets={}; enemies={}; powers={}
    ftexts={}; combo=0; combo_t=0
   end
  end
 end
end

-- ============================================================
-- MAIN TIC LOOP
-- ============================================================
function TIC()
 frame=frame+1
 update_stars()
 update_particles()

 if state=="TITLE" then
  draw_title()
  if btnp(4) or btnp(7) then
   reset_game()
   init_stars()
   start_story(WAVES[1].story)
  end

 elseif state=="STORY" then
  update_story()
  if victory_done and state=="STORY" then
   draw_victory()
  else
   draw_story()
  end

 elseif state=="PLAYING" then
  update_playing()
  update_ftexts()
  draw_playing()

 elseif state=="GAMEOVER" then
  -- keep particles running on game over screen
  draw_gameover()
  if btnp(4) or btnp(7) then
   reset_game()
   init_stars()
   start_story(WAVES[1].story)
  end

 end
end

-- ============================================================
-- INIT
-- ============================================================
init_stars()
