# Known failures (symptom → cause → fix)

Lessons from earlier projects that no configuration file tells you. Look up the symptom.

## Rust builds and worktrees

| Symptom | Cause | Fix |
|---|---|---|
| `/tmp` full, RAM exhausted during a build | Cargo target on tmpfs | `CARGO_TARGET_DIR` on `/home`; delete the targets in `/tmp` |
| Errors E0061/E0063, doctest E0460 or "migration N unknown to the binary" unrelated to your diff | two worktrees of the same workspace with the same `CARGO_TARGET_DIR`: the crates have the same hash and one uses the other's artifacts | one target folder per active worktree (e.g. `~/.cache/cargo-target-side`); the main checkout never uses a worktree's folder; `cargo clean -p <crate>` and retry |
| `cargo metadata non eseguibile` (assertion message of `purity.rs`) after removing a worktree | test binary compiled in the worktree, with the `CARGO_MANIFEST_DIR` of the deleted folder | `touch crates/<domain>/tests/purity.rs` and run again |
| `/home` disk full of builds | Cargo never deletes old builds | the pre-push empties the folder above `TARGET_CAP_GB`; when a worktree is removed, delete its target folder; `df -h /home` before long builds |
| Commits landed on the wrong branch | a subagent writing in the same folder while the main session switches branch | `isolation: "worktree"` for every subagent that writes in parallel |

## Tests and BDD

| Symptom | Cause | Fix |
|---|---|---|
| BDD suite stuck with no output at 100% CPU, for minutes or hours | wiremock's `MockServer::drop` does `block_on(verify())` inside a tokio task whose cooperative budget is exhausted | release the mock servers on `spawn_blocking` (in the cucumber runner's `after` hook or in the World's `Drop`); the pre-push stops the tests after `PRE_PUSH_TEST_MINUTES` (25 by default), Ctrl-C stops the whole group |
| Green gate, bug found by the user on an external service | the stub diverged from the real service (data shape, `this` of the methods, branches never observed) | spike on the real service, stub only from what the spike observed, one scenario per branch, real smoke test before the PR |
| Wrong numbers (`3.0` instead of `3`) passed the BDD | the scenario compared values without their type | compare the exact JSON |

## User proofs and migrations

| Symptom | Cause | Fix |
|---|---|---|
| The user proves the story and sees the old behaviour | dev script launched from the main checkout instead of the story's worktree | `cd <worktree>` in the proof steps; the script prints branch and last migration |
| `migration N was previously applied but has been modified` | migration modified after the user had applied it | fixes always in a new migration; in an emergency: delta in a transaction and checksum updated in `_sqlx_migrations` |
| Invalid SQL in the PR's proof steps | SQL written without running it | run it on the dev database (transaction with `ROLLBACK`) before the PR |
| Diagnosis communicated and then disproved | cause inferred without verification | `superpowers:systematic-debugging`; only a cause verified on the code or the fixtures is communicated |

## Subagents

| Symptom | Cause | Fix |
|---|---|---|
| Subagent stuck for hours on "waiting for the result" | the completion notice of a background command does not wake a subagent; a command that reads stdin hangs | check the command (`pgrep`, logs) and wake it with SendMessage; in the prompt: foreground only, timeout ≤ 10 min, no stdin |
| The user's server or containers are gone | a subagent killed processes that were not its own (`pkill -f`, busy port) | in the prompt: never touch processes it did not start, stop only by the saved PID |

## `claude` in scripts

| Symptom | Cause | Fix |
|---|---|---|
| Dozens of sessions saved to disk | `claude --print` without the flag in hooks or scripts | `--no-session-persistence` (works only with `--print`); for interactive runs `--session-id <uuid>` and deletion of `~/.claude/projects/*/<uuid>*` in a trap |
| The hook's review created commits or called services | `claude` inherited `GIT_DIR` from the hook and had tools and MCP | `env -u GIT_DIR -u GIT_INDEX_FILE -u GIT_WORK_TREE`, `--tools "Read,Grep,Glob"`, `--strict-mcp-config --mcp-config '{"mcpServers":{}}'`, `--permission-mode default` (already in `review-diff.sh`) |
| Review prompt cut short or `syntax error` in `review-diff.sh` | an apostrophe in a value inside a single-quoted prompt | instructions in a quoted heredoc (`<<'EOF'`, already in the template) |

## Penpot (dashboard prototypes)

| Symptom | Cause | Fix |
|---|---|---|
| The export shows the texts and colours of the source shape | changes made in the same `execute_code` as the clone | clone in one call, modify in another |
| Children in the wrong order in a flex | `insertChild(i)` does not follow the visual order | reorder with `appendChild` in the wanted order |
| Overlapping boards | positions remembered instead of read | compute the bounding boxes of all the boards before creating or moving one |
| Penpot MCP with ECONNREFUSED | plugin not open in the file | ask the user to open Penpot and the plugin, then `/mcp` |
