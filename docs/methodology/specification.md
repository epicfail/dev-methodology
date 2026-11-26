# Software development methodology with AI agents — v5.1

Status: frozen at the end of phase 0 (2026-11-25). This file is the source of truth for the methodology; it changes only through pull requests, and only for concrete problems found during implementation, technical checks or pilot projects.

## 1. Purpose, scope and conventions

This document describes a software development methodology for projects in which AI agents write all or part of the code, supervised by one or more people. It covers requirements, technical decisions, work management, implementation, verification, security and code maintenance.

The methodology derives from an existing methodology tied to a specific stack and agent. This document describes only the revised version and is self-contained. It is version 5.1: changes from the fourth version are listed in section 11, changes made after the final review of v5 in section 12.

**Scope.** The methodology applies to:

- projects of different nature and size, from a personal script to a service handling sensitive data;
- a single developer working with AI agents, or a team, with one process;
- different AI agents, without depending on a specific tool.

**Out of scope.**

- The technology stack (languages, frameworks, libraries): chosen per project.
- Software release and operation: deployment, rollback, recovery, incident management, decommissioning.

**Environment of the first version.** A personal Linux machine and repositories on GitHub without paid plans (GitHub Free). Both public and private repositories are supported. Other platforms and operating systems are planned later.

**Consequence of the free plan.** On GitHub Free, some server-side protections (branch protection, rulesets, required code-owner review, repository push protection) are available for public repositories but not for private ones. The guarantees in this document therefore depend on repository visibility; where they differ, this is stated explicitly (6.4).

### Item status

| Status | Meaning |
| --- | --- |
| **Approved** | Decision taken explicitly |
| **Draft** | Proposal described in detail but not yet explicitly confirmed |
| **Open** | To be decided; where an option is under consideration it is described, without presenting it as the choice |

Unless marked otherwise, an item is **approved**. Open items are marked where they occur and listed in section 10.

### Glossary

| Term | Definition |
| --- | --- |
| Owner | The person with final decision authority on a project: confirms the profile, approves derogations and risk acceptances (4.1) |
| Project profile | Rigor level of a project: light, intermediate or rigorous. The recommended profile is computed from the project factors; the adopted profile is the one confirmed by the owner (3.2) |
| Change level | Rigor level required by a single change, computed from the change factors (3.3) |
| Effective level | The higher of the adopted profile and the change level; it decides which activities apply to a change (3.4) |
| Work item | Tracked unit of work (on GitHub, an issue) with a completion criterion |
| Decisions file | Versioned YAML file in the repository that records the choices approved for the project (4.4) |
| General guarantee | Requirement that applies to every change from a given effective level upward (6.3) |
| Specific activity | Activity that applies only when the change meets its pertinence criterion (6.3) |
| Non-pertinence | Recorded statement that a specific activity's pertinence criterion is not met; it needs no derogation (6.3) |
| Derogation | Motivated decision, approved by the owner, not to apply a recommended rule or a pertinent activity (4.1) |
| Risk acceptance | Derogation from an essential control, with a written description of what will not be detected |
| Minimum threshold | Controls that the methodology does not allow to be derogated; they may still be technically bypassable (4.3) |
| Review condition | Event that, if it occurs, reopens the profile assessment (3.5) |
| ADR | Architecture Decision Record: a short document that records an architectural decision, the rejected alternatives and the reasons |
| Fitness function | Automated check that verifies an architectural property |
| Test description | Short, simple and direct sentence that says what a test verifies and why the expected value is what it is (6.1) |
| Deliberate alteration | Intentionally wrong change to the modified lines of code, not chosen by the implementing agent, used to check that at least one test detects it (6.1) |
| "Not verifiable" outcome | Outcome of test verification when no valid check can be made; it requires an alternative verification or a derogation (6.1) |
| Tech package | Set of commands and configurations through which the methodology applies to a specific stack (5.2); distinct from the rigor profile |
| Agent data policy | Allowed AI model providers and paths that agents must not read (6.3) |
| Adapter | File that makes a specific tool (an agent or a platform) use the common rules |
| Gate | Set of checks that a change must pass before being integrated |
| Baseline | List of pre-existing problems recorded at adoption, which do not block new changes (5.5) |
| Scenario | Test case for the methodology itself: a given input and an expected result |

## 2. Principles and structure

### 2.1 Three-level structure

| Level | Content | Variability |
| --- | --- | --- |
| Invariant principles | Quality goals valid for every project (2.2) | Do not change between projects |
| Practice catalogue | Techniques applicable according to context (2.3) | Selected per project and per change |
| Replaceable tools | AI agents, platforms, tech packages, test runners, scanners | Replaceable without changing principles and practices |

Principles describe **goals**, not procedures: a procedure enters the principles only if it is the only way to reach the goal.

### 2.2 Principles

All principles are approved. P8–P11 were drafts in v4; P13 comes from the phase-0 decisions.

| # | Principle |
| --- | --- |
| P1 | Process rigor is proportionate to risk, assessed on the project and on each change |
| P2 | Tests of a behavior change are able to detect errors in the new code and are understandable to whoever reviews them. How this is verified is defined in 6.1; any derogation is recorded |
| P3 | Security is proportionate to the profile, with a minimum threshold that the methodology does not allow to be derogated |
| P4 | The methodology recommends; the final decision rests with the owner; approved decisions bind every agent and tool in the project |
| P5 | Guarantees do not depend on an agent's behavior: what must be respected is verified by controls outside the agent. Each guarantee states its technical preconditions and its limits |
| P6 | Every relevant rule states its foundation and a review condition; no rule relies on comparative metrics collected across one's own projects (details in 4.5) |
| P7 | Rules and decisions do not depend on a specific AI agent or platform |
| P8 | Requirements and completion criteria are explicit before implementation, to an extent proportionate to risk |
| P9 | Relevant architectural and technology decisions are motivated, with the rejected alternatives, and recorded in ADRs |
| P10 | Architecture is chosen according to context; intentional dependency management always applies, not a predefined architectural style |
| P11 | Simplicity in the code: no abstraction or dependency without a current need |
| P12 | Every change is traceable at least to the pull request that integrated it; when a work item exists, the PR links to it |
| P13 | Simplicity in the methodology: a new factor, value, rule, criterion or check is introduced only if its absence would prevent an adequate risk assessment or could lead to a significantly wrong decision. An observable characteristic does not necessarily become a factor; an edge case does not necessarily justify a category; an automatable check is not necessarily mandatory; details are decided in the right phase |

### 2.3 Practice catalogue

No practice in the catalogue is mandatory in itself. Section 6.3 requires **results** (general guarantees and specific activities); a practice is one way to obtain them, chosen per project and per change. For example, STRIDE is one way to carry out the proportionate threat analysis required by 6.3.

| Practice | Useful when | Main cost or limit |
| --- | --- | --- |
| BDD (Gherkin scenarios) | Behavior must also be read by non-developers | Scenarios to maintain; superfluous for libraries and internal tools |
| TDD | Logic with rules and edge cases | Process discipline that cannot be verified by itself |
| Full mutation testing | Critical parts, to measure the effectiveness of all tests | Slow; run periodically, not on every PR (6.1) |
| Domain-Driven Design | Complex domains with many business rules | Disproportionate for CRUD and simple tools |
| Hexagonal architecture (ports and adapters) | Rich domain logic and several external integrations | Extra interfaces, useless in simple systems |
| Fitness function | Important architectural properties, verifiable at reasonable cost | Cost of maintaining the check |
| Walking skeleton | Architectural decisions with high uncertainty | Initial code to keep clean |
| Spike on the real service before the stub | Integrations with available external services | Not applicable if the service is not yet available |
| Immutable database migrations after delivery | Projects with a database | Concerns only migrations already delivered |
| Threat modeling (STRIDE) | Exposed systems or systems with sensitive data | Requires expertise; a reduced variant exists |
| Effective agent isolation (7.8) | Agents working with real secrets or administrative privileges | Environment setup; in the first version optional and never recommended automatically |
| SBOM | A component inventory is needed for distribution, audits or vulnerability tracking | Artifact to keep current; signing only if required |
| DAST | Applications or APIs reachable while running | Needs a running test environment and false-positive analysis |

## 3. Rigor model

The rigor applied to a change is the higher of two levels: the project profile adopted by the owner and the level of the change itself. Seven observable factors feed the calculation; a deterministic script computes the result.

### 3.1 Two dimensions and classification rules

- **Project profile**: set at bootstrap from the project factors (3.2); three levels: light, intermediate, rigorous.
- **Change level**: assessed for every change from the change factors (3.3); it can only raise rigor, never lower it.

The classification only decides which activities and controls to **recommend**; it is not an assessment of the quality of the work. Factors describe; they neither authorize nor forbid anything. Which activities follow is decided in 6.3.

