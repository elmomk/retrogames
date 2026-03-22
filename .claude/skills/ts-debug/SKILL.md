---
name: ts-debug
description: Debug Tailscale connectivity issues in the retrogames deployment
user_invocable: true
---

Diagnose Tailscale connectivity for the retrogames deployment.

Run these diagnostics in sequence:
```bash
cd /home/mo/data/Documents/git/retrogames
docker compose ps
docker compose logs --tail=30 tailscale
docker compose exec tailscale tailscale status 2>&1
docker compose exec tailscale tailscale serve status
docker compose exec tailscale wget -qO- --timeout=5 http://127.0.0.1:8080/ 2>&1 | head -5
grep -c TS_AUTHKEY .env
```

Report: auth status, serve proxy status, app reachability, and fix recommendations.

Common fixes: expired auth key (regenerate + update .env), serve misconfigured (check ts-serve.json), app unreachable (check network_mode).
