#!/bin/bash
cd "$(dirname "$0")/.."

echo "=== Container Status ==="
docker compose ps
echo ""

echo "=== Tailscale ==="
docker compose exec tailscale tailscale status 2>&1 | head -10 || echo "tailscale container not running"
echo ""

echo "=== Health Check ==="
docker compose exec tailscale wget -qO- --timeout=5 http://127.0.0.1:8080/ 2>&1 | head -3 && echo "OK" || echo "FAIL - app not responding"
echo ""

echo "=== Recent Errors ==="
docker compose logs --tail=20 2>&1 | grep -iE "error|warn|fail" || echo "none"
