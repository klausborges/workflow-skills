# Plan Prototype Dogfooding Plan

## Goal

Clarify how `plan-prototype` should behave when visual or logic prototypes are useful during planning.

## Related Docs

- `docs/workflow.md`
- `GLOSSARY.md`

## Phase Checklist

- [ ] Phase 1: Collect prototype-use examples
- [ ] Phase 2: Tighten prototype guidance

## Verification Strategy

- Targeted tests: run generated-reference drift checks after skill/doc changes.
- Integration checks: run `npx skills add . --list` after skill changes.
- Type/lint/build: run `mise run check`.
- Manual checks: compare the updated guidance against real planning sessions that used prototypes.
- Final review triggers: review for accidental coupling to one local app/browser/server workflow.

## Out of Scope

- Recreating Superpowers' full visual companion loop by default.
- Requiring every planning task to produce a prototype.
- Adding repo-local app scaffolding unless a later Plan chooses that explicitly.

## Research Notes

- Current repo decision: `plan-prototype` should stay tooling-neutral by default and use existing project tooling when possible.
- Current repo decision: prototype code should answer one planning question, then be deleted or absorbed.
- Open question: when a prototype exists, the skill should be clearer about when to show it to the user.

## Phase 1: Collect Prototype-Use Examples

Mode: HITL

### Goal

Gather enough real dogfooding examples to tune `plan-prototype` without designing around guesses.

### Context

The current skill already supports visual and logic prototypes, and `docs/workflow.md` keeps a full visual companion loop out of scope. The remaining question is practical behavior: when to build, show, compare, discard, or absorb prototypes.

### Tasks

- [ ] Use `plan-prototype` in at least two real planning sessions.
- [ ] Note whether the prototype was visual, logic/state/API, diagrammatic, or mixed.
- [ ] Record when the prototype helped the decision and when it felt like ceremony.
- [ ] Identify whether the user needed a screenshot, local URL, code artifact, or just a short explanation.

### Acceptance Criteria

- [ ] At least two examples are summarized in this Plan.
- [ ] Each example identifies the planning question the prototype answered.
- [ ] Each example records whether the prototype should be deleted, absorbed, or kept temporarily.
- [ ] Plan checklist is updated with completed work and newly discovered tasks.

### Notes

Keep examples concise. The goal is guidance, not a portfolio.

## Phase 2: Tighten Prototype Guidance

Mode: Mixed

### Goal

Update `plan-prototype` and docs only where dogfooding shows a real gap.

### Context

The skill should remain lightweight and should not force one browser, server, or UI stack onto unrelated repos.

### Tasks

- [ ] Decide whether `plan-prototype` needs explicit "show the prototype" guidance.
- [ ] Update `skills/plan-prototype/SKILL.md` if the examples reveal missing behavior.
- [ ] Update `docs/workflow.md` only if the decision becomes durable workflow policy.
- [ ] Run generated-reference sync if shared references change.
- [ ] Run repository checks.

### Acceptance Criteria

- [ ] The skill clearly says when a prototype should be shown to the user.
- [ ] The skill remains tooling-neutral unless a repo explicitly opts into specific tooling.
- [ ] The workflow still treats prototypes as disposable planning aids.
- [ ] `mise run check` passes.
- [ ] Implementation self-review completed.
- [ ] Plan checklist is updated with completed work and newly discovered tasks.

## Phase Self-Review

Status: PASS

### Findings

- No blocking findings. The Plan keeps the existing tooling-neutral decision and narrows the live work to dogfooding evidence and guidance cleanup.

### Plan Updates

- Tasks checked: none.
- Acceptance criteria checked: none.
- Tasks added/moved: split from the previous combined dogfooding follow-up plan.

### Verification

- not run; this is plan/doc restructuring only.

## Discovered Follow-Ups

- Consider a concrete local-server workflow only if multiple real planning sessions show tooling-neutral guidance is too vague.
