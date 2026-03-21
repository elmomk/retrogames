---
name: serve
description: Start or restart the local web dev server
user_invocable: true
---

Start (or restart) the local Python HTTP server for testing browser games.

1. Check if a server is already running:
```bash
pgrep -af "python.*http.server"
```

2. If running, kill it and restart:
```bash
pkill -f "python.*http.server" 2>/dev/null; sleep 1
```

3. Start the server:
```bash
cd /home/mo/data/Documents/git/retrogames/web && python3 -m http.server 8000 &
```

4. Confirm it's running:
```bash
echo "Server running at http://localhost:8000/"
```
