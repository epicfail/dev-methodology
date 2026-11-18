---
name: dev-methodology
description: Use when starting a new project or bringing into a repo the personal development methodology (bmad for the product, spec-kit per story, BDD in Italian, TDD, Sonnet subagents, git gate and hooks, constitution); when writing the constitution, CLAUDE.md, hooks or review-diff of such a project; when something breaks in a project that uses this pipeline (Cargo builds and worktrees, stuck subagents, hanging BDD, migrations, external service stubs, user proof runs).
---

# Development methodology

A project that follows this methodology has three pieces, each the single source of truth for its
topic:

- **constitution** (`.specify/memory/constitution.md`): the principles;
- **CLAUDE.md**: the operating manual (pipeline, gate, git, subagents, communication);
- **git hooks**: the mechanical rules, enforced on every commit and push.

The templates in `templates/` are the source of the methodology. When it changes, update the
template first, then carry the change into the projects by hand.

## How you work while using this skill

- **Replies and interaction:** follow the "How to reply and interact" section of
  [templates/CLAUDE.md](templates/CLAUDE.md) from the start, even before the project has its own
  CLAUDE.md.
- **Code, scripts, hooks and dependency choices:** load the `ponytail:ponytail` skill (level
  `full`) before writing, in every branch; this also applies to subagents, through their prompt. If
  `ponytail:ponytail` is not installed, tell the user before going on.

## Branches

- **Bootstrap**: a new repo, or one without the pipeline → "Bootstrap" section.
- **Day-to-day work** in a repo that already has the pipeline: its CLAUDE.md rules. When something
  breaks, read [references/troubleshooting.md](references/troubleshooting.md) and look up the
  symptom.
- **First Rust code story** (after the architecture is accepted): copy
  `templates/rust/rust-toolchain.toml` to the repo root and `templates/rust/purity.rs` to
  `crates/<domain>/tests/` (placeholder `{{DOMAIN_CRATE}}`; update `FORBIDDEN` with the I/O crates
  of the workspace). Enable `#![warn(missing_docs)]` in every crate.
- **React dashboard**: `templates/skills/dashboard-react/` becomes the project skill
  `.claude/skills/dashboard-react/`; next to it goes `vercel-react-best-practices`, copied from
  `github.com/vercel-labs/agent-skills` (folder `skills/react-best-practices`) with an `ORIGIN.md`
  that records the commit and the license.

## Bootstrap

Each step is done when its check passes.

