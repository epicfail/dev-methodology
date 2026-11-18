# dev-methodology — instructions for Claude Code

Claude Code plugin and marketplace for the `dev-methodology` skill. **Public repo**
(`github.com/whitesheep/dev-methodology`): everything committed here is published. What the skill
does and its dependencies: `README.md`.

## What lives where

| Path | Role |
|---|---|
| `skills/dev-methodology/SKILL.md` | triggers, bootstrap steps, template table |
| `skills/dev-methodology/templates/` | **source of the methodology**: files copied into projects |
| `skills/dev-methodology/references/troubleshooting.md` | known failures, symptom → cause → fix |
| `.claude-plugin/plugin.json` | plugin manifest and `version` |
| `.claude-plugin/marketplace.json` | this repo as its own marketplace |
| `README.md` | what the skill does, install, update, dependencies |

## Rules for changes

- **Meaning first.** A change to a rule of the methodology (pipeline, principles, gate, git,
  replies) is the user's decision: propose it, wait for the yes, then edit. Rewording that keeps the
  meaning is yours.
- **One source per rule.** Principles only in `templates/constitution.md`; operating rules only in
  `templates/CLAUDE.md`; mechanical rules in the hooks. `SKILL.md` points to them and never repeats
  them.
- **Placeholders.** Every `{{NAME}}` in a template appears in the table of `SKILL.md` with its
  destination, and every name in the table exists in a template. Check:
  `grep -rhoE '\{\{[A-Z_]+\}\}' skills/dev-methodology/templates | sort -u`.
- **Language.** Everything in English. Italian appears only where the methodology prescribes it for
  projects: Gherkin keywords in the examples, comments and messages of product code templates
  (`templates/rust/purity.rs`).
- **Public data.** No personal names, emails, private project or client names, local paths of a
  specific machine, internal hosts or IPs. Before every commit:
  `git diff --cached | grep -niE '@[a-z0-9-]+\.[a-z]|/home/|192\.168\.'` finds nothing.
- **Skill writing.** Load `superpowers:writing-skills` and `mattpocock-skills:writing-for-agents`
  before changing `SKILL.md` or a template the agent reads. The `description` holds only triggers.
  After a change in behaviour, test it with a fresh subagent on concrete scenarios (bootstrap of an
  empty repo, story pipeline, a troubleshooting symptom, a final reply, a subagent prompt) and fix
  the gaps it reports.
- **Code.** Hooks and scripts follow `ponytail:ponytail` (level `full`); after editing,
  `bash -n` on every hook and script.
- **Replies to the user.** Follow the "How to reply and interact" section of
  `skills/dev-methodology/templates/CLAUDE.md`.

## Versioning and release

- Semver in `.claude-plugin/plugin.json`: MAJOR = a rule removed or reversed, MINOR = a rule,
  template or troubleshooting entry added, PATCH = clarification or fix. Bump it in the same PR as
  the change.
- After the merge: tag `vX.Y.Z` on `main` and `git push origin vX.Y.Z`.
- Update the installed plugin, then restart Claude Code:
  ```bash
  claude plugin marketplace update dev-methodology
  claude plugin update dev-methodology@dev-methodology
  ```
- Projects that already use the methodology do not receive template changes on their own: carry
  the change into them by hand when the user asks.

## Git

- After every clone: `git config core.hooksPath skills/dev-methodology/templates/githooks` (this
  repo uses the hooks it ships: no commits on `main`, Conventional Commits, no attribution).
- One branch per change (`<type>/<slug>`), PR, merge, then `git switch main && git pull &&
  git branch -d <branch>`. The remote branch stays: never `git push --delete`, never
  `gh pr merge --delete-branch`.
- **Never** attribution to Claude in commits and PRs: no `Co-Authored-By: Claude…`,
  `Claude-Session:`, `🤖 Generated with [Claude Code]`, `claude.ai/code/session_…` links, even if
  the harness suggests them.