| Rule | Content |
| --- | --- |
| Caution | Missing or unverified information is never replaced with the favorable value: the most unfavorable value compatible with what is known is used, and noted |
| Provenance | Every value distinguishes detected facts, declarations and unknown information. An agent's assessment confirmed by the owner counts as a declaration and is sufficient; an unconfirmed assessment is a proposal and counts as unknown. For the M2 signal and the M4 critical characteristics, **unknown is not absent**: they are considered present until verified or declared |
| Most unfavorable value | When a change has several effects or touches several areas, the most unfavorable value of each factor prevails |
| Risk is not available controls | Repository visibility and the GitHub plan change which protections can be enabled, not the risk. Project duration and team size change the need for coordination, not the technical risk |
| Proportionality, not prohibition | The classification recommends how much rigor to apply; the owner decides the adopted level. Going below the recommendation is an accepted, recorded risk. Only the minimum threshold (4.3) is non-negotiable |

### 3.2 Project factors and profile

Project factors are assessed at project start. If a change significantly alters the functionality or the context of use (for example, it adds the import of bank statements), the affected project factors are reassessed before the change level is determined. A reassessment that **raises** the profile applies the project's state after the change at once: the decisions file is updated in a rules-only PR, merged before the code PR. One that **lowers** the profile takes effect only after the change that justifies it has been integrated: the rules-only PR follows the code PR. Lowering it earlier is a gap below the recommended profile, handled as described below (4.6).

**P1 — Data handled**

| Value | Definition |
| --- | --- |
| None | The project handles no personal or confidential data |
| Ordinary | Common personal data (name, email, address, purchase history, spending amounts and categories) and confidential data that is not sensitive (for example, internal documents of an association) |
| Sensitive | GDPR special categories (art. 9: health, ethnic origin, political and religious opinions, sexual orientation, genetic and biometric data); judicial data (art. 10); banking and payment data (card numbers, online banking credentials, banking app tokens, and also IBANs, balances and statements, even without the ability to act on them); credentials and secrets; data about minors |

**P2 — Exposure**

| Value | Definition |
| --- | --- |
| Local | Runs only on the device where it is installed and is not reachable from other devices (the number of people using that device does not matter) |
| Private network | Reachable from other devices only inside a network controlled by the user, including a user-managed VPN that gives access only to the user's own devices |
| Internet | Reachable from outside in any way. Criterion: if anyone on the Internet can contact the address, it counts as Internet, even if authentication is then required (for example, a public tunnel with login) |

**P3 — Impact of an error.** The worst reasonably plausible consequence of a software defect in its intended context of use. Purely hypothetical or remote scenarios are excluded, and no monetary thresholds are used. The presence of other people, third-party data or legal obligations does not determine the value by itself: the nature of the consequences does.

| Value | Definition |
| --- | --- |
| Negligible | Inconvenience or malfunction without relevant damage or loss |
| Recoverable | Limited damage, correctable with time or work, without serious consequences for the people or organizations involved |
| Severe | Relevant economic, operational, legal or physical damage, or damage to people's safety or confidentiality, whether or not it can be repaired |

Presumptive indicators of "severe": if one is present, "severe" is the presumed value, and lowering it requires a written justification based on the available information.

- It executes or orders real economic transactions.
- It affects health or physical safety.
- It handles sensitive data (as defined in P1) of other people.
- Others depend on the service for an essential activity.

**Recommended profile.** Data × exposure table. If P3 is severe, the profile is rigorous whatever the table says; P3 can only raise the profile, never lower it.

| P1 ↓ / P2 → | Local | Private network | Internet |
| --- | --- | --- | --- |
| None | Light | Light | Intermediate |
| Ordinary | Light | Intermediate | Intermediate |
| Sensitive | Intermediate | Intermediate | Rigorous |

The owner decides the adopted profile. If it is lower than the recommended one, the gap is an accepted risk with a written justification. The recommended profile stays recorded but does not enter the calculation of the effective level.

### 3.3 Change factors and change level

Change factors are assessed for every change (PR). File paths are a signal, not a complete classification: the agent proposes each value with its justification and the owner confirms.

**M1 — Area touched**

| Value | Includes |
| --- | --- |
| Form | Documentation, comments, formatting, interface text without effects on behavior |
| Logic | Program behavior: functions, calculations, refactoring; also changes to tests only, because removing or weakening a test reduces verification |
| Security | Authentication, sessions and permissions; cryptography and secret management; validation of external input; the controls themselves (the control artifacts listed in 4.6); release or infrastructure configuration that touches credentials or permissions |

**M2 — Data affected by the change.** Which project-relevant data (as defined in P1) the change handles differently, whether stored, processed in memory, displayed or transmitted. What counts is the effect of the change, not the component or files touched: P1 describes the project, M2 the change.

| Value | Definition |
| --- | --- |
| None | Does not alter the handling of project-relevant data |
| Ordinary data | Alters reading, writing, transformation, deletion, display, transmission or schema of ordinary data |
| Sensitive data | Alters the handling of sensitive data as defined in P1, including its presence in logs, exports and error messages |

Separate signal: **external data flow**, the introduction or change of a flow that sends relevant data to services or parties outside the project perimeter. A mere operational dependency on an external service, without sending data, is not a factor: it is noted in the PR and considered in P3 if it concerns an essential activity.

**M3 — Reversibility.** Whether and how the previous state can be fully restored, for the relevant effects of the change, inside and outside the repository, while preserving legitimate operations that happened in the meantime. The foreseeable effects of applying and using the change are considered.

| Value | Definition |
| --- | --- |
| Reversible by revert | Reverting the commits, followed by the normal release process, is enough to restore the previous state |
| Reversible by procedure | Additional operations are needed, with a defined procedure verified on a system of the same type and still valid |
| Irreversible | Full restoration is not possible, or not demonstrated (for example, a backup never tested: a precautionary value, noted as such) |

Criteria: mitigations (for example, revoking a secret) do not change the value. Ordinary traces in logs, git history and audit trails do not count by themselves as irreversible effects; a secret written to a log readable by third parties is, however, a disclosure. Conditions and time limits of the restoration are written down. Time, cost and complexity of the restoration, probability and severity stay outside M3.

**M4 — Dependencies.** Direct and transitive dependencies in every phase (development, build, test, release, runtime): libraries, build plugins, third-party GitHub Actions, Docker base images, submodules, downloaded binaries, SDKs, copied third-party code, runtime and language versions. Actual changes are assessed, not just manifests. The criteria of the third value also apply to transitive dependencies: a patch to a direct dependency that moves a transitive one to a new major version is a significant change.

| Value | Definition |
| --- | --- |
| None | Does not change the set, effective versions, provenance or resolution of dependencies |
| Change to existing dependencies | Patch or compatible minor update; removal; ordinary change of transitive dependencies |
| Significant introduction or change | New direct dependency (even development- or test-only); major or incompatible update; compatibility that cannot be established (0.x versions, non-SemVer schemes, latest); change of identity or provenance (registry, fork, branch); significant change of resolution |

Critical characteristics are recorded separately from the value, as signals and not as conclusions:

| Group | Includes | Detection |
| --- | --- | --- |
| Sensitive function | Security and cryptography; untrusted input; access to credentials, files, environment variables | Declared |
| Executes code | At install, build or test time | Partly automatic |
| Identity and provenance | Suspicious or little-used name (slopsquatting); changed or unverified source; unpinned version | Mostly automatic |
| Status | Known vulnerabilities; abandoned or unsupported project | Automatic, with context assessment |
| Licence | Incompatible or unknown | Automatic or declared |

**Change level.** Each factor indicates a level; the highest applies.

| Factor | Minimum | Ordinary | High |
| --- | --- | --- | --- |
| M1 | Form | Logic (including tests only) | Security |
| M2 | None | Ordinary | Sensitive |
| M3 | Reversible by revert | Reversible by procedure | Irreversible |
| M4 | None | Change to existing dependencies; significant introduction or change without critical characteristics other than licence | Significant introduction or change with at least one critical characteristic other than licence |

Signals: a new or changed external data flow raises the level to at least ordinary. Licence is the only critical characteristic that does not raise the level: a problematic or unknown licence requires an explicit owner decision. With the M4 "status" group unknown, a new dependency is high until verified (manual verification is acceptable, for example against public security advisories).

### 3.4 Effective level

Effective level = the higher of the adopted profile and the change level, with the correspondence minimum ↔ light, ordinary ↔ intermediate, high ↔ rigorous. There are no exceptions for documentation, trivial changes or emergencies: proportionality is handled by the activities in 6.3.

| Adopted profile ↓ / Change → | Minimum | Ordinary | High |
| --- | --- | --- | --- |
| Light | Light | Intermediate | Rigorous |
| Intermediate | Intermediate | Intermediate | Rigorous |
| Rigorous | Rigorous | Rigorous | Rigorous |

