---
name: deploy
description: Build and deploy the retrogames arcade via Docker Compose with Tailscale
user_invocable: true
args: "[--rebuild|--restart|--down]"
---

Deploy the retrogames arcade using Docker Compose with Tailscale networking.

Run the deploy script:
```bash
cd /home/mo/data/Documents/git/retrogames && ./scripts/deploy.sh $ARGUMENTS
```

Options:
- No args: Full build and deploy
- `--restart`: Restart containers without rebuilding
- `--down`: Tear down all containers
