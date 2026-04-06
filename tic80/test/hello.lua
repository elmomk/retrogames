-- title: Hello Miyoo
-- author: retrogames
-- script: lua

t=0
x=96
y=48
vx=0
vy=0

function TIC()
 -- input
 if btn(0) then y=y-2 end
 if btn(1) then y=y+2 end
 if btn(2) then x=x-2 end
 if btn(3) then x=x+2 end

 -- bounce
 if x<0 then x=0 end
 if x>230 then x=230 end
 if y<0 then y=0 end
 if y>128 then y=128 end

 t=t+1

 -- draw
 cls(0)

 -- background stars
 for i=0,30 do
  local sx=(i*37+t)%240
  local sy=(i*53)%136
  pix(sx,sy,6)
 end

 -- player rect
 rect(x,y,10,8,12)
 rect(x+2,y+2,6,4,11)

 -- animated circle
 local cx=120+math.cos(t*0.05)*40
 local cy=68+math.sin(t*0.05)*20
 circ(cx,cy,8,9)
 circb(cx,cy,8,10)

 -- text
 print("HELLO MIYOO!",80,10,12)
 print("D-Pad to move",72,126,6)
 print("Frame: "..t,2,2,15)
end