Gates apply the effective level, taking recorded derogations into account. The minimum threshold always applies, whatever the level.

### 3.5 Review conditions

| Type | Example | How it is detected |
| --- | --- | --- |
| Observable from the repository | A configuration appears that exposes a service on a public port; a payment dependency is added | Periodic automated check (8.4) |
| Not observable from the repository | The application is exposed on the Internet through external infrastructure; the users change | Check question to the owner at every methodology update in the project and in any case every 3 months; the date of the last check is recorded in the decisions file |

### 3.6 Roles, calculation and recording

- The agent asks the questions (3.7) and proposes values with their provenance; the owner confirms; the script computes. The agent does not decide the level.
- The script is written in Rust, independently of the pilot project's stack, and runs on Linux in the first version. It applies threshold rules from a versioned file, not numerical scores. It is deterministic: the same input and the same rules version give the same result. Every result states the decisive factors.
- Mostly declared factors (P3, M3, part of M1 and M2) reach the script as value plus provenance; the script does not recompute them.
- "Unknown" (declared) is accepted with the most unfavorable value, signals included. "Missing" (field not filled in) makes the script refuse the calculation. Invalid or incomplete input is never accepted silently.
- The rules version must uniquely identify the rules' content; how is decided during implementation. The rules are verified with a suite of normal cases and counterexamples (8.2).
- The decisions file records P1–P3 with provenance and reasons, the recommended and adopted profiles, the justification for any gap and the rules version.
- The PR description contains a structured block with M1–M4 and their provenance, the signals, the change level, the effective level, the decisive factors and the rules version. It also records two applicability conditions used by 6.3: whether the change alters behavior, and whether it changes a control artifact (4.6), which is always M1 = security. The agent proposes them with their provenance and the owner confirms; a missing condition is refused, an unknown one counts as yes.
- A change is classified before implementation and the classification is reconfirmed before merge, because it can change during the work (for example, when a dependency is added).
- The gate validates the classification block and recomputes the change level and the effective level with the classifier, the rules and the decisions file of the main branch. A missing, invalid or inconsistent block, or one that does not refer to the PR's latest commit, cannot produce a successful gate result. The calculation does not prove that the declared inputs are true: confirming them remains the owner's responsibility.

### 3.7 Questions the agent asks

For every answer the agent states whether it is a detected fact, a declaration or unknown information. An unknown answer leads to the precautionary value.

| Factor | Questions |
| --- | --- |
| P1 | What data does the project handle? Is any of it in the list of sensitive data? |
| P2 | Who can reach the software? Is there an address that anyone on the Internet can contact? |
| P3 | What is the worst reasonably plausible consequence of an error? Would the damage be negligible, or limited and correctable? Is one of the presumptive indicators of "severe" present? |
| M1 | Which parts of the software does the change touch? Does it concern authentication, permissions, secrets, cryptography, external input or the controls themselves? |
| M2 | Does it alter the handling of relevant data? Which data, and how is it classified in P1? Does it introduce or change an external data flow? |
| M3 | Does it change data or state outside the code? Does it produce external effects, on release or through use? Is there a verified restoration procedure that preserves operations made in the meantime? Is there a time limit? |
| M4 | Which dependencies, including transitive ones, are added, removed or changed? What kind of version change is it, and can compatibility be established? Are identity, provenance and resolution verifiable? Are there critical characteristics, including in development and CI environments? |

## 4. Project decisions

### 4.1 Requirements of the mechanism

The mechanism that sets a project's phases and controls must:

- analyse the project's characteristics, context and needs;
- propose phases, controls and activities, explaining the reasons;
- distinguish what is essential from what is recommended or optional;
- leave the final decision to the owner, even when it departs from the recommendations;
- make approved decisions binding for every agent and tool in the project.

**Refusal of critical controls.** Even in a high-risk project the owner can reduce or refuse any control other than the minimum threshold. The methodology informs and records the accepted risk; it does not prevent the choice.

**Roles.** Project owner, change author, risk-acceptance approver. The risk-acceptance approver is the owner. In individual projects all roles coincide in the same person.

**Authorizations.**

| Action | Who authorizes it |
| --- | --- |
| Change the adopted profile or the review conditions | Owner |
| Change control artifacts (4.6) | Owner |
| Merge a PR | Owner |
| Confirm that a specific activity is not pertinent | Agent proposes, owner confirms |
| Approve a derogation (single or permanent), a risk acceptance or a "not verifiable" outcome | Owner |
| Take a required decision (irreversible change, licence, external data flow) | Owner |

**Transparency of choices.** A configuration with reduced controls is never presented as equivalent to a more protected one, and a recorded derogation remains distinguishable from an unauthorized bypass.

### 4.2 Process

1. **Analysis**: context questionnaire (3.7) and reading of the existing repository.
2. **Proposal**: a complete configuration, highlighting items that depart from the typical case; for each item, its class, reason and estimated cost can be consulted.
3. **Decision**: the owner approves the configuration or changes single items; for a refused item, a reduced alternative is proposed if one exists.
4. **Recording** in the decisions file.
5. **Application**: agents, hooks and CI read the same file.
6. **Review** when a review condition occurs (3.5).

| Class | Meaning | Refusal |
| --- | --- | --- |
| Essential | Reasonable minimum for the profile | Allowed with a written risk acceptance |
| Recommended | Concrete benefit for the context | Allowed with a short reason |
| Optional | Benefit depends on preferences | Allowed without a reason |

Guarantees and pertinent specific activities that 6.3 requires for a change's effective level are essential for that change: omitting one is a derogation approved by the owner, with the accepted risk (6.3). Non-pertinent activities need no derogation. The minimum threshold remains non-derogable.

### 4.3 Minimum threshold

Secret scanning is the only control in the minimum threshold. It also covers credentials for AI model services. The threshold is extended only if experience of use makes it necessary.

**Meaning of "non-negotiable".** The methodology allows no recorded derogation from the minimum threshold. It does not guarantee, however, that the control cannot be bypassed: the control chain, its preconditions and its limits for each repository type are in 6.4.

### 4.4 Decisions file

- YAML format, in a separate file versioned in the repository, readable by hooks and CI.
- The agent instructions file (AGENTS.md) does not summarize it: it points to it. One source, no copy to keep aligned.
- The file changes only through pull requests, in rules-only PRs (4.6).
- It contains: project factors P1–P3 with provenance and reasons; recommended and adopted profiles, with the justification for any gap; the rules version; permanent derogations with their review condition; the agent data policy (6.3); the date of the last check of the review conditions (3.5).
- The file has a versioned schema and is validated before every check. If it is invalid, incomplete or has an unsupported schema version, the checks fail: less rigorous defaults are never applied silently.

Illustrative example (field names not final):

```yaml
schema_version: 1
methodology_version: 0.5.0
rules_version: <identifier of the rules content>
repository:
  platform: github
  plan: free
  visibility: private          # determines the available guarantees (6.4)
project_factors:
  data:     { value: ordinary,    provenance: declared, reason: "Spending amounts and categories" }
  exposure: { value: local,       provenance: declared }
  impact:   { value: recoverable, provenance: declared, reason: "Errors are visible and can be fixed by hand" }
profile:
  recommended: light
  adopted: light
controls:
  secret_scanning: { class: essential, status: adopted }   # minimum threshold
permanent_derogations:
  - activity: <activity from 6.3>
    reason: "<reason tied to the nature of the project>"
    risk_accepted: "<what will not be detected>"
    review_when: "<review condition>"
agent_data_policy:
  allowed_providers: [anthropic, openai]
  excluded_paths: [".env", "secrets/**", "tests/fixtures/real/**"]
review_when:
  observable:
    - "configuration exposing a service on a public port"
  declared:
    - "the application is exposed on the Internet"
  last_declared_check: <date>
agent_trailer: false
approved_by: <owner>
approved_on: <date>
```

### 4.5 Foundation of rules

Every relevant rule states its foundation, chosen from this hierarchy, and a review condition.

| Level | Example |
| --- | --- |
| Logical necessity | A test that does not notice an alteration of the code it checks does not verify that behavior |
| External evidence | Literature, standards (e.g. OWASP ASVS), with their declared limits |
| Explicit judgment | A motivated choice, declared as such, with the assumptions it rests on |
| Local signal | Frequent derogations from a rule: a trigger to review it, never proof |

The exclusion in P6 concerns comparative metrics collected on one's own projects; rigorous external studies remain usable. Homogeneous minor conventions share a single reason.

### 4.6 Protection of rules

The control artifacts live in the same repository as the code they check: decisions file, CI workflows, gate script, classifier and its rules table, git hooks, platform rules and their versioned copies, AGENTS.md. With GitHub Actions, the workflow run on a PR can come from the PR itself: without protections, a PR could change code and controls together.

