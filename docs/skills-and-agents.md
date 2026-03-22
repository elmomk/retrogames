# Claude Code: Skills, Agents, Commands, and Scripts

*A comprehensive guide to extending Claude Code with custom automations.*

---

## The Mental Model

Claude Code is an AI coding assistant that runs in your terminal. Out of the
box, it can read files, edit code, run commands, and search your codebase. But
every project has its own workflows — deploying, testing, building, reviewing.
Claude Code provides three extension mechanisms to encode those workflows:

```
┌─────────────────────────────────────────────┐
│                  You (the human)            │
│                                             │
│   "deploy the app"    "/check-rust"         │
│         │                   │               │
│         ▼                   ▼               │
│   ┌───────────┐    ┌──────────────┐         │
│   │   Agent   │    │    Skill     │         │
│   │ (Claude   │    │ (slash cmd   │         │
│   │  decides) │    │  you invoke) │         │
│   └─────┬─────┘    └──────┬───────┘         │
│         │                 │                 │
│         ▼                 ▼                 │
│   ┌──────────────────────────────┐          │
│   │       Shell Scripts          │          │
│   │   (the actual work)         │          │
│   └──────────────────────────────┘          │
└─────────────────────────────────────────────┘
```

**Skills** = slash commands you invoke explicitly (`/deploy`, `/check-rust`)
**Agents** = specialists Claude delegates to automatically
**Commands** = simpler precursors to skills (legacy, but still work)
**Scripts** = shell scripts that do the actual work

---

## Skills

### What they are

A skill is a slash command. When you type `/deploy` in Claude Code, it reads
the skill file and follows the instructions. Skills are the primary way to
encode repeatable workflows.

### Where they live

```
.claude/skills/<name>/SKILL.md     # Project-level (checked into git)
~/.claude/skills/<name>/SKILL.md   # User-level (available everywhere)
```

User-level skills take priority over project-level skills with the same name.

### Anatomy of a skill

```markdown
---
name: deploy
description: Build and deploy the app via Docker Compose
args: "[--rebuild|--restart|--down]"
---

Deploy the app. Action: $ARGUMENTS (default: full build).

Run: `./scripts/deploy.sh $ARGUMENTS`

If the deploy fails, show error logs and suggest a fix.
```

