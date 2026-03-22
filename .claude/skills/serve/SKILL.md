---
name: serve
description: Start or restart the local web dev server
user_invocable: true
---

Start or restart the local Python HTTP server for browser game testing.

```bash
pkill -f "python.*http.server" 2>/dev/null; sleep 1
cd /home/mo/data/Documents/git/retrogames/web && python3 -m http.server 8000 &
echo "Server running at http://localhost:8000/"
```
