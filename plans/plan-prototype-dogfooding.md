# Plan Prototype Dogfooding Plan

## Goal

Dogfood the improved `plan-prototype` flow in real planning sessions and tune it from evidence.

## Related Docs

- `docs/workflow.md`
- `GLOSSARY.md`
- `plans/plan-prototype-reference-improvements.md`

## Phase Checklist

- [ ] Phase 1: Collect improved prototype-use examples
- [ ] Phase 2: Tune prototype behavior from evidence

## Verification Strategy

- Test/verification cadence: Acceptance gate.
- Targeted tests: run generated-reference checks after shared reference changes.
- Integration checks: run `npx skills add . --list` after skill changes.
- Type/lint/build: run `mise run check`.
- Manual checks: compare updated guidance against real planning sessions that used prototypes.
- Final review triggers: review for accidental coupling to one local browser/server workflow, stale prototype artifacts, or unclear delete/absorb guidance.

## Out of Scope

- Recreating Superpowers' full visual companion loop by default.
- Requiring every planning task to produce a prototype.
- Adding repo-local app scaffolding unless a later Plan chooses that explicitly.
- Making invocation policy decisions; record observations here and mirror broader routing conclusions to `plans/invocation-policy-dogfooding.md`.

## Research Notes

- This Plan intentionally starts after the research-backed improvements in `plans/plan-prototype-reference-improvements.md`.
- Current dogfooding question: does the improved skill help agents pick the right prototype mode, show artifacts clearly, capture the answer, and delete or absorb prototype output?
- Prototype prompts should include both explicit calls, such as `$plan-prototype`, and natural prompts that might implicitly route through `use-workflow`.

## Phase 1: Collect Improved Prototype-Use Examples

Mode: HITL

### Goal

Gather enough real examples to evaluate the improved `plan-prototype` behavior.

### Context

The improved skill should be tested across the next few days rather than judged from reference research alone. The evidence should stay concise and anonymized enough to remain public.

### Evidence Template

For each session, record:

- Prompt style: explicit skill call, natural prompt, or ambiguous prompt.
- Planning question: the specific uncertainty the prototype answered.
- Mode used: visual, diagram, option canvas, logic/state/data/API, mixed, or text-only after deciding no prototype was needed.
- Artifact surface: screenshot, local URL, file path, one-command TUI/script, diagram, or terminal-only explanation.
- Feedback channel: terminal only, browser plus terminal, manual QA, or other.
- Result: decision made, prototype deleted, prototype absorbed, or temporarily kept with reason.
- Skill issue vs project/tooling issue: what failed or worked because of the skill instructions versus local constraints.
- Invocation note: whether routing was correct, too eager, too weak, or explicitly requested.

### Tasks

- [ ] Use the improved `plan-prototype` in at least two real planning sessions.
- [ ] Include at least one visual, diagram, or option-canvas example when possible.
- [ ] Include at least one logic/state/data/API example when possible.
- [ ] Try at least one natural prompt that does not name the skill.
- [ ] Record whether the prototype helped the decision and when it felt like ceremony.
- [ ] Record whether the user needed a screenshot, local URL, code artifact, command, or short explanation.
- [ ] Mirror broader implicit-routing observations into `plans/invocation-policy-dogfooding.md`.

### Acceptance Criteria

- [ ] At least two examples are summarized in this Plan.
- [ ] Each example identifies the planning question the prototype answered.
- [ ] Each example records whether the prototype was deleted, absorbed, temporarily kept, or unnecessary.
- [ ] Each surprising invocation has a likely cause or is mirrored to invocation-policy dogfooding.
- [ ] Plan checklist is updated with completed work and newly discovered tasks.

### Notes

Keep examples concise. The goal is guidance, not a portfolio.

## Phase 2: Tune Prototype Behavior From Evidence

Mode: Mixed

### Goal

Update `plan-prototype` and docs only where dogfooding shows a real gap.

### Context

The skill should remain lightweight and tooling-neutral. This phase is for evidence-backed tightening after the first improved version has been used.

### Tasks

- [ ] Decide whether the improved "show or inspect" guidance worked.
- [ ] Decide whether visual, diagram, option-canvas, and logic modes need clearer triggers.
- [ ] Decide whether cleanup/absorption guidance prevented stale prototype artifacts.
- [ ] Decide whether implicit invocation behavior should affect this skill's description or only invocation-policy work.
- [ ] Update `skills/plan-prototype/SKILL.md` if the examples reveal missing behavior.
- [ ] Update `docs/workflow.md` only if the decision becomes durable workflow policy.
- [ ] Run generated-reference sync if shared references change.
- [ ] Refresh README token counts if skill text changes.
- [ ] Run repository checks and skill discovery.

### Acceptance Criteria

- [ ] The skill still says when a prototype should be shown or made inspectable.
- [ ] The skill remains tooling-neutral unless a repo explicitly opts into specific tooling.
- [ ] The workflow still treats prototypes as disposable planning aids.
- [ ] README token counts are current if changed.
- [ ] `npx skills add . --list` discovers the expected skills.
- [ ] `mise run check` passes.
- [ ] Implementation self-review completed.
- [ ] Plan checklist is updated with completed work and newly discovered tasks.

## Discovered Follow-Ups

- Consider a concrete local-server workflow only if multiple real planning sessions show tooling-neutral guidance is too vague.
- Consider moving repeated evidence-template language into a shared reference only if another dogfooding Plan needs the same structure.

## Plan Self-Review

Status: PASS

### Findings

- No blocking findings. The Plan now focuses on real post-improvement dogfooding instead of blocking initial skill improvements on future evidence.
- No blocking findings. The evidence template captures prompt style and invocation surprises while leaving broad invocation-policy decisions in `plans/invocation-policy-dogfooding.md`.

### Plan Updates

- Tasks checked: none.
- Acceptance criteria checked: none.
- Tasks added/moved: moved reference-backed first-pass improvements to `plans/plan-prototype-reference-improvements.md`.

### Verification

- `git diff --check`: passed.
- `mise run check`: passed.
