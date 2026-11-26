# Phase 1 skip-marker check

| Field | Value |
|---|---|
| Work item | Issue #14 |
| Test repositories | `epicfail/dm-test-public` (public), `epicfail/dm-test-private` (private), owned by a personal account on GitHub Free |
| Reference specification | `docs/methodology/specification.md`, v5.1, §6.4 (secret control chain, step 3) and §10 (Q9); §7.7 for the attribution patterns of H11 only |
| Tools | `gh` 2.102.0, `git` 2.56.1 |
| Status | Test protocol approved by the owner on 2026-11-26. Tests completed on 2026-11-26; scheduled runs (R2) last read at 16:38 UTC. Remedy evaluation reviewed independently on 2026-11-26 |
| Normative status | Non-normative |

The record states what GitHub and Git did for the account and the date of the tests. GitHub can change its behaviour: a result is valid at its date only.

## Question

§6.4 makes the push-triggered secret-scanning job the only remote defense of a private repository, and states that a skip marker in a commit message prevents push-triggered workflows. This activity checks when markers stop such a workflow, on pushes and on pull requests, and evaluates candidate remedies, including the `commit-msg` hook listed among the roadmap candidates.

The observable is whether a workflow run is created. No secret was used, real or invented. A probe workflow stands for the scanning job: skip instructions act on whether a workflow is triggered (E1), not on what its job does.

## Evidence classes

| Class | Meaning |
|---|---|
| tested | Observed in this activity, on a test repository or in a throwaway local repository; the record gives the action and the relevant output |
| documented | Read in GitHub or Git documentation, with the source and the date of reading; not tested |
| deduced | Inferred from tested results, with the reasoning stated; not tested directly |
| not stated | The documentation read does not say it |

References to the phase-1 capability matrix (`docs/audit/github-free-capabilities.md`) are written as "capability matrix" followed by its own identifiers. E1–E13 below are this record's own.

## Observations before the tests

Read-only, 2026-11-26.

| # | Observation | Evidence |
|---|---|---|
| O1 | `dm-test-public`: ruleset `protect-main` at stage A of the capability matrix, with required status check `check`; workflow `check.yml` triggered by `push` to `main` and by `pull_request` | `gh api`; capability matrix, fixtures |
| O2 | `dm-test-public`: `merge_commit_title: MERGE_MESSAGE`, `merge_commit_message: PR_TITLE`; the last merge commit on `main` carried the pull request title in its body | `gh api repos/{owner}/{repo}`; commit message |
| O3 | `dm-test-private`: same `check.yml`, unchanged since `5aedb64`; four successful `push` runs on `main` in its history | `gh run list` |

## Documented expectations

GitHub pages read on 2026-11-26 as served by `https://docs.github.com/api/article/body?pathname=<path>`; Git manual pages of `git` 2.56.1. The GitHub pages show no last-updated date. Every quoted string was matched against the saved text with `grep -F`.

