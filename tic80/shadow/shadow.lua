-- title: Shadow Blade
-- author: retrogames
-- desc: The Crimson Oath - ninja action platformer
-- script: lua

-- ============================================================
-- SHADOW BLADE - TIC-80 port (240x136)
-- Scale: web 800x600 -> 240x136 (0.3x)
-- TILE=5 (web TILE=16 * 0.3 ~= 5)
-- ============================================================

local TILE=5
local GW=240
local GH=136
local GRAVITY=0.17  -- 0.55*0.3
local MAX_FALL=2.7  -- 9*0.3

-- Sweetie-16 palette indices
local C_BLACK=0    -- dark navy
local C_DPURP=1    -- dark purple
local C_RED=2      -- red
local C_ORANGE=3   -- orange
local C_YELLOW=4   -- yellow
local C_LGREEN=5   -- light green
local C_GREEN=6    -- green
local C_TEAL=7     -- dark teal
local C_WHITE=8    -- white/light
local C_ORAN2=9    -- orange2
local C_GREN2=10   -- green2
local C_LBLUE=11   -- light blue
local C_MAGEN=12   -- magenta
local C_CYAN=13    -- teal/cyan
local C_DBLUE=14   -- dark blue-gray
local C_GRAY=15    -- gray

-- Game states
local STATE_TITLE=0
local STATE_STORY=1
local STATE_PLAYING=2
local STATE_GAMEOVER=3
local STATE_VICTORY=4

-- ============================================================
-- GLOBAL STATE
-- ============================================================
local gstate=STATE_TITLE
local frame=0
local score=0
local hiscore=0
local cur_level=0  -- 0,1,2

-- Camera
local cam_x=0
local cam_y=0

-- Player
local pl={}
-- Entities (enemies)
local ents={}
-- Projectiles
local projs={}
-- Particles
local parts={}
-- Pickups
local picks={}
-- Level tiles
local tiles={}
local lvl_w=0
local lvl_h=0

-- Input previous state
local prev_btn={}
for i=0,7 do prev_btn[i]=false end

-- ============================================================
-- INPUT HELPERS
-- ============================================================
local function btnp_now(b)
  local cur=btn(b)
  local p=prev_btn[b]
  return cur and not p
end

local function inp_left()  return btn(2) end
local function inp_right() return btn(3) end
local function inp_up()    return btn(0) end
local function inp_down()  return btn(1) end
local function inp_jump()  return btnp_now(4) end   -- A just pressed
local function inp_jumpH() return btn(4) end        -- A held
local function inp_atk()   return btnp_now(5) end   -- B just pressed
local function inp_dash()  return btnp_now(6) end   -- X just pressed (if mapped)
local function inp_star()  return btnp_now(7) end   -- Y just pressed

-- ============================================================
-- STORY SYSTEM
-- ============================================================
local story_lines={}
local story_full=""
local story_shown=""
local story_idx=0
local story_timer=0
local story_done=false
local story_cb=nil
local STORY_SPD=2  -- chars per frame tick

local STORY_PRE={
  -- Pre-game
  {
    "YOU ARE KAEDE,",
    "NINJA OF THE SHADOW LOTUS.",
    "",
    "ACCUSED OF KILLING THE SHOGUN,",
    "STRIPPED OF RANK, MARKED FOR DEATH.",
    "",
    "YOUR MASTER CAST YOU OUT.",
    "BUT YOU WERE FRAMED.",
    "",
    "TO CLEAR YOUR NAME, COMPLETE",
    "THE THREE TRIALS OF THE",
    "CRIMSON OATH.",
  },
  -- Pre-level 1
  {
    "--- TRIAL I: SPEED ---",
    "",
    "RACE THROUGH THE BAMBOO FOREST",
    "BEFORE DAWN BREAKS.",
    "",
    "ITS GUARDIANS SEE ALL",
    "INTRUDERS AS THREATS.",
  },
  -- Pre-level 2
  {
    "--- TRIAL II: COURAGE ---",
    "",
    "INFILTRATE CASTLE KURODA,",
    "WHERE THE SHOGUN WAS KILLED.",
    "",
    "HIS GUARDSMEN REFUSE",
    "TO LEAVE THEIR POSTS,",
    "EVEN IN DEATH.",
  },
  -- Pre-level 3
  {
    "--- TRIAL III: TRUTH ---",
    "",
    "SENSEI TAKESHI AWAITS",
    "AT THE CRIMSON SHRINE.",
    "",
    "HE KNEW YOU WOULD COME.",
    "HE HAS ALWAYS KNOWN.",
  },
}

local STORY_POST={
  -- Post-level 1
  {
    "YOU FIND A SCROLL:",
    "",
    "'THE SHOGUN'S KILLER MOVED",
    " LIKE WIND THROUGH STILL AIR.",
    " ONLY TWO IN THE SHADOW LOTUS",
    " POSSESS SUCH SPEED --",
    " YOU, AND YOUR MASTER.'",
  },
  -- Post-level 2
  {
    "THE ASSASSINATION REPORT:",
    "",
    "THE KILLING BLOW CAME",
    "FROM THE RAFTERS. NO ENTRY",
    "POINT WAS FOUND.",
    "",
    "ONLY TWO HAD KEYS TO THE",
    "SHOGUN'S CHAMBERS:",
    "THE HEAD OF SECURITY...",
    "AND SENSEI TAKESHI.",
  },
  -- Post-level 3
  {
    "TAKESHI:",
    "'YOU WERE MY FINEST STUDENT.",
    " THE SHOGUN ORDERED OUR CLAN",
    " DESTROYED. I KILLED HIM.",
    " I FRAMED YOU BECAUSE YOU",
    " WOULD HAVE STOPPED ME.'",
    "",
    "TAKESHI KNEELS, OFFERING",
    "HIS SWORD.",
    "",
    "'THE CLAN IS SAFE.",
    " TAKE MY BLADE, KAEDE.'",
    "",
    "THE CRIMSON OATH IS COMPLETE.",
  },
}

local function show_story(lines, cb)
  gstate=STATE_STORY
  story_lines=lines
  story_full=""
  for i,l in ipairs(lines) do
    if i>1 then story_full=story_full.."\n" end
    story_full=story_full..l
  end
  story_shown=""
  story_idx=0
  story_timer=0
  story_done=false
  story_cb=cb
end

local function update_story()
  story_timer=story_timer+1
  if story_timer>=STORY_SPD and story_idx<#story_full then
    story_idx=story_idx+1
    story_shown=string.sub(story_full,1,story_idx)
    story_timer=0
  end
  if story_idx>=#story_full then story_done=true end
end

local function advance_story()
  if not story_done then
    story_idx=#story_full
    story_shown=story_full
    story_done=true
  else
    if story_cb then story_cb() end
  end
end

-- ============================================================
-- LEVEL BUILDING
-- ============================================================
-- Tile types: 0=air, 1=solid ground, 2=wall, 3=platform, 4=spike
local LVL_H=30
local LVL_WIDTHS={90,100,95}  -- in tiles