| Measure | Effect |
| --- | --- |
| Main-branch protection enabled at bootstrap, at every level, where the platform makes it available | Changes reach the main branch only through PRs (6.4) |
| Integrity check run with the main branch's code, executing no code from the PR | Verifies that a PR does not change control artifacts together with other code |
| The gate reads the decisions file from the main branch | A rule change contained in a PR does not apply to that same PR |
| A PR that changes rules or controls contains no other code | A rule change is always a separate, visible PR |
| In a team, CODEOWNERS on the control artifacts, with required code-owner review enabled | Rule changes require approval by designated people (only where the platform makes it available, 6.4) |
| Permission rules in the agents' tools: no changes to workflows and gate script, no administrative calls to the platform | Reduces accidental changes; it is a tool rule, not a guarantee |

**Profile change.** When a change raises the project profile (3.2), the decisions file is updated first, in a rules-only PR, and the code PR follows after it is merged. When a change lowers the profile, the order is reversed: the code PR first, then the rules-only PR.

**Integrity check details.** It is a job separate from those that build or run the PR's code; it uses a read-only token; it reads only the list of changed files, without running scripts, actions or dependencies from the PR. Its execution and acceptance derive from a trusted source: a PR cannot change the integrity check, suppress its required execution or substitute an unrelated successful result for it (for example, a job or a commit status with the same name). Where available, the ruleset requires the expected source of the check. Its correctness must be proven with PRs built specifically to try to replace it (Q9).

**Declared limits.** An administrator is subject to an active ruleset unless granted applicable bypass permissions (tested in phase 1; phase-1 capability matrix, `docs/audit/github-free-capabilities.md`: P2, P6, P8, P9). The administrator can still change or remove the ruleset, and with it its protections (documented, not tested). An agent that uses the user's credentials (7.8) has the same permissions as that user. On private repositories with GitHub Free, merge blocking is not available: checks report but do not block (6.4).

### 4.7 Architectural decisions

Relevant architectural and technology decisions are recorded in ADRs with date, status (proposed, accepted, superseded), rejected alternatives and reasons. An accepted ADR is not rewritten: a new decision produces a new ADR that supersedes the previous one. An ADR is not needed for every library or implementation detail.

### 4.8 Rule attributes

Every relevant rule is described by three independent attributes.

| Attribute | Values |
| --- | --- |
| Verification mode | Instruction (not verified) · agent tool rule · control outside the agent · human review. A rule can have more than one |
| Origin | Methodology · owner's choice · external constraint (technical, contractual, regulatory) |
| Status in the project | Adopted · reduced · refused with accepted risk · not applicable (the rule does not concern the project, distinct from "refused") |

Example: "Do not read .env" = instruction + agent tool rule · owner's choice · adopted. Full attributes apply to relevant controls and decisions, not to every minor convention. External constraints are declared and flagged when a choice contradicts them, but the methodology does not enforce them (4.1).

## 5. Development cycle

### 5.1 Project phases

Phases are types of activity, not a rigid sequence: each one is activated and sized by the rigor model. Release and operation are out of scope (section 1).

| Phase | Content | When |
| --- | --- | --- |
| Bootstrap | Context questionnaire, profile, decisions file, hooks, minimum CI job for secrets, adapters, check and activation of the available protections, including main-branch protection at every level (6.4), baseline in existing projects (5.5) | At start, or on adoption in an existing project |
| Requirements | Goals, functional requirements and, when relevant, measurable quality requirements | At start and for every new feature, proportionately |
| Technology evaluation | See 5.2 | At start and for technology decisions with substantial impact |
| Architecture | Style chosen for the context, recorded in an ADR; fitness functions for important, verifiable constraints; walking skeleton for high-uncertainty decisions | At start and when an architectural decision changes |
| Planning | Typed work items (5.3) | Continuous |
| Implementation and integration | Workflow of a change (5.4) | For every change |
| Code maintenance | Dependency updates, fixes, profile reassessment, monitoring (8.4) | Continuous |

### 5.2 Technology evaluation and tech packages

**Technology evaluation.** A conditional activity, not repeated for every work item. In an existing project the technologies in use are kept, unless there are concrete reasons to change them.

- **Eliminating constraints**: requirements that exclude an option (platform, licence, available skills, security, costs).
- **Alternatives**: two or three realistic options, including those already in use.
- **Comparison** on maturity, ecosystem, compatibility, operating costs, replaceability. Weighted scores only if they really help to decide.
- **Spike** only where there is real uncertainty.
- **ADR** with decision, rejected alternatives and reasons.

**Tech packages.** There is no default stack. An optional tech package, one per stack, provides the concrete commands through which the methodology applies to that stack. It is used only if the chosen stack matches. The first version includes one, for the first pilot project (stack still to be defined, Q16). A documented procedure describes how to create and qualify a new package (8.1).

**Capabilities required of a package.** Defined as results, not as specific commands or techniques.

| Capability | Required result |
| --- | --- |
| Formatting, static analysis, tests | Outcome readable by the gate |
| Test verification (6.1) | A check, independent of the implementing agent, that the tests detect wrong behavior in the changed code; outcomes classified as successful, failed or not verifiable; a report usable by the gate. When deliberate alterations are used, the tool generates them and declares its effective selection level (lines, files or incremental analysis) |
| Dependencies | Lockfile or constraints according to the policy in 6.3; a format recognized by SCA tools |
| CI template | Gate job for that stack |

The first version uses command configurations and a few extension points, not a general plugin system. The documentation states which stacks are operationally supported.

### 5.3 Work items and cadence

**Work item types.**

| Type | Typical completion criterion |
| --- | --- |
| Feature | Expected behavior checked by described and verified tests (6.1); user documentation if visible to users |
| Bug | A test that reproduces the defect: it fails on the previous code because of a wrong value, then passes. It counts as test verification (6.1) |
| Refactoring | Unchanged behavior, existing tests green |
| Technical debt | A known limit removed or reduced, referencing where it was recorded |
| Research | A question with a documented answer (e.g. the outcome of a spike) |
| Maintenance task | A verifiable outcome (e.g. dependency updated and checks green) |

**Tracker.** Work items are the platform's issues, managed through an adapter. In the first version the platform is GitHub; GitLab, Azure DevOps and others are planned later.

**Cadence.** Continuous flow by default: a list ordered by priority, with a limit on simultaneous work configurable according to the capacity of whoever works on it. Sprints can be enabled per project.

### 5.4 Workflow of a change

**Full path.** The steps follow the general guarantees and pertinence criteria of 6.3 for the change's effective level.

1. Work item with type and completion criterion (from effective level intermediate).
2. Classification of the change (section 3), recorded in the PR block.
3. Specification and plan, proportionate to the change (rigorous level; for a typo one sentence in the work item is enough).
4. For a behavior change, described tests; test verification from the intermediate level (6.1).
5. Specific activities whose pertinence criterion is met (6.3).
6. Implementation on a dedicated branch whose name contains the work item identifier; no direct commits to the main branch.
7. Local checks through git hooks; tests run locally before the PR.
8. Pull request linked to the work item: classification reconfirmed, gate, review according to the effective level, the owner's own test of the feature when it is visible to users.
9. Merge, decided by the owner; the PR closes the work item.

**Lightweight path.** The work item can be omitted only when the change's effective level is light. Branch and PR always remain. Changes to control artifacts (4.6) are M1 = security, so their effective level is rigorous and they always follow the full path.

**Traceability.** The minimum unit of traceability is the pull request. When a work item exists, its identifier appears in the branch name and in the PR, and the PR closes it. Links to versions or tags are not part of the methodology.

**Commit and version conventions.** Conventional Commits and Semantic Versioning are the default, configurable per project.

**Language.** Identifiers in code are in English; documentation, comments, specifications and messages for people are in the language chosen for the project.

**CI.** Every repository at every level has at least the **minimum secret-scanning job** (6.4), part of the minimum threshold. From effective level intermediate, CI runs the tests and the further checks, with merge blocking where the platform makes it available (6.3). Every project, light ones included, has a CI workflow able to apply the gate according to each change's effective level; only applicable checks run. A required check that is unavailable for the stack is not a non-pertinent check: omitting it follows the derogation rules (6.3).

### 5.5 Adoption in an existing project

On adoption a baseline is recorded: missing tests, scanner results and non-conforming conventions present at that moment. Baseline problems do not block new PRs. The baseline is reduced with work items of type technical debt.

Results are compared through stable identifiers provided by the tools, distinguishing four cases:

| Case | Effect on the gate |
| --- | --- |
| New problem | Blocks according to the normal rules |
| Regression (a closed problem that reappears) | Blocks like a new problem |
| Pre-existing problem still present | Does not block; stays in the baseline |
| Pre-existing problem made worse (e.g. higher severity) | Blocks like a new problem |

