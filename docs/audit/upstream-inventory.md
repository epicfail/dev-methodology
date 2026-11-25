# Phase 1 inventory of the upstream repository

| Field | Value |
|---|---|
| Upstream repository | `whitesheep/dev-methodology` |
| Baseline commit | `53af85eeff92a0f09d65c61dea159f1e8565c38e` (tag `upstream-baseline`) |
| Reference specification | `docs/methodology/specification.md`, v5.1 |
| Purpose | Classify every element of the upstream repository at the baseline against the specification, to prepare phase 2 |
| Normative status | Non-normative |

The specification decides what the new repository contains; this inventory only records what can be reused. A class is neither a commitment to implement nor a permission to reuse: reuse of upstream text and of third-party material depends on Q12.

## Classes

| Class | Meaning |
|---|---|
| keep | Valid as it is |
| generalize | Valid, but tied to a stack or an agent |
| make optional | Useful only in some contexts; no implementation commitment. The sentence at the end of each row states the condition for its reuse. |
| replace | The goal stays, the solution changes |
| delete | In conflict with the specification, or without a current need (P11, P13) |

Between `delete` and `replace`: an integration with a tool that the specification does not provide for (bmad, spec-kit, ponytail, graphify) is `delete`, even when its general goal survives elsewhere in the specification; a rule or process whose goal the specification keeps with a different solution is `replace`.

Every `make optional` row ends with a sentence that states the condition for its reuse. Rows about specific technologies use: "Candidate for a future tech package or catalogue practice; implemented only if the pilot stack (Q16) or a demonstrated need requires it."

## Conventions

- Rows keep the numbers of the approved table; letters mark the functional groups of a row split for phase 2.
- `SKILL.md` stands for `skills/dev-methodology/SKILL.md`, `templates/` for `skills/dev-methodology/templates/`, troubleshooting for `skills/dev-methodology/references/troubleshooting.md`. Line numbers (L) refer to the files at the baseline.
- Renamed after the baseline by PR #1 of this fork: `templates/CLAUDE.md` is now `templates/CLAUDE.md.tmpl`, `templates/skills/*/SKILL.md` are now `SKILL.md.tmpl`; the root `CLAUDE.md` was replaced.
- § is a section of the specification, P a principle (§2.2), Q an open question (§10).

## Inventory

