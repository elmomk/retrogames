#!/bin/sh
# Retro Arcade — TIC-80 Cartridge Deployer for Miyoo Mini Plus
# Builds .tic cartridges from Lua sources and deploys via SCP.
#
# Usage:
#   sh install.sh                # build & deploy all games
#   sh install.sh micro          # build & deploy one game
#   sh install.sh --run micro    # build, deploy, and launch via RetroArch
#   sh install.sh --run          # build, deploy all, launch last one
#
# Environment:
#   MIYOO_HOST  — override SSH target (default: onion@192.168.0.63)

set -e

MIYOO_HOST="${MIYOO_HOST:-onion@192.168.0.63}"
MIYOO_PASS="onion"
ROM_DIR="/mnt/SDCARD/Roms/TIC"
RETROARCH="/mnt/SDCARD/RetroArch/retroarch"
CORE="/mnt/SDCARD/RetroArch/.retroarch/cores/tic80_libretro.so"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
TIC80_DIR="$REPO_ROOT/tic80"

RUN_AFTER=false
FILTER=""

# Parse args
while [ $# -gt 0 ]; do
    case "$1" in
        --run) RUN_AFTER=true ;;
        *)     FILTER="$1" ;;
    esac
    shift
done

# Verify build_tic.py exists
BUILD_SCRIPT="$TIC80_DIR/build_tic.py"
if [ ! -f "$BUILD_SCRIPT" ]; then
    echo "ERROR: build_tic.py not found at $BUILD_SCRIPT" >&2
    exit 1
fi
echo "Using builder: $BUILD_SCRIPT"

# Game mapping: short_name:display_name
GAMES="
micro:NanoWizards
space:NeonDefender
shadow:ShadowBlade
arena:ArenaBlitz
dragon:DragonFury
mariolike:PixelKnight
cyber:ChromeViper
neon:NeonRunner
nova:NovaEvader
"

scp_cmd() {
    sshpass -p "$MIYOO_PASS" scp -o StrictHostKeyChecking=no "$@"
}

ssh_cmd() {
    sshpass -p "$MIYOO_PASS" ssh -o StrictHostKeyChecking=no "$MIYOO_HOST" "$@"
}

# Check sshpass is available
if ! command -v sshpass >/dev/null 2>&1; then
    echo "ERROR: sshpass is required. Install it: sudo pacman -S sshpass" >&2
    exit 1
fi

echo "TIC-80 Cartridge Deployer for Miyoo Mini Plus"
echo "Target: ${MIYOO_HOST}:${ROM_DIR}"
echo ""

# Ensure remote ROM dir exists
ssh_cmd "mkdir -p '${ROM_DIR}'"

LAST_TIC=""
BUILT=0
FAILED=0

for entry in $GAMES; do
    [ -z "$entry" ] && continue
    game="$(echo "$entry" | cut -d: -f1)"
    display="$(echo "$entry" | cut -d: -f2)"

    # Filter check
    if [ -n "$FILTER" ] && [ "$FILTER" != "$game" ]; then
        continue
    fi

    echo "--- ${display} (${game}.tic) ---"

    lua_src="${TIC80_DIR}/${game}/${game}.lua"
    if [ ! -f "$lua_src" ]; then
        echo "  SKIP: ${lua_src} not found"
        FAILED=$((FAILED + 1))
        echo ""
        continue
    fi

    # Build .tic cartridge using build_tic.py
    echo "  Building cartridge..."
    build_dir="${TIC80_DIR}/${game}"
    tic_output="${build_dir}/${game}.tic"

    python3 "$BUILD_SCRIPT" "$game"

    if [ ! -f "$tic_output" ]; then
        echo "  FAILED: cartridge not produced"
        FAILED=$((FAILED + 1))
        echo ""
        continue
    fi

    echo "  Built: $(du -h "$tic_output" | cut -f1)"

    # Deploy to Miyoo
    echo "  Deploying to Miyoo..."
    if scp_cmd "$tic_output" "${MIYOO_HOST}:${ROM_DIR}/${game}.tic"; then
        echo "  OK"
        LAST_TIC="${ROM_DIR}/${game}.tic"
        BUILT=$((BUILT + 1))
    else
        echo "  FAILED: SCP error"
        FAILED=$((FAILED + 1))
    fi

    echo ""
done

echo "Deployed: ${BUILT} cartridge(s), failed: ${FAILED}"

# Launch via RetroArch if --run was given
if [ "$RUN_AFTER" = true ] && [ -n "$LAST_TIC" ]; then
    echo ""
    echo "Launching ${LAST_TIC} via RetroArch..."
    ssh_cmd "killall retroarch 2>/dev/null; sleep 0.5; ${RETROARCH} -L ${CORE} '${LAST_TIC}' &"
    echo "Game launched."
fi
