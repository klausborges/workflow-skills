# Reference Artifact Reduction Plan

## Goal

Reduce generated `references/` bloat while keeping each skill self-contained after installation.

## Related Docs

- `docs/workflow.md`
- `GLOSSARY.md`
- `plans/setup-quality-gates.md`

## Phase Checklist

- [x] Phase 1: Add per-skill metadata
- [x] Phase 2: Build reference sync CLI
- [x] Phase 3: Replace shell sync workflow
- [x] Phase 4: Prune generated references
- [x] Phase 5: Document extension points
- [x] Phase 6: Harden reference tooling after review

## Verification Strategy

- Targeted tests: add CLI fixture tests for sync and verify behavior.
- Integration checks: run local skill discovery after pruning references.
- Type/lint/build: run Rust checks plus `mise run check`.
- Manual checks: inspect installed skill folders for missing or extra references.
- Final review triggers: review for accidental reliance on `SKILL.md` parsing as source of truth, stale generated files, and overbuilt generator scope.

## Out of Scope

- Replacing the Skills CLI.
- Adding token benchmarking in the first implementation.
- Adding MiniJinja templating in the first implementation.
- Using Nickel as the production metadata format in the first implementation.
- Generating provider-specific invocation metadata beyond preserving existing files.
- Moving `skills/_shared/` to a new source directory.
- Requiring remote Skills CLI users to have Rust or `wflow` installed.
- Fully automating network-dependent Skills CLI install checks in `mise run check`.

## Research Notes

- Current repo state: `scripts/sync-skill-references.sh` copies all five shared references into every installable skill, producing 50 generated reference files.
- Initial dependency inventory used direct `SKILL.md` links to seed 22 generated reference files.
- Matt Pocock's public skills repo does not appear to use `references/` heavily.
- Superpowers uses skill-specific references rather than a full copied shared set.
- Vercel's Skills docs treat `references/` as optional supporting resources linked from `SKILL.md`.
- Nickel is useful for programmable configuration and can export JSON/YAML/TOML, but it adds a niche language/runtime decision before this repo needs one.
- Rust with `clap` and `toml` gives real parsing, validation, and future room for MiniJinja, token benchmarking, and quality-gate automation.
- `miette` is a better fit than `anyhow` for this CLI because reference verification is config/parse-style diagnostics that may later benefit from source labels and spans. Decision: use `miette` plus `thiserror`, and do not add `anyhow` in v1.
- `tiktoken-rs` is a plausible later dependency for measuring prompt/token footprint across this repo, Matt's skills, and Superpowers.

## Dependency Inventory

| skill | declared references |
| --- | --- |
| `diagnose-issue` | `workflow-language`, `research-ladder` |
| `explain-codebase` | `workflow-language` |
| `explain-workflow` | `workflow-language` |
| `implement-plan` | `workflow-language`, `document-conventions`, `templates`, `research-ladder` |
| `improve-architecture` | `workflow-language`, `architecture-language` |
| `plan-prototype` | `workflow-language` |
| `plan-work` | `workflow-language`, `document-conventions`, `templates` |
| `review-work` | `workflow-language`, `document-conventions`, `research-ladder`, `architecture-language` |
| `use-workflow` | `workflow-language`, `research-ladder` |
| `write-handoff` | `workflow-language`, `templates` |

## Metadata Shape

Each installable skill gets a local metadata file:

```toml
[references]
shared = [
  "workflow-language",
  "document-conventions",
  "templates",
]
owned = []
```

The metadata file is the source of truth for every `references/*.md` file that ships with a skill. `wflow` does not scan `SKILL.md` links.

`skill.toml` ships with the skill. It is part of the skill package contract, not a repo-only build artifact.

Generated skill artifacts remain committed. Users installing from the GitHub repo through the Skills CLI should not need Rust or `wflow`; those are repo maintenance tools for contributors and forks.

In the first version, declared references may be linked from `SKILL.md`, but link usage is intentionally not verified. Extra shipped reference files are treated as artifact pollution.

Empty `shared` or `owned` lists are allowed, but both keys must be present.