local function tile_idx(tx,ty)
  if tx<0 or tx>=lvl_w or ty<0 or ty>=lvl_h then
    if ty>=lvl_h then return 1 end
    return 0
  end
  return tiles[ty*lvl_w+tx+1]
end

local function is_solid(tx,ty)
  local t=tile_idx(tx,ty)
  return t==1 or t==2
end

local function set_tile(tx,ty,v)
  if tx>=0 and tx<lvl_w and ty>=0 and ty<lvl_h then
    tiles[ty*lvl_w+tx+1]=v
  end
end

local function build_level(lvl)
  local W=LVL_WIDTHS[lvl+1]
  lvl_w=W
  lvl_h=LVL_H
  tiles={}
  for i=1,W*LVL_H do tiles[i]=0 end

  local H=LVL_H

  local function add_ground(gaps)
    for x=0,W-1 do
      local is_gap=false
      for _,g in ipairs(gaps) do
        if x>g[1] and x<g[2] then is_gap=true break end
      end
      if is_gap then
        set_tile(x,H-1,4)
      else
        set_tile(x,H-1,1)
        set_tile(x,H-2,1)
      end
    end
  end

  local function add_wall(x,y1,y2)
    for y=y1,y2 do set_tile(x,y,2) end
  end

  local function add_plat(x,y,len,tp)
    for i=0,len-1 do set_tile(x+i,y,tp or 3) end
  end

  if lvl==0 then
    add_ground({{30,34},{70,74}})
    add_plat(8,24,5) add_plat(15,21,4) add_plat(21,18,5) add_plat(10,17,3)
    add_wall(28,15,27) add_wall(32,12,27)
    add_plat(29,14,3) add_plat(33,11,4)
    add_plat(42,24,6) add_plat(50,22,4) add_plat(46,19,5)
    add_plat(55,20,3) add_plat(60,17,6) add_plat(52,15,3) add_plat(58,13,4)
    add_wall(68,10,27) add_wall(72,8,27)
    add_plat(69,14,3) add_plat(64,22,4) add_plat(73,10,5)
    add_plat(80,24,10,1) add_plat(80,23,10,1)
    for y=0,H-3 do set_tile(W-1,y,2) set_tile(W-2,y,2) end

  elseif lvl==1 then
    add_ground({{30,34},{55,59},{80,84}})
    add_plat(8,24,8) add_plat(18,22,6) add_plat(12,19,5) add_plat(26,20,4)
    add_plat(21,16,6) add_plat(32,18,5) add_plat(28,14,4)
    add_plat(38,22,5) add_plat(36,16,3)
    add_wall(44,10,27) add_wall(48,8,27)
    add_plat(45,12,3) add_plat(49,9,4)
    add_plat(54,25,4) add_plat(60,23,3) add_plat(56,20,4)
    add_plat(64,18,3) add_plat(68,21,4) add_plat(62,15,4)
    add_plat(72,16,5) add_plat(76,13,4) add_plat(70,11,3)
    add_wall(82,8,27) add_wall(86,6,27)
    add_plat(83,10,3) add_plat(87,7,4)
    add_plat(90,24,10,1) add_plat(90,23,10,1)
    for y=0,H-3 do set_tile(W-1,y,2) set_tile(W-2,y,2) end

  else -- lvl==2
    add_ground({{25,29},{50,54},{75,79}})
    add_plat(8,24,5) add_plat(15,22,4) add_plat(11,18,5) add_plat(21,20,6)
    add_plat(28,17,4) add_plat(24,14,3)
    add_wall(34,8,27) add_wall(38,6,27)
    add_plat(35,10,3) add_plat(39,8,4)
    add_plat(42,24,6) add_plat(44,18,4) add_plat(50,20,3)
    add_plat(55,22,5) add_plat(53,16,5) add_plat(60,18,4) add_plat(58,13,3)
    add_wall(66,8,27) add_wall(70,6,27)
    add_plat(67,10,3) add_plat(71,7,4)
    add_plat(74,22,4) add_plat(76,16,4) add_plat(80,20,3)
    add_plat(84,24,11,1) add_plat(84,23,11,1)
    for y=0,H-3 do set_tile(W-1,y,2) set_tile(W-2,y,2) end
  end
end

-- ============================================================
-- COLLISION HELPERS
-- ============================================================
local function move_x(obj, dx)
  obj.x=obj.x+dx
  local eff_y=obj.y
  local eff_h=obj.h
  if dx<0 then
    local tx=math.floor(obj.x/TILE)
    local ty0=math.floor(eff_y/TILE)
    local ty1=math.floor((eff_y+eff_h-1)/TILE)
    for ty=ty0,ty1 do
      if is_solid(tx,ty) then
        obj.x=(tx+1)*TILE obj.vx=0 break
      end
    end
  elseif dx>0 then
    local tx=math.floor((obj.x+obj.w)/TILE)
    local ty0=math.floor(eff_y/TILE)
    local ty1=math.floor((eff_y+eff_h-1)/TILE)
    for ty=ty0,ty1 do
      if is_solid(tx,ty) then
        obj.x=tx*TILE-obj.w obj.vx=0 break
      end
    end
  end
end

local function move_y(obj, dy)
  obj.y=obj.y+dy
  obj.on_ground=false
  if dy>=0 then
    local by=math.floor((obj.y+obj.h)/TILE)
    local tx0=math.floor(obj.x/TILE)
    local tx1=math.floor((obj.x+obj.w-1)/TILE)
    for tx=tx0,tx1 do
      local t=tile_idx(tx,by)
      if t==1 or t==2 then
        obj.y=by*TILE-obj.h obj.vy=0 obj.on_ground=true break
      elseif t==3 then
        -- one-way: only land if falling from above
        if obj.y+obj.h-dy<=by*TILE+2 then
          obj.y=by*TILE-obj.h obj.vy=0 obj.on_ground=true break
        end
      end
    end
  else
    local ty=math.floor(obj.y/TILE)
    local tx0=math.floor(obj.x/TILE)
    local tx1=math.floor((obj.x+obj.w-1)/TILE)
    for tx=tx0,tx1 do
      if is_solid(tx,ty) then
        obj.y=(ty+1)*TILE obj.vy=0 break
      end
    end
  end
end

-- ============================================================
-- PARTICLES
-- ============================================================
local MAX_PARTS=60

local function spawn_parts(x,y,count,col,spread,life)
  for i=1,count do
    if #parts>=MAX_PARTS then break end
    table.insert(parts,{
      x=x,y=y,
      vx=(math.random()-0.5)*spread,
      vy=(math.random()-0.5)*spread-0.3,
      life=life or (6+math.random()*5),
      max_life=life or 10,
      col=col,
      sz=1+(math.random()>0.5 and 1 or 0)
    })
  end
end

local function update_parts()
  for i=#parts,1,-1 do
    local p=parts[i]
    p.x=p.x+p.vx p.y=p.y+p.vy
    p.vy=p.vy+0.05
    p.life=p.life-1
    if p.life<=0 then table.remove(parts,i) end
  end
