# Setup Quality Gates Plan

## Goal

Add a local-only `setup-quality-gates` skill that detects project stack profiles, proposes opinionated defaults, and configures local verification through native tooling, `mise`, and `prek`.

## Related Docs

- `docs/workflow.md`
- `GLOSSARY.md`

## Phase Checklist

- [ ] Phase 0: Research and grill stack defaults
- [ ] Phase 1: Support skill-owned reference docs
- [ ] Phase 2: Define stack profiles and docs
- [ ] Phase 3: Add detection and proposal workflow
- [ ] Phase 4: Implement local gate setup behavior

## Verification Strategy

- Targeted tests: validate any helper script with fixture repos or dry-run fixtures.
- Integration checks: run `npx skills add . --list` and confirm the skill is discovered.
- Type/lint/build: run `mise run check`.
- Manual checks: test prompts against Node, React/Vite, Svelte, Rust, and mixed-stack repo examples.
- Final review triggers: review for CI scope creep and unsafe dependency edits.

## Out of Scope

- CI setup, GitHub Actions, branch protection, release gates, and deploy checks.
- Enforcing one universal JS toolchain across all frameworks.
- Rewriting application code to satisfy newly added gates.
- Making every gate strict on the first pass when a repo needs staged adoption.

## Research Notes

- Matt Pocock's `setup-pre-commit` is useful as a shape, but too Husky/Prettier-specific. Decision: use stack profiles and `mise`/`prek` instead.
- Factory's official plugin marketplace includes focused skills with supporting docs. Decision: keep per-stack knowledge in skill references such as `node.md`, `node-react-vite.md`, `node-svelte.md`, and `rust.md`.
- User preference: allow adding tools and dependencies after presenting the detected repo profile and proposed gate set.
- This Plan is not implementation-ready until Phase 0 completes. Tool defaults need a fresh research pass against current first-party docs and real repo examples.
- The current generated-reference script treats extra `references/*.md` files as orphan drift. Decision: before adding stack profile docs, update the tooling so generated shared references remain drift-checked while skill-owned reference docs are allowed.

## Phase 0: Research And Grill Stack Defaults

Mode: HITL

### Goal

Research current tool choices and grill the opinionated defaults before encoding them into a setup skill.

### Context

Quality gate tooling changes quickly, especially in JS frameworks. The current plan direction is intentional, but the exact defaults should be validated before implementation.

### Tasks

- [ ] Research current first-party docs for `mise`, `prek`, `oxlint`, ESLint, Prettier, TypeScript, Vite/React, Svelte, Rust `cargo fmt`, and Rust `clippy`.
- [ ] Check real-world repo examples where first-party docs leave ambiguity.
- [ ] Compare `oxlint` suitability for Node React/Vite against ESLint/Prettier fallback cases.
- [ ] Research Svelte's current lint/format/typecheck expectations and document why it needs a separate profile.
- [ ] Research Rust `clippy` pedantic/nursery adoption patterns and practical allow/deny defaults.
- [ ] Grill the user on profile defaults, strictness ramps, generated files, and acceptable dependency additions.
- [ ] Update this Plan with confirmed defaults before Phase 1 starts.

### Acceptance Criteria

- [ ] Each stack profile has sourced tool choices and version/current-behavior notes.
- [ ] Unstable assumptions are called out explicitly.
- [ ] The user has approved the profile defaults and strictness posture.
- [ ] Later phase tasks reflect researched defaults rather than placeholders.
- [ ] Plan checklist is updated with completed work and newly discovered tasks.

## Phase 1: Support Skill-Owned Reference Docs

Mode: AFK

### Goal

Update generated-reference tooling so skills can keep their own `references/*.md` docs alongside generated shared references.

### Context

`setup-quality-gates` needs stack-specific reference docs such as `node.md`, `node-react-vite.md`, `node-svelte.md`, and `rust.md`. The current `scripts/sync-skill-references.sh --check` flags any non-shared `.md` in `references/` as orphan drift, which blocks that structure.

### Tasks

- [ ] Update `scripts/sync-skill-references.sh` to distinguish generated shared references from skill-owned references.
- [ ] Preserve drift checks for generated shared files copied from `skills/_shared/`.
- [ ] Allow extra skill-owned `references/*.md` files that do not collide with shared reference filenames.
- [ ] Decide whether the convention needs an explicit marker, manifest, filename rule, or just "shared filename collision means generated; other files are owned".
- [ ] Add or update docs/comments so future skills understand the generated-vs-owned reference convention.
- [ ] Verify the script still detects missing or stale generated shared references.
- [ ] Verify the script allows a skill-owned reference file.

### Acceptance Criteria

- [ ] `scripts/sync-skill-references.sh --check` passes when a skill has extra owned reference docs.
- [ ] Missing generated shared references are still reported.
- [ ] Stale generated shared references are still reported.
- [ ] Shared filename collisions remain protected from accidental skill ownership.
- [ ] `mise run check` passes.
- [ ] Implementation self-review completed.
- [ ] Plan checklist is updated with completed work and newly discovered tasks.