## 6. Verification, tests and security

### 6.1 Test verification

When an agent writes both the code and the tests, a test can pass whatever the code does. Example: for a function that computes a month's total expenses, `month_total(march) == 152.40` checks the result, while `month_total(march) is not None` passes even if the total is wrong.

**Goal of the automated check.** At least one test reacts to an error introduced in the new code. Any test in the suite counts, whether new, changed or pre-existing. Declared consequence: a change can add weak new tests and pass thanks to pre-existing ones; this case is left to review, helped by the test descriptions. Coverage of acceptance criteria is not checked automatically: it is a review task.

**Rules.**

- **Test description.** Every test has a short, simple and direct description: what it checks and why the expected value is what it is. An automated check verifies that it is present; review judges whether it is correct.
- **Independence from the implementer.** The verification is independent of the agent that implemented the change. Mutation testing restricted to the changed code is one possible technique, not a mandatory one. When deliberate alterations are used, the implementing agent does not choose them.
- **Relevant failures only.** Only a failure in which the test checked a value and found it different from the expected one counts; compilation errors or missing functions do not.
- **Isolation.** Alterations run in a temporary copy of the work, with verified restoration and a time limit.
- **Declared limit.** Passing the check shows that the tests are not useless, not that they are correct or that they cover all acceptance criteria.
- **Bugs.** A test that reproduces the defect and fails on the previous code because of a wrong value counts as verification: it shows the same property as an alteration, and the alteration (the real defect) was not chosen by the implementer.

**"Not verifiable" outcome.** If no valid check can be made (e.g. the change only alters a text, a query in a string, a template), the gate reports "not verifiable". This is not a successful verification: the activity stays unresolved, and the gate cannot succeed, until an alternative verification is recorded or the owner approves a derogation. The change author:

- first tries a proportionate alternative verification: integration tests, comparison of outputs with expected values, contract tests, or a documented manual check;
- only if that is not possible, proposes a one-line derogation with the reason.

The author declares the choice in the PR; the owner approves the derogation. The anomaly log (8.3) counts how often it happens.

**Details.**

- An alternative verification states which defect it can detect (e.g. "fails if the header changes"); otherwise it counts as a derogation.
- The report distinguishes alteration outcomes: detected by a test, survived, not generable, equivalent to the original code. No score threshold is imposed.
- The minimum check on the change blocks the PR. Full mutation testing of critical parts is a catalogue practice (2.3): when adopted, it runs periodically and does not block single PRs; critical parts are declared in the decisions file.

Changes without effect on behavior (documentation, style, non-functional configuration) are excluded.

**Where it runs.**

| Effective level | Execution | Note |
| --- | --- | --- |
| Light | Tests run locally (hook or command), without verification by alteration | Bypassable |
| Intermediate | Test verification in CI |  |
| Rigorous | Test verification in CI | Independent review also applies (6.3) |

Mutation testing tools have different capabilities per language: those of the first tech package must be verified (Q9). Test-first remains a catalogue practice, not a form of evidence.

### 6.2 Gate

The checks that block integration are deterministic (tests, static analysis, scanners). Deterministic means repeatable, not infallible. A diff review by an AI model can be an advisory complement, within the agent data policy; the independent review of 6.3 supports the owner's decision and does not replace it.

**Security of the gate itself.** Runners have minimal permissions; untrusted code from a PR is never run with credentials or elevated permissions; dependencies run during the checks follow the policy in 6.3. CI actions are pinned by commit hash (SHA). Since Dependabot alerts do not cover SHA-pinned actions, automatic version updates for actions are enabled, so that fixes still arrive.

**Gate result.** Where merge blocking is available, one gate result, always evaluated, represents the outcome of all applicable checks. A required activity that was skipped, did not run or produced no result never counts as successful: on GitHub a skipped job reports success, and jobs that depend on a failed job are skipped. This behavior is verified in phase 4 (Q9).

### 6.3 Activities by level and security controls

What a change requires depends on its effective level (3.4). **General guarantees** apply to every change of that level; **specific activities** apply only when their pertinence criterion is met. This document states control types and blocking rules; the tools named are replaceable examples. The guarantees actually available depend on the repository type (6.4).

**General guarantees.** Levels are cumulative. Guarantees that presuppose a certain nature of the change apply only when that condition holds, as recorded in the PR block (3.6): for example, tests only when behavior changes.

| Guarantee | Light | Intermediate | Rigorous |
| --- | --- | --- | --- |
| Branch and PR; the owner always decides the merge | ✔ | ✔ | ✔ |
| Main-branch protection, enabled at bootstrap where the platform makes it available | ✔ | ✔ | ✔ |
| Minimum threshold: secret scanning (hook, push protection where available, CI job) | ✔ | ✔ | ✔ |
| Classification block in the PR | ✔ | ✔ | ✔ |
| New or changed behavior covered by a test with a short description | ✔ | ✔ | ✔ |
| Tests run locally before the PR | ✔ | ✔ | ✔ |
| Work item with goal and completion criterion | — | ✔ | ✔ |
| CI execution of tests and further checks, with merge blocking where the platform allows it | — | ✔ | ✔ |
| Test verification independent of the implementing agent (6.1) | — | ✔ | ✔ |
| PR review with a checklist by the owner | — | ✔ | ✔ |
| Specification and plan before implementing, proportionate to the change (for a typo, one sentence in the work item) | — | — | ✔ |
| Independent review: another person, or an agent in a separate context that receives the diff, the specification and the check results, not the implementer's conversation. A model from another provider is preferable, not mandatory. It supports the owner's decision and does not replace it | — | — | ✔ |

**Specific activities.** Mandatory because of the nature of the risk, when the pertinence criterion is met.

| Activity | Pertinent when | From level |
| --- | --- | --- |
| Dependency verification (identity, provenance, security advisories, licence) | M4 = significant introduction or change | All |
| Restoration test; earlier evidence is reused if still valid and on a system of the same type | M3 = reversible by procedure | All |
| Explicit owner confirmation before merge, distinguishing established from precautionary irreversibility | M3 = irreversible | All |
| External flow sheet: which data, to whom, why | Signal "external data flow" | All |
| Test of the changed control: it blocks what it must and lets through what it must | The change modifies a control artifact (3.6, 4.6) | All |
| Proportionate threat analysis | M1 = security, or M2 = sensitive, or external data flow | Rigorous |
| Automated security checks of the tech package | M1 = security, or M2 = sensitive | Intermediate |

Non-pertinence follows from the nature of the change and the activity's applicability criteria, never from the cost, difficulty or convenience of omitting the activity.

**Omitted activities and owner decisions.**

| Case | Who decides | Where | Content |
| --- | --- | --- | --- |
| Not pertinent | Agent proposes, owner confirms | PR block | Activity and reason tied to the nature of the change |
| Single derogation | Owner only | PR block | Activity, reason, accepted risk, confirmation |
| Permanent derogation | Owner only | Decisions file | Activity, reason, accepted risk, review condition |
| Required decisions (irreversible change, licence, external flow) | Owner | PR block | Decision and reason |

The minimum threshold admits no derogation. "Not verifiable" outcomes of test verification become derogations only after an alternative verification has been sought, and are counted in the anomaly log (8.3).

**Automated controls.** Control types and blocking rules.

| Area | What it checks | When | Outcome | Example tools |
| --- | --- | --- | --- | --- |
| Secrets | Credentials of the types supported by the tool, including AI service credentials. It does not prove the absence of every secret | Local hooks; push (server side, where available); CI job on every push | See the chain in 6.4 | gitleaks; platform push protection |
| Dependencies (SCA) | Known vulnerabilities in libraries | Before merge (gate) and periodically (notification) | Gate: blocks on critical or high severity with a fix available. Notification: opens a work item | OSV-Scanner, Trivy, Dependency-Track; Dependabot alerts |
| Dependency licences | Licences incompatible with the project | PR | Blocks if on the exclusion list | Dependency-Track, Trivy |
| SAST | Vulnerabilities in the code | PR | Blocks on new code | SonarQube, Semgrep |
| Container and IaC | Vulnerable images, insecure configurations | PR, if present | Blocks on critical severity | Trivy, Checkov |
| SBOM | Component inventory | On request or periodically | Artifact | Syft, CycloneDX |
| DAST | Vulnerabilities of running, reachable applications and APIs | Periodically or on a test environment | Blocks on high severity, after false-positive analysis | ZAP |

**Activation.** Correction 2 of phase 0: activation follows the conditions above, not the v4 profile matrix.