end

local function draw_parts()
  for _,p in ipairs(parts) do
    if p.life>0 then
      pix(math.floor(p.x-cam_x),math.floor(p.y-cam_y),p.col)
    end
  end
end

-- ============================================================
-- PLAYER
-- ============================================================
local function make_player(px,py)
  return {
    x=px,y=py,w=3,h=6,
    vx=0,vy=0,
    facing=1,
    on_ground=false,
    hp=5,max_hp=5,
    shurikens=15,
    invuln=0,
    -- jump
    jump_hold=0,max_jump_hold=8,
    coyote=0,jump_buf=0,
    -- wall
    wall_dir=0,wall_sliding=false,
    -- dash
    can_dash=true,dashing=false,
    dash_timer=0,dash_dir=0,
    -- attack
    attacking=false,combo=0,
    atk_timer=0,combo_win=0,
    atk_hb=nil,  -- {x,y,w,h,dmg,hit={}}
    shur_cd=0,
    -- anim
    anim_t=0,
    state="idle",
    -- death
    dead=false,death_t=0,
  }
end

local function player_take_dmg(p,dmg)
  if p.invuln>0 or p.dead then return end
  p.hp=p.hp-dmg
  p.invuln=45
  spawn_parts(p.x+p.w/2,p.y+p.h/2,6,C_RED,2,8)
  if p.hp<=0 then
    p.hp=0 p.dead=true p.death_t=0
    spawn_parts(p.x+p.w/2,p.y+p.h/2,15,C_RED,3,12)
  end
end

local function update_player(p)
  if p.dead then p.death_t=p.death_t+1 return end
  if p.invuln>0 then p.invuln=p.invuln-1 end
  if p.shur_cd>0 then p.shur_cd=p.shur_cd-1 end

  -- Wall detection
  p.wall_dir=0
  if not p.on_ground then
    local mid_ty=math.floor((p.y+p.h/2)/TILE)
    local top_ty=math.floor(p.y/TILE)
    if is_solid(math.floor((p.x-1)/TILE),mid_ty) or
       is_solid(math.floor((p.x-1)/TILE),top_ty) then
      p.wall_dir=-1
    end
    if is_solid(math.floor((p.x+p.w)/TILE),mid_ty) or
       is_solid(math.floor((p.x+p.w)/TILE),top_ty) then
      p.wall_dir=1
    end
  end

  -- Attack
  if p.attacking then
    p.atk_timer=p.atk_timer-1
    if p.atk_timer<=0 then
      p.attacking=false p.atk_hb=nil p.combo_win=12
    end
  end
  if p.combo_win>0 then p.combo_win=p.combo_win-1 end
  if p.combo_win<=0 and not p.attacking then p.combo=0 end

  if inp_atk() and not p.attacking and not p.dashing then
    p.attacking=true
    if p.combo_win>0 and p.combo<3 then p.combo=p.combo+1
    else p.combo=1 end
    p.combo_win=0
    local dur=p.combo==3 and 10 or 7
    p.atk_timer=dur p.anim_t=0
    local hbw=p.combo==3 and 6 or 5
    local hbh=p.combo==3 and 5 or 4
    local hbx=p.facing>0 and (p.x+p.w) or (p.x-hbw)
    local hby=p.y+(p.combo==3 and -1 or 1)
    p.atk_hb={x=hbx,y=hby,w=hbw,h=hbh,dmg=p.combo==3 and 2 or 1,hit={}}
    spawn_parts(p.x+p.w/2+p.facing*3,p.y+p.h/2,4,C_WHITE,2,5)
  end

  -- Shuriken
  if inp_star() and p.shurikens>0 and p.shur_cd<=0 and not p.attacking then
    p.shurikens=p.shurikens-1 p.shur_cd=18
    table.insert(projs,{
      x=p.x+(p.facing>0 and p.w or -2),y=p.y+2,
      vx=p.facing*2.1,vy=0,w=2,h=2,
      dmg=1,from_pl=true,life=80,kind="shur",rot=0
    })
  end

  -- Update attack hitbox position
  if p.atk_hb then
    local hbw=p.combo==3 and 6 or 5
    p.atk_hb.x=p.facing>0 and (p.x+p.w) or (p.x-hbw)
    p.atk_hb.y=p.y+(p.combo==3 and -1 or 1)
  end

  -- Dashing
  if p.dashing then
    p.dash_timer=p.dash_timer-1
    p.vx=p.dash_dir*4.2 p.vy=0
    if p.dash_timer<=0 then
      p.dashing=false p.vx=p.dash_dir*0.9
    end
  end

  -- Movement
  if not p.dashing and not p.attacking then
    local spd=1.05
    local accel=p.on_ground and 0.8 or 0.5
    local tvx=0
    if inp_left() then tvx=-spd p.facing=-1 end
    if inp_right() then tvx=spd p.facing=1 end
    p.vx=p.vx+(tvx-p.vx)*accel
    if math.abs(p.vx)<0.1 and tvx==0 then p.vx=0 end
  end

  -- Dash trigger
  if inp_dash() and not p.dashing then
    if not p.on_ground and p.can_dash then
      p.dashing=true p.dash_timer=5
      p.dash_dir=p.facing p.can_dash=false
      p.invuln=math.max(p.invuln,7)
    end
  end

  -- Wall slide
  p.wall_sliding=false
  if not p.on_ground and not p.dashing and p.wall_dir~=0 and p.vy>0 then
    if (p.wall_dir==-1 and inp_left()) or (p.wall_dir==1 and inp_right()) then
      p.wall_sliding=true
      p.vy=math.min(p.vy,0.45)
      p.can_dash=true
    end
  end

  -- Gravity
  if not p.dashing then
    p.vy=p.vy+GRAVITY
    if p.jump_hold>0 and inp_jumpH() then
      p.vy=p.vy-0.1
      p.jump_hold=p.jump_hold-1
    end
    if p.vy>MAX_FALL then p.vy=MAX_FALL end
  end

  -- Jump
  if p.on_ground then p.coyote=6 p.can_dash=true
  else if p.coyote>0 then p.coyote=p.coyote-1 end
  end
  if inp_jump() then p.jump_buf=5 end
  if p.jump_buf>0 then p.jump_buf=p.jump_buf-1 end

  if p.jump_buf>0 then
    if p.coyote>0 and not p.dashing then
      p.vy=-2.85 p.jump_hold=p.max_jump_hold
      p.on_ground=false p.coyote=0 p.jump_buf=0
    elseif p.wall_sliding and not p.dashing then
      p.vy=-2.7 p.vx=-p.wall_dir*1.5
      p.facing=-p.wall_dir
      p.jump_hold=5 p.wall_sliding=false
      p.can_dash=true p.jump_buf=0
    end
  end

  -- Move & collide
  move_x(p,p.vx)
  move_y(p,p.vy)

  -- Clamp to level
  p.x=math.max(0,math.min(lvl_w*TILE-p.w,p.x))

  -- Spike/fall death
  local ft=tile_idx(math.floor((p.x+p.w/2)/TILE),math.floor((p.y+p.h+1)/TILE))
  if ft==4 then player_take_dmg(p,5) end
  if p.y>lvl_h*TILE+20 then player_take_dmg(p,5) end

  -- Animation state
  if p.attacking then p.state="atk"..p.combo
  elseif p.dashing then p.state="dash"
  elseif p.wall_sliding then p.state="wall"
  elseif not p.on_ground then p.state=p.vy<0 and "jump" or "fall"
  elseif math.abs(p.vx)>0.2 then p.state="run"
  else p.state="idle" end

  p.anim_t=p.anim_t+1
