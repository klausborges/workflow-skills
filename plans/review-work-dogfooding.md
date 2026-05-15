# Review Work Dogfooding Plan

## Goal

Use `review-work` on real project changes and tune the review workflow from evidence.

## Related Docs

- `docs/workflow.md`
- `GLOSSARY.md`

## Phase Checklist

- [ ] Phase 1: Collect real review sessions
- [ ] Phase 2: Refine review behavior

## Verification Strategy

- Targeted tests: run generated-reference drift checks after skill/doc changes.
- Integration checks: run `npx skills add . --list` after skill changes.
- Type/lint/build: run `mise run check`.
- Manual checks: compare review output against user usefulness, false positives, missed issues, and follow-up cost.
- Final review triggers: review for scope creep into architecture refactoring or diagnosis.

## Out of Scope

- Making multi-aspect review the default for every small review.
- Turning review into architecture improvement unless findings justify it.
- Replacing `diagnose-issue` for bugs or failing tests.

## Research Notes

- Current repo decision: review output leads with findings, ordered by severity.
- Current repo decision: multi-aspect review is explicit/requested or inferred only when useful, then reconciled before reporting.
- Current repo decision: `improve-architecture` may be recommended from review, but should not start automatically.

## Phase 1: Collect Real Review Sessions

Mode: Mixed

### Goal

Evaluate `review-work` on nontrivial changes outside this repo.

### Context

The skill text already encodes the desired review shape. The open question is whether it is ergonomic and sharp enough in real use.

### Tasks

- [ ] Run `review-work` against at least two nontrivial real project changes.
- [ ] Try one multi-aspect review where the change merits it.
- [ ] Record which findings were useful, noisy, duplicated, or missed.
- [ ] Record whether the review should have routed to `diagnose-issue`, `improve-architecture`, or `write-handoff` instead.

### Acceptance Criteria

- [ ] At least two review sessions are summarized in this Plan.
- [ ] At least one session evaluates multi-aspect behavior.
- [ ] The summaries distinguish skill problems from project-specific context gaps.
- [ ] Plan checklist is updated with completed work and newly discovered tasks.

### Notes

Do not paste private project details. Keep examples anonymized enough for a public roadmap.

## Phase 2: Refine Review Behavior

Mode: Mixed

### Goal

Update `review-work` only where dogfooding shows repeated friction.

### Context

Review should stay findings-first and evidence-led. Improvements should reduce false positives, missed risks, or unclear routing.

### Tasks

- [ ] Decide whether the output shape needs tighter instructions.
- [ ] Decide whether multi-aspect review needs stronger trigger guidance.
- [ ] Decide whether routing boundaries with `diagnose-issue` and `improve-architecture` need clarification.
- [ ] Update `skills/review-work/SKILL.md` and related docs if needed.
- [ ] Run repository checks.

### Acceptance Criteria

- [ ] Review output remains findings-first.
- [ ] Multi-aspect review remains explicit or clearly justified.
- [ ] `improve-architecture` remains a recommendation unless explicitly invoked.
- [ ] `diagnose-issue` remains the better route for root-cause debugging.
- [ ] `mise run check` passes.
- [ ] Implementation self-review completed.
- [ ] Plan checklist is updated with completed work and newly discovered tasks.

## Phase Self-Review

Status: PASS

### Findings

- No blocking findings. The Plan removes already-completed checklist claims and keeps only evidence-gathering plus possible refinement.

### Plan Updates

- Tasks checked: none.
- Acceptance criteria checked: none.
- Tasks added/moved: split from the previous combined dogfooding follow-up plan.

### Verification

- not run; this is plan/doc restructuring only.

## Discovered Follow-Ups

- Consider a reusable review-fix handoff template if repeated review rounds show the same handoff shape.
