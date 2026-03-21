# Skills & Agents Guide

*A guide to the Claude Code automations that power the retrogames development*
*workflow.*

---

## What Are Claude Code Skills and Agents?

Claude Code supports two types of automation that extend its capabilities within
a project:

**Skills** are slash-commands that a developer invokes directly. They are
procedural recipes: "when I say `/deploy`, run this script and report the
results." Skills live in `.claude/skills/<name>/SKILL.md`.

**Agents** are specialized sub-agents that Claude Code can delegate to. They
have focused expertise and limited tool access. When Claude Code encounters a
task that matches an agent's specialty, it can hand the task off entirely.
Agents live in `.claude/agents/<name>.md`.

The distinction: you invoke a skill explicitly with a slash command. An agent is
invoked by Claude Code itself when it determines delegation would be more
effective.

---

## Available Skills

### /deploy -- Build and Deploy

**Purpose:** Build the Docker image and deploy the full stack (Tailscale +
busybox httpd).

**Usage:**
```
/deploy              # Full build and deploy
/deploy --restart    # Restart containers without rebuilding
/deploy --down       # Tear down all containers
```

**What it does:**
1. Runs `./scripts/deploy.sh` with the provided arguments
2. Reports container status and Tailscale connectivity

**When to use:** After modifying any files in `web/`, after updating
`docker-compose.yml` or `ts-serve.json`, or when containers need restarting.

---

### /status -- Check Deployment Health

**Purpose:** Get a complete health report of the running deployment.

**Usage:**
```
/status
```

**What it does:**
1. Runs `./scripts/status.sh`
2. Reports container status, Tailscale connectivity, HTTP health check, and
   recent errors

**When to use:** To verify the deployment is healthy, or as a first step when
debugging issues.

---

### /logs -- View Container Logs

**Purpose:** View Docker container logs for troubleshooting.

**Usage:**
```
/logs                    # All containers, last 50 lines
/logs app 100            # App container, last 100 lines
/logs tailscale 20       # Tailscale container, last 20 lines
```

**What it does:**
1. Runs `./scripts/logs.sh` with the provided arguments
2. Summarizes any errors or warnings found in the output

**When to use:** When something is not working and you need to see what the
containers are doing.

---

### /test-site -- Run Playwright Smoke Tests

**Purpose:** Verify all game pages load correctly in a browser.

**Usage:**
```
/test-site                       # Test against localhost:8080
/test-site http://localhost:8000 # Test against dev server
```

**What it does:**
1. Runs `./scripts/test.sh` with Playwright
2. Loads every game page and the launcher
3. Checks for JavaScript errors
4. Takes screenshots to `/tmp/retrogames-test/`
5. Reports pass/fail per page

**When to use:** After modifying any game's `index.html`, after adding a new
game, or after deployment to verify everything is working.

---

### /check-rust -- Verify Miyoo Ports Compile

**Purpose:** Run `cargo check` on all Miyoo Rust ports and report results.

**Usage:**
```
/check-rust
```

**What it does:**
1. Runs `./scripts/check-rust.sh`
2. Reports pass/fail for each game
3. If any game fails, reads the error output, diagnoses the issue, and applies
   fixes automatically
4. Re-runs until all games compile clean

**Common fixes applied automatically:**
- E0499 (borrow checker): Converts `iter_mut()` loops to index-based loops
- E0689 (float ambiguity): Adds `: f32` type annotations
- E0004 (non-exhaustive match): Adds missing enum variants to match blocks

**When to use:** After modifying any `miyoo/*/src/main.rs` file, or after
adding a new Miyoo port.

---

### /build-miyoo -- Build Miyoo Binaries

**Purpose:** Compile Miyoo Rust ports as native desktop binaries or
cross-compile for ARM.

**Usage:**
```
/build-miyoo                    # Build all, native
/build-miyoo micro              # Build one game, native
/build-miyoo all --arm          # Cross-compile all for Miyoo hardware
/build-miyoo dragon --arm       # Cross-compile one game
```