end

-- ============================================================
-- DRAW PLAYER (pixel art, tiny scale)
-- ============================================================
-- Colors: body=C_DPURP, head=C_DPURP, eyes=C_WHITE, scarf=C_RED
local function draw_ninja(sx,sy,facing,state,anim_t,alpha)
  -- alpha: if true, skip every other frame (invuln flash)
  local flip=facing<0
  -- sx,sy are screen coords (already cam-adjusted), top-left of 3x6 hitbox

  -- Draw relative to center of sprite (7x9 pixels rendered at ~1px each)
  -- Simplified pixel art ninja
  local function px2(dx,dy,c)
    local rx=flip and (6-dx) or dx
    pix(sx+rx-2,sy+dy-1,c)
  end

  -- Head (dark purple/black with red scarf)
  px2(3,0,C_DPURP) px2(4,0,C_DPURP)
  px2(2,1,C_DPURP) px2(3,1,C_DPURP) px2(4,1,C_DPURP) px2(5,1,C_DPURP)
  -- Eyes (white)
  local eye1x=flip and 3 or 3
  local eye2x=flip and 5 or 5
  px2(eye1x,1,C_WHITE) px2(eye2x,1,C_WHITE)
  -- Red scarf/mask line
  px2(2,2,C_RED) px2(3,2,C_RED) px2(4,2,C_RED) px2(5,2,C_RED)
  -- Body (dark purple)
  px2(2,3,C_DPURP) px2(3,3,C_DPURP) px2(4,3,C_DPURP) px2(5,3,C_DPURP)

  -- Arms depend on state
  if state=="atk1" or state=="atk2" then
    -- sword arm extended
    local ax=flip and 0 or 6
    px2(ax,2,C_GRAY) px2(ax,3,C_GRAY)
    -- blade
    local bx1=flip and -1 or 7
    local bx2=flip and -2 or 8
    px2(bx1,3,C_WHITE) px2(bx2,3,C_GRAY)
  elseif state=="atk3" then
    -- overhead slash
    px2(3,0,C_GRAY) -- sword up
    local ax=flip and 1 or 5
    px2(ax,-1,C_WHITE)
  else
    -- normal arms
    px2(1,3,C_DPURP) px2(6,3,C_DPURP)
  end

  -- Legs based on movement
  if state=="run" then
    local fr=math.floor(anim_t/4)%2
    if fr==0 then
      px2(2,4,C_DPURP) px2(4,5,C_DPURP)
    else
      px2(4,4,C_DPURP) px2(2,5,C_DPURP)
    end
  elseif state=="jump" or state=="fall" or state=="dash" or state=="wall" then
    px2(2,4,C_DPURP) px2(5,4,C_DPURP)
  else
    px2(2,4,C_DPURP) px2(2,5,C_DPURP)
    px2(4,4,C_DPURP) px2(4,5,C_DPURP)
  end
end

local function draw_player(p)
  if p.dead and p.death_t>20 then return end
  if p.invuln>0 and math.floor(frame/3)%2==1 and not p.dead then return end
  local sx=math.floor(p.x-cam_x)
  local sy=math.floor(p.y-cam_y)
  draw_ninja(sx,sy,p.facing,p.state,p.anim_t)

  -- Slash arc indicator
  if p.attacking and p.atk_timer>3 then
    local cx=sx+p.w/2+p.facing*4
    local cy=sy+p.h/2+(p.combo==3 and -2 or 0)
    local c=p.combo==3 and C_YELLOW or C_WHITE
    line(cx,cy,cx+p.facing*3,cy-2,c)
    line(cx+p.facing*3,cy-2,cx+p.facing*4,cy+1,c)
  end
end

-- ============================================================
-- ENEMIES
-- ============================================================
local function make_guard(ex,ey)
  -- find ground
  local sy=ey
  local tx=math.floor(ex/TILE)
  for ty=math.floor(ey/TILE),LVL_H-1 do
    if tile_idx(tx,ty)==1 or tile_idx(tx,ty)==2 or tile_idx(tx,ty)==3 then
      sy=ty*TILE-6 break
    end
  end
  return {
    x=ex,y=sy,w=4,h=6,
    vx=0,vy=0,facing=-1,on_ground=false,
    dead=false,death_t=0,stun=0,anim_t=0,
    kind="guard",hp=2,spd=0.36,dmg=1,pts=100,
    pat_l=ex-18,pat_r=ex+18,chasing=false,
  }
end

local function make_archer(ex,ey)
  local sy=ey
  local tx=math.floor(ex/TILE)
  for ty=math.floor(ey/TILE),LVL_H-1 do
    local t=tile_idx(tx,ty)
    if t==1 or t==2 or t==3 then sy=ty*TILE-6 break end
  end
  return {
    x=ex,y=sy,w=4,h=6,
    vx=0,vy=0,facing=-1,on_ground=false,
    dead=false,death_t=0,stun=0,anim_t=0,
    kind="archer",hp=1,spd=0,dmg=1,pts=150,
    shoot_t=60+math.random()*30,shoot_cd=90,
  }
end

local function enemy_take_dmg(e,dmg)
  e.hp=e.hp-dmg e.stun=8
  local c=e.kind=="guard" and C_YELLOW or C_MAGEN
  spawn_parts(e.x+e.w/2,e.y+e.h/2,5,c,2,6)
  if e.hp<=0 then
    e.dead=true
    score=score+e.pts
    spawn_parts(e.x+e.w/2,e.y+e.h/2,10,c,3,10)
    if math.random()<0.3 then
      local kinds={"health","scroll","ammo"}
      local k=kinds[math.random(1,3)]
      table.insert(picks,{x=e.x,y=e.y,w=3,h=3,kind=k,vy=-0.9,life=400})
    end
  end
end

