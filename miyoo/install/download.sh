#!/bin/sh
# Download Miyoo binaries from GitHub Releases to ./bin/
#
# Usage:
#   sh download.sh                    # download all
#   sh download.sh micro_miyoo        # download one
#
# Environment:
#   RETRO_TAG=v2.2.0  — pin to a specific release (default: latest)

set -e

REPO="elmomk/retrogames"
TAG="${RETRO_TAG:-latest}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
BIN_DIR="${SCRIPT_DIR}/bin"

BINARIES="micro_miyoo space_miyoo shadow_miyoo arena_miyoo dragon_miyoo mariolike_miyoo cyber_miyoo neon_miyoo"

# Resolve latest tag
if [ "$TAG" = "latest" ]; then
    echo "Fetching latest release tag..."
    TAG=$(wget -qO- "https://api.github.com/repos/${REPO}/releases/latest" 2>/dev/null | grep '"tag_name"' | head -1 | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')
    if [ -z "$TAG" ]; then
        # fallback with curl
        TAG=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | head -1 | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')
    fi
    if [ -z "$TAG" ]; then
        echo "ERROR: Could not determine latest release tag." >&2
        exit 1
    fi
fi

BASE_URL="https://github.com/${REPO}/releases/download/${TAG}"
mkdir -p "$BIN_DIR"

# Filter to specific binary if provided
if [ -n "$1" ]; then
    BINARIES="$1"
fi

echo "Downloading Retro Arcade ${TAG} binaries to ${BIN_DIR}"
echo ""

for binary in $BINARIES; do
    echo "  ${binary}..."
    if wget -q -O "${BIN_DIR}/${binary}" "${BASE_URL}/${binary}" 2>/dev/null || \
       curl -sL -o "${BIN_DIR}/${binary}" "${BASE_URL}/${binary}" 2>/dev/null; then
        chmod +x "${BIN_DIR}/${binary}"
        echo "  OK ($(wc -c < "${BIN_DIR}/${binary}") bytes)"
    else
        echo "  FAILED"
        rm -f "${BIN_DIR}/${binary}"
    fi
done

echo ""
echo "Done. Binaries in ${BIN_DIR}/"
ls -lh "$BIN_DIR"/