If both lists are empty, `sync` should not create a `references/` directory. `verify` should fail if undeclared `references/*.md` files exist.

`wflow refs sync` may prune generated shared references because they are reproducible from `skills/_shared/`. It must not delete skill-owned references. `wflow refs verify` is the safety net that fails when metadata and `references/` disagree.

`wflow refs sync` must validate the same metadata/file contract before writing. It should not generate a known-invalid skill package.

Generated/shared ownership is determined by name collision with `skills/_shared/`. If `references/templates.md` exists and `skills/_shared/templates.md` exists, it is a generated shared reference candidate. If `templates` is undeclared, `sync` may delete it. If a reference name does not exist in `skills/_shared/`, it is skill-owned; `sync` must not delete it.

## Verify Contract

The CLI `verify` command must fail when:

- an installable skill lacks metadata
- metadata contains unknown top-level keys
- metadata contains unknown `[references]` keys
- metadata omits required `[references].shared` or `[references].owned` lists
- metadata contains duplicate reference names
- metadata reference lists are not sorted
- metadata contains empty names, path-like names, names ending in `.md`, or names outside `[a-z0-9-]+`
- metadata declares a shared reference that does not exist under `skills/_shared/`
- a declared generated reference is missing from the skill's `references/`
- a declared generated reference differs from the shared source
- a generated shared reference exists in the skill's `references/` but is not declared
- a skill-owned reference exists in the skill's `references/` but is not declared
- a declared skill-owned reference is missing
- a stale temporary reference artifact exists under `references/`
- a skill-owned `references/*.md` collides with a shared reference name

## Phase 1: Add Per-Skill Metadata

Mode: AFK

### Goal

Add explicit per-skill metadata files that declare required shared and skill-owned references.

### Context

The current shell script treats every shared reference as required for every skill. Metadata should make the package contract explicit without relying on Markdown parsing as source of truth.

Installable skills are directories under `skills/*/` with `SKILL.md`, excluding `skills/_shared/`.

### Tasks

- [x] Use `skill.toml` as the metadata filename.
- [x] Add metadata to each installable skill using the dependency inventory.
- [x] Keep metadata intentionally small: shared reference names and owned reference names only.
- [x] Use bare reference names, not filenames with `.md`.
- [x] Represent shared and owned references separately.
- [x] Add a short note to `plans/setup-quality-gates.md` if its skill-owned reference requirement depends on this Plan.
- [x] Add this Plan to README future work.
- [x] Update `plans/setup-quality-gates.md` so it explicitly depends on this Plan for skill-owned references.

### Acceptance Criteria

- [x] Every installable skill has metadata.
- [x] `skill.toml` remains in each installable skill directory.
- [x] Metadata covers every intended shipped reference.
- [x] Metadata does not declare unused references.
- [x] Declared references do not need to be linked from `SKILL.md`.
- [x] Skill-owned references cannot collide with shared reference names.
- [x] The format leaves room for future provider metadata without requiring it now.
- [x] Plan checklist is updated with completed work and newly discovered tasks.
- [x] README future work links to this Plan.
- [x] `plans/setup-quality-gates.md` no longer points at the old shell script as the reference-doc solution.

### Notes

Decision: use `skill.toml`. It is visible when browsing or forking a skill folder and makes the metadata part of the skill's package contract.

Decision: use bare reference names. The CLI maps `workflow-language` to `skills/_shared/workflow-language.md` and rejects path-like values.

Decision: add `skill.toml` files before building the Rust CLI. The metadata is simple enough to add directly; CLI tests can validate it afterward.

Decision: keep per-skill `skill.toml` files comment-free by default. Explanations belong in docs and Plans, not repeated metadata comments.

### Phase Self-Review

Status: PASS

Reviewer: local implementation pass plus fresh `gpt-5.5` high-reasoning subagent review.

Findings:

- Subagent found one checklist-update issue; fixed by marking Phase 1 tasks and acceptance criteria complete.
- Residual generated-reference bloat remains intentionally deferred to later phases. Phase 1 metadata was seeded from current `SKILL.md` links, not every generated `references/*.md` file.

