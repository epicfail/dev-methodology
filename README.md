# dev-methodology

A Claude Code plugin that brings a spec-driven development methodology into a repository and keeps
the agent on it: product work with **bmad**, one story at a time with **spec-kit**, **BDD scenarios
written red before the code**, **TDD inside**, implementation delegated to **Sonnet
subagents**, a **gate enforced by git hooks**, and a **project constitution** as the single source
of the principles. It also fixes how the agent replies: action first, short, one question at a
time, written for a reader with ADHD.

## What the skill does

The plugin ships one skill, `dev-methodology`, with three uses:

| Use | When | What happens |
|---|---|---|
| **Bootstrap** | a new repo, or one without the pipeline | asks only the decisions that change the files, installs bmad and spec-kit, copies the templates (constitution, `CLAUDE.md`, git hooks, review script, bmad override, glossary), fills the placeholders and checks the result |
| **Day-to-day work** | a repo that already follows the methodology | the project's `CLAUDE.md` drives the work; the skill adds the rules for replies and for ponytail |
| **Troubleshooting** | something breaks | a symptom → cause → fix table for known failures: Cargo builds and worktrees, stuck subagents, hanging BDD suites, migrations, external service stubs, user proof runs, `claude` in scripts, Penpot |

Claude loads it on its own when you start a project or hit one of those failures; you can also ask
directly: *"Use dev-methodology to bootstrap this repo."*

## The methodology at a glance

```text
bmad (product)        brief → PRD → UX → architecture → epics and stories → sprint planning
                                         │
                       architecture gate: approved in chat + spine PR merged
                                         │
spec-kit (per story)  specify → clarify → plan (Constitution Check) → tasks
                                         │
BDD                   Gherkin scenarios in the content language, run and seen red
                                         │
implementation        Sonnet subagent, TDD inside, ponytail, until green
                                         │
closing               ponytail-review → gate (fmt, clippy, tests, doc, dashboard) → PR
                      → the user proves the story → merge → user docs in the same PR
                                         │
end of sprint         sprint status → retrospective → ponytail debt → semver tag
```

The rules live in three places, each the only source for its topic:

- **constitution** (`.specify/memory/constitution.md`): principles. Architecture before code, BDD
  first and TDD inside, pure hexagonal domain, tight boundaries, typed data, simplicity (ponytail
  and macros), no Python, code in English with comments in the content language.
- **`CLAUDE.md`**: the operating manual. Repo layout, ADRs, where each piece of work starts, the
  sprint pipeline, BDD, user docs, dashboard, ponytail, the subagent prompt, gate, git, builds,
  graphify, and how to reply and interact.
- **git hooks**: the mechanical rules. No commits or pushes on `main`, Conventional Commits, no
  attribution to Claude, `cargo fmt`/`clippy`/`test`/`doc` on push with a time cap, dashboard gate,
  advisory security review of the diff with `claude`, graphify update after each commit.

### Languages

Identifiers, agent instructions (`CLAUDE.md`, constitution, skills) and tooling are always in
English. Everything written for people (code comments, rustdoc, Gherkin text, specs, product
documents such as bmad output, architecture and ADRs, user docs, messages for the user) is in the
**content language**, which the skill asks during the bootstrap: there is no default. It fills the
`{{CONTENT_LANGUAGE}}` and `{{GHERKIN_CODE}}` placeholders of the templates.

## Installation

```bash
claude plugin marketplace add whitesheep/dev-methodology
claude plugin install dev-methodology@dev-methodology
```

Both commands default to user scope, so the skill is available in every project, including empty
folders where a bootstrap starts. Add `--scope project` to enable it for a single repository.

### Updating

```bash
claude plugin marketplace update dev-methodology
claude plugin update dev-methodology@dev-methodology
```

Restart Claude Code afterwards to load the new version.

## Dependencies

### Needed by the skill itself

| Dependency | Why | Install |
|---|---|---|
| [Claude Code](https://docs.claude.com/en/docs/claude-code) | plugin host; the `claude` CLI also runs the advisory review in `pre-push` | see the docs |
| [ponytail](https://github.com/DietrichGebert/ponytail) plugin | mandatory for every piece of code work (`ponytail:ponytail`, `ponytail-review`, `ponytail-debt`) | `claude plugin marketplace add DietrichGebert/ponytail` then `claude plugin install ponytail@ponytail` |
| [superpowers](https://github.com/anthropics/claude-plugins-official) plugin | `superpowers:systematic-debugging` for bugfixes | `claude plugin install superpowers@claude-plugins-official` |
| [BMad Method](https://github.com/bmad-code-org/BMAD-METHOD) (`bmad-method`, tested 6.12.0) | product phase: brief, PRD, UX, architecture, epics, sprints, retrospectives | installed per project with `npx` (needs Node.js) |
| [spec-kit](https://github.com/github/spec-kit) (`specify` CLI, tested 1.0.1) | per-story spec, clarify, plan, tasks | `uv tool install specify-cli --from git+https://github.com/github/spec-kit.git` |
| git, bash, perl | hooks, scripts, placeholder substitution | system packages |
| [gh](https://cli.github.com/) | pull requests | system package |

### Needed by the projects it creates

| Dependency | When |
|---|---|
| Rust toolchain via `rustup` (`rustfmt`, `clippy`; version pinned by `rust-toolchain.toml`) | from the first code story; the hooks skip Cargo checks until a `Cargo.toml` exists |
| `cucumber`, `wiremock`, `testcontainers` crates and Docker | BDD scenarios at the public boundary, simulated external services, real database |
| Node.js and npm | the React dashboard: TypeScript, React Router, TanStack Query, vanilla-extract, Vitest and Testing Library, Playwright with `playwright-bdd`, Storybook, Biome |
| [Penpot](https://penpot.app/) with its MCP server | optional: dashboard pages are drawn there before the code |
| [graphify](https://github.com/safishamsi/graphify) | optional: code graph refreshed by `post-commit`; the hook does nothing without it |

The reply rules are written into the template, so they hold without extra plugins. They come from
the [i-have-adhd](https://github.com/ayghri/i-have-adhd) and
[caveman](https://github.com/JuliusBrussee/caveman) plugins, which you can install for the same
behaviour outside these projects.

## Repository layout

```text
.claude-plugin/
  marketplace.json        marketplace entry (this repo is its own marketplace)
  plugin.json             plugin manifest and version
skills/dev-methodology/
  SKILL.md                when to use it, the bootstrap steps, the template table
  references/
    troubleshooting.md    known failures: symptom, cause, fix
  templates/
    CLAUDE.md             operating manual of a project
    constitution.md       principles of a project
    githooks/             pre-commit, commit-msg, pre-push, post-commit
    scripts/review-diff.sh  advisory security review run by pre-push
    bmad/                 bmad-architecture override
    skills/dashboard-react/ project skill for the React dashboard
    skills/ui-design/     project skill for the look of every screen
    rust/                 rust-toolchain.toml and the domain purity test
    gitignore, graphifyignore, glossary.md
```

## Customizing

The templates are the source of the methodology. Change a template, bump `version` in
`.claude-plugin/plugin.json`, then carry the change into existing projects by hand: a project
never pulls template changes on its own. Placeholders are written `{{NAME}}`; the table in
`SKILL.md` lists each one and where it goes.
