---
name: deploy-tic80
description: Build .tic cartridges and deploy to Miyoo Mini Plus via SCP
user_invocable: true
args: "[game-name|--all] [--run]"
---

Build TIC-80 .tic cartridges from Lua sources and deploy to Miyoo.

## Build cartridges

```bash
cd /home/mo/data/Documents/git/retrogames/tic80

# Build one game
python3 build_tic.py micro

# Build all games
python3 build_tic.py --all
```

## Deploy to Miyoo

```bash
MIYOO="onion@192.168.0.63"

# Upload one
sshpass -p onion scp -o StrictHostKeyChecking=no tic80/<game>/<game>.tic $MIYOO:/mnt/SDCARD/Roms/TIC/

# Upload all
for g in micro space shadow arena dragon mariolike cyber neon nova; do
    sshpass -p onion scp -o StrictHostKeyChecking=no tic80/$g/$g.tic $MIYOO:/mnt/SDCARD/Roms/TIC/
done
```

## Launch on Miyoo (optional, with --run)

```bash
sshpass -p onion ssh $MIYOO "
killall -9 retroarch MainUI 2>/dev/null; sleep 1
export HOME=/mnt/SDCARD/RetroArch
/mnt/SDCARD/RetroArch/retroarch -L /mnt/SDCARD/RetroArch/.retroarch/cores/tic80_libretro.so /mnt/SDCARD/Roms/TIC/<game>.tic &
"
```

## Kill and restore

```bash
sshpass -p onion ssh $MIYOO "killall -9 retroarch; cd /mnt/SDCARD/.tmp_update && ./runtime.sh &"
```

## Check errors

```bash
sshpass -p onion ssh $MIYOO "grep -i 'error\|empty' /tmp/ra.log | grep -v audio | head -5"
```

## Games
micro, space, shadow, arena, dragon, mariolike, cyber, neon, nova