local function update_enemy(e)
  if e.dead then e.death_t=e.death_t+1 return end
  if e.stun>0 then e.stun=e.stun-1 return end
  e.anim_t=e.anim_t+1

  local px=pl.x py=pl.y
  local dx=px-e.x
  local dist=math.abs(dx)+math.abs(py-e.y)

  if e.kind=="guard" then
    if dist<60 then
      e.facing=dx>0 and 1 or -1
      e.chasing=dist<39
    end
    if e.chasing and not pl.dead then
      e.vx=e.facing*0.84
    else
      e.vx=e.facing*e.spd
      if e.x<=e.pat_l then e.facing=1 end
      if e.x>=e.pat_r then e.facing=-1 end
      local ahead_x=e.facing>0 and (e.x+e.w+1) or (e.x-1)
      local below=tile_idx(math.floor(ahead_x/TILE),math.floor((e.y+e.h+1)/TILE))
      if below==0 or below==4 then e.facing=-e.facing end
    end
  elseif e.kind=="archer" then
    e.facing=dx>0 and 1 or -1
    e.shoot_t=e.shoot_t-1
    if e.shoot_t<=0 and dist<75 and not pl.dead then
      table.insert(projs,{
        x=e.x+(e.facing>0 and e.w or -3),y=e.y+2,
        vx=e.facing*1.2,vy=0,w=3,h=1,
        dmg=1,from_pl=false,life=100,kind="arrow"
      })
      e.shoot_t=e.shoot_cd
    end
  end

  e.vy=e.vy+GRAVITY
  if e.vy>MAX_FALL then e.vy=MAX_FALL end
  move_x(e,e.vx)
  move_y(e,e.vy)

  -- Wall bounce for guard
  if e.kind=="guard" then
    if e.vx<0 then
      local tx=math.floor(e.x/TILE)
      if is_solid(tx,math.floor((e.y+e.h/2)/TILE)) then
        e.x=(tx+1)*TILE e.facing=1
      end
    elseif e.vx>0 then
      local tx=math.floor((e.x+e.w)/TILE)
      if is_solid(tx,math.floor((e.y+e.h/2)/TILE)) then
        e.x=tx*TILE-e.w e.facing=-1
      end
    end
  end

  -- Contact dmg to player
  if not pl.dead and pl.invuln<=0 then
    if e.x<pl.x+pl.w and e.x+e.w>pl.x and
       e.y<pl.y+pl.h and e.y+e.h>pl.y then
      player_take_dmg(pl,e.dmg)
      pl.vx=(pl.x<e.x and -1 or 1)*1.2
      pl.vy=-0.9
    end
  end

  if e.y>lvl_h*TILE+30 then e.dead=true end
end

local function draw_enemy(e)
  if e.dead and e.death_t>12 then return end
  local sx=math.floor(e.x-cam_x)
  local sy=math.floor(e.y-cam_y)
  if sx<-10 or sx>GW+10 or sy<-10 or sy>GH+10 then return end

  local c1=e.kind=="guard" and C_ORANGE or C_MAGEN
  local c2=e.kind=="guard" and C_ORAN2 or C_DPURP

  -- Simple enemy sprite (4x6)
  -- Head
  pix(sx+1,sy,c2) pix(sx+2,sy,c2)
  pix(sx,sy+1,c2) pix(sx+1,sy+1,c2) pix(sx+2,sy+1,c2) pix(sx+3,sy+1,c2)
  -- Eyes
  pix(sx+1,sy+1,C_WHITE) pix(sx+3,sy+1,C_WHITE)
  -- Body
  pix(sx,sy+2,c1) pix(sx+1,sy+2,c1) pix(sx+2,sy+2,c1) pix(sx+3,sy+2,c1)
  pix(sx,sy+3,c1) pix(sx+1,sy+3,c1) pix(sx+2,sy+3,c1) pix(sx+3,sy+3,c1)
  -- Weapon indicator
  if e.kind=="archer" then
    local wx=e.facing>0 and sx+4 or sx-1
    pix(wx,sy+2,C_GRAY) pix(wx,sy+3,C_GRAY)
  end
  -- Legs
  local fr=math.floor(e.anim_t/6)%2
  if math.abs(e.vx)>0.1 then
    if fr==0 then
      pix(sx,sy+4,c1) pix(sx+2,sy+5,c1)
    else
      pix(sx+2,sy+4,c1) pix(sx,sy+5,c1)
    end
  else
    pix(sx,sy+4,c1) pix(sx+3,sy+4,c1)
    pix(sx,sy+5,c1) pix(sx+3,sy+5,c1)
  end

  -- HP bar for guards
  if not e.dead and e.kind=="guard" then
    rect(sx,sy-2,4,1,C_RED)
    if e.hp>=2 then rect(sx,sy-2,4,1,C_LGREEN) end
  end
end

-- ============================================================
-- PROJECTILES
-- ============================================================
local function update_projs()
  for i=#projs,1,-1 do
    local p=projs[i]
    p.x=p.x+p.vx p.y=p.y+p.vy
    if p.kind=="shur" then p.rot=(p.rot or 0)+0.3 end
    p.life=p.life-1
    local tx=math.floor((p.x+p.w/2)/TILE)
    local ty=math.floor((p.y+p.h/2)/TILE)
    if is_solid(tx,ty) or p.life<=0 then
      table.remove(projs,i) goto continue
    end
    if p.from_pl then
      local hit=false
      for _,e in ipairs(ents) do
        if not e.dead and
           p.x<e.x+e.w and p.x+p.w>e.x and
           p.y<e.y+e.h and p.y+p.h>e.y then
          enemy_take_dmg(e,p.dmg)
          hit=true break
        end
      end
      if hit then table.remove(projs,i) end
    else
      if not pl.dead and pl.invuln<=0 and
         p.x<pl.x+pl.w and p.x+p.w>pl.x and
         p.y<pl.y+pl.h and p.y+p.h>pl.y then
        player_take_dmg(pl,p.dmg)
        table.remove(projs,i)
      end
    end
    ::continue::
  end
end

local function draw_projs()
  for _,p in ipairs(projs) do
    local sx=math.floor(p.x-cam_x)
    local sy=math.floor(p.y-cam_y)
    if p.kind=="shur" then
      pix(sx,sy,C_WHITE)
      pix(sx+1,sy,C_GRAY)
      pix(sx,sy+1,C_GRAY)
      pix(sx+1,sy+1,C_WHITE)
    else
      -- arrow
      pix(sx,sy,C_YELLOW)
      pix(sx+1,sy,C_GRAY)
      pix(sx+2,sy,C_GRAY)
    end
  end
end

-- ============================================================
-- PICKUPS
-- ============================================================
local function update_picks()
  for i=#picks,1,-1 do
    local p=picks[i]
    if p.vy~=0 then
      p.vy=p.vy+0.09
      p.y=p.y+p.vy
      local by=math.floor((p.y+p.h)/TILE)
      for tx=math.floor(p.x/TILE),math.floor((p.x+p.w-1)/TILE) do
        local t=tile_idx(tx,by)
        if t==1 or t==2 or t==3 then
          p.y=by*TILE-p.h p.vy=0 break
        end
      end
    end
    p.life=p.life-1
    -- Collect
    if not pl.dead and
       p.x<pl.x+pl.w and p.x+p.w>pl.x and
       p.y<pl.y+pl.h and p.y+p.h>pl.y then
      if p.kind=="health" then pl.hp=math.min(pl.max_hp,pl.hp+1)
      elseif p.kind=="scroll" then score=score+200
      elseif p.kind=="ammo" then pl.shurikens=math.min(30,pl.shurikens+5) end
      spawn_parts(p.x+p.w/2,p.y+p.h/2,6,C_YELLOW,2,8)
      table.remove(picks,i) goto cont2
    end
    if p.life<=0 then table.remove(picks,i) end
    ::cont2::
  end