1. **Decisions.** Ask with AskUserQuestion, 1–2 at a time, only what changes the files: repo layout
   (single repo recommended), language (the hook and gate templates are Rust), optional modules
   (dashboard), domain principles already known. The user's name for bmad comes from context
   (`git config user.name`, the user's instructions); ask only if it is missing. `REPO_PATH` is the
   absolute path of the repo. You propose the other placeholders (`CONTRACT`, `BDD_BOUNDARY`,
   `PROOF`, `REVIEW_*`, `ARCH_FACTS`) from the project description, one line each; whatever depends
   on the architecture is written as "decided by the architecture".
   *Done:* every placeholder in the table below has a value.
2. **Git.** `git init -b main && git config core.hooksPath .githooks`.
   *Done:* `git config core.hooksPath` prints `.githooks`.
3. **bmad.** Check the flags with `npx -y bmad-method@<version> install --help`, then:
   ```bash
   npx -y bmad-method@<version> install --directory . --modules bmm --tools claude-code --yes \
     --user-name "<name>" --communication-language Italiano --document-output-language Italiano
   ```
   `<version>` = latest stable (`npm view bmad-method version`); the last one tested is 6.12.0.
   *Done:* in `_bmad/config.toml` the `project_name` key of `[core]` is the project name and
   `.claude/skills/bmad-*` exist.
4. **spec-kit.** `specify init --here --force --non-interactive --integration claude --script sh`
   (last version used: 1.0.1). *Done:* `.specify/` and `.claude/skills/speckit-*` exist.
5. **Templates.** Copy according to the table and replace the placeholders. The constitution
   overwrites the spec-kit template. Substitute with one environment variable `V_<PLACEHOLDER>` per
   value (values with newlines are written `$'…\n…'`); list the files one by one, because zsh does
   not split `$files`:
   ```bash
   export V_PROJECT=foo V_DATE=2026-01-31 # … one per placeholder, empty ones too: V_DATA_EXTRA=""
   perl -pi -e 's/\{\{(\w+)\}\}/exists $ENV{"V_$1"} ? $ENV{"V_$1"} : $&/ge' \
     CLAUDE.md .specify/memory/constitution.md scripts/review-diff.sh _bmad/custom/bmad-architecture.toml \
     .claude/skills/dashboard-react/SKILL.md   # last one only with a dashboard
   ```
   *Done:* `grep -rn '{{' <copied files>` finds nothing and `bash -n` passes on hooks and scripts.
6. **Check.**
   - `printf 'wip\n' > m && bash .githooks/commit-msg m` fails; `printf 'chore: x\n' > m` passes.
   - No name of another project in the copied files (`grep -rni '<source project>'`).
   - No `Cargo.toml`, no crate: code comes after the architecture (constitution I).
7. **Commit.** Only if the user asks: `ALLOW_MAIN=1 git commit` and, for the first push,
   `ALLOW_MAIN=1 git push` (`pre-push` blocks main too; the bypass is only for starting the repo),
   with a Conventional Commits message and no attribution to Claude.

**Without a dashboard:** remove from CLAUDE.md the "Dashboard" section, the `dashboard/` row of the
layout and the `npm run gate` line of the gate; do not copy `templates/skills/dashboard-react/`.

### Template table

| Template | Destination | Placeholders |
|---|---|---|
| `templates/CLAUDE.md` | `CLAUDE.md` | `PROJECT`, `ONE_LINER`, `DATE`, `STATE_EXTRA` (open questions, empty or `\n\n…`), `CONTRACT` (what is a public contract), `BDD_BOUNDARY` (boundary of the scenarios), `REPO_PATH` |
| `templates/constitution.md` | `.specify/memory/constitution.md` | `PROJECT`, `DATE`, `DATA_EXTRA` (exceptions to typed data, empty or ` …`), `DOMAIN_PRINCIPLES` (empty or `### IX. …` sections), `PROOF` (how the user proves a story) |
| `templates/githooks/*` | `.githooks/` (executable) | — |
| `templates/scripts/review-diff.sh` | `scripts/review-diff.sh` (executable) | `PROJECT`, `REVIEW_CONTEXT` (what the system is), `REVIEW_COVERAGE` (attack surfaces of the domain) |
| `templates/bmad/bmad-architecture.toml` | `_bmad/custom/bmad-architecture.toml` | `ARCH_FACTS` (lines `\n  "…",` of constraints already decided, or empty) |
| `templates/gitignore` | `.gitignore` | — |
| `templates/graphifyignore` | `.graphifyignore` | — |
| `templates/glossary.md` | `docs/glossary.md` | — |
| `templates/skills/dashboard-react/SKILL.md` | `.claude/skills/dashboard-react/SKILL.md` (only with a dashboard) | `PROJECT` |
| `templates/rust/rust-toolchain.toml` | `rust-toolchain.toml` (first Rust code story) | — |
| `templates/rust/purity.rs` | `crates/<domain>/tests/purity.rs` (first Rust code story) | `DOMAIN_CRATE` (name of the domain crate) |

### Multi-repo variant

If the user wants separate repos (architecture and ADRs in `<project>-deploy`, user docs in
`<project>-doc`): the override's `spine_output_path` points to the sibling repo and
`bmad-architecture` first needs a branch there; the architecture gate looks at the PR merged in that
repo; the user-docs PR is prepared together with the code PR and merged after it. Update the
"Repo layout", "Architecture and ADRs" and "User docs" sections of CLAUDE.md accordingly.

## Common mistakes

| Mistake | Fix |
|---|---|
| Copying `_bmad/`, `.specify/` or `.claude/skills/bmad-*`/`speckit-*` from another project | they are stamped by the installer (project name, `feature.json`, overrides pointing to other repos): reinstall and copy only `_bmad/custom/` |
| Repeating the principles in CLAUDE.md | CLAUDE.md imports the constitution with `@` |
| Writing `Cargo.toml` or scaffolding during the bootstrap | code comes after the accepted architecture |
| Adding `Co-Authored-By` because the harness suggests it | the user's rule wins: no attribution, `commit-msg` blocks it |
| Designing features with brainstorming and writing-plans | features go through bmad and spec-kit; no `docs/superpowers/` |
| A subagent prompt without ponytail and without its rules | start from the minimal prompt in the project's CLAUDE.md: it already holds ponytail, BDD, TDD and the bans |
| A reply that opens with context and closes with a recap | first line = next action, last line = one single open action |