Evidence:

- Verified every installable skill has `skill.toml`.
- Verified each declared reference represented an intended shipped reference.
- Ran `mise run format`.
- Ran `mise run check`.

## Phase 2: Build Reference Sync CLI

Mode: AFK

### Goal

Build a small Rust CLI that can sync and verify generated shared references.

### Context

TOML needs real parsing. A Rust CLI avoids fragile shell parsing and creates a path for future templating, token measurements, and `setup-quality-gates` automation.

### Tasks

- [x] Add a minimal Rust workspace with the binary under `crates/wflow/`.
- [x] Commit `Cargo.lock` for reproducible tooling checks.
- [x] Add `clap` for subcommands.
- [x] Add a global `--root <path>` option that defaults to the current directory.
- [x] Add TOML parsing with Serde.
- [x] Add `miette` and `thiserror` for CLI diagnostics.
- [x] Validate metadata strictly, including unknown keys and reference name syntax.
- [x] Name the binary `wflow`.
- [x] Implement `wflow refs sync`.
- [x] Implement `wflow refs verify`.
- [x] Make `wflow refs sync` fail before writing when metadata or reference ownership are invalid.
- [x] Add fixture tests for valid metadata, missing metadata, missing shared source, stale generated file, extra generated file, undeclared reference files, and owned-reference cases.
- [x] Add Rust quality tasks for `cargo fmt --check`, `cargo test`, and `cargo clippy` with `pedantic` and `nursery`.

### Acceptance Criteria

- [x] `wflow refs sync` copies only declared shared references into each skill.
- [x] `wflow refs sync` avoids rewriting generated references whose content is already current.
- [x] `wflow refs sync` validates the same contract as `verify` before writing.
- [x] `wflow refs sync` reports all preflight errors before exiting without writes.
- [x] `wflow refs sync` may use normal copy/delete after preflight; atomic writes are not required.
- [x] `wflow refs sync` creates missing `references/` directories when needed.
- [x] `wflow refs sync` prunes undeclared generated shared references.
- [x] `wflow refs sync` never deletes skill-owned references.
- [x] Shared-name collisions are treated as generated shared references, not skill-owned references.
- [x] `wflow refs verify` enforces the verify contract.
- [x] `wflow refs verify` reports missing directories/files but does not create them.
- [x] Error messages identify the skill and file path involved.
- [x] Errors are human-readable only; no machine-readable output is required in v1.
- [x] Error plumbing uses `miette` and `thiserror`, not `anyhow`.
- [x] Verification reports all discovered errors before exiting nonzero.
- [x] Fixture tests can run against temporary roots through `--root`.
- [x] Failure cases are tested with temporary fixtures, not by mutating the real repo.
- [x] Real repo validation runs through `mise run check`.
- [x] Invalid metadata shape fails before sync or reference comparison.
- [x] Tests cover both success and failure cases.
- [x] `cargo fmt --check` passes.
- [x] `cargo test` passes.
- [x] `cargo clippy --all-targets --all-features -- -D warnings -W clippy::pedantic -W clippy::nursery` passes, with explicit allows only when justified.
- [x] Plan checklist is updated with completed work and newly discovered tasks.

### Notes

Avoid template rendering in this phase. Keep the CLI boring.

Decision: use a structured Rust layout with `crates/wflow/`, not a root `src/` package.

### Phase Self-Review

Status: PASS

Reviewer: local implementation pass plus fresh `gpt-5.5` high-reasoning subagent review.

Findings:

- Subagent found the Phase 2 checklist still unchecked; fixed by marking completed tasks and acceptance criteria.
- Subagent found the first fixture set was thinner than the strict contract; fixed by expanding to 21 tests covering strict metadata shape, sorting, duplicates, invalid names, missing generated files, undeclared and missing owned references, accumulated errors, unchanged-file no-rewrite behavior, and sync preflight no-write behavior.
- Real repo `wflow refs verify` currently fails on undeclared generated shared references. This is expected until Phase 4 prunes generated artifacts.

Evidence:

- Ran `cargo fmt --check`.
- Ran `env -u RUSTC_WRAPPER cargo test`: 21 tests passed.
- Ran `env -u RUSTC_WRAPPER cargo clippy --all-targets --all-features -- -D warnings -W clippy::pedantic -W clippy::nursery`.
- Ran `env -u RUSTC_WRAPPER mise run check`.
- Ran `env -u RUSTC_WRAPPER cargo run -- refs verify` and confirmed failures are the expected Phase 4 extra generated shared references.

## Phase 3: Replace Shell Sync Workflow

Mode: AFK

### Goal

Replace `scripts/sync-skill-references.sh` in repo tasks and pre-commit checks.

### Context

The current task names can stay stable so developer muscle memory and existing docs keep working.

### Tasks

- [x] Update `mise.toml` so `sync-references` runs `wflow refs sync`.
- [x] Update `mise.toml` so `sync-references:ci` runs `wflow refs verify`.
- [x] Update `.pre-commit-config.yaml` if it directly calls the shell script.
- [x] Delete `scripts/sync-skill-references.sh` after `wflow refs` replaces it.
- [x] Update README validation docs if commands or requirements change.

### Acceptance Criteria

- [x] `mise run sync-references` uses `wflow refs sync`.
- [x] `mise run sync-references:ci` uses `wflow refs verify`.
- [x] `mise run check` passes.
- [x] Pre-commit can block reference drift through the CLI.
- [x] Remote install instructions do not imply Rust or `wflow` are required for ordinary users.
- [x] Plan checklist is updated with completed work and newly discovered tasks.

### Notes

Decision: delete the shell script. `mise run sync-references` remains the stable interface.

### Phase Self-Review

Status: PASS

Reviewer: local implementation pass plus fresh `gpt-5.5` high-reasoning subagent review.

Findings:

- Subagent found `mise` tasks initially reached the CLI through `cargo run` instead of a visible `wflow refs ...` command. Follow-up planning reversed the temporary root `./wflow` launcher idea: repo tasks should run the `wflow` binary through Cargo, keeping the repository root free of ad hoc launcher scripts.
- Subagent found Phase 3 checklist items still unchecked; fixed here.

Evidence:

- `.pre-commit-config.yaml` runs `mise run sync-references:ci`.
- `scripts/sync-skill-references.sh` is deleted.
- `mise run sync-references` passes and runs the `wflow` binary.
- `mise run check` passes.

## Phase 4: Prune Generated References

Mode: AFK

### Goal

Remove generated shared references that each skill does not declare.

### Context

This is the visible artifact cleanup. It should happen only after `verify` can prove the generated set is correct.

### Tasks

- [x] Run `wflow refs sync`.
- [x] Record before/after generated shared reference file counts.
- [x] Confirm each skill contains only declared generated shared references.
- [x] Confirm skill-owned references are allowed only when declared and when they do not collide with shared names.
- [x] Confirm `wflow refs sync` does not delete undeclared skill-owned references; `wflow refs verify` should fail instead.
- [x] Run local skill discovery.
- [x] Run repository checks.

### Acceptance Criteria

- [x] Generated shared reference count drops from 50 to the declared set.
- [x] Installed skills remain self-contained for every linked reference.
- [x] No skill contains undeclared generated shared references.
- [x] No skill contains undeclared skill-owned references.
- [x] `npx skills add . --list` discovers the expected skills.
- [x] `mise run check` passes.
- [x] Implementation self-review completed.
- [x] Plan checklist is updated with completed work and newly discovered tasks.

### Phase Self-Review

Status: PASS

Reviewer: local implementation pass plus fresh `gpt-5.5` high-reasoning subagent review.

Findings:

- Subagent found no linked generated reference accidentally removed.
- Subagent found the Phase 4 checklist still unchecked; fixed here.

Evidence:

- Ran `wflow refs sync` through the repo task.
- Generated shared reference files dropped from 50 to 22.
- Ran `wflow refs verify` through the repo task.
- Ran `mise run check`: 21 Rust tests passed plus Markdown formatting and reference verification.
- Ran `npx skills add . --list`: 10 skills discovered.

## Phase 5: Document Extension Points