**What it does:**
1. Runs `./scripts/build-miyoo.sh` with the provided arguments
2. Reports build status and binary locations

**When to use:** After `cargo check` passes and you want actual binaries for
testing or deployment.

---

### /new-game -- Scaffold a New Game

**Purpose:** Create a complete new game with both web and Miyoo versions from
a concept description.

**Usage:**
```
/new-game <name> <genre> <description>
/new-game runner "Endless Runner" "Auto-running character dodging obstacles"
```

**What it does:**
1. Creates `web/<name>/spec.md` with a technical specification
2. Creates `web/<name>/index.html` with a complete, playable game including:
   - Press Start 2P font, 640x480 canvas, 60fps fixed timestep
   - Procedural pixel-art sprites
   - Touch controls + keyboard input
   - Web Audio API sound effects
   - Story with typewriter text
   - Title, gameplay, game over, and victory screens
3. Adds a game card to `web/index.html` launcher
4. Creates `miyoo/<name>/Cargo.toml` and `src/main.rs`
5. Runs `cargo check` and fixes any compilation errors
6. Updates `CLAUDE.md` game table if needed

**When to use:** When you want to add a new game to the collection.

---

### /polish -- Add Visual Effects

**Purpose:** Add "game juice" and visual polish to an existing game.

**Usage:**
```
/polish micro              # Polish web + Miyoo versions
/polish dragon web         # Polish only the web version
/polish shadow miyoo       # Polish only the Miyoo port
```

**What it does:**
Analyzes the game and adds missing visual effects:
- Screen shake on hits/deaths
- Hit stop/freeze frames on impacts
- CRT scanline overlay and vignette
- Particle explosions with varied colors
- Dash/movement afterimage trails
- Floating damage/score popups
- Combo/kill streak text
- Ambient dust/ember particles
- Background weather effects

**When to use:** After a game is functionally complete but feels "flat" or
lacks visual feedback.

---

### /serve -- Start Local Dev Server

**Purpose:** Start or restart the Python HTTP server for testing web games
locally.

**Usage:**
```
/serve
```

**What it does:**
1. Checks if a server is already running
2. Kills any existing server
3. Starts `python3 -m http.server 8000` in the `web/` directory
4. Confirms the server is accessible

**When to use:** Before testing web games in a browser during development.

---

### /ts-debug -- Debug Tailscale Issues

**Purpose:** Diagnose and fix Tailscale connectivity issues.

**Usage:**
```
/ts-debug
```

**What it does:**
1. Checks container health
2. Examines Tailscale logs for auth/connection errors
3. Verifies Tailscale status and serve configuration
4. Tests internal proxy path
5. Checks auth key validity
6. Reports findings with specific fix recommendations

**Common issues diagnosed:**
- Expired auth key (with link to admin console)
- Serve proxy misconfiguration
- Missing network capabilities (NET_ADMIN)
- App not reachable internally

**When to use:** When the deployment is running but not accessible via
Tailscale hostname.

---

## Available Agents

### game-builder

**Specialty:** Creating complete retro games from a concept description.

**Tools:** Read, Write, Edit, Glob, Grep, Bash

**What it does:** Given a game concept, creates:
1. Web version (`web/<game>/index.html`) with all required patterns
2. Technical spec (`web/<game>/spec.md`)
3. Miyoo port (`miyoo/<game>/Cargo.toml` + `src/main.rs`)
4. Launcher card in `web/index.html`
5. Verifies compilation with `cargo check`

**Invoked by:** Claude Code when asked to "create a new game" or "build a game
about X."

---

### rust-fixer

**Specialty:** Fixing Rust compilation errors in Miyoo game ports.

**Tools:** Read, Edit, Bash, Grep

**Workflow:**
1. Run `cargo check` to get errors
2. Read the relevant code sections
3. Apply fixes using Edit
4. Re-run `cargo check`
5. Repeat until clean