### Notes

This phase is a prerequisite for stack-profile docs. Do not add the `setup-quality-gates` skill references until this is solved.

## Phase 2: Define Stack Profiles And Docs

Mode: HITL

### Goal

Define the durable opinionated defaults before writing the skill.

### Context

This skill is intentionally personal/opinionated. It should be clear about stack-specific defaults and escape hatches.

### Tasks

- [ ] Create a Target-State Doc under `docs/` for local quality gate behavior if the decisions exceed the skill body.
- [ ] Create `skills/setup-quality-gates/references/common.md`.
- [ ] Create `skills/setup-quality-gates/references/node.md`.
- [ ] Create `skills/setup-quality-gates/references/node-react-vite.md`.
- [ ] Create `skills/setup-quality-gates/references/node-svelte.md`.
- [ ] Create `skills/setup-quality-gates/references/rust.md`.
- [ ] Decide default `mise` task names: `format`, `format:ci`, `lint`, `typecheck`, `test`, and `check`.
- [ ] Decide default `prek` hook composition for fast local gates.
- [ ] Decide strictness ramps for existing repos with many current violations.
- [ ] Confirm skill-owned references are allowed by the sync script before adding profile docs.

### Acceptance Criteria

- [ ] Stack profile docs describe what gets added and when.
- [ ] Node React/Vite defaults prefer `oxlint` where appropriate.
- [ ] Svelte defaults explicitly use ESLint/Prettier where current tooling requires it.
- [ ] Rust defaults cover `cargo fmt`, `cargo clippy`, and the pedantic/nursery policy.
- [ ] CI remains explicitly out of scope.
- [ ] Implementation self-review completed.
- [ ] Plan checklist is updated with completed work and newly discovered tasks.

### Notes

Avoid pretending this is universally correct. The value is your repeatable baseline.

## Phase 3: Add Detection And Proposal Workflow

Mode: AFK

### Goal

Create the skill and any helper script needed to detect repo profiles and present a proposed gate set before editing.

### Context

The skill should not silently add tools. It may add tools after it explains the detected stack and proposed changes.

### Tasks

- [ ] Create `skills/setup-quality-gates/SKILL.md`.
- [ ] Add `skills/setup-quality-gates/agents/openai.yaml`.
- [ ] Add or design a helper script that detects package manager, framework, Rust crates, existing `mise.toml`, existing `prek`/pre-commit config, and current scripts.
- [ ] Define the proposal output: detected profile, proposed tools, proposed `mise` tasks, proposed `prek` hooks, verification commands, and risks.
- [ ] Require user approval before dependency installation or broad config rewrites.
- [ ] Define behavior for ambiguous or mixed-stack repos.

### Acceptance Criteria

- [ ] The skill name is exactly `setup-quality-gates`.
- [ ] The skill detects common repo profiles before proposing edits.
- [ ] The skill asks before adding dependencies or replacing existing gate configuration.
- [ ] The proposal distinguishes native tooling from the `mise` adapter layer.
- [ ] Local skill discovery finds the new skill.
- [ ] Implementation self-review completed.
- [ ] Plan checklist is updated with completed work and newly discovered tasks.

### Notes

Scripts should help inspect and avoid mistakes; the skill still owns judgment.

## Phase 4: Implement Local Gate Setup Behavior

Mode: Mixed

### Goal

Make the skill able to configure local verification for supported stacks.

### Context

`mise` is the cross-repo control plane. Native package scripts and tool configs remain native.

### Tasks

- [ ] Implement Node base behavior: package manager detection, package scripts, formatter/linter/typecheck/test mapping.
- [ ] Implement React/Vite behavior with `oxlint`-first defaults.
- [ ] Implement Svelte behavior with ESLint/Prettier defaults.
- [ ] Implement Rust behavior with `cargo fmt`, `cargo clippy`, optional pedantic/nursery policy, and test mapping.
- [ ] Add or update `mise.toml` with stable task names.
- [ ] Add or update `prek` configuration for fast local gates.
- [ ] Add ignored cache/generated paths only when the configured tools require them.
- [ ] Run the configured local gates and report remaining violations separately from setup success.

### Acceptance Criteria

- [ ] Supported repos get stable `mise` task names without deleting useful native scripts.
- [ ] `prek` runs a fast local subset and does not become a slow CI replacement.
- [ ] Existing repo conventions are preserved unless the proposal explicitly calls out a replacement.
- [ ] The skill reports when setup succeeded but the existing codebase still has violations.
- [ ] `mise run check` passes for this repo after adding the skill.
- [ ] Implementation self-review completed.
- [ ] Plan checklist is updated with completed work and newly discovered tasks.

## Discovered Follow-Ups

- Add a separate `setup-ci` skill later, reusing stack profiles where possible.
- Add more stack profiles only after real dogfooding creates concrete defaults.