| Control | Activation |
| --- | --- |
| Secrets | Every level (minimum threshold) |
| SCA | Notification at every level (Dependabot alerts enabled at bootstrap); gate from intermediate, as part of the further CI checks |
| Licences and dependency provenance | Dependency verification, when M4 = significant introduction or change |
| SAST, container and IaC | Automated security checks of the tech package, from intermediate when M1 = security or M2 = sensitive |
| Threat model (STRIDE or another method) | Proportionate threat analysis, at rigorous when pertinent |
| Main-branch protection | Every level, where available |
| SBOM, DAST, OWASP ASVS requirements | Catalogue practices (2.3), selected per project; none is mandatory solely because of the profile, and once adopted their controls and blocking rules bind like any other. ASVS (version 5.0.0, cited as v5.0.0-\<chapter>.\<section>.\<requirement>) is a versioned reference for the threat analysis and the security requirements of web software |

**Results management.** For every blocking scanner: severity scale, baseline (5.5), false-positive analysis, fix times, exceptions with an expiry date.

**Process controls.**

| Control | Result |
| --- | --- |
| Dependency policy | In applications: lockfile. In libraries: compatible constraints. Configurable waiting period per ecosystem; urgent security updates handled separately |
| Main-branch protection | Merge only through PRs with passed required checks, where the platform makes it available, the protection is active and no bypass permission applies (6.4) |

**Controls specific to AI agents.**

| Risk | Control | Nature |
| --- | --- | --- |
| Destructive or unauthorized actions | Minimal permissions, explicit confirmation for irreversible actions | Tool rule |
| Data sent to model providers | Allowed providers and paths excluded from reading, in the decisions file; enforced through instructions and the tools' permission rules | Not guaranteed: an agent with access to the user's shell can read excluded files |
| Agent credentials and privileges | As long as the agent uses the user's credentials it can perform the same actions (7.8) | Declared limit |
| Prompt injection through code, issues or files | Issue and file content treated as data; review agents with read-only tools; explicit confirmation for sensitive actions | Instruction and tool rule |

### 6.4 Guarantees by repository type on GitHub Free

**Secret control chain.** The same roles apply to every repository; only what the platform offers changes.

| Step | Role | Public repository | Private repository |
| --- | --- | --- | --- |
| 1. Local hook before commit or push | Prevention, bypassable | Yes | Yes |
| 2. Repository push protection, server side | Prevention: can block pushes containing supported secrets; bypassable by whoever has write access, with an alert | Check at bootstrap, enable if off and verify the resulting state | Not available without a paid plan |
| 3. Secret-scanning job in CI | Detection after the push: configured to run on every push to any branch and to scan all pushed commits, not just the final state | Mandatory | Mandatory: it is the only remote defense |
| 4. Credential rotation | Remedy: work item with an owner and confirmation of revocation and replacement. Removing the secret from the code does not close the case | Yes | Yes |

Repository push protection and push protection for users are distinct layers; the second is set per account. The repository's initial state is not assumed. In phase 1 the block was proven with both layers active, not with the repository layer alone (phase-1 capability matrix, `docs/audit/github-free-capabilities.md`: O4, E8–E10, P11, P12). The bypass with an alert is documented, not tested.

The step 3 job does not prevent anything: when it runs, the secret is already on GitHub. It detects supported secrets that escaped the hooks and triggers the rotation. In private repositories it consumes the compute usage included in GitHub Free, a monthly monetary amount rather than a number of minutes: each job's time is rounded up to the whole minute, and without a valid payment method usage is blocked once the included amount is used up. These rules are documented; in phase 1 the usage amount shown on the billing page was consistent with the rounding per job, and the block was not tested (phase-1 capability matrix: E14–E19, V9). Remote detection depends on the job being triggered, completing and covering the relevant commits and secret formats: a commit message containing a skip marker such as \[skip ci\] prevents push-triggered workflows, and in private repositories the included compute usage can be used up. A missing or incomplete run is never evidence that no secret was pushed: it is reported as incomplete coverage. The remedy for skip markers is chosen after the Q9 check.

**Other guarantees.**

| Guarantee | Public repository | Private repository |
| --- | --- | --- |
| Required checks before merge | Available; see the conditions below | Not available: checks report, but merging remains possible |
| Required code-owner review | Available; see the single-account limitation below | Not available |
| Vulnerable dependency alerts (Dependabot) | Available, to enable at bootstrap | Available, to enable at bootstrap |

In phase 1, required checks blocked the merge on a public repository with an active ruleset and no applicable bypass permissions. Classic branch protection on a public repository was not tested; the expected source of required status checks was not pinned and remains a Q9 check (phase-1 capability matrix: P1, P5–P9, V1, V2, V6).

With a single account acting as both PR author and sole code owner, required code-owner review did not block the merge with zero required approvals. With one required approval, the merge was blocked by the approval requirement, and self-approval was refused. In the configurations tested, the owner's review remains a process step rather than a server-enforced approval (6.3; phase-1 capability matrix: P10). Enforcement with multiple accounts was not tested.

**Control status summary.** After changing a protection setting, the bootstrap reads the effective state back: a successful request is not evidence that the protection is active (phase-1 capability matrix: V7). From the decisions file and the platform check, the bootstrap generates a summary of the controls that are **blocking**, **advisory** (they report but do not block) and **absent**. Whether a control blocks is distinct from whether the methodology makes it mandatory. The summary is generated, not written by hand, and is referenced in PRs, so that no message presents an advisory control as blocking.

**Consequence.** On a private repository with GitHub Free, the minimum threshold requires bypassable local prevention plus remote detection, which depends on the job actually running, and mandatory rotation; gates are reports: compliance depends on the discipline of whoever integrates. The document states this instead of attributing to these controls a guarantee they do not have.

## 7. AI agents and tool independence

### 7.1 Requirement

The methodology must be usable with different AI agents, keeping rules and decisions consistent when the agent, model or environment changes. The first version supports **Claude Code and Codex**, on Linux and with GitHub Free.

The methodology addresses **agents** (programs that read the repository and perform actions). Models affect how well instructions are followed, not the format of the instructions; editors (e.g. VS Code, Zed) host agents. The goal is consistency of rules and controls, not identical behavior across agents.

### 7.2 Four-layer architecture

| Layer | Content | Rule |
| --- | --- | --- |
| 1. Source of truth | Principles, decisions file, specifications, ADRs | Tool-neutral |
| 2. Shared instructions | A short AGENTS.md that points to the decisions file; methodology procedures in the open Agent Skills format (SKILL.md), loaded when needed | Valid for every agent that supports the formats |
| 3. Adapters | Tool-specific files, only when needed | They contain no project rules: they point to layer 2 and add only the tool's mechanics |
| 4. Controls | Git hooks, CI, scanners, server-side protections where available (6.4), which read the decisions file | Identical for every agent and for people |

**Short always-loaded context.** Many or long instructions reduce the probability that the agent uses the relevant ones at the right moment. AGENTS.md contains only what is needed in every session; detailed procedures and references live in the skills.

Layer 4 is what makes decisions binding (P5), within the limits of its technical preconditions.

### 7.3 Adapters

Adapters are written by hand and created only when they add something the agent does not get from the shared formats. Example: Claude Code reads AGENTS.md directly if the project has no CLAUDE.md; a CLAUDE.md is needed only for Claude Code-specific instructions, and in that case it imports AGENTS.md:

```markdown
@AGENTS.md

## Claude Code only
- Subagents that write code work in an isolated worktree.
```

A CI check verifies that every agent adapter imports the shared instructions file; an automatic generator is planned only if the supported tools exceed three. For every supported agent, the minimum version and configuration are recorded, with an actual test of instruction loading (7.5).

### 7.4 Agent roles

The core defines abstract roles (design, implementation, review), not specific models or agents. The assignment of a model or subagent to a role lives in the adapters. An agent does not approve its own proposals. At the rigorous level, the independent review runs in a context separate from the implementer's (6.3).

### 7.5 Agent qualification

Before adopting an agent, or a significant new version of it, a small set of scenarios is run with it and the outcome is recorded. For minor versions a quick regression subset is enough. Scenarios produce **observable evidence**, not agent statements.

| Scenario | Observable evidence |
| --- | --- |
| Instruction loading | A canary rule present only in AGENTS.md (e.g. a mandatory branch-name prefix) is respected without being mentioned in the task |
| Work item that requires a security change | The agent proposes M1 = security and the PR block shows the higher level computed by the script |
| File or issue containing malicious instructions | The agent does not perform the actions those instructions ask for |
| Contradictory project configuration | The inconsistency is reported |
| Attempt to change a mandatory control together with code | The integrity check (4.6) reports it |
| Switch from Claude Code to Codex on the same project | Decisions accessible and controls applied in the same way |
| Action requiring human authorization | The agent does not proceed on its own |

Qualification makes some operational guarantees observable; it does not prove that an agent will always behave correctly.

### 7.6 Web interfaces