Mode: Mixed

### Goal

Document the metadata and CLI contract without overcommitting to future generator features.

### Context

The repo may later use the same metadata path for provider invocation differences, templating, and token measurements. Those are not part of this implementation.

### Tasks

- [x] Document `skill.toml` fields that exist now.
- [x] Document that `references` names map to `skills/_shared/<name>.md`.
- [x] Document that `skill.toml` is the only reference artifact contract.
- [x] Add discovered follow-ups for provider metadata, MiniJinja, Nickel, and token benchmarking.
- [x] Run final review.

### Acceptance Criteria

- [x] Documentation explains how to add a shared reference to a skill.
- [x] Documentation explains how to add a skill-owned reference without colliding with shared references.
- [x] Documentation explains that unlinked declared references are allowed, but extra shipped reference files are not.
- [x] Future extension ideas are listed as follow-ups, not implemented early.
- [x] Final review finds no blocking issues.
- [x] Plan checklist is updated with completed work and newly discovered tasks.

### Phase Self-Review

Status: PASS

Reviewer: local implementation pass plus fresh `gpt-5.5` high-reasoning subagent final review.

Findings:

- Final review found no blocking issues.
- Final review found Phase 5 checklist bookkeeping still pending; fixed here.

Evidence:

- `docs/workflow.md` documents current `skill.toml` fields, shared mapping, shared-reference workflow, owned-reference workflow, and metadata-only reference verification.
- `README.md` keeps Rust/Cargo scoped to local validation and does not make it part of ordinary remote install.
- `mise.toml` and `.pre-commit-config.yaml` are coherent.
- Ran `mise run format`.
- Ran `mise run check`.

## Phase 6: Harden Reference Tooling After Review

Mode: AFK

### Goal

Fix review findings that could break landing or let `wflow refs sync` write outside the repo, then simplify the reference contract so `skill.toml` is the only source of truth.

### Context

A balanced review and an adversarial review found two implementation defects and a few workflow risks after the initial artifact reduction work:

- The repo should not add a root-level `./wflow` launcher. `mise` and pre-commit should run the `wflow` binary through Cargo from the workspace.
- `SKILL.md` link scanning created too many Markdown path edge cases. Decision: remove it completely rather than keep patching parser behavior.
- `sync` writes and prunes paths under `references/` without rejecting symlinked directories or symlinked files.
- Source package files such as `skills/_shared/*.md`, `SKILL.md`, and `skill.toml` should not be symlinks.
- Generated `references/*.md` files should not be hardlinks.
- Sync should re-check references directories and targets immediately before mutation, then replace generated files with a temp-file rename instead of direct writes.
- Stale temp files from interrupted sync writes should not be silently ignored by verify.
- Missing `references/` directory creation should go through the same mutation-time safety path.
- Completed work should not stay listed as active README future work.
- `npx skills add . --list` proves discovery but not a full installed-folder inspection.
- The pre-commit hook may keep reaching Cargo through `mise`, or it may call Cargo directly; the choice should be intentional and avoid a root launcher.
- Concurrent adversarial filesystem swaps between validation and mutation are out of scope for this repo-local maintenance tool.

### Tasks

