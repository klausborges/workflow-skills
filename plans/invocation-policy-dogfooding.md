# Invocation Policy Dogfooding Plan

## Goal

Decide whether `use-workflow` should remain implicit by default and how provider-specific invocation controls should be documented.

## Related Docs

- `docs/workflow.md`
- `GLOSSARY.md`

## Phase Checklist

- [ ] Phase 1: Observe implicit routing behavior
- [ ] Phase 2: Decide provider policy
- [ ] Phase 3: Update skills and install docs

## Verification Strategy

- Targeted tests: run generated-reference drift checks after skill/doc changes.
- Integration checks: run `npx skills add . --list` after skill metadata changes.
- Type/lint/build: run `mise run check`.
- Manual checks: compare behavior across Codex, Claude Code, and other harnesses available during dogfooding.
- Final review triggers: review for provider-specific claims that are not backed by current docs or live behavior.

## Out of Scope

- Building a custom installer before Skills CLI plus docs are proven insufficient.
- Shipping plugin packages; that belongs in `plans/workflow-plugin-experiments.md`.
- Disabling implicit invocation based on one noisy session.

## Research Notes

- Starting decision: `use-workflow` may be implicit so users can ask natural prompts.
- Current risk: different providers expose different controls for model invocation, skill permissions, and install-time metadata.
- Related Plan: `plans/workflow-plugin-experiments.md` owns provider packaging and hook/plugin experiments.

## Phase 1: Observe Implicit Routing Behavior

Mode: HITL

### Goal

Gather enough usage evidence to decide whether implicit `use-workflow` helps or gets in the way.

### Context

The desired behavior is that ordinary workflow prompts work without forcing the user to name a skill, while small direct tasks should not be interrupted.

### Tasks

- [ ] Dogfood the installed skills in at least three real work sessions.
- [ ] Record cases where `use-workflow` invoked correctly.
- [ ] Record cases where it invoked too often, too rarely, or routed to the wrong skill.
- [ ] Identify whether the issue is skill description wording, provider policy, or user prompt ambiguity.

### Acceptance Criteria

- [ ] At least three sessions are summarized in this Plan.
- [ ] Each surprising invocation has a likely cause.
- [ ] The Plan recommends keeping implicit invocation, narrowing descriptions, or disabling model invocation.
- [ ] Plan checklist is updated with completed work and newly discovered tasks.

### Notes

Prefer concrete prompts and outcomes over vibes.

## Phase 2: Decide Provider Policy

Mode: HITL

### Goal

Choose the smallest provider policy that matches observed behavior.

### Context

Some providers may support skill-level implicit-invocation controls through skill metadata. Others may require repo-local config, external config, or plain documentation.

### Tasks

- [ ] Research current first-party docs for provider-specific invocation controls before making claims.
- [ ] Decide whether `use-workflow` remains implicit by default.
- [ ] If implicit invocation is too noisy, decide whether to disable it for all providers that support disabling.
- [ ] Decide how to handle providers that cannot express the chosen policy in installed skill files.
- [ ] Decide whether custom installer work should move to `plans/workflow-plugin-experiments.md`.

### Acceptance Criteria

- [ ] The chosen policy is backed by observed dogfooding behavior.
- [ ] Provider-specific claims cite current docs or live behavior.
- [ ] The policy distinguishes skill metadata, repo-local config, and external harness config.
- [ ] The workflow-plugin plan owns plugin/package experiments instead of duplicating them here.
- [ ] Plan checklist is updated with completed work and newly discovered tasks.

## Phase 3: Update Skills And Install Docs

Mode: Mixed

### Goal

Apply the chosen invocation policy to skill metadata and user-facing install guidance.

### Context

The README already explains basic post-install usage. This phase should only add provider-specific policy guidance if evidence shows it is needed.

### Tasks

- [ ] Update `skills/use-workflow/SKILL.md` if the description or invocation policy changes.
- [ ] Update `skills/use-workflow/agents/openai.yaml` if Codex policy changes.
- [ ] Update README install or after-install guidance if users need provider-specific controls.
- [ ] Update `docs/workflow.md` if the policy becomes durable workflow behavior.
- [ ] Run repository checks and local skill discovery.

### Acceptance Criteria

- [ ] The chosen invocation behavior is documented with provider-specific exceptions.
- [ ] The repo does not imply the Skills CLI normalizes runtime policy across all providers.
- [ ] OpenCode or other plugin-capable harnesses are linked to `plans/workflow-plugin-experiments.md` when package behavior matters.
- [ ] `npx skills add . --list` discovers the expected skills.
- [ ] `mise run check` passes.
- [ ] Implementation self-review completed.
- [ ] Plan checklist is updated with completed work and newly discovered tasks.

## Phase Self-Review

Status: PASS

### Findings

- No blocking findings. The Plan separates observed invocation behavior from plugin/package experiments and avoids premature installer work.

### Plan Updates

- Tasks checked: none.
- Acceptance criteria checked: none.
- Tasks added/moved: split from the previous combined dogfooding follow-up plan.

### Verification

- not run; this is plan/doc restructuring only.

## Discovered Follow-Ups

- Build a custom installer only if Skills CLI plus documented provider config cannot express the chosen invocation policy clearly enough.
