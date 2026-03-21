---
name: logs
description: View Docker container logs for the retrogames deployment
user_invocable: true
args: "[app|tailscale] [lines]"
---

Show logs from the retrogames Docker containers.

```bash
cd /home/mo/data/Documents/git/retrogames && ./scripts/logs.sh $ARGUMENTS
```

Arguments:
- Service: `app`, `tailscale`, or omit for all
- Lines: number of log lines (default: 50)

After showing logs, briefly summarize any errors or warnings found.