| # | Statement | Source |
|---|---|---|
| E1 | "Workflows that would otherwise be triggered using `on: push` or `on: pull_request` won't be triggered if you add any of the following strings to the commit message in a push, or the HEAD commit of a pull request:" `[skip ci]`, `[ci skip]`, `[no ci]`, `[skip actions]`, `[actions skip]` | [Skipping workflow runs](https://docs.github.com/en/actions/how-tos/manage-workflow-runs/skip-workflow-runs) |
| E2 | Trailer `skip-checks:true` or `skip-checks: true`: "The trailers section should be included at the end of your commit message and be preceded by two empty lines. If you already have other trailers in your commit message, `skip-checks` should be last." "By default, Git automatically removes consecutive newlines. To leave the commit message exactly as you entered it, use the `--cleanup=verbatim` option on your commit." | Same page |
| E3 | "Skip instructions only apply to the `push` and `pull_request` events. For example, adding `[skip ci]` to a commit message won't stop a workflow that's triggered `on: pull_request_target` from running." | Same page |
| E4 | "Skip instructions only apply to the workflow run(s) that would be triggered by the commit that contains the skip instructions." | Same page |
| E5 | Checks of a workflow skipped by a commit message "will remain in a "Pending" state. A pull request that requires those checks to be successful will be blocked from merging." "To allow the pull request to be merged you can push a new commit to the pull request without the skip instruction in the commit message." | Same page |
| E6 | For a workflow skipped by a commit message: "Associated checks stay in a "Pending" state and block merging"; remedy: "Avoid requiring workflows that can be skipped." | [Troubleshooting required status checks](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/collaborating-on-repositories-with-code-quality-features/troubleshooting-required-status-checks) |
| E7 | Event `push`, `GITHUB_SHA`: "Tip commit pushed to the ref." | [Events that trigger workflows](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows) |
| E8 | `pull_request_target`: "This event runs in the context of the default branch of the base repository, rather than in the context of the merge commit, as the `pull_request` event does." "Avoid using this event if you need to build or run code from the pull request." | Same page |
| E9 | `schedule`: "Scheduled workflows run on the latest commit on the default branch." "The shortest interval you can run scheduled workflows is once every 5 minutes." "The `schedule` event can be delayed during periods of high loads of GitHub Actions workflow runs. High load times include the start of every hour. If the load is sufficiently high enough, some queued jobs may be dropped." "In a public repository, scheduled workflows are automatically disabled when no repository activity has occurred in 60 days." | Same page |
| E10 | Merge commit message: "The default message includes the pull request number and title." "You can also choose to use just the pull request title, or the pull request title and description." | [Configuring commit merging for pull requests](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/configuring-commit-merging-for-pull-requests) |
| E11 | `commit-msg`: "This hook is invoked by git-commit(1) and git-merge(1), and can be bypassed with the --no-verify option." `pre-push`: "This hook is called by git-push(1) and can be used to prevent a push from taking place." `git push`: "With --no-verify, the hook is bypassed completely." | `githooks(5)`, `git-push(1)` |
| E12 | Cron syntax, operator `-`: "Range of values" | [Events that trigger workflows](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows), section `schedule` |
| E13 | `--cleanup=default`: "Same as strip if the message is to be edited. Otherwise whitespace." | `git-commit(1)` |

Not stated by the pages read, and therefore turned into test cases:

- which commit of a push with several commits is evaluated;
- whether the strings are matched regardless of case;
- the trailer without two empty lines, after Git's default cleanup;
- the pull request title: it is not a commit message, but it ends up in the merge commit (O2) and therefore in the `push` to `main`;
- dependence on repository visibility;
- whether `git cherry-pick`, `git rebase` and `git revert` invoke `commit-msg` (E11 names only `git commit` and `git merge`).

A repository setting that turns the markers off is not mentioned by the pages read and was not looked for.

## Fixtures

`.github/workflows/probe.yml`, added to `main` of `dm-test-public` by PR #4:

```yaml
name: probe
on:
  push:
    branches-ignore: [main]
  pull_request_target:
  schedule:
    - cron: "17 * 26-27 11 *"
permissions: {}
jobs:
  probe:
    runs-on: ubuntu-latest
    timeout-minutes: 2
    steps:
      - name: Record trigger
        env:
          BEFORE: ${{ github.event.before }}
          PR_HEAD_SHA: ${{ github.event.pull_request.head.sha }}
        run: |
          echo "event=$GITHUB_EVENT_NAME ref=$GITHUB_REF sha=$GITHUB_SHA"
          echo "before=${BEFORE:-none} pr_head=${PR_HEAD_SHA:-none}"
```

No checkout, no token permissions, no action: under `pull_request_target` the job runs no code from the pull request. Commit messages never reach an expression.

Local hook prototypes, kept outside this repository and never installed in `.githooks/` or `skills/**/templates/`. They are fixtures for the tests, not rules.

| Fixture | Behaviour |
|---|---|
| `commit-msg` | Scans the whole message file, comment lines included (with `--cleanup=verbatim` they stay in the commit), and ignores the text below the scissors line of `git commit -v`. Group 1, skip instructions, case-insensitive, any line: `\[(skip ci\|ci skip\|no ci\|skip actions\|actions skip)\]\|^[[:space:]]*skip-checks[[:space:]]*:[[:space:]]*true`. Group 2, agent attribution lines, patterns for Claude only: `^(co-authored-by:.*(claude\|anthropic)\|claude-session:)\|generated with \[?claude\|claude\.ai/code/session_`. Group 3, human `Co-authored-by` lines: no pattern; they must not match group 2 |
| `pre-push` | For every pushed ref except deletions, checks the new tip and the commits introduced by the push with the group 1 pattern. Introduced commits: `git rev-list <new> --not --remotes=<remote>` for a new ref, `<old>..<new>` otherwise. "Already on the remote" is judged from the local remote-tracking refs |

## Test cases

`$PUB` is `epicfail/dm-test-public`, `$PRIV` is `epicfail/dm-test-private`. Every write on GitHub ran after the owner's approval; the owner ran the block K pushes from their terminal. Commit identity on the test repositories: the owner's noreply identity. Local tests: throwaway repositories, invented identity and values (`example.invalid`), never `--no-verify`.

Evidence for every push: tip SHA; message as stored by GitHub (`gh api repos/{owner}/{repo}/commits/{sha}`); runs filtered by SHA (`gh run list`); check suites (`gh api repos/{owner}/{repo}/commits/{sha}/check-suites`); for probe runs, the `event=… sha=… before=… pr_head=…` lines of the log. "No run" was declared after at least two minutes and checked again at the end of the block and at 14:48 UTC.

### Setup and pushes, public repository

| ID | Action | Expected | Basis |
|---|---|---|---|
| P0 | PR #4 from `setup/probe` adding `probe.yml`; merge after `check` passes | `probe` runs on the branch push; no `pull_request_target` run, because the workflow is not yet on `main` | E8 |
| K1 | Branch `k/push`, clean commit | Run | E7 |
| K2–K6 | One push per string, each string in the subject | No run | E1 |
| K7 | Marker in the body only | No run | E1 |
| K8 | `skip-checks: true` after two empty lines, `--cleanup=verbatim` | No run | E2 |
| K9 | `skip-checks:true` after two empty lines, default cleanup | Not stated | — |
| K10 | `[SKIP CI]` in the subject | Not stated | — |
| K11 | Two commits: marker in the first, clean tip | Not stated; run by inference from E4 and E7 | — |
| K12 | Two commits: clean first, marker on the tip | Not stated; no run by inference from E4 and E7 | — |
| K13 | One clean commit | Run; `before` shows where the next run starts after a skipped tip | E7 |

### Pull requests, public repository

| ID | Action | Expected | Basis |
|---|---|---|---|
| Q1 | Branch `q/head` with one commit carrying `[skip ci]`; PR #5 with a clean title | No `check` run; `probe` runs on `pull_request_target`; merge blocked | E1, E3, E5, E6 |
| Q2 | Merge of PR #5 through the REST API | Refused, HTTP 405 naming `check` | E5 |
| Q3 | Clean commit on `q/head`; then close PR #5 unmerged | `check` runs and passes; merge state clean | E5 |
| Q4 | Branch `q/title` with a clean commit; PR #6 with `[skip ci]` in the title; merge with `gh pr merge --merge` | `check` runs on the PR; the merge commit carries the title; the push to `main` starts no `check` run | E1, E10, O2 |

### Remedies

| ID | Remedy | Action | Expected |
|---|---|---|---|
| R1 | Event not subject to markers: `pull_request_target` | Covered by Q1 | As Q1 |
| R2 | Event not subject to markers: `schedule` | Hourly cron at minute 17 on 26–27 November; after Q4 the tip of `main` is the marked merge commit | A `schedule` run on the tip of `main`; delay recorded, or no start recorded (E9) |
| R3 | Local `commit-msg` hook | Cases H below, in a throwaway local repository | See H |
| R4 | Local `pre-push` check | Cases R4 below, against a local bare remote | See R4 |

| ID | Action (`commit-msg` fixture) | Expected |
|---|---|---|
| H1a–H1e | `git commit -m` with each of the five strings | Rejected |
| H2 | Marker in the body only (`-F`) | Rejected |
| H3a | `skip-checks: true` after two empty lines, `--cleanup=verbatim` | Rejected |
| H3b | `skip-checks:true` after two empty lines, default cleanup (`-F`); record what the hook receives | Rejected |
| H4a | `[SKIP CI]` | Rejected |
| H4b | Marker on a `#` line, `--cleanup=verbatim` | Rejected |
| H4c (added) | Marker on a `#` line, default cleanup (`-F`) | To record |
| H5 | Clean Conventional Commits message | Accepted |
| H6 | `git merge --no-ff -m` with a marker | Rejected (E11) |
| H7 | `git commit --amend -m` with a marker | Rejected |
| H8 | `git cherry-pick` of a marked commit created without the hook | Not stated; record whether the hook is invoked |
| H8b (added) | `git cherry-pick -e`, the editor keeps the proposed message | As H8 |
| H9 | `git rebase` of a branch that contains the marked commit | As H8 |
| H9b (added) | `git rebase -i` with `reword` on the marked commit, the editor keeps the message | As H8 |
| H10 | `git revert --no-edit` of the marked commit | As H8 |
| H10b (added) | `git revert` with the editor keeping the proposed message | As H8 |
| H11a–H11c | Attribution lines with invented values: `Co-Authored-By` naming Claude, `Claude-Session` with an invented URL, `Generated with [Claude Code](…)` | Rejected |
| H11d | Human `Co-authored-by` with invented values | Accepted |
| H11e (added) | `Co-authored-by` naming another agent, invented values | To record |
| H12 (added) | `git commit -v --edit` with a staged diff that mentions a marker | Accepted |

| ID | Action (`pre-push` fixture) | Expected |
|---|---|---|
| R4a | Push of a clean `main` | Accepted |
| R4b | New branch, marker on the tip | Rejected |
| R4c | New branch, marker on a non-tip commit, clean tip | Rejected |
| R4d (added) | New branch whose tip is a marked commit already on the remote, pushed there from a clone without the hook | Rejected |
| R4e (added) | Deletion of a remote branch | Accepted |
| R4f (added) | Clean new branch | Accepted |
| R4g (added) | New branch with a clean commit on top of a marked commit already on the remote | Accepted |

### Private repository

| ID | Action | Expected | Jobs expected |
|---|---|---|---|
| W1 | Direct push to `main` of a commit carrying `[skip ci]` | No `check` run | 0 |
| W2 | Direct push to `main` of a clean commit | `check` runs | 1 |

W3 (pull request with a marked head on the private repository) was not run, by the owner's decision.

## Limits and deviations from the protocol

- No scanner and no secret: the probe shows whether a workflow starts. That a later scan recovers a secret in a skipped commit is deduced, not tested.
- Each string was tested once in the subject of a push. Case handling was tested with `[SKIP CI]` only (K10). On the private repository only `[skip ci]` was tested, on a direct push to `main` (W1).
- R2: the protocol draft planned a single scheduled run; the owner approved an hourly cron. Minute 17 is not the start of the hour, which E9 names as a high-load time.
- W2 was run on `check.yml`, which does not log `before`: its start point comes from the output of the push on the client, not from the workflow payload.
- The W2 job was counted from the run list; the billing page was not read.
- The `pre-push` fixture judges "already on the remote" from the local remote-tracking refs, so a stale fetch changes the set of introduced commits.
- `--no-verify` was not tested: its bypass of both hooks is documented (E11).
- Cases added beyond the approved protocol, all local: H4c, H8b, H9b, H10b, H11e, H12, R4d–R4g.
- Push protection was not tested again, by the owner's decision (see the remedy evaluation).
- `probe.yml` and its cron are still on `main` of `dm-test-public` at the date of this record. Removing them is a separate cleanup PR on the test repository, after the owner's decision. The test branches stay until this record is merged.

## Deviations from the specification

Text of `docs/methodology/specification.md` on `main` at `7de401a`.

| # | Section | Specification | Observed | Proposal |
|---|---|---|---|---|
| D1 | §6.4, secret control chain, step 3 | "configured to run on every push to any branch and to scan all pushed commits, not just the final state" | A marker on the tip of a push prevented the run (K2–K8, K12, W1). The next run's `before` was the skipped tip (K9, K13; W2 from the client push output), so a scan of `before..after` would not reach the skipped commits | Rules-only PR on §6.4, after this record |
| D2 | §6.4, paragraph on the step 3 job | "A missing or incomplete run is never evidence that no secret was pushed: it is reported as incomplete coverage." | The first part holds. For the second, a skipped push left no run and no check suite in the cases tested (K2–K8, K12, Q4, W1), and nothing reported the missing run | Same PR |
| D3 | §6.4, same paragraph | "a commit message containing a skip marker such as \[skip ci\] prevents push-triggered workflows" | Holds for the cases K2–K8 and K12. A marker also reached `main` through a merge message composed by GitHub from the PR title, which no local hook sees (Q4) | Same PR |
| D4 | §6.3, automated controls, row "Secrets"; §9, bootstrap step | "CI job on every push"; "which runs on every push to any branch" | As D1 | Same PR, as consequential alignments |

## Execution log

Times in UTC, 2026-11-26.

| ID | Time | Outcome | Evidence |
|---|---|---|---|
| P0 | 11:26–11:40 | PR #4, head `bdd4087`: `probe` on `push` (run 45810629763), `check` on `pull_request` (run 45810638598), no `pull_request_target` run. Merged at 11:40:31 as `29c3db4`; `check` on `push` to `main` (run 45812712918) | `gh pr view`, `gh run list` |
| K1 | 11:51 | `d2cade5`: run 45814255849 | `gh run list`, check suites |
| K2–K6 | 11:51 | `3faa595`, `783ff84`, `e7de672`, `6157d8c`, `c3b78ef`: no run, 0 check suites each | `gh run list`, check suites |
| K7 | 11:51 | `2d5c4b3`: no run, 0 check suites | Same |
| K8 | 11:51 | `992eece`: no run, 0 check suites; stored message has two empty lines before the trailer | Same; stored message |
| K9 | 11:51 | `96f53db`: run 45814288749, `before=992eece`; stored message has one empty line before the trailer | Same; probe log |
| K10 | 11:51 | `c237ea3`: run 45814294002 | Same |
| K11 | 11:51 | Tip `e40936b`: run 45814297803, `before=c237ea3`; 0 check suites on the marked non-tip commit `e290774` | Same; probe log |
| K12 | 11:51 | Tip `e9c0619`: no run, 0 check suites; nothing either for the clean non-tip commit `d74bc95` | Same |
| K13 | 11:51 | `02cd0f5`: run 45814305751, `before=e9c0619`, the skipped tip of K12 | Same; probe log |
| K, re-read | 11:54 and 14:48 | Unchanged | Same |
| Q1 | 11:56 | PR #5, head `f7fcff9`: the push of `q/head` started no run; only `probe` on `pull_request_target` (run 45815087950), with `sha=29c3db4`, the tip of `main`, and `pr_head=f7fcff9`. `mergeStateStatus: BLOCKED`. The status rollup showed SUCCESS, because the only check present was the non-required probe | `gh pr view --json`, probe log |
| Q2 | 11:59 | REST merge refused: HTTP 405, "Repository rule violations found", followed by "Required status check "check" is expected." | `PUT /repos/{owner}/{repo}/pulls/5/merge` |
| Q3 | 11:59–12:03 | `01eab88`: `probe` on `push` (run 45815460296) and on `pull_request_target` (run 45815466277), `check` on `pull_request` (run 45815469232), passed; merge state `CLEAN`. PR #5 closed unmerged at 12:03:11 | `gh run list`, `gh pr view --json` |
| Q4 | 12:03–12:23 | PR #6, head `9bb1a82`, `[skip ci]` in the title: `probe` on `push` (run 45816096985), `check` on `pull_request` (run 45816106900), `probe` on `pull_request_target` (run 45816107215). Merged at 12:23:18 as `90fd34f`, message "Merge pull request #6 from epicfail/q/title" with the title in the body. Push to `main`: no run, 0 check suites (read at 12:25:55 and 14:48) | `gh pr view`, stored message, `gh run list`, check suites |
| R2 | 12:17–16:17 | No `schedule` run at 12:17, 13:17, 14:17, 15:17 or 16:17. At 12:17 the tip of `main` was `29c3db4`, without a marker; from 13:17 it was `90fd34f`, with the marker. Workflow `probe` `active`. Last read at 16:38:13 | `gh run list --event schedule`, `gh api …/actions/workflows` |
| W1 | 12:34:17 | Direct push to `main`, `e32763a..bfe19ed`, `[skip ci]` in the subject: no run, 0 check suites (read at 12:36:30 and 14:48) | `git push` output, `gh run list`, check suites |
| W2 | 12:36:46 | Direct push to `main`, `bfe19ed..f3f415d`, clean: `check` on `push` (run 45821286247), success. One job in total for block W | `git push` output, `gh run list` |
| H, R4 | 14:52 (H, R4a–R4f), 16:40 (R4g) | Local results below | Exit code, hook invocation log, commit count, remote refs |

## Results

Every cell is tested unless it says otherwise.

### Markers on GitHub

| Behaviour | Result | Evidence |
|---|---|---|
| The five documented strings, in the subject | No run and no check suite | K2–K6 |
| Marker in the body only | No run | K7 |
| `skip-checks` trailer after two empty lines | No run | K8 |
| Same trailer after Git's default cleanup | Run: the cleanup left one empty line, and in this case the trailer did not skip | K9 |
| `[SKIP CI]` | Run: in this case the upper-case string did not skip | K10 |
| Marker on a non-tip commit of a push, clean tip | Run, observed in K11 | K11 |
| Marker on the tip of a push | No run, also for the clean non-tip commit of the same push | K12 |
| Start point of the next run after a skipped push | `before` is the skipped tip, so a `before..after` range excludes the skipped commits. Public: from the probe log (K9, K13). Private: from the client push output only (W2) | K9, K13, W2 |
| Trace of a skipped push | No run and no check suite were observed in the cases tested. Other traces were not examined | K2–K8, K12, Q4, W1 |
| Marked head commit of a PR, required check | `check` does not start; merge blocked server side (HTTP 405, "Required status check "check" is expected."); a clean commit unblocks it | Q1–Q3 |
| `pull_request_target` with a marked PR head | Runs, on the tip of the base branch; the PR head is available only in the payload | Q1 |
| Marker in the PR title | The PR runs; the merge commit composed by GitHub carries the title, and the push to `main` starts no run | Q4 |
| `schedule` | No run in five hourly slots. Inconclusive: the first slot fell on a `main` without a marker, so the marker does not explain that missing run, and the later slots cannot separate the marker from the undetermined cause | R2 |
| Private repository, `[skip ci]` on a direct push to `main` | No run, as for the same string on the public repository (K2) | W1, W2 |

### Local hooks

| Behaviour | Result | Evidence |
|---|---|---|
| `commit-msg` on `git commit`, `--amend`, `git merge` | Invoked; every marked message rejected, comment lines with `--cleanup=verbatim` included | H1–H4b, H6, H7 |
| Marker on a `#` line, default cleanup with `-F` | Rejected, correctly: with `-F` the message is not edited, the cleanup is `whitespace` (E13), and the `#` line stayed in the commit (checked) | H4c |
| What the hook receives | With `-F`, the empty lines were already collapsed (H3b) and `#` lines were present (H4c). With `--edit -v`, comment lines and the diff below the scissors line were present; the fixture ignored the diff | H3b, H4c, H12 |
| `git cherry-pick`, also with `-e` | Hook not invoked; marked commit created | H8, H8b |
| `git rebase` | Hook not invoked; both commits of the branch replayed, the marked one included. The commit count shows one new commit only, because the replayed marked commit is the same object created by H8 | H9 |
| `git rebase -i` with `reword` | Hook invoked; rejected; the rebase stopped, and after `git rebase --abort` the branch was unchanged | H9b |
| `git revert`, with `--no-edit` or with the editor | Hook not invoked; the revert commit quotes the marked subject (`Revert "test: S1 [skip ci] …"`) | H10, H10b |
| Attribution lines naming Claude | Rejected | H11a–H11c |
| Human `Co-authored-by` | Accepted | H11d |
| `Co-authored-by` naming another agent | Accepted: the fixture recognizes Claude only. A limit of the prototype; it concerns §7.7, not §6.4 | H11e |
| `pre-push`, marker on the tip or on a non-tip commit introduced by the push | Rejected. Rejecting a non-tip marker is stricter than what GitHub did in K11: a choice of the fixture | R4b, R4c |
| `pre-push`, boundary on commits already on the remote | New branch whose tip is a marked commit already on the remote: rejected (R4d). New branch with a clean commit on top of that marked commit: accepted (R4g). The boundary is the commits introduced by the push plus the new tip of the ref | R4d, R4g |
| `pre-push`, clean pushes and deletion | Accepted | R4a, R4e, R4f |

Not tested on GitHub: whether a push that creates a branch on an existing marked commit (the R4d case) starts a workflow.

## Remedy evaluation

Criterion of §6.4 (v5.1): the secret-scanning job runs on every push, on any branch, over all pushed commits, including on a private repository with no other remote defense.

| Remedy | Covers, in the cases tested | Does not cover, or not shown | Cost on the private repository |
|---|---|---|---|
| Local `commit-msg` hook | Markers in commits created by `git commit` (also `--amend`), `git merge` and the reword step of `git rebase -i` (H1–H7, H9b), with a pattern wider than what GitHub matched | Commits created by `git cherry-pick`, non-interactive `git rebase` and `git revert`, with or without the editor: the hook is not invoked (H8–H10b). Messages composed by GitHub (Q4). `--no-verify` (documented, E11). Clones without the hook | None |
| Local `pre-push` check | The cases R4a–R4g: markers on the tip, on non-tip commits introduced by the push, on a new tip already on the remote; no rejection for clean pushes, deletions and a clean commit on top of a marked commit already on the remote | Not tested: pushes of several refs at once, force pushes, tags, history not available locally. Messages composed by GitHub (Q4). `--no-verify` (documented, E11). Clones without the hook. It checks markers, not secrets | None |
| `pull_request_target` workflow | Pull requests whose head commit carries a marker (Q1, as E3 states) | Branches without a pull request. Direct pushes to `main` of the private repository, which are accepted there (capability matrix V3; W1). Merge commits composed by GitHub (Q4). The run is on the tip of the base branch; the PR head must be read from the payload (Q1) | One job per pull request event |
| `schedule` workflow | Documented as not subject to markers (E3). Not shown: no scheduled run was observed (R2, inconclusive) | Commits made unreachable before the scheduled run. Delays and dropped jobs (E9). Runs on the default branch (E9): other branches only if the job fetches them | Deduced: about USD 0.18 per month for one daily run (30 jobs × 1 minute × USD 0.006; capability matrix E15–E16) |
| Push-triggered job that scans beyond `before..after` | Deduced from the tests, not tested with a scanner: commits skipped by an earlier push, at the next successful scan whose history reaches them. Basis: the next run's `before` was the skipped tip, read from the probe log on the public repository (K9, K13) and from the client push output only on the private repository (W2) | Recovery of a secret in a skipped commit not shown. Commits not reached by a later successful scan (residual limit below). Merge commits and history boundaries depend on the implementation (phase 4) | Not measured. Each job is rounded up to the whole minute (capability matrix E16), so the cost equals a `before..after` scan while the whole job stays under one minute; the job duration and its growth with the history are not measured |

**Residual limit.** A commit skipped by one push is covered only if a later successful scan includes it, through any ref whose history reaches it, not only the same branch. A branch deletion, a history rewrite or the absence of a later scan can leave it unscanned. Deleting a ref does not delete the commit, which can stay reachable by its SHA on GitHub (capability matrix V8), but a scan of the refs no longer reaches it.

**`pull_request_target`, owner's condition.** The event is admissible only if the job treats the pull request's code as data, without running it, with a read-only token. Q1 adds that the job runs on the tip of the base branch and must take the PR head commit from the payload explicitly.

**Push protection, owner's note.** Skip instructions are documented for GitHub Actions workflows only (E1, E3). Push protection on the public repository (capability matrix P11) is documented as a check on the push itself (capability matrix E10, E13); its independence from markers follows from that documented scope. Not tested in combination: the P11 push carried no marker. No new test was run.

**Not evaluated in the tests.** A `workflow_dispatch` trigger, suggested by the independent review as an optional manual recovery, is listed below as not verified.

An independent review of this evaluation (GPT-6, 2026-11-26) supports a push-triggered job that scans beyond `before..after`, with the `pre-push` check as the main local control of markers and the `commit-msg` hook as earlier feedback, and does not support adopting `pull_request_target` or `schedule` as remedies. It asked for the corrections of wording applied in this record. The choice of the remedy belongs to the owner and is made in the rules-only PR on §6.4 that follows this record.

## Implications for later phases

- Specification: §6.4 defers the skip-marker remedy to this check ("The remedy for skip markers is chosen after the Q9 check."). The change goes in a rules-only PR, with an independent review of its final diff.
- Phase 4, scanning job: a range `before..after` misses commits of skipped pushes (K9 and K13, from the probe log on the public repository; W2, from the client push output only on the private repository). The job's coverage must be defined and tested (list below).
- Phase 4, missing runs: a skipped push left no run and no check suite in the cases tested, and a job that does not start cannot report that it did not start. Without something else looking for them, missing runs can go unnoticed (deduced).
- Phase 4, gate and merge: markers can reach `main` through a message composed by GitHub (Q4), which no local hook sees. Depending on the settings, the message can include the PR title or the title and description (E10, O2).
- Phase 4, required checks: with a marked PR head, the status rollup showed SUCCESS while the required check was missing (Q1). The merge state, not the rollup, shows the block.

## Verification for phase 4

From the independent review. These are checks of the implementation, not new normative rules.

| Check | Reason |
|---|---|
| Actual recovery of a secret in a skipped commit | It is the central property of the remedy |
| Secret introduced and removed in intermediate commits | Scanning the tip only would miss it |
| Secret introduced during a merge | Checks how the scanner treats merge commits |
| Exclusion of the upstream history | Avoids both false positives and the exclusion of the project's own commits |
| Synchronization of a fork with its upstream | Checks that the history boundary stays valid |
| First push and creation of new branches | No previous remote tip exists |
| Force push | Checks the scope of the new history |
| Pushes of tags and of several refs | Checks the coverage of the job and of the `pre-push` check |
| Full history available on the runner | A shallow checkout can make the scan incomplete |
| Branch where the workflow is missing or modified | Checks and declares the limit of the remote control |
| PR metadata changed after the gate | Avoids stale results |
| Custom merge and squash messages | Checking the title alone is not enough |
| Actual duration of the job | Validates the cost assumption based on rounding per job |
| Credential formats of AI services | Already planned under Q9 |

## Not verified, not blocking

From the independent review. None of these needs to be settled before closing issue #14; they stay recorded as not verified.

- R2: whether the scheduled workflow actually starts.
- GitHub's behaviour when a branch is created on an existing marked commit (the R4d case).
- Whether markers on non-tip commits are ignored in general.
- Independence of push protection from markers, by a combined test.
- Billed cost of the single W2 job.
- Feasibility of a manual recovery through `workflow_dispatch`.
- Automatic detection of missing runs through the API.
- Effectiveness of an incremental check based on the last successful scan.