Models' web interfaces are outside the guaranteed perimeter in the first version: what is produced there enters the project through a PR and its controls. An exportable "context package" is planned as a later improvement.

### 7.7 Agent traceability

A trailer in the commit message can state the agent and model used. It is off by default and is enabled in the decisions file; when enabled, the commit-message hook checks its format.

### 7.8 Agent credentials and isolation

**Situation in the first version.** The agent works as the person's user and can reach their credentials. The limit is declared (4.6, 6.3); the permission rules of the agents' tools reduce accidental actions but are not a guarantee.

**Requirement.** When segregation is needed, it will be expressed as a requirement of **effective isolation**: the agent cannot use credentials more privileged than intended. The means stays open (dedicated user, container, sandbox, virtual machine), as long as it is verifiable. Passing the agent a different token without isolation is not adopted.

**First version.** Effective isolation is an optional catalogue practice (2.3), with no automatic trigger. After the pilot projects, if the anomaly log shows problems tied to the agent's privileges, it will be defined when to recommend it.

**Warning.** When the decisions file or the repository show that the agent works with real secrets or administrative privileges, the methodology does not recommend isolation, but shows an explicit warning about the residual risk, consistent with the transparency rule of 4.1.

### 7.9 Known limits

- Instructions to agents are context, not constraint.
- Features such as subagents, permissions and agent hooks are not standardized across tools.
- Open formats are recent and their support changes between agent versions.
- Different models follow the same instructions with different adherence.

## 8. Validation and improvement of the methodology, project monitoring

### 8.1 Minimum complete version and milestones

The methodology is first built as a thin but complete version: every phase exists in essential form, and it is tested by taking a real feature of a pilot project from request to merge.

| Milestone | Conditions |
| --- | --- |
| 1.0 experimental | The starting methodology repository has been converted to the new methodology; the scenario suite passes; the first pilot project is completed |
| Stable | A second pilot project completed in a **different language** from the first, so as to test the procedure for adding a tech package; of the two pilots, **one public and one private**; changes of different risk, both supported agents, at least one deliberate attempt to bypass a control |

**Scope of the stable version.** It guarantees operation within the tested perimeter (stacks, agents, repository types) and provides a documented procedure, already tested once, to create and qualify a new tech package. Two pilot projects do not prove generalizability: the tested perimeter is declared.

**Exit criterion.** Completing the pilots is not enough to declare the stable version: no unresolved critical problems may remain within the declared perimeter, essential controls must have been verified on both repository types, and limitations must be documented.

### 8.2 How the methodology is verified

| Type of measure | Question | Use in the methodology |
| --- | --- | --- |
| Comparative effectiveness on one's own projects | "Is practice X better than practice Y?" | Not used as the foundation of a rule |
| Property check | "Does this artifact respect this rule?" | Gate, fitness functions, security controls |
| Methodology behavior | "Given this context, does the methodology produce the expected result?" | Scenario suite |

**Scenario suite.** Every scenario contains an input and the expected result, written by whoever maintains the methodology and derived from the rules. The suite is rerun at every change of the methodology and includes counterexamples. It also verifies the classification script's rules (3.6).

**Pilot projects.** They serve to discover unexpected behavior, to be turned into new scenarios. Events are recorded, not measures to compare.

**Three validation levels.**

| Level | Question | How it is verified |
| --- | --- | --- |
| Correctness of the rules | Does the classifier produce the expected result? | Scenario suite |
| Tool compatibility | Do gate and agents perform the procedures as expected? | Agent qualification (7.5), tech package tests, bypass attempts |
| Adequacy of the methodology | Do the practices produce useful results at reasonable cost? | Observation in the pilots only, without claiming causal proof |

**Minimum scenarios for the rigor model.** Expected results derived from sections 3 and 6.3.

| Scenario | Expected result |
| --- | --- |
| Local script, documentation-only change | Effective level light; no tests or test verification; work item optional |
| Multi-user web app reachable from the Internet with ordinary data, new CSV export | Profile intermediate; effective level intermediate; work item, test verification in CI, owner review |
| Light project that introduces authentication | M1 = security, change level high, effective level rigorous; threat analysis and the tech package's security checks are pertinent |
| Rigorous project, typo fix | Effective level rigorous; guarantees that presuppose a behavior change (tests, test verification) do not apply; specification reduced to one sentence; independent review still applies |
| Rigorous profile recommended, light adopted | Both recorded with the justification of the gap; effective level = the higher of light and the change level |
| A factor answered "unknown" / a field left empty | Most unfavorable value applied and noted / calculation refused |
| New dependency with the "status" group unknown | Change level high until verified |
| Private repository without merge blocking | The absence of remote blocking is reported, without declaring non-existent protections |
| Change to the decisions file together with code | The integrity check reports it; separate PRs are required |

Further scenarios to prepare for validating the rigor model: the household expenses app (local; then shared in the family; then with bank statement import), a banking app, a family medication app, a management app for an association, the methodology repository itself.

### 8.3 Anomaly log

A single simple log per project, recording rules that do not fit the case, unjustified blocks, derogations and recurring "not verifiable" outcomes. Improvement proposals for the shared methodology go through a voluntary channel (e.g. an issue in the methodology repository). Before submission from a private project, the text is cleaned of code, secrets and internal names and approved by the owner.

Postponed: a global user journal, local customizations layered on the methodology, semi-automatic promotion of changes.

### 8.4 Project monitoring

Monitoring is limited to:

- unresolved known vulnerabilities in dependencies;
- review conditions: those observable from the repository with a deterministic periodic check, those not observable with the check question of 3.5.

Every signal opens a work item to analyse, with no automatic action on the code.

### 8.5 Updating projects

Every project records the methodology version it applies. An update is never automatic: a command shows the differences with the new version and the update enters through a PR. Incompatible changes require a compatibility policy between versions and migration guidance, including migrations of the decisions file schema.

## 9. Worked example (illustrative)

The example applies the rules of sections 3 and 6.3; the factor values are plausible examples, not prescriptions.

**Project:** an application for managing household expenses, in three variants.

| Variant | Characteristics | Factors | Recommended profile |
| --- | --- | --- | --- |
| A | Local application, one user | P1 ordinary, P2 local, P3 recoverable | Light |
| B | Web application reachable from the Internet, several family users, authentication | P1 ordinary, P2 Internet, P3 recoverable | Intermediate |
| C | Like B, connected to bank accounts | P1 sensitive, P2 Internet, P3 severe (presumptive indicator: sensitive data of other people) | Rigorous |

**Bootstrap of variant C, private repository.** The methodology recommends the rigorous profile. The owner adopts the light profile and records the justification and the accepted risk. The gap protects less than it seems: every change that alters the handling of banking data has M2 = sensitive, so its effective level is rigorous anyway. The owner keeps secret scanning (mandatory) and excludes .env from the agents' reading. The bootstrap:

- installs the local hooks and the CI secret-scanning job, which runs on every push to any branch;
- enables Dependabot alerts;
- records that branch protection, push protection and merge blocking are not available;
- generates the summary: no blocking server-side control; advisory: the secrets job and the alerts; absent: branch protection, push protection and merge blocking;
- shows a warning: the agent works with credentials that give access to banking data (7.8).

**Changes in variant B (intermediate profile, public repository).**

| Change | Factors and change level | Effective level | Activities and controls |
| --- | --- | --- | --- |
| Change a button's color | M1 form, nothing else → minimum | Intermediate | Work item; no tests, because behavior does not change; CI checks; owner review. The lightweight path is not available, because the profile is intermediate |
| Change the CSV header from "Amount" to "Amount €" | M1 logic, M2 ordinary → ordinary | Intermediate | No logic to alter, so test verification reports "not verifiable". The agent adds a test that compares the exact header and states what it detects: "fails if the header changes" |
| Export expenses to CSV | M1 logic, M2 ordinary → ordinary | Intermediate | Work item with completion criterion; described tests, verified in CI independently of the implementer; owner review with checklist; user documentation |
| Fix: the total ignores the expenses of the last day of the month | M1 logic, M2 ordinary → ordinary | Intermediate | A test reproduces the defect and fails on the previous code (152.40 expected, 140.90 obtained): it counts as test verification |
| Sign in with a Google account | M1 security; M4 new authentication library, with a critical characteristic (sensitive function) → high | Rigorous | Specification and plan; proportionate threat analysis; the tech package's security checks; dependency verification; tests for denied access; independent review. If the change sends project data to Google, the external flow sheet is also pertinent |

Example of a described test for the export:

```python
def test_export_csv_one_row_per_expense():
    """3 expenses in: 4 rows expected (header + 3)."""
    rows = export_csv(three_expenses()).splitlines()
    assert len(rows) == 4
```

**Working with two agents.** The export feature is implemented with Codex and its review is prepared with Claude Code. Both read the same AGENTS.md, which points to the decisions file. The controls that decide the merge are the same hooks and the same CI, whatever the agent; the merge is decided by the owner.

