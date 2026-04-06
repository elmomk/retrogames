---
name: deploy-miyoo
description: Deploy ChaiLove games to the Miyoo Mini Plus, kill RetroArch, upload, relaunch
user_invocable: true
args: "<game-name> [--run]"
---

Deploy a ChaiLove game to the Miyoo Mini Plus via SCP. Optionally launch it immediately.

## Steps

1. Kill any running RetroArch on Miyoo
2. Upload game files to `/mnt/SDCARD/Roms/CHAILOVE/<game>/`
3. If `--run` flag is present, kill MainUI and launch via RetroArch

```bash
MIYOO="onion@192.168.0.63"
GAME="$ARGUMENTS"
RUN=false

# Parse args
for arg in $ARGUMENTS; do
    if [ "$arg" = "--run" ]; then
        RUN=true
    else
        GAME="$arg"
    fi
done

GAME_DIR="/home/mo/data/Documents/git/retrogames/chailove/$GAME"
REMOTE_DIR="/mnt/SDCARD/Roms/CHAILOVE/$GAME"

if [ ! -d "$GAME_DIR" ]; then
    echo "Game not found: $GAME_DIR"
    exit 1
fi

# Kill existing RetroArch
sshpass -p onion ssh -o StrictHostKeyChecking=no "$MIYOO" "killall -9 retroarch 2>/dev/null" 2>&1 | grep -v "WARNING\|post-quantum"

# Upload
sshpass -p onion ssh -o StrictHostKeyChecking=no "$MIYOO" "mkdir -p $REMOTE_DIR" 2>&1 | grep -v "WARNING\|post-quantum"
sshpass -p onion scp -r -o StrictHostKeyChecking=no "$GAME_DIR"/* "$MIYOO:$REMOTE_DIR/" 2>&1 | grep -v "WARNING\|post-quantum"
echo "Uploaded $GAME to Miyoo"

if [ "$RUN" = true ]; then
    sshpass -p onion ssh -o StrictHostKeyChecking=no "$MIYOO" "
    killall MainUI 2>/dev/null
    sleep 1
    export HOME=/mnt/SDCARD/RetroArch
    /mnt/SDCARD/RetroArch/retroarch -v -L /mnt/SDCARD/RetroArch/.retroarch/cores/chailove_libretro.so $REMOTE_DIR/main.chai > /tmp/ra.log 2>&1 &
    echo 'Launched PID:' \$!
    " 2>&1 | grep -v "WARNING\|post-quantum\|store now\|upgraded\|pq.html"
fi
```

After deploying, check for errors:
```bash
sshpass -p onion ssh -o StrictHostKeyChecking=no onion@192.168.0.63 "grep -i 'Failed to call\|Error' /tmp/ra.log | grep -v 'reset\|joystick\|mouse\|key\|state\|cheat\|save\|exit\|load\|audio' | head -10"
```

To kill and restore MainUI:
```bash
sshpass -p onion ssh -o StrictHostKeyChecking=no onion@192.168.0.63 "killall -9 retroarch 2>/dev/null; cd /mnt/SDCARD/.tmp_update && ./runtime.sh &"
```

## Available games
test, micro (more coming after micro is validated)
