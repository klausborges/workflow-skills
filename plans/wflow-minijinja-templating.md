# Wflow MiniJinja Templating Plan

## Goal

Plan provider-specific generated skill files only if the repo needs templating later.

## Related Docs

- `docs/workflow.md`
- `plans/invocation-policy-dogfooding.md`
- `plans/workflow-plugin-experiments.md`

## Phase Checklist

- [ ] Phase 0: Confirm templating need
- [ ] Phase 1: Design template inputs and generated outputs
- [ ] Phase 2: Implement templating if still justified

## Verification Strategy

- Targeted tests: fixture tests for template rendering, generated file drift, missing variables, and provider-specific outputs.
- Integration checks: run `npx skills add . --list` after generated output changes.
- Type/lint/build: run `mise run check`.
- Manual checks: inspect generated skill folders for provider-specific files only when a provider requires them.

## Out of Scope

- Token benchmarking.
- Metadata source-span diagnostics.
- Replacing `skill.toml` as the reference artifact contract.
- Generating provider-specific files before a provider need is proven.

## Research Notes

- MiniJinja was intentionally deferred from the reference artifact work because the repo did not yet need provider-specific generated files.
- This Plan should start only after invocation-policy or plugin experiments show a real generation need.
- The first choice should remain static files plus `skill.toml` metadata when that is enough.

## Phase 0: Confirm Templating Need

Mode: HITL

### Goal

Decide whether templating solves a current provider packaging problem.

### Context

Templating adds another source/generation layer. It should be justified by repeated provider differences or generated files that are painful to maintain manually.

### Tasks

- [ ] Review `plans/invocation-policy-dogfooding.md` for provider-specific metadata needs.
- [ ] Review `plans/workflow-plugin-experiments.md` for generated plugin or hook files.
- [ ] Identify the exact files that would become templates.
- [ ] Confirm static files are insufficient.
- [ ] Grill the user on whether generation complexity is worth it.

### Acceptance Criteria

- [ ] A concrete provider-specific generation need is documented.
- [ ] Static-file alternatives were considered first.
- [ ] The user has approved adding template rendering.
- [ ] Plan checklist is updated with completed work and newly discovered tasks.

## Phase 1: Design Template Inputs And Generated Outputs

Mode: HITL

### Goal

Define the smallest template data model and generated file contract.

### Context

Template inputs should stay boring and declarative. Existing `skill.toml` reference metadata should not become a general programming language.

### Tasks

- [ ] Decide where template source files live.
- [ ] Decide whether template inputs live in `skill.toml` or a separate metadata file.
- [ ] Define generated file paths and ownership.
- [ ] Decide how `wflow` verifies generated files are current.
- [ ] Update this Plan with the chosen shape before implementation.

### Acceptance Criteria

- [ ] Template source, metadata, and generated output locations are clear.
- [ ] Generated files remain installable through the Skills CLI.
- [ ] Verification catches stale generated output.
- [ ] The design avoids duplicating ordinary reference sync behavior.

## Phase 2: Implement Templating If Still Justified

Mode: AFK

### Goal

Add MiniJinja rendering to `wflow` after the template contract is approved.

### Context

Implementation should remain a repo-maintenance feature. Installed skills should receive generated files, not require Rust or `wflow`.

### Tasks

- [ ] Add MiniJinja dependency.
- [ ] Add template rendering to the relevant `wflow` command.
- [ ] Add stale-output verification.
- [ ] Add fixture tests for rendering and verification failures.
- [ ] Update docs for contributors.

### Acceptance Criteria

- [ ] Generated files are deterministic.
- [ ] Stale generated files fail verification.
- [ ] Installed skills do not require MiniJinja, Rust, or `wflow`.
- [ ] `mise run check` passes.

## Plan Self-Review

Status: PASS

### Findings

- No blocking findings. This Plan keeps MiniJinja separate from token benchmarking and diagnostics.
- No blocking findings. The first phase requires proof that templating is needed before implementation.

### Plan Updates

- Tasks checked: none.
- Acceptance criteria checked: none.
- Tasks added/moved: captured MiniJinja follow-up from the completed reference artifact plan.

### Verification

- Local context: read the completed reference artifact work, `plans/invocation-policy-dogfooding.md`, and `plans/workflow-plugin-experiments.md`.