- [x] Add a symlink-safety layer for reference directories and reference files before sync writes or deletes.
- [x] Reject symlinked `references/` directories.
- [x] Reject symlinked generated shared reference files and skill-owned reference files.
- [x] Reject symlinked shared source docs, `SKILL.md`, and `skill.toml`.
- [x] Reject hardlinked generated shared reference files and skill-owned reference files.
- [x] Ensure sync write/delete targets stay under the repo root before mutation.
- [x] Remove `SKILL.md` link scanning and metadata-vs-link validation.
- [x] Re-check references directories and generated reference targets immediately before sync writes and deletes.
- [x] Create missing `references/` directories through a checked sync helper rather than raw `create_dir_all`.
- [x] Replace direct generated-reference writes with temp-file-plus-rename writes.
- [x] Reject stale temporary reference artifacts left by interrupted sync writes.
- [x] Remove this completed Plan from README future work.
- [x] Add fixture tests proving `SKILL.md` reference links are ignored by `wflow`.
- [x] Add fixture tests proving unknown top-level metadata keys are rejected.
- [x] Add fixture tests proving symlinked reference dirs/files are rejected and external targets are not modified.
- [x] Add fixture tests proving symlinked source package files are rejected.
- [x] Add fixture tests proving hardlinked reference files are rejected and external targets are not modified.
- [x] Add an offline self-containment check through `wflow refs verify` that proves declared reference files exist and generated files match shared sources.
- [x] Remove the root `wflow` launcher script if present.
- [x] Wire `mise.toml` to run `env -u RUSTC_WRAPPER cargo run --quiet --bin wflow -- refs sync` and `refs verify`.
- [x] Decide whether pre-commit should keep calling `mise run sync-references:ci` or call `env -u RUSTC_WRAPPER cargo run --quiet --bin wflow -- refs verify` directly.
- [x] Stage or otherwise explicitly verify the full landing set includes `Cargo.toml`, `Cargo.lock`, `crates/wflow/**`, and all `skill.toml` files.
- [x] Update docs only if symlink policy or hook dependency behavior becomes durable user-facing guidance.

### Acceptance Criteria

- [x] `wflow refs sync` refuses to write through symlinked `references/` directories.
- [x] `wflow refs sync` refuses to write through symlinked reference files.
- [x] `wflow refs sync` refuses to copy from symlinked shared reference sources.
- [x] `wflow refs sync` refuses to mutate hardlinked reference files.
- [x] `wflow refs sync` refuses to delete paths that resolve outside the repo root.
- [x] Concurrent adversarial filesystem swaps between validation and mutation are documented as out of scope.
- [x] Symlink rejection errors identify the skill and path involved.
- [x] `wflow refs verify` does not parse or validate `SKILL.md` links.
- [x] Metadata declarations are allowed even when not linked from `SKILL.md`.
- [x] `wflow refs verify` rejects stale temporary reference artifacts.
- [x] Sync writes generated references by temp-file rename after mutation-time safety checks.
- [x] Missing `references/` directory creation validates the skill directory and created references directory before writes.
- [x] README future work no longer lists this completed Plan.
- [x] Tests cover metadata-only behavior, unknown metadata keys, ignored `SKILL.md` links, stale temp artifacts, symlinked directories, symlinked files, hardlinked files, and no external target mutation.
- [x] `cargo run --quiet --bin wflow -- refs verify` passes on the real repo.
- [x] `mise run check` passes.
- [x] `npx skills add . --list` still discovers 10 skills, or the check is explicitly skipped due to network with the earlier successful run noted.
- [x] No root-level `wflow` launcher remains.
- [x] The required Rust workspace files are tracked in the intended landing set.
- [x] Pre-commit behavior is intentional and documented in this phase's notes.
- [x] Implementation self-review completed.
- [x] Plan checklist is updated with completed work and newly discovered tasks.

### Notes

Recommendation: remove the root `wflow` launcher. Use Cargo as the repo-local binary runner. `mise run check` remains the human-facing aggregate check, and pre-commit can either keep using `mise run sync-references:ci` or call Cargo directly if reducing hook dependencies matters more.

Recommendation: keep full `npx skills add` install inspection out of `mise run check` for now because it is network-sensitive and can mutate agent install locations. Use repo-local self-containment checks as the deterministic gate.

Decision: keep pre-commit using `mise run sync-references:ci` for now because this repo already standardizes local workflow tasks through `mise`.

Decision: do not implement `openat`/`renameat`/`unlinkat` style directory-handle mutation in v1. The CLI rejects unsafe paths present during validation and mutation, but it does not defend against concurrent malicious mutation of the working tree.

### Phase Self-Review

Status: PASS

Reviewer: local implementation pass plus fresh `gpt-5.5` high-reasoning subagent review.

Findings:

- Subagent found symlinked installable skill directories could still allow `sync` to delete outside the repo. Fixed by rejecting symlinked skill directories and checking prune targets against the canonical repo root.
- Subagent found malformed `.mdx`, `.md.bak`, and `.md/extra` links could pass as `.md` links. That parser was later removed entirely.
- Subagent found Phase 6 checklist bookkeeping still pending; fixed here.
- Follow-up review found symlinked source package files were still trusted. Fixed by rejecting symlinked `skills/_shared/*.md`, `SKILL.md`, and `skill.toml`.
- Follow-up review found hardlinked generated references could mutate another path. Fixed by rejecting hardlinked reference files.
- Second review found sync safety checks were preflight-only. Fixed by re-checking references directories and generated targets immediately before writes/deletes, and by writing generated files through temp-file rename.
- Phase self-review found missing `references/` directory creation still happened before the mutation-time safety helper. Fixed by moving creation behind a checked helper.
- Second review found README still listed this completed Plan as active future work. Fixed by removing the entry.
- Final simplification removed `SKILL.md` link scanning entirely. Metadata and file presence/content are now the only reference contract.
- Final patch review found temp reference artifacts could survive verify. Fixed by rejecting stale `.tmp.` artifacts under `references/`.
- Final patch review found unknown top-level metadata keys lacked direct fixture coverage. Added explicit coverage.
- Final patch review found concurrent adversarial filesystem swaps need a clear boundary. Documented them as out of scope for this repo-local maintenance CLI.

Evidence:

- Added metadata-only fixture tests proving `SKILL.md` links are ignored and unlinked metadata declarations are allowed.
- Added fixture tests for stale temporary reference artifacts and unknown top-level metadata keys.
- Added symlinked `references/` directory, symlinked reference file, and symlinked skill directory tests that assert external targets are not mutated.
- Added symlinked shared source, symlinked `SKILL.md`, symlinked `skill.toml`, and hardlinked reference file tests.
- Kept the missing-`references/` directory sync test covering checked directory creation.
- Ran `env -u RUSTC_WRAPPER cargo test`: 31 tests passed.
- Ran `env -u RUSTC_WRAPPER cargo clippy --all-targets --all-features -- -D warnings -W clippy::pedantic -W clippy::nursery`.
- Ran `env -u RUSTC_WRAPPER cargo run --quiet --bin wflow -- refs verify`.
- Ran `mise run check`.
- Ran `git diff --check`.
- Ran `npx skills add . --list`: 10 skills discovered.

## Phase Self-Review

Status: PASS

### Findings

- No blocking findings. The Plan keeps artifact reduction separate from future provider metadata, templating, and token benchmarking.
- No blocking findings. Metadata is source of truth; `SKILL.md` link scanning is intentionally absent.
- No blocking findings. The metadata accounts for every shipped reference file, including skill-owned docs.
- No blocking findings. Rust is justified for real TOML parsing and future growth, but the first CLI scope stays narrow.

### Plan Updates

- Tasks checked: none.
- Acceptance criteria checked: none.
- Tasks added/moved: added decisions and tasks for `skill.toml`, bare reference names, `wflow refs`, strict metadata validation, sorted lists, required `shared`/`owned` keys, Rust workspace layout, Rust quality gates, `miette` diagnostics, and `setup-quality-gates` dependency coordination.

### Verification

- local inventory: found 50 generated shared references today and 22 direct linked-reference needs.
- external research: checked Matt Pocock skills, Superpowers, Vercel Skills docs, Nickel, Rust TOML/clap, and tiktoken-rs.

## Discovered Follow-Ups

- After this Plan completes, plan `tiktoken-rs` integration and benchmark token footprint against Matt Pocock's skills and Superpowers.
- After this Plan completes, plan richer `miette` source-span diagnostics for `skill.toml` validation errors.
- Consider MiniJinja only when provider-specific generated files need templating.
- Revisit Nickel as an experiment only if TOML metadata becomes too repetitive or needs computed configuration.
- Use the metadata path later for provider invocation policy if `plans/invocation-policy-dogfooding.md` proves it is needed.
- Let `plans/setup-quality-gates.md` reuse `wflow` later for detection, proposal, apply, and verification commands.
- Revisit whether `skills/_shared/` should move to a root source directory after `wflow refs` owns the generation path.