## 10. Open questions

Three questions remain open: Q9, Q12 and Q16. Q17 and Q18, left in draft by v5, were closed in v5.1. Numbering from earlier versions is kept; closed questions stay listed with their outcome.

| # | Question | Status | Section |
| --- | --- | --- | --- |
| Q1 | Classification criteria for project and change | Closed (phase 0): seven factors with definitions, signals and classification rules | 3.1–3.3 |
| Q2 | How factors combine; who computes; script language | Closed (phase 0): threshold tables, effective level, Rust script, owner confirms | 3.2–3.4, 3.6 |
| Q3 | Activities required by each level | Closed (phase 0): general guarantees, specific activities, omission rules | 6.3 |
| Q4 | Journal and improvement of the methodology | Closed (v2) | 8.3 |
| Q5 | Project monitoring | Closed (v2); frequency of declared checks defined in v4 | 8.4, 3.5 |
| Q6 | Work cadence | Closed (v2, v3) | 5.3 |
| Q7 | Default technology stack | Closed (v3): optional tech packages | 5.2 |
| Q8 | CI in the light profile | Closed (v4): minimum secret-scanning job mandatory in every repository | 5.4, 6.4 |
| Q9 | Tool checks, by priority: actual GitHub Free capabilities on public and private repositories (branch protection, push protection, bypasses, CODEOWNERS, GitHub Actions included compute usage and billing); test verification in the first pilot's stack; loading of AGENTS.md and skills in Claude Code and Codex; AI credential formats recognized by scanners; Dependabot alerts on private repositories; integrity check security against PRs built to replace it; skip markers such as \[skip ci\] on the secret-scanning job; skipped or failed jobs and the always-evaluated gate result; expected source of required status checks | **Open** (verification activity). Partly done in stage 0: on a public repository, a ruleset without bypass actors rejects direct pushes to main, including the owner's | 4.6, 5.2, 6.1, 6.2, 6.4, 7 |
| Q10 | Roles in a team | Closed (v3); authorizations approved in v5 | 4.1 |
| Q11 | Content language | Closed (v3) | 5.4 |
| Q12 | Licence | **Open**: to be clarified with the author of the original methodology, starting now and in parallel; it does not block v5 | — |
| Q13 | When to recommend agent isolation | Closed for the first version (v3); residual-risk warning approved in v5 | 7.8 |
| Q14 | Test verification when alteration is not applicable | Closed (v3, v4): "not verifiable" outcome, alternative verification or owner-approved derogation | 6.1 |
| Q15 | Completion criterion | Closed (v3, v4): milestones and scope of the stable version; exit criterion approved in v5 | 8.1 |
| Q16 | Project and language of the first pilot | **Open**: not yet definable | 5.2, 8.1 |
| Q17 | SBOM, DAST and OWASP ASVS: not covered by the phase-0 decisions | Closed (v5.1): catalogue practices; ASVS as a versioned reference | 2.3, 6.3 |
| Q18 | Class of the guarantees and activities required by 6.3 | Closed (v5.1): essential for the change | 4.2 |

## 11. Changes from v4

v5 closes Q1–Q3, applies the six phase-0 corrections and approves the v4 draft items. Origin: **P0** = phase-0 decision (Q1–Q3); **C0-n** = phase-0 correction n; **D** = author's decision; **C** = consequence of another change.

| # | Section | Change | Origin | Status |
| --- | --- | --- | --- | --- |
| 1 | All | Document written in English; v4 draft items reviewed in phase 0 and approved; default status becomes "approved" | D | Approved |
| 2 | 2.2 | P8–P11 approved; new P13, simplicity in the methodology (against excess complexity) | P0 | Approved |
| 3 | 2.3 | Catalogue practices are ways to obtain the results required by 6.3 | C | Approved |
| 4 | 3.1–3.3, 3.7 | Seven factors (P1–P3, M1–M4) with definitions, signals, critical characteristics, classification rules and agent questions; closes Q1 | P0 | Approved |
| 5 | 3.2–3.4 | Profile table P1 × P2, with P3 severe → rigorous; change level = highest factor; effective level table, with no exceptions; closes Q2 | P0 | Approved |
| 6 | 3.6 | Rust script with deterministic threshold rules; "unknown" vs "missing"; PR classification block; classification reconfirmed before merge | P0 | Approved |
| 7 | 6.3 | General guarantees per level, specific activities with pertinence criteria, rules for omitted activities; closes Q3 | P0 | Approved |
| 8 | 5.4 | Work item can be omitted only at effective level light | C0-1 | Approved |
| 9 | 6.3 | Control activation follows the conditions of 6.3; the v4 profile matrix is removed | C0-2 | Approved; SBOM, DAST, ASVS open (Q17) |
| 10 | 4.6, 5.1, 6.3 | Main-branch protection at every level, where available | C0-3 | Approved |
| 11 | 2.3, 5.2, 6.1 | Test verification independent of the implementer; mutation testing optional; packages require the result, not the technique; periodic full mutation testing becomes a catalogue practice | C0-4, C | Approved |
| 12 | 4.1, 6.1 | Derogations, including "not verifiable" outcomes, approved by the owner; the risk-acceptance approver is the owner | C0-5 | Approved |
| 13 | 3.2, 4.6 | A profile change assesses the future state; rules-only PR merged before the code PR | C0-6 | Approved |
| 14 | 6.1 | At light level, tests run locally without verification by alteration | P0 | Approved |
| 15 | 5.4 | CI follows the effective level of the change; a light project needs a CI workflow able to run the gate | C | Approved |
| 16 | 4.2 | Guarantees and activities required by 6.3 are essential for the change | D | Draft (Q18) |
| 17 | 4.4 | Decisions file contents extended (factors with provenance, rules version, permanent derogations); example updated | C | Approved |
| 18 | 7.5 | Security-change scenario: the evidence is the level computed by the script | C | Approved |
| 19 | 8.2 | Minimum scenarios aligned to the v5 rules; further scenarios listed | C | Approved |
| 20 | 9 | Example aligned to the factors and the effective level; a button-color change at intermediate profile no longer uses the lightweight path | C | Approved |
| 21 | 10 | Q1–Q3 closed; Q17 and Q18 added | C | — |

v4 draft items approved without changes other than those above: practice catalogue; authorizations and transparency (4.1); process (4.2); schema validation of the decisions file; precisions on rule foundations; integrity check details; ADRs; rule attributes; phases; technology evaluation and package capabilities; work item types; full path and commit conventions; baseline cases; 6.1 precisions; gate; results management; other guarantees and control summary (6.4); four-layer architecture; agent roles; agent qualification; isolation warning; exit criterion; three validation levels; cleaning of anomaly reports; project updates.

## 12. Changes after the final review (v5.1)

v5.1 applies the corrections from the final GPT-6 review of v5 and the roadmap, as evaluated and approved on 25 November 2026. No phase-0 decision was reopened. Origin: **R01–R09** = findings of that review; **A** = additions made while evaluating it; Review = its recommendations on Q17, Q18 and 5.4.

| # | Section | Change | Origin |
| --- | --- | --- | --- |
| 1 | 3.3 | M4: the critical characteristics that raise the level exclude the licence; a significant change without them is ordinary | R01 |
| 2 | 3.6, 6.3 | The PR block records two applicability conditions: behavior change and change to a control artifact | R02 |
| 3 | 3.6, 4.1, 4.6 | The gate recomputes the levels with the main branch's classifier, rules and decisions file; "control artifacts" defined once in 4.6, classifier and rules included | R03 |
| 4 | 3.2, 4.6 | A raised profile: rules-only PR first. A lowered profile: rules-only PR after the code PR | R04 |
| 5 | 6.1 | "Not verifiable" stays unresolved until an alternative verification or an owner-approved derogation | R05 |
| 6 | 6.2 | One always-evaluated gate result; skipped or failed jobs never count as successful | R06, A |
| 7 | 6.4 | Remote secret detection depends on the job running; skip markers and minutes declared | R07, A |
| 8 | 4.6 | The integrity check derives its execution and acceptance from a trusted source | R08, A |
| 9 | 2.3, 6.3, 10 | SBOM and DAST are catalogue practices; ASVS is a versioned reference; Q17 closed | Review |
| 10 | 4.2, 10 | Required guarantees and activities are essential for the change; Q18 closed | Review |
| 11 | 5.4 | Every project has a CI able to apply the gate by effective level; an unavailable check is not non-pertinent | Review |
| 12 | 10 | Q9 extended: skip markers, skipped or failed jobs, expected source of required checks | A |

From v5.1 the methodology is frozen against further preventive design reviews. Later changes come only from concrete problems found during implementation, technical checks or pilot projects.
