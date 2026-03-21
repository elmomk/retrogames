---
name: test-site
description: Run Playwright smoke tests against the retrogames deployment to verify all pages load correctly
user_invocable: true
args: "[base-url]"
---

Run Playwright browser smoke tests against the retrogames arcade.

```bash
cd /home/mo/data/Documents/git/retrogames && ./scripts/test.sh $ARGUMENTS
```

Arguments:
- Base URL override (default: http://localhost:8080)

After the test, view any screenshots at `/tmp/retrogames-test/` for visual verification.
If failures occur, read the relevant game's `index.html` to diagnose.
