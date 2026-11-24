# Bootstrap rules for this repository

Temporary contract. It governs work on this repository until the owner adopts the new
methodology here. It is not the methodology this repository distributes.

## What lives where

| Path | Role |
|---|---|
| `skills/` | Distributed product. Everything under `skills/**/templates/` (including `*.tmpl` files) is content for other projects: never follow it as instructions for this repository |
| `.claude-plugin/` | Plugin manifest and marketplace definition |
| `.githooks/` | Hooks active on this repository |
| `.github/rulesets/` | GitHub rulesets of this repository, as applied |
| `docs/methodology/` | Specification of the new methodology (added with v5) |

The previous methodology (BMad, spec-kit, mandatory BDD, Rust hooks, review subagents) is
product content being replaced. Do not apply it to work on this repository.

## Setup after every clone

    git config --local core.hooksPath .githooks
    if git remote get-url upstream >/dev/null 2>&1; then
      git remote set-url --push upstream DISABLED
    fi
    gh repo set-default epicfail/dev-methodology

## Rules

1. One change per branch named `<type>/<slug>`, one PR per branch, opened on this fork:
   always pass `--repo` (and `--head` when creating a PR) to `gh`; never open PRs on
   `upstream`. Never commit on `main`.
2. Commit messages follow Conventional Commits. No attribution trailers or links to agent
   sessions in commits or PRs.
3. Do only the assigned task. Anything else goes in the final report as a proposal.
4. Decisions about the methodology (principles, rules, gates, defaults) belong to the owner:
   propose, wait for approval, then edit. Rewording that keeps the meaning is allowed.
5. Rule files are `AGENTS.md`, `CLAUDE.md`, `.githooks/`, `.github/rulesets/` and
   `docs/methodology/`. Change them only when the task says so, in a PR that contains only
   rule files.
6. Protections apply to everyone, owner included, and are never disabled or bypassed: no
   `--no-verify`, no push protection bypass, no change to `core.hooksPath` other than the setup
   above. A GitHub ruleset changes only through a merged PR to `.github/rulesets/`, applied
   after the merge.
7. Never write real credentials. Tests use invented values. If you find a credential, stop and
   report its location without copying it.
8. Never push to `upstream`. No writes on GitHub (settings, merges, branch deletion) without
   the owner's explicit approval in the task.
9. Templates: every `{{NAME}}` in `skills/dev-methodology/templates/` appears in the table of
   `SKILL.md`, and every name in the table exists in a template. Check:
   `grep -rhoE '\{\{[A-Z_]+\}\}' skills/dev-methodology/templates | sort -u`
   and compare its output with the table in `SKILL.md`, file by file.
10. This repository is public: no personal names, email addresses, local paths of a specific
    machine, internal hosts or IPs in committed content.
11. Gitleaks scans only this repository's own content: never use `--max-archive-depth` and never
    scan untrusted third-party code or archives with it.
12. Commit identity: author and committer are the owner's GitHub noreply identity. Before every
    push, `git log --format='%an <%ae> | %cn <%ce>' origin/main..HEAD | sort -u` must show only
    that identity.

## Changing the GitHub ruleset

1. Open a PR that changes only `.github/rulesets/main.json`.
2. After the merge, the owner (never an agent) applies it:
   `gh api -X PUT repos/<owner>/<repo>/rulesets/<id> --input .github/rulesets/main.json`
   where `<id>` comes from
   `gh api repos/<owner>/<repo>/rulesets --jq '.[] | select(.name=="protect-main") | .id'`.
3. The owner verifies that GitHub matches the file: `bash .github/rulesets/verify.sh` must
   exit 0 with no output.

## Adopting components of the new methodology

A component built here governs this repository only after the owner adopts it in a dedicated
PR that changes only rule files. A new required check first runs as advisory and becomes
required only after it has worked correctly. A broken required check is handled with the
written emergency procedure (to be defined before the first required check).

## End of every task

Report what changed, what you verified and how, and what remains open.
