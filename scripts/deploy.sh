#!/bin/bash
set -e
cd "$(dirname "$0")/.."

case "${1:-}" in
  --restart)
    echo "==> Restarting containers..."
    docker compose restart
    ;;
  --down)
    echo "==> Tearing down..."
    docker compose down
    ;;
  *)
    echo "==> Building image..."
    docker compose build --no-cache
    echo "==> Starting containers..."
    docker compose up -d
    ;;
esac

echo ""
docker compose ps
echo ""
echo "==> Tailscale status:"
docker compose logs --tail=5 tailscale 2>&1 | grep -iE "Startup complete|Running|error|serve" || true