**Known patterns:**
- E0499 (borrow checker): Converts to index-based loops or deferred side effects
- E0689 (float ambiguity): Adds `: f32` type annotations
- E0004 (non-exhaustive match): Adds missing variants to all match blocks
- Unclosed delimiters: Checks indentation around reported line

**Invoked by:** Claude Code when `cargo check` fails and the error is in a
Miyoo game port.

---

### story-writer

**Specialty:** Creating narratives for retro games and implementing them in
code.

**Tools:** Read, Edit, Write, Grep

**Design principles:**
- Every game needs a twist that recontextualizes the gameplay
- Environmental storytelling through floating text in levels
- Intercepted transmissions, memos, or dialogue for world-building
- Character development through brief, punchy lines
- Endings should be emotionally resonant

**Implementations:**
- Web: Story text arrays, typewriter effect, STORY game state, color-coded text
- Miyoo: Story/BossIntro enum variants, static string slices, typewriter
  rendering

**Invoked by:** Claude Code when asked to "add a story" or "write narrative"
for a game.

---

### docker-ops

**Specialty:** Managing the Docker deployment stack.

**Tools:** Read, Bash, Grep, Edit

**Knowledge:**
- Stack architecture: Tailscale sidecar + busybox httpd app
- Key files: Dockerfile, docker-compose.yml, ts-serve.json, .env
- All operational scripts in `scripts/`
- Troubleshooting patterns: logs first, then status, then config

**Invoked by:** Claude Code when asked about deployment issues, container
problems, or Docker configuration.

---

### tailscale-debug

**Specialty:** Debugging Tailscale networking issues.

**Tools:** Read, Bash, Grep

**Debug workflow:**
1. Container health check
2. Tailscale logs examination
3. Tailscale status verification
4. Serve config validation
5. Internal connectivity test
6. Auth key verification
7. DNS and certificate checks

**Common fixes:**
- Expired auth key: Regenerate and update .env
- Serve not working: Verify ts-serve.json format and mount
- App not reachable: Check network_mode configuration

**Invoked by:** Claude Code when Tailscale-specific networking issues are
suspected.

---

### playwright-test

**Specialty:** Running browser tests against the deployment.

**Tools:** Read, Write, Edit, Bash, Grep, Glob

**Environment notes:**
- Requires `NODE_PATH=/home/mo/.npm/_npx/e41f203b7505f1fb/node_modules`
- Uses trailing-slash URLs (required by launcher redirect)
- Tests against `http://localhost:8080/` (Docker) or `:8000` (dev server)

**Test coverage:**
- Page loads without JavaScript errors
- Canvas element exists and has dimensions
- Title/start screen renders
- Screenshots captured for visual verification

**Invoked by:** Claude Code when asked to test the site, verify a game works,
or check for JavaScript errors.

---

## The Scripts Directory

All operational scripts live in `scripts/` and are invoked by skills:

```
scripts/
+-- deploy.sh        Runs: docker compose build + up
|                    Args: [--restart|--down]
|                    Used by: /deploy skill, docker-ops agent
|
+-- status.sh        Runs: docker compose ps, tailscale status,
|                           health check, error scan
|                    Used by: /status skill
|
+-- logs.sh          Runs: docker compose logs
|                    Args: [service] [line-count]
|                    Used by: /logs skill, docker-ops agent
|
+-- test.sh          Runs: Playwright browser smoke tests
|                    Args: [base-url]
|                    Used by: /test-site skill, playwright-test agent
|
+-- check-rust.sh    Runs: cargo check on all miyoo/* ports
|                    Reports: pass/fail per game
|                    Used by: /check-rust skill, rust-fixer agent
|
+-- build-miyoo.sh   Runs: cargo build (native or ARM cross-compile)
|                    Args: [game|all] [--native|--arm]
|                    Used by: /build-miyoo skill
```

### Script Design Principles

