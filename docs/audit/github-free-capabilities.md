# Phase 1 GitHub Free capability matrix

| Field | Value |
|---|---|
| Work item | Issue #8 |
| Test repositories | `epicfail/dm-test-public` (public), `epicfail/dm-test-private` (private), owned by a personal account on GitHub Free |
| Reference specification | `docs/methodology/specification.md`, v5.1, §4.6 and §6.4 |
| Tools | `gh` 2.102.0, `git` 2.56.1 |
| Status | Test protocol approved by the owner on 2026-11-25. Tests completed on 2026-11-25; the billing page reading for V9 is pending |
| Normative status | Non-normative |

The record states what the platform does for the account and the date of the tests. GitHub can change its plans and defaults: a result is valid at its date only.

## Evidence classes

| Class | Meaning |
|---|---|
| tested | Observed in this activity on a test repository; the record gives the action and the relevant output |
| observed before | Observed earlier on another repository, with the reference |
| documented | Read in GitHub documentation, with the URL and the date of reading; not tested |
| not stated | The documentation read does not say it |

## Observations before the tests

Read-only, 2026-11-25.

| # | Observation | Evidence |
|---|---|---|
| O1 | Both test repositories are empty: no default branch | `gh repo view --json defaultBranchRef` returns an empty name |
| O2 | On `dm-test-private`, `GET /repos/{owner}/{repo}/rulesets` fails with HTTP 403: "Upgrade to GitHub Pro or make this repository public to enable this feature." | `gh api` output |
| O3 | On `dm-test-private`, `security_and_analysis` is `null` | `gh api repos/{owner}/{repo}` |
| O4 | On `dm-test-public`, secret scanning and push protection are enabled; the owner did not enable them | `security_and_analysis.secret_scanning_push_protection.status = "enabled"`; owner's statement. Deviation D1 |
| O5 | On the fork (public), a ruleset without bypass actors rejects direct pushes to `main`, including the owner's | Observed before: stage 0, recorded in §10 of the specification (Q9) |
| O6 | Billing page, 2026-11-25 22:45: `dm-test-private` does not appear in "Usage by repository" (usage 0) | Owner's reading, baseline for V9 |

## Documented expectations

Read on 2026-11-25. The pages show no last-updated date. Every quoted string was matched against the page text on that date.