The frontmatter defines metadata:
- **name**: the slash command name (you'll type `/deploy`)
- **description**: shown in skill listings and used for routing
- **args** or **argument-hint**: documents expected arguments

The body is instructions for Claude Code to follow when the skill is invoked.
`$ARGUMENTS` is replaced with whatever the user typed after the slash command.

### Writing effective skills

**Good skill — wraps a script:**
```markdown
Run: `./scripts/deploy.sh $ARGUMENTS`
Report the result. If it fails, show logs and suggest a fix.
```

**Bad skill — inline logic:**
```markdown
1. Run `docker compose build app`
2. Run `docker compose up -d`
3. Run `docker compose exec tailscale tailscale status`
4. Run `curl -s -o /dev/null -w "%{http_code}" https://myapp.ts.net/`
5. If the status code is not 200, run `docker compose logs app --tail 50`
6. ...
```

The bad version has the deploy logic scattered across the skill file. If you
want to deploy from your terminal without Claude Code, you can't — the
knowledge is trapped in the skill. The good version wraps a script that works
anywhere.

**Delegation over duplication:**

If a skill's job is to invoke an agent, say so directly:

```markdown
# BAD — copies the entire agent prompt into the skill
Launch a general-purpose agent with this prompt:
> You are a UI/UX reviewer... [65 lines of duplicated instructions]

# GOOD — delegates to the existing agent
Use the `uiux-designer` agent to review the page at $ARGUMENTS.
```

---

## Agents

### What they are

Agents are specialized sub-processes that Claude Code delegates to. When you
ask Claude Code to "fix the Rust compile errors," it can spawn a `rust-fixer`
agent with focused expertise and limited tools. The agent works autonomously
and reports back.

The key difference from skills: you don't invoke agents with a slash command.
Claude Code decides when to use them based on the task and the agent's
description.

### Where they live

```
.claude/agents/<name>.md              # Project-level
.claude/agents/<name>/AGENT.md        # Project-level (directory format)
~/.claude/agents/<name>.md            # User-level (available everywhere)
```

Project-level agents take priority over user-level agents with the same name.
This means you can have a generic `docker-ops` agent at user-level and override
it with a project-specific version if needed.

### Anatomy of an agent

```markdown
---
name: rust-fixer
description: Fixes Rust compile errors in Miyoo game ports.
tools:
  - Read
  - Edit
  - Bash
  - Grep
model: haiku
---

You fix Rust compilation errors under `miyoo/*/src/main.rs`.

## Workflow
1. Run `cargo check` to get errors
2. Read the relevant code
3. Fix using Edit
4. Re-run `cargo check`
5. Repeat until clean

## Common patterns
- E0499 (borrow checker): use index-based loops
- E0689 (float ambiguity): add `: f32` annotation
- E0004 (non-exhaustive match): add missing variants
```

The frontmatter defines:
- **name**: identifier, also used for delegation
- **description**: Claude Code reads this to decide when to use the agent
- **tools**: which tools the agent can access (principle of least privilege)
- **model**: which Claude model to use (haiku/sonnet/opus)

### Writing effective agents

**1. Constrain, don't teach**

The model already knows how to write Rust, JavaScript, SQL, and Playwright
scripts. Your agent prompt should contain what the model *can't derive on its
own*:

- Project-specific file paths and conventions
- Constraints that would be surprising ("never use `iter_mut()` here")
- Environment details (where Firefox is installed, what port the app uses)

```markdown
# BAD — teaching the model to write Rust
fn create_sprite(art: &[&str], colors: &[Color]) -> Texture2D {
    let width = art[0].len() as u16;
    // ... 20 lines of code the model could write itself
}

# GOOD — stating the constraint
- Sprites: character array → Image → Texture2D with FilterMode::Nearest
```

**2. Rules over examples**

State the rule, not a specific instance. Rules are compact and generalize.
Examples are long and cover one case.

```markdown
# BAD — 8-line example of index-based loops
for i in 0..self.enemies.len() {
    let ex = self.enemies[i].x;
    let ey = self.enemies[i].y;
    // ...
}

# GOOD — the rule in one line
- Index-based loops: `for i in 0..self.vec.len()` when body calls self methods
```

Exception: include a brief example when the pattern is genuinely
counterintuitive and the model would get it wrong without seeing it.

**3. Match the model to the task**

| Model | Cost | Speed | Use for |
|-------|------|-------|---------|
| haiku | $ | Fast | Mechanical tasks: run commands, report results, apply known patterns |
| sonnet | $$ | Medium | Reasoning tasks: code review, porting, refactoring |
| opus | $$$$ | Slow | Deep analysis: security review, architecture decisions |

A build-checker that runs `cargo check` and reports pass/fail doesn't need
Opus. A security reviewer where missing a vulnerability has real consequences
does.

**4. Minimize tools**

Only grant tools the agent actually needs:
- Debugging agent: Read, Bash, Grep (no Write/Edit — it shouldn't change code)
- Fixer agent: Read, Edit, Bash, Grep (needs to change code)
- Builder agent: Read, Write, Edit, Bash, Grep, Glob (creates new files)

Fewer tools = less risk of unintended actions.

---

## Commands

Commands are the predecessor to skills. They're simpler — just a markdown file
with instructions, no frontmatter metadata.

```
.claude/commands/<name>.md
```

```markdown
Build the app for production.

Run: `./scripts/build.sh`

If the build fails, analyze the errors and suggest fixes.
```

Commands work but skills are preferred because:
- Skills have structured metadata (description, arguments, tool restrictions)
- Skills support argument hints shown in autocomplete
- Skills have a richer feature set (allowed-tools, disable-model-invocation)

If you have both a command and a skill with the same name, you'll get
duplication. Pick one — prefer skills.

---

## User-Level vs Project-Level

Claude Code supports two scopes for agents and skills:

### Project-level (`.claude/`)

```
your-repo/.claude/agents/    # Agents specific to this project
your-repo/.claude/skills/    # Skills specific to this project
```

- Checked into git, shared with your team
- Contains project-specific knowledge (your file structure, conventions, URLs)
- **Agents override user-level** agents with the same name
- **Skills are overridden by user-level** skills with the same name

### User-level (`~/.claude/`)

```
~/.claude/agents/    # Agents available in ALL projects
~/.claude/skills/    # Skills available in ALL projects
```

- Not checked into any repo, personal to you
- Contains generic knowledge that works across projects
- Good candidates: docker-ops, tailscale-debug, build-checker, code reviewer

### What goes where?

| Agent/Skill | Scope | Why |
|------------|-------|-----|
| docker-ops | User-level | Same Docker+Tailscale pattern across all repos |
| build-checker | User-level | `cargo check && clippy && test` is universal |
| security-reviewer | User-level | Security checklist applies everywhere |
| /review | User-level | Code review is project-agnostic |
| game-builder | Project-level | Only makes sense for retrogames |
| rust-fixer | Project-level | Knows retrogames-specific Rust patterns |
| migration-writer | Project-level | Knows this project's PostgreSQL schema |
| /deploy | Project-level | Each project deploys differently |
| uiux-designer | Project-level | Each project has its own design language |

The principle: **if it mentions project-specific files, URLs, or conventions,
it's project-level. If it would work in any Rust/Docker/web project, it's
user-level.**

---

## Why Scripts Beat Inline Commands

The single most important pattern: **skills should wrap scripts, not contain
inline command sequences.**

### The anti-pattern

```markdown
# deploy skill
1. Run `docker compose build app`
2. Run `docker compose up -d`
3. Wait 5 seconds
4. Run `docker compose exec tailscale tailscale status`
5. Run `curl -s https://myapp.ts.net/`
6. If status is not 200, run `docker compose logs app --tail 50`
```

### The pattern

```bash
# scripts/deploy.sh
#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")/.."

docker compose build app
docker compose up -d
sleep 5

echo "=== Tailscale Status ==="
docker compose exec tailscale tailscale status

echo "=== Health Check ==="
status=$(curl -s -o /dev/null -w "%{http_code}" https://myapp.ts.net/)
if [ "$status" != "200" ]; then
    echo "UNHEALTHY (HTTP $status)"
    docker compose logs app --tail 50
    exit 1
fi
echo "HEALTHY (HTTP 200)"
```

```markdown
# deploy skill
Run: `./scripts/deploy.sh $ARGUMENTS`
Report the result. If it fails, show logs and suggest a fix.
```

### Why this is better — seven reasons

**1. Scripts work without Claude Code**

Your deploy script works from a plain terminal, in CI/CD, from SSH, from a
cron job. The inline version only works inside Claude Code. You're not locked
into one tool.

**2. Scripts are testable**

You can run `./scripts/deploy.sh` manually and see if it works. You can't
"test" a skill prompt without invoking Claude Code. When something breaks at
2am, you want `bash scripts/deploy.sh`, not an AI conversation.

**3. Scripts are versionable**

`git diff scripts/deploy.sh` shows exactly what changed in your deploy
process. `git diff .claude/skills/deploy/SKILL.md` shows a prompt changed,
but the actual behavioral change is buried in natural language.

**4. Scripts are debuggable**

```bash
bash -x scripts/deploy.sh    # trace every command
```

You can't `bash -x` a skill prompt.

**5. Scripts don't consume context**

Every line in a skill file is loaded into Claude Code's context window when
the skill is invoked. A 50-line inline command sequence wastes 50 lines of
context on commands the model could have read from a script. A 2-line skill
that calls a script wastes 2 lines.

**6. Scripts compose**

```bash
./scripts/build.sh && ./scripts/deploy.sh && ./scripts/test.sh
```

You can chain scripts. You can call one script from another. Skills can't
call other skills.

**7. Scripts have proper error handling**

```bash
set -euo pipefail    # exit on error, undefined vars, pipe failures
```

Inline commands in a skill prompt rely on Claude Code to handle errors, which
is best-effort. A script with `set -e` will stop immediately on failure.

### Script design principles

1. **Start with `cd "$(dirname "$0")/.."`.** Scripts work from any directory.
2. **Use `set -euo pipefail`.** Fail loudly on errors.
3. **Accept arguments.** `$1`, `$2`, etc. for flexibility.
4. **Print clear output.** Section headers, pass/fail, actionable messages.
5. **Exit non-zero on failure.** Makes chaining and CI/CD work.
6. **Be idempotent.** Running twice produces the same result.

---

## Prompt Engineering for Agents

Writing agent prompts is different from writing documentation. The audience is
an LLM, not a human. Here are the principles:

### Every line must earn its place

Ask: "Would the agent get this wrong without this line?" If no, cut it.

The model knows how to write Playwright scripts. It doesn't know where your
Firefox binary is installed. Include the path, skip the tutorial.

### Descriptions are routing signals

The description's job is to help Claude Code decide *when to use this agent*.
It's not a user manual.

```markdown
# BAD — 200 characters of examples and trigger phrases
description: "Use this agent when the user wants to refactor Rust code
for better quality... Examples:\n- User: 'This code feels outdated'..."

# GOOD — 80 characters that cover the key verbs
description: Refactor Rust code for quality, update deps, modernize idioms.
```

### Don't duplicate the codebase

The agent can read files. Don't copy function implementations into the prompt.
State the rules and constraints; let the agent read the actual code.

```markdown
# BAD — copying a 20-line function into the prompt
fn create_sprite(art: &[&str], colors: &[Color]) -> Texture2D { ... }

# GOOD — stating the pattern name so the agent can find it
- Sprites use the `create_sprite()` pattern in existing ports — read one first
```

### Reference, don't repeat

If information exists in CLAUDE.md, don't repeat it in every agent. Say
"Read CLAUDE.md for project conventions" and move on.

### Structure for scanning, not reading

Agents don't read prompts linearly like humans. Use:
- **Headers** for categories (`## Workflow`, `## Rules`, `## Common errors`)
- **Tables** for lookup data (symptom → fix)
- **Bullet lists** for rules (not paragraphs)
- **Bold** for the key term in each bullet

---

## Patterns and Anti-Patterns

### Pattern: Skill → Agent escalation

```
/check-rust
    │
    ├── All pass → Report "all clean"
    │
    └── Some fail → Delegate to rust-fixer agent
                         │
                         └── Fix → Re-check → Report
```

The skill handles the happy path. The agent handles the unhappy path. This
keeps the skill simple and the agent focused.

### Pattern: User-level generic + project-level override

```
~/.claude/agents/docker-ops.md          # Generic Docker troubleshooting
my-project/.claude/agents/docker-ops.md # Project-specific overrides
```

The project-level agent overrides the user-level one (for agents, project
wins). This lets you have a generic baseline with project-specific knowledge.

### Anti-pattern: Skill that duplicates an agent

```markdown
# The skill
Launch an agent with this prompt:
> You are a UI/UX reviewer...
> [entire agent prompt copy-pasted, 65 lines]
```

If the agent already exists, the skill should just say "use the agent." The
duplicated prompt doubles context usage and will drift out of sync.

### Anti-pattern: Agent that teaches the model to code

```markdown
# 40 lines of "how to write Rust"
Use the `?` operator for error propagation. Here's an example:
fn read_file(path: &str) -> Result<String, io::Error> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}
```

The model knows Rust. Tell it your project's conventions, not the language.

### Anti-pattern: Inline commands instead of scripts

```markdown
# 15 separate bash blocks doing deployment
1. Run `docker compose build`
2. Run `docker compose up -d`
3. Run `sleep 5`
...
```

Wrap it in a script. The skill becomes two lines. The script works everywhere.

---

## Quick Reference

### File locations

| Type | Project-level | User-level |
|------|--------------|------------|
| Agent | `.claude/agents/<name>.md` | `~/.claude/agents/<name>.md` |
| Skill | `.claude/skills/<name>/SKILL.md` | `~/.claude/skills/<name>/SKILL.md` |
| Command | `.claude/commands/<name>.md` | — |
| Script | `scripts/<name>.sh` | — |

### Priority (same name)

| Type | Winner |
|------|--------|
| Agents | Project-level overrides user-level |
| Skills | User-level overrides project-level |

### Frontmatter fields

**Skills:**
```yaml
name: skill-name
description: What it does (one line)
args: "[arguments]"              # Optional
allowed-tools: Bash, Read        # Optional tool restrictions
disable-model-invocation: true   # Optional, prevents auto-invocation
```

**Agents:**
```yaml
name: agent-name
description: What it does and when to use it
tools: Read, Edit, Bash, Grep    # Only grant what's needed
model: haiku | sonnet | opus     # Match cognitive demand to cost
```

---

## Cross-References

- [Architecture](architecture.md) — how the project is structured
- [Deployment Guide](deployment-guide.md) — what `/deploy` automates
- [Adding New Games](adding-games.md) — the `/new-game` skill in detail
- [Miyoo Porting Guide](miyoo-porting-guide.md) — what `/check-rust` verifies
