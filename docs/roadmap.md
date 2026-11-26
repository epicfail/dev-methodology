# Roadmap of the methodology fork

Status: non-normative planning document, subordinate to `docs/methodology/specification.md` (v5.1). It is the source of truth for planning from 2026-11-25. It changes through pull requests, at phase transitions or when the plan changes; the owner approves every change.

## Current status

| Item | Status |
| --- | --- |
| Stage 0 — repository bootstrap | Completed on 2026-11-24 (PRs #1–#3): public fork protected by a ruleset that applies to the owner too, upstream instructions neutralized, bootstrap contract in `AGENTS.md` |
| Phase 0 — design closure | Completed on 2026-11-25: specification v5.1 frozen and added to the repository (PR #4) |
| Phase 1 — fork and audit | In progress. Done: baseline tag `upstream-baseline` (`53af85eeff92a0f09d65c61dea159f1e8565c38e`); upstream inventory (`docs/audit/upstream-inventory.md`); GitHub Free capability matrix (`docs/audit/github-free-capabilities.md`, PR #9), with the specification corrections it required (PR #12); skip-marker check (`docs/audit/skip-markers.md`, PR #15), with the remedy in §6.4 (PR #17). Remaining: instruction loading in Claude Code and Codex, end-of-phase independent review |

## Phases

Every phase has a verifiable exit criterion. References in brackets point to sections of the specification.

| Phase | Goal | Products | Depends on | Exit criterion |
| --- | --- | --- | --- | --- |
| **0. Design closure** | Make the rigor model executable | Classification criteria (Q1), combination rules (Q2), level → activities (Q3), expected scenarios; specification v5.1 | — | Done |
| **1. Fork and audit** | Know what exists and what the platform really offers | Baseline tag; classified inventory of the upstream repository; GitHub Free capability matrix tested on `epicfail/dm-test-public` and `epicfail/dm-test-private` (the public case is partly proven on the fork); loading of `AGENTS.md` and skills in Claude Code and Codex, with a canary rule (§7.5) in a clone of the public test repository; skip-marker check on the secret-scanning job and choice of the remedy | Phase 0 | Every element of the upstream repository classified; platform capabilities proven, not only read in the documentation; results recorded under `docs/audit/` |
| **2. Repository structure** | Turn the three levels into files | Principles and catalogue (§2); short `AGENTS.md` pointing to the decisions file (§7.2); procedures as skills; decisions file schema v1 with validator (§4.4); Claude Code adapter only if needed (§7.3); distribution and versioning (inventory open point B2); language convention for distributed artifacts (B5) | Phase 1 | No reference to specific languages or tools in the core; the validator rejects invalid files |
| **3. Classifier** | Compute profile and change level repeatably | Rust script for Linux (§3.6); rules table; scenario suite with counterexamples (§8.2) | Phases 0 and 2 | Scenario suite passes; every result states the decisive factors |
| **4. Controls** | Make decisions binding | Local hooks; push-triggered secret-scanning job, with the coverage and limits defined in §6.4; CI gate with one always-evaluated result and recomputation of the classification (§3.6, §6.2); integrity check from a trusted source (§4.6); push protection and Dependabot alerts; generated control summary; bootstrap command, including the initial-commit sequence without bypass (inventory B4); baseline for existing projects (§5.5) | Phases 2 and 3 | Deliberate bypass attempts tested and documented on a public and a private repository, including skipped or failed jobs, the secrets job and the identity of the integrity check |
| **5. First tech package** | Apply the methodology to the first pilot's stack | Gate commands; independent verification of test effectiveness (for example mutation testing) with a declared selection level; alteration outcomes and "not verifiable" (§6.1); CI template for the stack | Phase 4; **Q16** | Test verification working on real changes in the stack, including "not verifiable" cases |
| **6. Conversion and qualification** | Apply the methodology to this repository | This repository converted to the methodology; Claude Code and Codex qualified with observable evidence (§7.5) | Phases 4 and 5 | Repository converted; qualification outcomes recorded with agent versions |
| **7. First pilot** | Test the full cycle on a real project | At least one real feature from request to merge; anomaly log; new scenarios from unexpected events | Phase 6; Q16 | **Milestone 1.0 experimental** (§8.1) |
| **8. Second pilot and stabilization** | Test portability and extension | Second pilot in a different language and with a different visibility from the first; procedure for new tech packages, tested by creating the second package; documented limitations | Phase 7 | **Stable milestone** (§8.1); licence decided (Q12) |

## Critical dependencies

- **Q16, the first pilot's stack,** blocks phases 5 and 7. It is decided at the latest by the end of phase 4. Fallback: a small project created for the purpose.
- **The classifier is written in Rust** (decision D1) and does not depend on the pilot's stack, so phases 0–4 are not blocked by Q16.
- **Q12, the licence:** the upstream repository has no licence file. Clarification with the upstream author starts now, in parallel. It must be settled before reusing upstream or third-party material and before the stable milestone.

## Technical checks (Q9)

The checks are spread over the phases that need them.

| Check, by priority | Phase |
| --- | --- |
| GitHub Free capabilities on public and private repositories: branch protection, push protection, bypasses, `CODEOWNERS`, GitHub Actions included compute usage and billing | 1 |
| Skip markers such as `[skip ci]` on the secret-scanning job; choice of the remedy | 1 |
| Commit coverage of the secret-scanning job | 4 |
| Loading of `AGENTS.md` and skills in Claude Code and Codex | 1 |
| AI credential formats recognized by secret scanners, with synthetic credentials | 4 |
| Dependabot alerts on a private repository | 4 |
| Integrity check security against PRs built to replace it | 4 |
| Skipped or failed jobs and the aggregate gate result | 4 |
| Expected source of required status checks | 4 |
| Verification of test effectiveness in the first pilot's stack | 5 |

## Reviews and approvals

- **Independent reviews** (GPT-6, or an agent in a separate context): at the end of phase 1, on the inventory, the capability matrix and the loading tests; before the stable milestone, on a working implementation; and for every PR that changes control artifacts (§4.6).
- After the phase-0 freeze, every rules-only PR that modifies the specification requires an independent review of its final diff and supporting evidence before merge. The end-of-phase review does not replace it.
- Review prompts ask to separate problems demonstrable from the text from those that only a practical test can confirm, and to propose a concrete test for the latter.
- The owner approves every merge and every change to this roadmap.

## How work proceeds

- **Session start:** read `AGENTS.md`, this roadmap and the open issues; propose the next activity and a plan; list the decisions that need the owner.
- **Work items:** an issue is created when an activity becomes concretely plannable (§5.3). No milestones are created in advance.
- **Branches and PRs:** one branch and one PR per activity. The agent stops before committing changes to control artifacts and whenever a decision is needed; the owner pushes and merges.
- **Specification:** frozen. It changes only for concrete problems found in phases 1–8, through rules-only PRs.

## Decisions taken

| # | Decision | Outcome |
| --- | --- | --- |
| D1 | Language of the classification script | Rust for the classifier and the decisions-file validator; Bash for hooks. Phases 0–4 are not blocked by Q16 |
| D2 | v4 draft items | Reviewed in phase 0; v5.1 contains only approved or open items |
| D3 | Start of the fork | In parallel with phase 0: public fork with baseline tag, two test repositories (public and private), loading test in Claude Code and Codex |
| D4 | Q16, first pilot's stack | Decided at the latest by the end of phase 4 |

## Candidates to evaluate (not decided)

| Candidate | Origin | Evaluate in |
| --- | --- | --- |
| `commit-msg` hook that rejects agent attribution lines | Agents add them by themselves; observed again in phase 1 | Phase 4, together with the design of the hooks (phase-1 data: `docs/audit/skip-markers.md`, H11a–H11e) |
| `pre-push` hook that checks the identity of all pushed commits | The platform's email check covers only the last commit | Phase 4 |
| Remote branch policy: delete after merge | Applied in stage 0 without a written rule | Phase 4 |
| Check of the agent's permission mode at session start | An automatic mode stayed active in one session | Phase 6 |

## Risks

| Risk | Effect | Mitigation |
| --- | --- | --- |
| Endless design: version 5.1 without code | The document improves, nothing is tested | v5.1 frozen against further preventive design reviews; changes only from concrete problems found in phases 1–8 |
| Q16 stays undefined | Milestone 1.0 experimental out of reach | Decide Q16 by phase 4 (D4); fallback: a small project created for the purpose |
| Agent behavior changes between versions | Qualification to repeat; instructions no longer loaded | Minimum versions recorded; quick regression subset at every update (§7.5) |
| GitHub Actions minutes on private repositories | Interrupted jobs or costs | Real measurement in phase 1; short jobs; possibly a runner on the owner's machine |
| Platform capabilities different from the documentation | Declared guarantees that are not true | Practical tests in phase 1 and bypass attempts in phase 4 |
| No licence in the upstream repository, while the fork is public | Reuse and redistribution legally uncertain | Start the clarification with the upstream author now (licence or written permission); then choose the fork's licence (Q12) |
| Reliability of the secret-scanning tool used for the minimum threshold | Minimum threshold weaker than expected | Use limited to own content; re-evaluate the tool in phase 4, together with the choice for CI |

The roadmap contains no duration estimates: they depend on available time and on the first pilot's stack, both unknown today.