| # | Statement | Source |
|---|---|---|
| E1 | The rulesets overview names "GitHub Team and GitHub Enterprise plans" for organizations; availability for personal accounts on GitHub Free is not stated. O2 suggests that public repositories are allowed | [About rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/about-rulesets) |
| E2 | Classic branch protection: "By default, the restrictions of a branch protection rule don't apply to people with admin permissions to the repository", unless "Do not allow bypassing the above settings" is enabled | [About protected branches](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches) |
| E3 | Ruleset bypass: `actor_type` includes `RepositoryRole`; `bypass_mode` is `always`, `pull_request` or `exempt`. The `actor_id` of the repository admin role is not stated; the protocol assumes `5` and checks it by reading the ruleset back | [REST: rules](https://docs.github.com/en/rest/repos/rules) |
| E4 | `deletion`: "only users with bypass permissions can delete branches or tags whose name matches the pattern"; `non_fast_forward` blocks force pushes; `pull_request` requires changes through a pull request, optionally with approvals and code-owner review; `required_status_checks` identifies a check by `context`, with `integration_id` optional | [Available rules for rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/available-rules-for-rulesets), [REST: rules](https://docs.github.com/en/rest/repos/rules) |
| E5 | "Pull request authors cannot approve their own pull requests." | [Approving a pull request with required reviews](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/reviewing-changes-in-pull-requests/approving-a-pull-request-with-required-reviews) |
| E6 | `CODEOWNERS` lives in `.github/`, the root or `docs/`, and must be on the base branch of the pull request. Whether an author who is also the code owner gets a review request is not stated | [About code owners](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners) |
| E7 | Standard GitHub-hosted runners are free in public repositories. GitHub Free includes a monthly amount for private repositories "expressed as a monetary amount, not a fixed number of minutes" (USD 12). Rounding of job time is not stated. The billing dashboard "may show your Actions usage as a dollar amount" | [About billing for GitHub Actions](https://docs.github.com/en/billing/managing-billing-for-your-products/managing-billing-for-github-actions/about-billing-for-github-actions) |
| E8 | Changelog of 2024-03-11: "All new public repositories owned by personal accounts will now have secret scanning and push protection enabled by default." "Existing public repositories are not affected, nor are new public repositories that belong to an organization." | [GitHub changelog, 2024-03-11](https://github.blog/changelog/2024-03-11-secret-scanning-and-push-protection-are-enabled-by-default-on-new-public-repositories/) |
| E9 | Current documentation contradicts E8 and O4. Repository push protection "Is disabled by default, and can be enabled by a repository administrator"; secret scanning and push protection "aren't enabled by default, even for public repositories" | [Push protection](https://docs.github.com/en/code-security/concepts/secret-security/push-protection), [Managing security and analysis settings](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-security-and-analysis-settings-for-your-repository) |
| E10 | Push protection for users is a separate, per-account layer. It "Is enabled by default" and "Stops you from pushing secrets to public repositories on GitHub", "regardless of whether the repository itself has secret scanning enabled". Its effect on private repositories is not stated | [Push protection](https://docs.github.com/en/code-security/concepts/secret-security/push-protection), [GitHub changelog, 2024-02-29](https://github.blog/changelog/2024-02-29-push-protection-is-enabled-for-free-users-on-github/) |
| E11 | Availability table: push protection "Yes" for a public repository and "No" for a private repository without GitHub Secret Protection | [About GitHub Advanced Security](https://docs.github.com/en/get-started/learning-about-github/about-github-advanced-security) |
| E12 | GitHub documents a dummy value of type "GitHub Secret Scanning secret" for practising with push protection; its tutorial shows a block on a commit made in the web interface. The behaviour on a command-line push is not stated. "If a particular secret has already been detected in the repository and an alert already exists, GitHub will not block that secret." | [Storing your secrets safely](https://docs.github.com/en/get-started/learning-to-code/storing-your-secrets-safely), [Command-line push protection](https://docs.github.com/en/code-security/concepts/secret-security/command-line-push-protection) |
| E13 | On a blocked command-line push, the remote output names the secret type and its location (commit, path and line), with a URL to unblock it. No error code for push protection is stated; `GH013` is documented only for ruleset violations | [Push protection on the command line](https://docs.github.com/en/code-security/how-tos/secure-your-secrets/work-with-leak-prevention/push-protection-on-the-command-line) |

Observation on E13 (P11): the error code `GH013` also appears for push protection, not only for rulesets. The blocked push reports `GH013: Repository rule violations found for refs/heads/pp/block`, followed by a "GITHUB PUSH PROTECTION" section.

## Fixtures

The initial commit of both test repositories contains:

| File | Content |
|---|---|
| `README.md` | One line: test repository for phase 1 of the methodology fork, not a real project |
| `.github/CODEOWNERS` | `* @epicfail` |
| `.github/workflows/check.yml` | Job `check`, triggered by `pull_request` and by `push` to `main`, no permissions (`permissions: {}`), no third-party actions. It fails when the head branch name starts with `fail/` and passes otherwise; the branch name reaches the shell through an environment variable, never through expression interpolation |

Rulesets applied to `dm-test-public` (`main` as `~DEFAULT_BRANCH`). They start from `.github/rulesets/main.json` of this repository, a shape GitHub already accepted, and add the status check:

| Stage | Rules | Bypass actors |
|---|---|---|
| A | `deletion`, `non_fast_forward`, `pull_request` with 0 approvals, `required_status_checks` with context `check` (not strict) | None |
| B | As A | `RepositoryRole`, `actor_id: 5`, `bypass_mode: always`. That 5 is the admin role is an assumption (E3) |
| C | As A, with `pull_request` requiring code-owner review and 0 approvals | None |
| C1 | As C, with 1 approval. Used only if the API rejects C | None |

The stage A ruleset is the final state of `dm-test-public` after the tests.

The push protection tests use only the dummy value published by GitHub (E12). It is not an invented credential of a real provider, and it is already public. This record does not reproduce it.

## Test cases

Every command runs only after the owner's approval. `$PUB` is `epicfail/dm-test-public`, `$PRIV` is `epicfail/dm-test-private`; clones live in a working directory outside the repository. No command uses `--no-verify`, a push protection bypass or `gh pr merge --admin`, except P6, which tests that the admin override is refused.

Order: the owner reads the billing page (O6); S1–S3; P1–P11; V1–V9. After every rejected push, the local branch is reset to its remote state (`git reset --hard origin/main` for `main`), so that the next test starts from the remote state. A ruleset rejects a push with one message listing every violated rule: for each test, the evidence is the line that names the rule under test.

### Setup

| ID | Repository | Action | Expected |
|---|---|---|---|
| S1 | both | Clone; check that the commit identity is the owner's noreply identity | Empty clones |
| S2 | both | Initial commit with the fixtures on `main`; `git push -u origin main` | Accepted: no protection yet. On the public repository, push protection scans the push and finds nothing |
| S3 | both | Read `security_and_analysis` and the list of rulesets | Same as O2–O4 |

### Public repository

| ID | Capability | Action | Expected | Basis |
|---|---|---|---|---|
| P1 | Ruleset available | `gh api -X POST repos/$PUB/rulesets --input ruleset-a.json` | HTTP 201 | E1, O2, O5 |
| P2 | Direct push blocked, owner included | Commit on `main`; `git push origin main`; reset | Rejected; the message names the pull request rule | E4, O5 |
| P3 | Force push blocked | From `origin/main`, `git commit --amend` (rewrites the remote tip); `git push --force origin HEAD:main`; reset | Rejected; the message names the force-push rule, not only the pull request rule | E4 |
| P4 | Deletion blocked | `git push origin --delete main` | Rejected; the message shows whether the deletion rule or the default-branch rule refused it | E4 |
| P5 | Failing required check blocks the merge | PR from `fail/check`; after the check fails, `gh pr merge N --merge` | Refused; `mergeStateStatus` is `BLOCKED` | E4 |
| P6 | Admin override refused without bypass actors | On the P5 PR, `gh pr merge N --merge --admin`; then close the PR | Refused | E3, O5 |
| P7 | Passing required check allows the merge | PR from `pass/check`; after the check passes, `gh pr merge N --merge` | Merged. Control case for P5 | E4 |
| P8 | Bypass actor allows a direct push | `gh api -X PUT repos/$PUB/rulesets/ID --input ruleset-b.json`; read it back; the owner checks on the ruleset page that the bypass list shows the repository admin role; direct push to `main` | Accepted. If the page does not show the admin role, the result is inconclusive and recorded as such | E3 |
| P9 | Removing the bypass actor restores the block | `PUT` with `ruleset-a.json`; direct push to `main`; reset | Rejected | E3 |
| P10 | Required code-owner review | `PUT` with `ruleset-c.json` (C1 if C is rejected); PR from `pass/codeowners`; record `reviewRequests`; `gh pr review N --approve`; after the check passes, `gh pr merge N --merge`; close the PR; `PUT` with `ruleset-a.json` | Approval refused (own PR); merge refused. With C, the block comes from code-owner review alone; with C1, the approval count also blocks and the result cannot be attributed to code-owner review | E5, E6 |
| P11 | Push protection blocks the dummy value, both layers active | Branch `pp/block` with the E12 dummy value; `git push origin pp/block`; record the output with the value redacted; no bypass | Rejected. The block cannot be attributed to one layer: repository (O4) and user (E10) are both active. If accepted: inconclusive, because the dummy's coverage on command-line pushes is not documented (E12); the remote branch is deleted and the following push protection tests stop | E8, E10, E12, E13 |
| P12 | Repository layer alone | Not run: it requires turning off push protection for users on the owner's account. Not tested, owner's decision | Not applicable | E8, E10 |

### Private repository

| ID | Capability | Action | Expected | Basis |
|---|---|---|---|---|
| V1 | Ruleset not available | `gh api -X POST repos/$PRIV/rulesets --input ruleset-a.json` | HTTP 403 | O2 |
| V2 | Classic branch protection not available | `gh api -X PUT repos/$PRIV/branches/main/protection --input protection.json` | HTTP 403 | Not stated |
| V3 | Direct push not blocked | Commit on `main`; `git push origin main` | Accepted | V1, V2 |
| V4 | Force push not blocked | Amend the tip of `main`; `git push --force origin HEAD:main` | Accepted | V1, V2 |
| V5 | Deletion of `main` | `git push origin --delete main` | Refused only as the default branch, not by a protection; the message is recorded | Not stated |
| V6 | Failing check does not block the merge; `CODEOWNERS` not enforced | PR from `fail/check`; record `reviewRequests`; after the check fails, `gh pr merge N --merge` | Merged | §6.4 |
| V7 | Repository push protection not available | `gh api -X PATCH repos/$PRIV --input pp-enable.json`; read `security_and_analysis` back | Error or no effect | O3, E11 |
| V8 | No push protection layer on a private repository (only if P11 is rejected) | Branch `pp/absent` with the E12 dummy value; `git push origin pp/absent`; delete the remote branch | Accepted: no repository layer (E11); the user layer covers public repositories only (E10). Without a P11 block, an acceptance here proves nothing | E10, E11 |
| V9 | Actions usage | `gh run list` and `gh run view --json jobs` for job durations; the owner reads the `dm-test-private` row of "Usage by repository" on the billing page, before S2 and after the last run. The account total is not used, because other repositories of the account consume Actions in parallel; no other repository is named in this record. If the row has not appeared yet after the last run, this is recorded and the reading is repeated later | Usage on the private repository only, within the included amount | E7 |

## Limits of the protocol

- Single account: approval by a code owner other than the author cannot be tested. P10 proves only that the merge stays blocked without that approval. Accepted by the owner.
- Push protection bypass is not tested, by the owner's decision. Its behaviour is documented only.
- P12 is not run, by the owner's decision. The P11 block therefore proves the combination of the two layers, not the repository layer alone. A contributor who turns off push protection for users relies on the repository layer only, which remains documented (E8, E11) but not tested.
- If the P11 push is accepted, the dummy value has reached the public repository. Deleting the branch does not remove the commit, which can stay reachable by its SHA. The harm is nil, because the value is a public dummy, but an alert may be created, and an existing alert stops later blocks of the same value (E12). The outcome is recorded and the following push protection tests stop. Fallback, owner's decision: repeat the tutorial's web-interface commit (E12), or record push protection on command-line pushes as not proven.
- P6 is a real admin-override attempt: if the expectation is wrong, it merges a pull request with a failing check into `main` of the test repository. The merge is recorded as the result.
- The dummy value has a GitHub-specific type; recognition of real providers' formats, including AI credentials, belongs to phase 4.
- Actions usage is read by the owner from the billing page; the token scopes stay unchanged. The page may show spend instead of minutes and may update with a delay.
- The source of required status checks is not pinned (`integration_id` unset): that check belongs to phase 4.
- The administrator can always change or delete a ruleset (§4.6, declared limits): not tested, because it is the documented behaviour of the role.

## Deviations from the specification

| # | Section | Specification | Observed | Proposal |
|---|---|---|---|---|
| D1 | §6.4, secret control chain, step 2, public repository | Repository push protection "to enable at bootstrap (off by default)" | Enabled at creation without the owner's action (O4), as announced in E8 for new public repositories of personal accounts. Current documentation still says off by default (E9); repositories of organizations and existing repositories are not covered by E8 | After the matrix closes, a rules-only PR: verify at bootstrap, enable only if off |

## Execution log

Times in UTC.

| ID | Date | Outcome | Evidence |
|---|---|---|---|
| S1 | 2026-11-25 | Both clones empty; commit identity is the owner's noreply identity; no hooks active in the clones | `git status`, `git config` |
| S2 | 2026-11-25 21:54 | Public: initial commit `c0c851d` pushed to `main`, accepted. Private: initial commit `5aedb64` pushed to `main`, accepted | `git push` output: `* [new branch] main -> main` |
| S3 | 2026-11-25 | Default branch `main` on both. Public: secret scanning and push protection enabled, no rulesets. Private: `security_and_analysis` null, rulesets HTTP 403 as in O2 | `gh api` output |
| V9, run 1 | 2026-11-25 21:54 | Private: workflow `check` on `push` to `main`, success; job `check` from 21:54:31 to 21:54:34 (3 s); run duration 6 s. The `timing` endpoint reports billable `total_ms: 0` for the job. Public, for comparison: job 4 s, billable 0 | `gh run view --json jobs`, `GET /repos/{owner}/{repo}/actions/runs/{id}/timing` |
| P1 | 2026-11-25 | Ruleset `protect-main` (stage A) created: HTTP 201, `enforcement: active` | `gh api --include` |
| P2 | 2026-11-25 | Direct push to `main` by the owner rejected: `GH013: Repository rule violations found for refs/heads/main`, with "Changes must be made through a pull request." (and "Required status check "check" is expected.") | `git push` output |
| P3 | 2026-11-25 | Force push of a rewritten tip (non-fast-forward checked with `git merge-base --is-ancestor`) rejected: `GH013`, with "Cannot force-push to this branch" | `git push` output |
| P4 | 2026-11-25 | Deletion of `main` rejected by the default-branch rule, not by the ruleset: "refusing to delete the current branch: refs/heads/main". Deletion rule not proven: GitHub refuses to delete the default branch anyway. Not needed for `main`; if the default branch changes, `~DEFAULT_BRANCH` follows it (administrator powers, §4.6 declared limits) | `git push` output |
| P5 | 2026-11-25 | PR #1 from `fail/check`: check `check` failed; `mergeStateStatus: BLOCKED`. `gh pr merge` refused ("the base branch policy prohibits the merge", a client-side message); the REST merge endpoint refused server side: HTTP 405, "Required status check "check" is failing." `reviewRequests` empty: the author, who is the only code owner, gets no review request | `gh pr view --json`, `gh pr merge`, `PUT /repos/{owner}/{repo}/pulls/1/merge` |
| P6 | 2026-11-25 | `gh pr merge --admin` on PR #1 refused server side: "Repository rule violations found. Required status check "check" is failing." (GraphQL `mergePullRequest`). PR #1 closed unmerged | `gh pr merge --admin` output |
| P7 | 2026-11-25 22:01 | PR #2 from `pass/check`: check passed, `mergeStateStatus: CLEAN`, merged (`f20b5c4`) | `gh pr view --json`, `gh pr merge` |
| P8, setup | 2026-11-25 | Ruleset updated to stage B: HTTP 200; read back `bypass_actors: [{actor_id: 5, actor_type: RepositoryRole, bypass_mode: always}]`, `current_user_can_bypass: always`. The owner checked the ruleset page: the bypass list shows "Repository admin" (Roles), "Always allow" | `gh api`; owner's check of the ruleset page |
| P8 | 2026-11-25 | Direct push to `main` accepted (`f20b5c4..bd4bb6a`), with "Bypassed rule violations for refs/heads/main" listing the pull request rule and the required check | `git push` output |
| P9 | 2026-11-25 | Ruleset restored to stage A: HTTP 200, `bypass_actors: []`, `current_user_can_bypass: never`. Direct push to `main` rejected again: `GH013`, "Changes must be made through a pull request." | `gh api`, `git push` output |
| P10 | 2026-11-25 22:28 | Different from expected: with a single account, code-owner review with 0 required approvals does not block the merge. Cause not determinable with one account (H1: code-owner review is enforced only together with at least one required approval; H2: the requirement counts as met when the author is a code owner; both not verified). Details: stage C accepted by the API (HTTP 200) and confirmed by the rules in effect on `main`; `CODEOWNERS` valid on the base branch (`codeowners/errors` empty). PR #3 from `pass/codeowners`, opened by the only code owner: check passed, `mergeStateStatus: CLEAN`, `reviewRequests` empty. Self-approval refused server side: "Can not approve your own pull request" (GraphQL `addPullRequestReview`). With stage C1 (1 approval and code-owner review): `BLOCKED`, `REVIEW_REQUIRED`; REST merge refused, HTTP 405, "At least 1 approving review is required by reviewers with write access." Back to stage C: REST merge accepted, `merged: true` (`e5063d7`). Ruleset then restored to stage A (HTTP 200, `bypass_actors: []`) | `gh api`, `gh pr view --json`, `gh pr review`, `PUT /repos/{owner}/{repo}/pulls/3/merge` |
| P11 | 2026-11-25, after 23:03 | As expected, push rejected (`GH013`, "GITHUB PUSH PROTECTION", "Push cannot contain secrets"), location commit and path:line shown. Both layers active, so the block is not attributable to one layer. Output: `GH013: Repository rule violations found for refs/heads/pp/block`; secret type "GitHub Secret Scanning"; location commit `908c946`, path `tests/p11.txt:2`; an unblock URL (`…/security/secret-scanning/unblock-secret/[REDACTED]`). The value does not appear in the output. The push was run by the owner from their terminal, twice, with the same result. No unblock was attempted | `git push` output pasted by the owner |
| V8 | 2026-11-25, after 23:06 | As expected: no push protection layer on a private repository; the user layer does not apply (E10, E11). The owner pushed branch `pp/absent` with the E12 dummy value from their terminal: accepted (`* [new branch] pp/absent -> pp/absent`); then deleted the remote branch (`- [deleted] pp/absent`). Commit `c7be599` stays reachable by SHA; it contains the dummy value only | `git push` output pasted by the owner |
| V1 | 2026-11-25 | Ruleset creation refused: HTTP 403, "Upgrade to GitHub Pro or make this repository public to enable this feature." | `gh api` output |
| V2 | 2026-11-25 | Classic branch protection refused: HTTP 403, same message | `gh api` output |
| V3 | 2026-11-25 23:08 | Direct push to `main` accepted (`5aedb64..cb7d670`) | `git push` output |
| V4 | 2026-11-25 23:08 | Force push of a rewritten tip (non-fast-forward checked) accepted: `+ cb7d670...e7a3816 HEAD -> main (forced update)` | `git push` output |
| V5 | 2026-11-25 | Deletion of `main` refused only as the default branch: "refusing to delete the current branch: refs/heads/main" | `git push` output |
| V6 | 2026-11-25 23:08 | PR #1 from `fail/check`: check failed; `mergeStateStatus: UNSTABLE`, `mergeable: MERGEABLE`, `reviewRequests` empty; merged with the failing check (`e32763a`). `CODEOWNERS` present, no review required | `gh pr view --json`, `gh pr merge` |
| V7 | 2026-11-25 | `PATCH` enabling push protection returned HTTP 200 without error, with no effect: the response showed `secret_scanning` disabled, and the read-back `security_and_analysis` is null. The HTTP status alone does not show whether a setting was applied | `gh api --include`, read-back |
| V9 | 2026-11-25 | Five runs on the private repository, all on the workflow `check`: S2 push, V3 push, V4 push, V6 pull request (failure), V6 merge push. Job times 3, 3, 2, 5 and 2 s (15 s in total); run durations 5–8 s. The `timing` endpoint reports billable `total_ms: 0` for every run. Billing page row: PENDING (owner's reading) | `gh run list`, `gh run view --json jobs`, `GET …/actions/runs/{id}/timing` |

## Results

Tests run on 2026-11-25 (UTC). Every cell is tested unless it says otherwise.

**Review with a single account (P10).** No setting imposes a review server side: with 0 required approvals, code-owner review does not block the merge; with 1 or more, the merge is blocked entirely, because the author cannot approve their own pull request. The owner's review remains a process step (§6.3).

| Capability | Public | Private | Evidence |
|---|---|---|---|
| Rulesets | Available | Not available (HTTP 403) | P1, V1, O2 |
| Classic branch protection | Not tested | Not available (HTTP 403) | V2 |
| Direct push to `main` blocked, owner included | Yes, with a ruleset without bypass actors | No: accepted | P2, P9, V3 |
| Force push to `main` blocked | Yes ("Cannot force-push to this branch") | No: accepted | P3, V4 |
| Deletion of `main` blocked | Refused as the default branch; deletion rule not proven | Refused as the default branch; no protection | P4, V5 |
| Failing required check blocks the merge | Yes, server side (HTTP 405) | No: merged with the failing check | P5, V6 |
| Admin override without bypass actors | Refused | Not applicable | P6 |
| Bypass actor allows a direct push | Yes, with a "Bypassed rule violations" notice | Not applicable | P8 |
| Required code-owner review blocks the merge (single account) | No with 0 required approvals; with 1, the merge is blocked by the approval count, not attributable to code-owner review | Not available (no rulesets); merged without review | P10, V6 |
| Self-approval of a pull request | Refused server side | Not tested | P10 |
| Repository push protection, state at creation | Enabled (O4) | Not available (O3) | O3, O4, E8, E9 |
| Repository push protection can be enabled | Not tested (already on) | No: `PATCH` returns HTTP 200 with no effect | V7 |
| Push protection blocks the dummy value, both layers | Yes (`GH013`, "Push cannot contain secrets") | Not applicable | P11 |
| Push protection blocks the dummy value, repository layer alone | Not tested, owner's decision | Not applicable | P12 |
| Any push protection layer on a private repository | Not applicable | None: push accepted | V8 |
| GitHub Actions usage | Free on standard runners (documented); `timing` endpoint billable 0 | 5 runs, 15 s of job time; `timing` endpoint billable 0; billing page row PENDING | E7, V9 |

## Implications for later phases

- Phase 4, bootstrap: always read the state back after a change; HTTP 200 alone does not prove a setting was applied (V7).
- Phase 4, output parsing: `GH013` is used for both ruleset violations and push protection (P11, note on E13).
- Process: with a single account no review can be enforced server side; the owner's review remains a process step (P10, §6.3).
- Specification: deviation D1 on §6.4, rules-only PR after the matrix is merged.