| # | Element (baseline path) | Content | Class | Reason | Spec |
|---|---|---|---|---|---|
| 1 | `.claude-plugin/marketplace.json` | Claude Code marketplace entry; description and owner refer to the previous methodology and the upstream account | make optional | Packaging specific to one distribution channel (Claude Code marketplace); depends on the distribution decision of phase 2 (open point B2). Candidate for agent-specific distribution packaging. Implementation depends on the distribution approach selected in phase 2. | §7.1, §8.5 |
| 2 | `.claude-plugin/plugin.json` | Claude Code plugin manifest, version 0.3.0 | make optional | As row 1; plugin version and methodology version are decided in phase 2 (B2). Candidate for agent-specific distribution packaging. Implementation depends on the distribution approach selected in phase 2. | §7.1, §8.5 |
| 3 | `CLAUDE.md` | Instructions for agents working on the upstream repository: layout, change rules, versioning, git | replace | Goal stays (rules for this repository); solution is AGENTS.md plus a CLAUDE.md adapter that imports it. Already done in the fork (PRs #1, #2) | §7.2, §7.3 |
| 4 | `README.md` › intro (L1–8) | Presents the plugin: bmad, spec-kit, red BDD, TDD, Sonnet subagents, hook gate, constitution, ADHD replies | replace | Goal stays (present the product); each listed component is changed or removed by the specification | §1, §2.1 |
| 5 | `README.md` › What the skill does (L10–21) | Three uses: bootstrap (installs bmad and spec-kit, copies templates), day-to-day work, troubleshooting | replace | Bootstrap stays as a phase with different content: questionnaire, decisions file, protections | §5.1 |
| 6 | `README.md` › The methodology at a glance (L23–52) | Fixed pipeline bmad → spec-kit → BDD → subagent → gate → sprint; rules split across constitution, CLAUDE.md and hooks | replace | Work becomes proportionate to the effective level, continuous flow by default, rules in four layers with a decisions file | §3.4, §5.3, §5.4, §6.3, §7.2, §4.4 |
| 7 | `README.md` › Languages (L54–60) | Identifiers in English; content for people in a content language asked at bootstrap, with no default | keep | Matches §5.4; the Gherkin placeholder follows the class of BDD (catalogue) | §5.4 |
| 8 | `README.md` › Installation (L62–70) | `claude plugin marketplace add/install` from the upstream repository | generalize | Claude Code-only; points to the upstream repository | §7.1 |
| 9 | `README.md` › Updating (L72–79) | Plugin update commands and restart | generalize | Claude Code-only; project updates follow §8.5 (diff command, PR) | §7.1, §8.5 |
| 10 | `README.md` › Dependencies needed by the skill (L83–93) | Claude Code, ponytail, superpowers, BMad, spec-kit, git/bash/perl, gh | replace | Goal stays (declare dependencies); ponytail, superpowers, BMad and spec-kit go, Claude Code becomes one of the supported agents | §2.1, §7.1, P7, P11 |
| 11 | `README.md` › Dependencies needed by the projects (L95–103) | Rust toolchain, cucumber/wiremock/testcontainers, Node/React stack, Penpot, graphify | make optional | No default stack: stack dependencies belong to tech packages, optional tools only where selected. Penpot (row 48b) and graphify (row 39) are deleted. Candidate for a future tech package or catalogue practice; implemented only if the pilot stack (Q16) or a demonstrated need requires it. | §5.2, §2.3 |
| 12 | `README.md` › origin of the reply rules (L105–108) | Reply rules embedded in the template, derived from two third-party plugins | delete | Agent communication style is outside the scope of the methodology | §1 |
| 13 | `README.md` › Repository layout (L110–130) | Layout of the plugin repository | replace | Goal stays; layout changes (AGENTS.md, `docs/methodology/`, new components) | §7.2 |
| 14 | `README.md` › Customizing (L132–137) | Templates are the source of the methodology; bump the version; carry changes into projects by hand | replace | The specification becomes the source of truth; projects record the methodology version and update through a diff command and a PR | spec header, §8.5 |
| 15 | `SKILL.md` › frontmatter (L1–4) | Name and trigger description (bmad, spec-kit, Sonnet subagents, Cargo, BDD) | replace | Agent Skills format stays; triggers describe the removed pipeline | §7.2 |
| 16 | `SKILL.md` › intro (L6–16) | Three single sources (constitution, CLAUDE.md, hooks); templates are the source | replace | As rows 6 and 14 | §7.2, §4.4, §8.5 |
| 17 | `SKILL.md` › How you work: replies (L20–22) | Apply the template's "How to reply and interact" from the start | delete | Agent communication style is outside the scope of the methodology | §1 |
| 18 | `SKILL.md` › How you work: ponytail (L23–25) | Load `ponytail:ponytail` before any code work, subagents included | delete | Mandatory third-party plugin, while tools must be replaceable and rules agent-independent; P11 stays as a principle | §2.1, P7, P11 |
| 19 | `SKILL.md` › Branches: bootstrap, day-to-day, troubleshooting (L29–32) | Routes to bootstrap, to the project CLAUDE.md, to troubleshooting | generalize | Routing is valid; the project CLAUDE.md becomes AGENTS.md pointing to the decisions file, and procedures go in skills (§7.2), not in AGENTS.md or the decisions file | §7.2, §4.4 |
| 20 | `SKILL.md` › Branches: first Rust code story (L33–36) | Copy rust-toolchain.toml and purity.rs, enable `missing_docs` | make optional | Rust tech package; the purity test is a fitness function for hexagonal architecture. Candidate for a future tech package or catalogue practice; implemented only if the pilot stack (Q16) or a demonstrated need requires it. | §5.2, §2.3 |
| 21 | `SKILL.md` › Branches: React dashboard (L37–40) | Install the dashboard-react skill plus vercel-react-best-practices with ORIGIN.md | make optional | React/TypeScript tech package. Installs a third-party skill (vercel-labs/agent-skills, `skills/react-best-practices`) with an ORIGIN.md that records commit and licence, consistent with dependency verification for copied third-party code (§3.3, §6.3); that material is not in the baseline. Candidate for a future tech package or catalogue practice; implemented only if the pilot stack (Q16) or a demonstrated need requires it. | §5.2, §3.3, §6.3 |
| 22 | `SKILL.md` › Branches: screens (L41–42) | Install the ui-design skill when the project has screens | delete | Not in the first version: no additions to the catalogue (open point B3); recoverable from `upstream-baseline` | §2.3, P13 |
| 23 | `SKILL.md` › Bootstrap 1, Decisions (L48–63) | Ask layout, language, modules, domain principles and content language with AskUserQuestion; propose placeholders | replace | Goal stays (collect decisions); questions become factors P1–P3 with provenance, recorded in the decisions file; AskUserQuestion is Claude Code-specific | §3.2, §3.7, §4.2, §4.4, §5.4 |
| 24a | `SKILL.md` › Bootstrap 2, `git init -b main` (L65) | Creates the repository with `main` as default branch | generalize | Valid for a new repository; adoption in an existing project skips it, and the branch name is an assumption | §5.1, §5.5 |
| 24b | `SKILL.md` › Bootstrap 2, hooks path (L65–66) | `git config core.hooksPath .githooks`, checked by its output | keep | Hooks are installed at bootstrap and run the local checks | §5.1, §5.4 |
| 25 | `SKILL.md` › Bootstrap 3, bmad (L67–75) | Install bmad-method with npx | delete | Tool integration not provided for; requirements, architecture and planning are covered by §5.1, §5.3, §4.7 | §5.1, §5.3, P13 |
| 26 | `SKILL.md` › Bootstrap 4, spec-kit (L76–77) | `specify init` with the Claude integration | delete | Tool integration not provided for; specification and plan are required only at rigorous level, proportionate to the change | §5.4, §6.3, P13 |
| 27 | `SKILL.md` › Bootstrap 5, Templates (L78–88) | Copy templates, replace placeholders with perl, check that no `{{` is left and `bash -n` | generalize | Mechanism is valid; file list is tied to Claude Code paths (`.claude/skills`, CLAUDE.md) and to the old templates | §5.1, §7.3 |
| 28 | `SKILL.md` › Bootstrap 6, Check (L89–92) | commit-msg test, no foreign project names, no Cargo.toml | replace | Goal stays; checks become activation and verification of the available protections plus a generated control summary; "no code before architecture" is not a specification rule | §5.1, §6.4 |
| 29a | `SKILL.md` › Bootstrap 7, initial commit (L93–95) | First commit and first push of the new repository, only when the user asks | replace | Goal stays; the sequence that creates the first commit and enables main-branch protection is designed in the bootstrap and verified in phase 4, without bypass (open point B4); message conventions follow row 38 | §4.6, §5.1 |
| 29b | `SKILL.md` › Bootstrap 7, `ALLOW_MAIN=1` (L93–95) | Hook bypass for the first commit and push on main | delete | A documented bypass contradicts the protection model; replaced by row 29a | §4.6, §6.4 |
| 30 | `SKILL.md` › Without a dashboard (L97–98) | Remove dashboard sections from CLAUDE.md and skip its skill | replace | Optional modules become tech packages chosen per project, not sections cut from a monolithic file | §5.2 |
| 31 | `SKILL.md` › Template table (L100–115) | Template → destination → placeholders | keep | Structure stays (AGENTS.md rule 9 relies on it); content follows the templates | §5.1 |
| 32 | `SKILL.md` › Multi-repo variant (L117–123) | Architecture, ADRs and user docs in sibling repositories through bmad `spine_output_path` | delete | Depends on the bmad override; no current need | P13, §4.6 |
| 33 | `SKILL.md` › Common mistakes (L125–136) | Eight mistake → fix rows (bmad, constitution, language, Cargo, attribution, superpowers, ponytail, replies) | replace | Format stays useful; each row follows the fate of the rule it refers to | various |
| 34a | troubleshooting › Rust builds, wiremock hang (L9–12, L19) | Cargo target on tmpfs, shared or stale targets across worktrees, disk growth, wiremock `MockServer::drop` hang | make optional | Rust tech package. Candidate for a future tech package or catalogue practice; implemented only if the pilot stack (Q16) or a demonstrated need requires it. | §5.2 |
| 34b | troubleshooting › wrong-branch commits, Subagents (L13, L32–37) | Worktree per writing subagent, waking a stuck subagent, never killing foreign processes | generalize | Claude Code subagent mechanics belong to the agent adapter (§7.3 gives this example) and to role assignment (§7.4). A worktree separates the working copies of concurrent agents: it is neither the temporary copy for alterations (§6.1) nor isolation from credentials (§7.8) | §7.3, §7.4, §6.3 |
| 34c | troubleshooting › exact JSON (L21) | BDD scenarios compare exact JSON, types included | make optional | Lesson of the BDD practice. Candidate for reuse within the corresponding catalogue practice, when selected and relevant. No implementation commitment. | §2.3 |
| 34d | troubleshooting › diverged stub (L20) | Spike on the real service, stub only from what it observed, real smoke test | make optional | Catalogue practice "spike on the real service before the stub". Candidate for reuse within the corresponding catalogue practice, when selected and relevant. No implementation commitment. | §2.3 |
| 34e | troubleshooting › modified migration (L28) | Fixes always in a new migration; sqlx checksum repair in an emergency | make optional | Catalogue practice "immutable database migrations after delivery"; the sqlx repair is Rust-specific. Candidate for reuse within the immutable migrations practice; stack-specific tooling depends on the selected tech package. | §2.3, §5.2 |
| 34f | troubleshooting › user proofs (L27, L29) | Proof steps run from the story worktree; SQL run on the dev database before the PR | generalize | Supports the owner's own test of a feature (§5.4 step 8); tied to worktrees and SQL databases | §5.4 |
| 34g | troubleshooting › `claude` in scripts (L39–45) | Flags and environment to run `claude` safely from hooks and scripts | make optional | Follows row 47. Candidate for optional advisory AI review tooling, conditional on the adoption of the mechanism described in row 47. | §6.2 |
| 34h | troubleshooting › removed parts (L30, L47–54) | superpowers debugging and "communicate only verified causes"; Penpot scripting failures | delete | superpowers is a tool not provided for and agent communication is out of scope (§1); Penpot follows row 48b | §1, P13 |
| 35a | `templates/CLAUDE.md` › frame (L1–12) | Title, one-liner, constitution import, current state with the next bmad step | replace | Short AGENTS.md that points to the decisions file; CLAUDE.md only as an adapter that imports it | §4.4, §7.2, §7.3 |
| 35b | `templates/CLAUDE.md` › Repo layout (L14–26) | Folders for crates, dashboard, features, specs, bmad, architecture, ADRs, user docs, glossary | replace | Goal stays; the layout follows the decisions file and the selected tech packages | §5.2, §7.2 |
| 35c | `templates/CLAUDE.md` › ADR rules (L38–42) | ADR header with date and status, never rewritten, superseded by a new ADR, discarded alternatives | keep | Matches §4.7 and P9 | §4.7, P9 |
| 35d | `templates/CLAUDE.md` › work entry points and sprint pipeline (L36, L44–97) | Feature, extension, bugfix and contract routes; bmad → spec-kit → red BDD → Sonnet → review → gate → PR; end-of-sprint steps; banned commands; spec checks | replace | Typed work items (§5.3) and a workflow driven by the effective level (§5.4); continuous flow by default, sprints optional; bmad and spec-kit steps follow rows 25 and 26 | §5.3, §5.4 |
| 35e | `templates/CLAUDE.md` › external services (L64–66, L103) | Cost and limits in clarify, spike on the real service, stub derived from the spike, real smoke test | make optional | Catalogue practice "spike on the real service before the stub". Candidate for reuse within the corresponding catalogue practice, when selected and relevant. No implementation commitment. | §2.3 |
| 35f | `templates/CLAUDE.md` › migrations (L85–86) | A migration handed over for proving is never modified; fixes go in a new migration | make optional | Catalogue practice "immutable database migrations after delivery". Candidate for reuse within the corresponding catalogue practice, when selected and relevant. No implementation commitment. | §2.3 |
| 35g | `templates/CLAUDE.md` › user proof (L79–84, L107) | Proof steps run before hand-over, from the story worktree, on real services; fixes after hand-over in a new PR | generalize | Supports the owner's own test (§5.4 step 8); tied to worktrees and SQL databases | §5.4 |
| 35h | `templates/CLAUDE.md` › BDD/TDD techniques and stack-specific test conventions (L74–76, L99–106) | Red Gherkin scenarios in the content language, TDD inside, scenario boundary, wiremock and testcontainers, exact JSON | make optional | BDD and TDD are optional catalogue practices; stack-specific test conventions belong to the relevant tech package. The applicable test guarantees remain required by §6.1 and §6.3. Candidate for reuse within the corresponding catalogue practice, when selected and relevant. No implementation commitment. | §2.3, §5.2, §6.1, §6.3 |
| 35i | `templates/CLAUDE.md` › User docs (L69–70, L109–118) | One user page per visible feature, in the same PR, with executable steps and screenshots | generalize | Already required by §5.3 (user documentation for visible features); the page model is tied to Chrome screenshots and dashboard i18n | §5.3 |
| 35j | `templates/CLAUDE.md` › Dashboard (L120–138, except Penpot) | React stack and patterns, Storybook, vanilla-extract, Vitest and Playwright BDD, Biome gate | make optional | React/TypeScript tech package. Candidate for a future tech package or catalogue practice; implemented only if the pilot stack (Q16) or a demonstrated need requires it. | §5.2 |
| 35k | `templates/CLAUDE.md` › subagent delegation (L156–184) | Sonnet subagent writes code; worktree and target folder per subagent; minimal prompt with rules and bans | replace | Fixed model and subagent assignments are replaced by abstract roles (§7.4); tool-specific delegation and worktree mechanics belong to adapters only where needed (§7.3). | §7.3, §7.4 |
| 35l | `templates/CLAUDE.md` › Gate (L186–196) | Cargo and dashboard gate commands; "no CI workflow until production" | replace | Commands come from tech packages; the blocking gate runs in CI from intermediate; every repository has at least the secret-scanning CI job | §5.4, §6.2, §6.3 |
| 35m | `templates/CLAUDE.md` › Git conventions (L198–202, L209–212) | Branch per piece of work, no commits on main, Conventional Commits and SemVer, branch cleanup, hook list | replace | Configurable defaults; the branch carries the work item identifier when a work item exists. The hooks-path line follows row 24b | §5.4 |
| 35n | `templates/CLAUDE.md` › attribution ban (L203–206) | No agent attribution in commits and PRs; commit-msg blocks it | generalize | Agent trailer off by default and configurable in the decisions file; patterns are Claude-only | §7.7 |
| 35o | `templates/CLAUDE.md` › Builds and environments (L217–220) | Cargo target outside tmpfs, one per worktree; a single docker compose | make optional | Rust tech package and project tooling. Candidate for a future tech package or catalogue practice; implemented only if the pilot stack (Q16) or a demonstrated need requires it. | §5.2 |
| 35p | `templates/CLAUDE.md` › dependency rules (L221–222) | Exact pins, versions published at least two weeks earlier, a new dependency only if needed | replace | Dependency policy of §6.3: lockfile in applications, compatible constraints in libraries, configurable waiting period per ecosystem; "only if needed" is P11 | §6.3, P11 |
| 35q | `templates/CLAUDE.md` › `claude` from scripts (L223) | `--no-session-persistence` with `--print` | make optional | Follows row 47. Candidate for optional advisory AI review tooling, conditional on the adoption of the mechanism described in row 47. | §6.2 |
| 35r | `templates/CLAUDE.md` › authorizations and irreversible actions (L271–272, L277–278) | Options only for choices that belong to the user; explicit confirmation before destructive or irreversible actions | replace | Already covered by the authorizations of §4.1 and by the tool rule for irreversible actions in §6.3 | §4.1, §6.3 |
| 35s | `templates/CLAUDE.md` › removed parts (L28–29, L33–35, L37, L50, L87–88, L136, L140–154, L207–208, L213, L225–282 except 35r) | bmad architecture acceptance and AD-n citations, superpowers references, sprint-end tags, Penpot, ponytail, ALLOW_MAIN, graphify, reply and interaction style | delete | Tools not provided for (rows 18, 25, 39, 48b); tags are not part of the methodology (§5.4); bypass as row 29b; communication style out of scope (§1) | §1, §5.4, P13 |
| 36 | `templates/bmad/bmad-architecture.toml` | bmad override: architecture output path and persistent facts | delete | bmad integration; ADRs cover architectural decisions | §4.7, P13 |
| 37a | `templates/constitution.md` › frame and governance (L1–5, L63, L71–75) | Version and dates, single source of principles, domain-principles placeholder, amendment by PR with SemVer, Constitution Check | replace | Invariant principles live in the specification (§2.2); project choices go to the decisions file, changed only through rules-only PRs (§4.4, §4.6); the Constitution Check follows row 26 | §2.2, §4.4, §4.6 |
| 37b | `templates/constitution.md` › general principles (from I, II, IV, VI, VIII) | Discarded alternatives stated, a bug starts from a reproducing test, validation at the boundary, no secrets in logs, YAGNI, identifiers in English and comments in the project language | replace | Goals already stated by the specification (P9, §5.3, P11, §5.4); input validation and secrets in logs are handled through classification and security activities, not as standing principles (§3.3, §6.3) | P9, P11, §3.3, §5.3, §5.4, §6.3 |
| 37c | `templates/constitution.md` › prescribed architectural style (I, III, IV, V) | No code before the accepted architecture, pure hexagonal domain with one composition root, Gateway vs Repository, error translation in adapters, typed columns, least-privilege database user | make optional | Hexagonal architecture is an optional catalogue practice. Context-based architectural decisions and relevant ADRs remain required by P9, P10 and §4.7. Candidate for reuse within the corresponding catalogue practice, when selected and relevant. No implementation commitment. | P9, P10, §2.3, §4.7 |
| 37d | `templates/constitution.md` › Rust rules (III, IV, VI, VIII) | purity.rs, serde validation, no `unwrap` on external input, clippy.toml for the HTTP client, macros, rustdoc with `missing_docs` and `cargo doc` | make optional | Rust tech package. Candidate for a future tech package or catalogue practice; implemented only if the pilot stack (Q16) or a demonstrated need requires it. | §5.2 |
| 37e | `templates/constitution.md` › II BDD first, TDD inside (L14–19) | Gherkin scenarios red before code, TDD inside, declared non-negotiable | make optional | Catalogue practices; P2 and §6.1 require effective, described tests, not a technique. Candidate for reuse within the corresponding catalogue practice, when selected and relevant. No implementation commitment. | §2.3, P2, §6.1 |
| 37f | `templates/constitution.md` › Process constraints (L64–69) | Sprints of 1–3 stories, bmad and spec-kit, branch, PR, Conventional Commits, SemVer, no attribution, user docs | replace | Continuous flow by default (§5.3), workflow and conventions of §5.4, configurable trailer (§7.7); user docs follow row 35i | §5.3, §5.4, §7.7 |
| 37g | `templates/constitution.md` › removed parts (VI, VII, VIII) | "No Python", the ponytail reference, the glossary rule | delete | The stack is chosen per project (§1, §5.2); ponytail follows row 18; the glossary follows row 43 | §1, §5.2, P13 |
| 38 | `templates/githooks/commit-msg` | Conventional Commits check; blocks Claude attribution | generalize | Conventional Commits is the configurable default; the agent trailer is off by default and, when enabled, this hook checks its format; patterns are Claude-only | §5.4, §7.7 |
| 39 | `templates/githooks/post-commit` | Updates the graphify code graph | delete | Tool not provided for; no current need | P11, P13 |
| 40a | `templates/githooks/pre-commit` › main guard (L6–10) | Blocks commits on main | keep | Local, bypassable prevention of direct changes to main; server-side protection provides enforcement where available (§6.4). Local secret scanning, the minimum threshold, is absent upstream and must be added | §4.3, §5.4, §6.4 |
| 40b | `templates/githooks/pre-commit` › `ALLOW_MAIN` (L3, L7) | Bypass of the main guard | delete | As row 29b | §4.6 |
| 40c | `templates/githooks/pre-commit` › `cargo fmt` (L12–18) | Formatting check when Cargo.toml exists | make optional | Rust tech package. Candidate for a future tech package or catalogue practice; implemented only if the pilot stack (Q16) or a demonstrated need requires it. | §5.2 |
| 41a | `templates/githooks/pre-push` › main guard (L10–14) | Blocks pushes to main | keep | Local, bypassable prevention of direct changes to main; server-side protection provides enforcement where available (§6.4). | §4.6, §6.4 |
| 41b | `templates/githooks/pre-push` › Rust checks (L27–47, L64–73) | Target-folder cap, clippy, test compilation, ts-rs generated types, cargo doc | make optional | Rust tech package; local tests before the PR are required at every level, the blocking gate is CI. Candidate for a future tech package or catalogue practice; implemented only if the pilot stack (Q16) or a demonstrated need requires it. | §5.2, §6.2, §6.3 |
| 41c | `templates/githooks/pre-push` › test time cap (L48–63) | Tests under `timeout` with process-group kill and a configurable cap | generalize | Reusable for the time limit that §6.1 requires on alterations; tied to `cargo test` | §6.1 |
| 41d | `templates/githooks/pre-push` › dashboard gate (L75–86) | `npm run gate` when `dashboard/` exists | make optional | React/TypeScript tech package. Candidate for a future tech package or catalogue practice; implemented only if the pilot stack (Q16) or a demonstrated need requires it. | §5.2 |
| 41e | `templates/githooks/pre-push` › advisory review (L88–90) | Calls review-diff.sh and never blocks | make optional | Follows row 47. Candidate for optional advisory AI review tooling, conditional on the adoption of the mechanism described in row 47. | §6.2 |
| 41f | `templates/githooks/pre-push` › removed parts (L4, L11, L15–25) | `ALLOW_MAIN` and `--no-verify` bypasses; deploy-remote check | delete | Bypass as row 29b; release and deployment are out of scope (§1) | §1, §4.6 |
| 42 | `templates/gitignore` | Ignores `.env`, Rust target, graphify, local `.claude` files, bmad user files, dashboard builds | generalize | `.env` rules are valid; the rest is tied to the stack, to Claude Code and to removed tools | §5.2, §6.3 |
| 43 | `templates/glossary.md` | Domain term (content language) → English identifier | delete | Not in the first version (open point B3); may return within the DDD catalogue practice; recoverable from `upstream-baseline` | §2.3, P13 |
| 44 | `templates/graphifyignore` | graphify exclusions | delete | Tool not provided for | P13 |
| 45 | `templates/rust/purity.rs` | Test: the domain crate depends on no I/O crate | make optional | Rust tech package; fitness function for hexagonal architecture. Candidate for a future tech package or catalogue practice; implemented only if the pilot stack (Q16) or a demonstrated need requires it. | §2.3, §5.2 |
| 46 | `templates/rust/rust-toolchain.toml` | Pins Rust 1.98.1 with rustfmt and clippy | make optional | Rust tech package. Candidate for a future tech package or catalogue practice; implemented only if the pilot stack (Q16) or a demonstrated need requires it. | §5.2 |
| 47 | `templates/scripts/review-diff.sh` | Advisory security review of the diff with the `claude` CLI, read-only tools | make optional | Claude-only advisory review of the diff. The independent review of §6.3 is a procedure, not this script; no multi-provider abstraction is built. Candidate for optional advisory AI review tooling. Implementation requires a demonstrated need and compliance with the agent data policy. | §6.2, §6.3 |
| 48a | `templates/skills/dashboard-react/SKILL.md` › React rules (all except Penpot) | Navigation, data, area states, components, style, text, accessibility, tests, dependencies | make optional | React/TypeScript tech package. The file records no third-party source or ORIGIN.md; it refers to the third-party skill of row 21. Candidate for a future tech package or catalogue practice; implemented only if the pilot stack (Q16) or a demonstrated need requires it. | §5.2 |
| 48b | `templates/skills/dashboard-react/SKILL.md` › Penpot (L90–91, L95–97) | Pages drawn in Penpot first; screenshots compared with Penpot boards | delete | Penpot integration is not in the first version (open point B3); recoverable from `upstream-baseline` | P13 |
| 49 | `templates/skills/ui-design/SKILL.md` | Fourteen checkable rules for screens that do not look AI-made, with sources | delete | Not in the first version (open point B3). Rules derived from five external sources listed in the file, one pinned to a commit, with no licence recorded: provenance and licence must be checked before any recovery from `upstream-baseline` | §2.3, P13 |

## Coverage

Every path at the baseline (`git ls-tree -r upstream-baseline`) and the rows that cover it.

| Baseline path | Rows |
|---|---|
| `.claude-plugin/marketplace.json` | 1 |
| `.claude-plugin/plugin.json` | 2 |
| `CLAUDE.md` | 3 |
| `README.md` | 4–14 |
| `skills/dev-methodology/SKILL.md` | 15–23, 24a–24b, 25–28, 29a–29b, 30–33 |
| `skills/dev-methodology/references/troubleshooting.md` | 34a–34h |
| `skills/dev-methodology/templates/CLAUDE.md` | 35a–35s |
| `skills/dev-methodology/templates/bmad/bmad-architecture.toml` | 36 |
| `skills/dev-methodology/templates/constitution.md` | 37a–37g |
| `skills/dev-methodology/templates/githooks/commit-msg` | 38 |
| `skills/dev-methodology/templates/githooks/post-commit` | 39 |
| `skills/dev-methodology/templates/githooks/pre-commit` | 40a–40c |
| `skills/dev-methodology/templates/githooks/pre-push` | 41a–41f |
| `skills/dev-methodology/templates/gitignore` | 42 |
| `skills/dev-methodology/templates/glossary.md` | 43 |
| `skills/dev-methodology/templates/graphifyignore` | 44 |
| `skills/dev-methodology/templates/rust/purity.rs` | 45 |
| `skills/dev-methodology/templates/rust/rust-toolchain.toml` | 46 |
| `skills/dev-methodology/templates/scripts/review-diff.sh` | 47 |
| `skills/dev-methodology/templates/skills/dashboard-react/SKILL.md` | 48a–48b |
| `skills/dev-methodology/templates/skills/ui-design/SKILL.md` | 49 |

## Open points

- **B2 — Distribution and versioning.** Decided in phase 2, separately: how the methodology reaches projects and agents, and how its versions are numbered. The update command is defined during implementation, because §8.5 already fixes its behaviour. Rows 1, 2, 8, 9, 14.
- **B3 — Practice catalogue.** No additions to the catalogue (§2.3). Elements outside it are not in the first version and remain recoverable from `upstream-baseline`. Rows 22, 43, 48b, 49; Penpot also in rows 34h and 35s.
- **B4 — Bootstrap sequence.** The sequence that creates the initial commit and enables the protections is designed in the bootstrap and verified in phase 4, without bypass. Rows 29a, 29b, 40b, 41f.
- **B5 — Language.** English for distributed artifacts, the project language for content written for people, as a phase-2 convention. Rows 7, 37b.
- **Q12 — Licence (open).** The baseline has no licence file. Reuse of upstream text and of third-party material depends on it. Rows 21, 49.