end

local function draw_picks()
  for _,p in ipairs(picks) do
    local sx=math.floor(p.x-cam_x)
    local sy=math.floor(p.y-cam_y+math.sin(frame*0.1)*0.6)
    if p.kind=="health" then
      pix(sx,sy,C_RED) pix(sx+2,sy,C_RED)
      pix(sx,sy+1,C_RED) pix(sx+1,sy+1,C_RED) pix(sx+2,sy+1,C_RED)
      pix(sx+1,sy+2,C_RED)
    elseif p.kind=="scroll" then
      rect(sx,sy,3,3,C_YELLOW)
      pix(sx+1,sy+1,C_ORANGE)
    else -- ammo
      pix(sx,sy,C_GRAY)
      pix(sx+1,sy,C_WHITE)
      pix(sx,sy+1,C_GRAY)
      pix(sx+1,sy+1,C_GRAY)
    end
  end
end

-- ============================================================
-- LEVEL RENDERING
-- ============================================================
-- Level color themes
local LEVEL_COLORS={
  -- lvl 0: bamboo forest
  {ground_top=C_LGREEN, ground_body=C_TEAL, wall=C_DBLUE, plat=C_GREEN, spike=C_RED},
  -- lvl 1: castle
  {ground_top=C_GRAY, ground_body=C_DBLUE, wall=C_DBLUE, plat=C_GRAY, spike=C_RED},
  -- lvl 2: shrine
  {ground_top=C_RED, ground_body=C_DPURP, wall=C_DPURP, plat=C_ORANGE, spike=C_RED},
}

local function draw_level()
  local lc=LEVEL_COLORS[cur_level+1]
  local start_tx=math.max(0,math.floor(cam_x/TILE)-1)
  local end_tx=math.min(lvl_w-1,math.floor((cam_x+GW)/TILE)+2)
  local start_ty=math.max(0,math.floor(cam_y/TILE)-1)
  local end_ty=math.min(lvl_h-1,math.floor((cam_y+GH)/TILE)+2)

  for ty=start_ty,end_ty do
    for tx=start_tx,end_tx do
      local t=tile_idx(tx,ty)
      if t~=0 then
        local sx=tx*TILE-math.floor(cam_x)
        local sy=ty*TILE-math.floor(cam_y)
        if t==1 then
          local above=tile_idx(tx,ty-1)
          if above==0 or above==3 or above==4 then
            rect(sx,sy,TILE,1,lc.ground_top)
            rect(sx,sy+1,TILE,TILE-1,lc.ground_body)
          else
            rect(sx,sy,TILE,TILE,lc.ground_body)
          end
        elseif t==2 then
          rect(sx,sy,TILE,TILE,lc.wall)
          rectb(sx,sy,TILE,TILE,lc.ground_top)
        elseif t==3 then
          rect(sx,sy,TILE,2,lc.plat)
        elseif t==4 then
          -- spikes
          rect(sx,sy,TILE,TILE,C_BLACK)
          for k=0,2 do
            local bx=sx+k*2
            line(bx,sy+TILE-1,bx+1,sy+TILE-4,C_RED)
          end
        end
      end
    end
  end
end