1. **All scripts `cd` to the project root first:**
   ```bash
   cd "$(dirname "$0")/.."
   ```
   This means they work regardless of where you invoke them from.

2. **Scripts are idempotent.** Running them twice produces the same result.

3. **Scripts report their own results.** No parsing required -- the output is
   human-readable.

4. **Scripts exit non-zero on failure.** This makes them safe to chain with
   `&&` or use in CI/CD.

---

## Creating New Skills

### Skill File Structure

Create a new skill at `.claude/skills/<name>/SKILL.md`:

```markdown
---
name: my-skill
description: One-line description of what this skill does
user_invocable: true
args: "[optional-args-description]"
---

Instructions for Claude Code to follow when this skill is invoked.

Include:
1. What to check first
2. What commands to run
3. How to interpret the output
4. What to do if something goes wrong
```

### Skill Design Guidelines

- **One skill, one task.** A skill should do one thing well.
- **Wrap scripts.** If the skill runs a shell command, put it in `scripts/` and
  have the skill call the script. This lets the script be used independently.
- **Report results.** The skill should always end with a summary for the user.
- **Handle errors.** Include fallback instructions for common failure modes.

---

## Creating New Agents

### Agent File Structure

Create a new agent at `.claude/agents/<name>.md`:

```markdown
---
name: my-agent
description: Multi-sentence description of the agent's specialty
tools:
  - Read
  - Edit
  - Bash
  - Grep
---

You are a [specialty] expert for the retrogames project.

## Your Knowledge
- What you know about
- Relevant files and patterns
- Common issues and fixes

## Your Workflow
1. First, do this
2. Then, check this
3. Finally, verify that

## Common Patterns
Detailed guidance for specific scenarios.
```

### Agent Design Guidelines

- **Limit tools.** Only grant the tools the agent actually needs. A debugging
  agent does not need Write. A writer does not need Bash.
- **Be specific.** The description should make it obvious when this agent is
  the right one to delegate to.
- **Include domain knowledge.** The agent markdown should contain enough
  context that the agent can work without reading CLAUDE.md.
- **Document workflows.** Step-by-step procedures prevent the agent from going
  off-track.

---

## Skill-Agent Relationship

```
User invokes /check-rust
        |
        v
    check-rust SKILL
    (runs check-rust.sh)
        |
        +-- All pass --> Report "all clean"
        |
        +-- Some fail --> Delegate to rust-fixer AGENT
                          |
                          v
                      Read errors
                      Read source code
                      Apply fixes
                      Re-run cargo check
                      Report results
```

Skills are the entry point. Agents are the specialists. The skill handles the
happy path (everything works) and delegates the unhappy path (something broke)
to the relevant agent.

---

## Quick Reference

| Command | Skill | Script | Agent |
|---|---|---|---|
| Deploy | `/deploy` | `deploy.sh` | docker-ops |
| Status check | `/status` | `status.sh` | docker-ops |
| View logs | `/logs` | `logs.sh` | docker-ops |
| Browser test | `/test-site` | `test.sh` | playwright-test |
| Rust check | `/check-rust` | `check-rust.sh` | rust-fixer |
| Build binaries | `/build-miyoo` | `build-miyoo.sh` | -- |
| New game | `/new-game` | -- | game-builder |
| Add polish | `/polish` | -- | -- |
| Dev server | `/serve` | -- | -- |
| Tailscale debug | `/ts-debug` | -- | tailscale-debug |
| Write story | -- | -- | story-writer |

---

## Cross-References

- [Architecture](architecture.md) -- How skills and agents fit in the project
- [Deployment Guide](deployment-guide.md) -- What the `/deploy`, `/status`,
  and `/logs` skills automate
- [Adding New Games](adding-games.md) -- The `/new-game` skill in detail
- [Miyoo Porting Guide](miyoo-porting-guide.md) -- What the `/check-rust` and
  `/build-miyoo` skills verify
