---
name: test-chailove
description: Package and test a ChaiLove game on the Miyoo Mini Plus via RetroArch
user_invocable: true
args: "[game-name]"
---

Package a ChaiLove game as `.chailove`, upload to Miyoo, and launch via RetroArch.

## Steps

1. Package the game directory as a `.chailove` zip:
```bash
cd /home/mo/data/Documents/git/retrogames/chailove/$ARGUMENTS
zip -r /tmp/$ARGUMENTS.chailove .
```

2. Upload to Miyoo:
```bash
sshpass -p onion scp /tmp/$ARGUMENTS.chailove onion@192.168.0.63:/mnt/SDCARD/Roms/CHAILOVE/
```

3. Launch via RetroArch (optional — can also launch from OnionOS Games menu):
```bash
sshpass -p onion ssh onion@192.168.0.63 "
killall -9 MainUI 2>/dev/null; sleep 1
/mnt/SDCARD/RetroArch/retroarch -L /mnt/SDCARD/RetroArch/.retroarch/cores/chailove_libretro.so /mnt/SDCARD/Roms/CHAILOVE/$ARGUMENTS.chailove > /tmp/retroarch.log 2>&1 &
"
```

4. To check logs after testing:
```bash
sshpass -p onion ssh onion@192.168.0.63 "cat /tmp/retroarch.log"
```

## Available games
test, micro, space, shadow, arena, dragon, mariolike, cyber, neon, nova