-- ============================================================
-- BACKGROUND
-- ============================================================
local function draw_bg()
  -- Sky gradient (approximate with bands)
  local c1=cur_level==0 and C_BLACK or (cur_level==1 and C_DBLUE or C_DPURP)
  local c2=cur_level==0 and C_DBLUE or (cur_level==1 and C_DPURP or C_BLACK)
  cls(C_BLACK)
  for y=0,GH//2 do
    rect(0,y,GW,1,y<GH//4 and c1 or c2)
  end

  -- Stars
  for i=0,24 do
    local sx=((i*37+50) % GW) - (math.floor(cam_x*0.02) % GW)
    local sy=(i*29+10) % (GH//2)
    if sx<0 then sx=sx+GW end
    if math.sin(frame*0.05+i)>0.3 then
      pix(sx,sy,C_WHITE)
    end
  end

  -- Moon
  circ(180-math.floor(cam_x*0.003),14,5,C_GRAY)
  circ(180-math.floor(cam_x*0.003),14,4,C_WHITE)

  -- Silhouette mountains
  local mc=cur_level==0 and C_TEAL or (cur_level==1 and C_DBLUE or C_DPURP)
  for i=0,8 do
    local mx=(i*40 - math.floor(cam_x*0.05) % 320)
    local mh=12+math.floor(math.sin(i*1.5)*8)
    if mx<GW+20 and mx>-20 then
      for dx=0,20 do
        local ht=mh - math.abs(dx-10)
        if ht>0 then
          line(mx+dx,GH//2-ht,mx+dx,GH//2,mc)
        end
      end
    end
  end

  -- Bamboo/trees (mid layer)
  local tc=cur_level==0 and C_GREEN or (cur_level==1 and C_DPURP or C_DPURP)
  for i=0,14 do
    local bx=(i*22 - math.floor(cam_x*0.15) % (14*22))
    local bh=18+math.floor(math.sin(i*2.1)*8)
    if bx<GW+5 and bx>-5 then
      rect(bx,GH//2-bh,1,bh,tc)
      line(bx-2,GH//2-bh+3,bx+3,GH//2-bh+3,tc)
    end
  end
end

-- ============================================================
-- HUD
-- ============================================================
local LEVEL_NAMES={"TRIAL I: SPEED","TRIAL II: COURAGE","TRIAL III: TRUTH"}

local function draw_hud()
  -- HP hearts
  for i=0,pl.max_hp-1 do
    local hx=2+i*7
    local c=i<pl.hp and C_RED or C_DPURP
    pix(hx,2,c) pix(hx+2,2,c)
    rect(hx-1,3,5,2,c)
    pix(hx,5,c) pix(hx+1,6,c) pix(hx+2,5,c)
  end

  -- Score
  local sc_str="SC:"..score
  print(sc_str,GW-#sc_str*4-1,2,C_YELLOW,false,1)

  -- Shurikens
  print("*"..pl.shurikens,2,10,C_LGREEN,false,1)

  -- Level name
  local lname=LEVEL_NAMES[cur_level+1]
  print(lname,GW//2-#lname*2,2,C_GRAY,false,1)
end

-- ============================================================
-- SPAWN ENTITIES FOR LEVEL
-- ============================================================
local function spawn_level_ents()
  ents={}
  picks={}
  projs={}
  parts={}

  local T=TILE
  if cur_level==0 then
    table.insert(ents,make_guard(200*T//16,25*T))
    table.insert(ents,make_guard(400*T//16,25*T))
    table.insert(ents,make_guard(600*T//16,25*T))
    table.insert(ents,make_guard(900*T//16,25*T))
    table.insert(ents,make_archer(15*T,20*T))
    table.insert(ents,make_archer(60*T,16*T))
    -- Pickups (convert from tile coords)
    table.insert(picks,{x=12*T,y=22*T,w=3,h=3,kind="scroll",vy=0,life=99999})
    table.insert(picks,{x=22*T,y=16*T,w=3,h=3,kind="scroll",vy=0,life=99999})
    table.insert(picks,{x=50*T,y=20*T,w=3,h=3,kind="scroll",vy=0,life=99999})
    table.insert(picks,{x=73*T,y=8*T,w=3,h=3,kind="scroll",vy=0,life=99999})
    table.insert(picks,{x=35*T,y=23*T,w=3,h=3,kind="health",vy=0,life=99999})
    table.insert(picks,{x=55*T,y=18*T,w=3,h=3,kind="ammo",vy=0,life=99999})
  elseif cur_level==1 then
    table.insert(ents,make_guard(150*T//16,25*T))
    table.insert(ents,make_guard(300*T//16,25*T))
    table.insert(ents,make_guard(450*T//16,25*T))
    table.insert(ents,make_guard(600*T//16,25*T))
    table.insert(ents,make_guard(800*T//16,25*T))
    table.insert(ents,make_guard(950*T//16,25*T))
    table.insert(ents,make_guard(1100*T//16,25*T))
    table.insert(ents,make_guard(1300*T//16,25*T))
    table.insert(ents,make_archer(21*T,15*T))
    table.insert(ents,make_archer(36*T,15*T))
    table.insert(ents,make_archer(62*T,14*T))
    table.insert(ents,make_archer(76*T,12*T))
    table.insert(picks,{x=10*T,y=22*T,w=3,h=3,kind="scroll",vy=0,life=99999})
    table.insert(picks,{x=49*T,y=8*T,w=3,h=3,kind="scroll",vy=0,life=99999})
    table.insert(picks,{x=70*T,y=10*T,w=3,h=3,kind="scroll",vy=0,life=99999})
    table.insert(picks,{x=30*T,y=18*T,w=3,h=3,kind="health",vy=0,life=99999})
    table.insert(picks,{x=65*T,y=20*T,w=3,h=3,kind="health",vy=0,life=99999})
    table.insert(picks,{x=45*T,y=10*T,w=3,h=3,kind="ammo",vy=0,life=99999})
  else
    table.insert(ents,make_guard(150*T//16,25*T))
    table.insert(ents,make_guard(350*T//16,25*T))
    table.insert(ents,make_guard(500*T//16,25*T))
    table.insert(ents,make_guard(700*T//16,25*T))
    table.insert(ents,make_guard(900*T//16,25*T))
    table.insert(ents,make_guard(1050*T//16,25*T))
    table.insert(ents,make_guard(1200*T//16,25*T))
    table.insert(ents,make_archer(15*T,16*T))
    table.insert(ents,make_archer(28*T,16*T))
    table.insert(ents,make_archer(53*T,15*T))
    table.insert(ents,make_archer(76*T,15*T))
    table.insert(picks,{x=20*T,y=18*T,w=3,h=3,kind="scroll",vy=0,life=99999})
    table.insert(picks,{x=39*T,y=6*T,w=3,h=3,kind="scroll",vy=0,life=99999})
    table.insert(picks,{x=67*T,y=8*T,w=3,h=3,kind="scroll",vy=0,life=99999})
    table.insert(picks,{x=48*T,y=20*T,w=3,h=3,kind="health",vy=0,life=99999})
    table.insert(picks,{x=78*T,y=20*T,w=3,h=3,kind="health",vy=0,life=99999})
    table.insert(picks,{x=58*T,y=12*T,w=3,h=3,kind="ammo",vy=0,life=99999})
  end
end

-- ============================================================
-- GAME FLOW
-- ============================================================
local function begin_level()
  gstate=STATE_PLAYING
  build_level(cur_level)
  pl=make_player(3*TILE,24*TILE)
  cam_x=0
  cam_y=lvl_h*TILE-GH
  spawn_level_ents()
end

local function advance_level()
  local post_i=cur_level+1
  show_story(STORY_POST[post_i],function()
    cur_level=cur_level+1
    if cur_level>2 then
      if score>hiscore then hiscore=score end
      gstate=STATE_VICTORY
    else
      show_story(STORY_PRE[cur_level+2],function()
        begin_level()
      end)
    end
  end)
end

local function start_game()
  cur_level=0
  score=0
  ents={} projs={} parts={} picks={}
  show_story(STORY_PRE[1],function()
    show_story(STORY_PRE[2],function()
      begin_level()
    end)
  end)
end

local function reset_title()
  if score>hiscore then hiscore=score end
  gstate=STATE_TITLE
  ents={} projs={} parts={} picks={}
end

-- ============================================================
-- TITLE SCREEN
-- ============================================================
local function draw_title()
  cls(C_BLACK)
  -- Background stars
  for i=0,30 do
    local sx=(i*37+50)%GW
    local sy=(i*23+5)%(GH//2)
    if math.sin(frame*0.05+i)>0.2 then pix(sx,sy,C_WHITE) end
  end

  -- Moon
  circ(200,20,8,C_GRAY)
  circ(200,20,7,C_WHITE)

  -- Silhouette ninja (larger, decorative)
  local nx=GW//2-8
  local ny=GH//2-10
  -- body
  rect(nx+3,ny,5,2,C_BLACK)
  rect(nx+2,ny+2,7,4,C_BLACK)
  -- scarf
  rect(nx+2,ny+2,7,1,C_RED)
  -- legs
  rect(nx+2,ny+6,3,3,C_BLACK)
  rect(nx+6,ny+6,3,3,C_BLACK)
  -- sword
  line(nx+10,ny+3,nx+15,ny-2,C_GRAY)
  pix(nx+15,ny-2,C_WHITE)

  -- Title text
  print("SHADOW",GW//2-12*3,28,C_RED,false,2)
  print("BLADE",GW//2-10*3,42,C_WHITE,false,2)

  print("THE CRIMSON OATH",GW//2-16*2,58,C_RED,false,1)
  print("A TALE OF HONOR & BETRAYAL",GW//2-26*2,66,C_GRAY,false,1)

  -- Blink press start
  if math.floor(frame/30)%2==0 then
    print("PRESS A TO START",GW//2-16*2,84,C_YELLOW,false,1)
  end

  print("B:ATTACK  A:JUMP",GW//2-16*2,96,C_DBLUE,false,1)
  print("X:DASH    Y:STAR",GW//2-16*2,103,C_DBLUE,false,1)
  print("DPAD:MOVE",GW//2-9*2,110,C_DBLUE,false,1)

  if hiscore>0 then
    print("HI:"..hiscore,GW//2-6*2,120,C_ORANGE,false,1)
  end
end

-- ============================================================
-- STORY SCREEN
-- ============================================================
local function draw_story()
  cls(C_BLACK)
  -- Border
  rectb(2,2,GW-4,GH-4,C_RED)
  rectb(4,4,GW-8,GH-8,C_DPURP)

  -- Text
  local lines={}
  local cur_line=""
  for i=1,#story_shown do
    local c=string.sub(story_shown,i,i)
    if c=="\n" then
      table.insert(lines,cur_line) cur_line=""
    else
      cur_line=cur_line..c
    end
  end
  table.insert(lines,cur_line)

  for i,l in ipairs(lines) do
    if i>14 then break end  -- max lines on screen
    local col=C_GRAY
    if string.sub(l,1,3)=="---" then col=C_RED
    elseif string.sub(l,1,7)=="TAKESHI" then col=C_YELLOW end
    print(l,8,(i-1)*9+10,col,false,1)
  end

  -- Prompt
  if story_done then
    if math.floor(frame/20)%2==0 then
      print("A/B TO CONTINUE",GW//2-15*2,GH-12,C_YELLOW,false,1)
    end
  else
    print("A/B TO SKIP",GW//2-11*2,GH-12,C_DBLUE,false,1)
  end
end

-- ============================================================
-- GAME OVER SCREEN
-- ============================================================
local function draw_gameover()
  -- dim overlay
  for y=0,GH do
    if y%2==0 then rect(0,y,GW,1,C_BLACK) end
  end
  print("GAME OVER",GW//2-9*3,GH//2-12,C_RED,false,2)
  print("SCORE:"..score,GW//2-7*2,GH//2+6,C_YELLOW,false,1)
  if math.floor(frame/30)%2==0 then
    print("A TO CONTINUE",GW//2-13*2,GH//2+18,C_WHITE,false,1)
  end
end

-- ============================================================
-- VICTORY SCREEN
-- ============================================================
local function draw_victory()
  cls(C_BLACK)
  -- Confetti particles
  for i=0,20 do
    local px2=(i*37+frame*2)%GW
    local py2=(i*23+frame)%GH
    pix(px2,py2,i%6+2)
  end

  print("CRIMSON OATH",GW//2-12*2,10,C_RED,false,2)
  print("COMPLETE!",GW//2-9*2,24,C_YELLOW,false,2)

  local vlines={
    "THE THREE TRIALS ARE DONE.",
    "THE TRUTH IS REVEALED.",
    "",
    "SENSEI TAKESHI SAVED THE",
    "SHADOW LOTUS BY SLAYING",
    "THE SHOGUN.",
    "",
    "HE FRAMED HIS FINEST",
    "STUDENT TO PROTECT THE CLAN.",
    "",
    "NOW KAEDE LEADS.",
    "THE OATH IS FULFILLED.",
  }
  for i,l in ipairs(vlines) do
    print(l,4,38+i*8,C_GRAY,false,1)
  end

  print("SCORE:"..score,GW//2-7*2,GH-22,C_YELLOW,false,1)
  if score>=hiscore then
    print("HI-SCORE!",GW//2-9*2,GH-14,C_CYAN,false,1)
  end
  if math.floor(frame/30)%2==0 then
    print("A TO RETURN",GW//2-11*2,GH-6,C_WHITE,false,1)
  end
end

-- ============================================================
-- UPDATE CAMERA
-- ============================================================
local function update_camera()
  local tx=pl.x - GW/2 + pl.w/2
  local ty=pl.y - GH/2 + pl.h/2 - 12
  cam_x=cam_x+(tx-cam_x)*0.1
  cam_y=cam_y+(ty-cam_y)*0.06
  cam_x=math.max(0,math.min(lvl_w*TILE-GW,cam_x))
  cam_y=math.max(0,math.min(lvl_h*TILE-GH,cam_y))
end

-- ============================================================
-- MAIN TIC FUNCTION
-- ============================================================
function TIC()
  frame=frame+1
  -- prev_btn was saved at end of last frame, so btnp_now() works correctly

  -- ---- STATE MACHINE ----
  if gstate==STATE_TITLE then
    cls(C_BLACK)
    draw_title()
    if btnp_now(4) or btnp_now(5) then start_game() end

  elseif gstate==STATE_STORY then
    update_story()
    draw_story()
    if btnp_now(4) or btnp_now(5) then advance_story() end

  elseif gstate==STATE_PLAYING then
    -- Update
    update_player(pl)

    -- Level end check
    local end_zone=(lvl_w-4)*TILE
    if pl.x>end_zone and not pl.dead then
      advance_level()
      goto done
    end

    -- Update enemies
    for i=1,#ents do
      local e=ents[i]
      if math.abs(e.x-cam_x-GW/2)<GW then
        update_enemy(e)
      end
    end

    -- Player attack hitbox vs enemies
    if pl.atk_hb then
      local hb=pl.atk_hb
      for _,e in ipairs(ents) do
        if not e.dead and not hb.hit[e] then
          if hb.x<e.x+e.w and hb.x+hb.w>e.x and
             hb.y<e.y+e.h and hb.y+hb.h>e.y then
            enemy_take_dmg(e,hb.dmg)
            hb.hit[e]=true
            e.vx=pl.facing*0.9
            e.vy=-0.6
          end
        end
      end
    end

    update_projs()
    update_picks()
    update_parts()

    -- Clean dead entities
    for i=#ents,1,-1 do
      if ents[i].dead and ents[i].death_t>40 then
        table.remove(ents,i)
      end
    end

    -- Camera
    update_camera()

    -- Death -> gameover
    if pl.dead and pl.death_t>60 then
      gstate=STATE_GAMEOVER
    end

    -- Draw
    draw_bg()
    draw_level()
    draw_picks()
    for _,e in ipairs(ents) do draw_enemy(e) end
    draw_player(pl)
    draw_projs()
    draw_parts()
    draw_hud()

    -- Red flash on hit
    if pl.invuln>35 then
      for y=0,GH-1 do
        if y%4==0 then rect(0,y,GW,1,C_RED) end
      end
    end

    -- Go arrow
    local near_alive=false
    for _,e in ipairs(ents) do
      if not e.dead and math.abs(e.x-pl.x)<90 then
        near_alive=true break
      end
    end
    if not near_alive and not pl.dead then
      if math.floor(frame/20)%3~=0 then
        print(">>>",GW-16,GH//2,C_YELLOW,false,1)
      end
    end

  elseif gstate==STATE_GAMEOVER then
    -- Draw frozen game world
    draw_bg()
    draw_level()
    for _,e in ipairs(ents) do draw_enemy(e) end
    draw_player(pl)
    draw_gameover()
    if btnp_now(4) or btnp_now(5) then reset_title() end

  elseif gstate==STATE_VICTORY then
    draw_victory()
    if btnp_now(4) or btnp_now(5) then reset_title() end
  end

  ::done::

  -- Save button state for next frame's btnp_now()
  for i=0,7 do prev_btn[i]=btn(i) end
end
