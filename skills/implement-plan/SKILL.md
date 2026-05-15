---
name: implement-plan
description: Implement selected phases from an approved Plan. Use when the user asks to implement a Plan, continue planned work, work through phase(s), run work unattended, or execute checklist items with TDD, verification, Plan updates, and phase self-review.
---

# Implement Plan

Implement approved Plan phases while keeping the Plan current.

Use workflow terms from [workflow-language.md](references/workflow-language.md).
Use [document-conventions.md](references/document-conventions.md), [templates.md](references/templates.md), and [research-ladder.md](references/research-ladder.md).

## Before Code Changes

1. Read the Plan and related Docs.
2. Identify selected phase(s), or infer the next unchecked phase if not specified.
3. Restate phase work and mode/override in a short checklist.
4. Research implementation-specific gaps.
5. Ask questions only for real ambiguity, HITL work, or mode override confirmation.
6. Implement with pragmatic red/green TDD unless the user explicitly opts out.

Confirmation is not required for straightforward AFK work with no ambiguity.

## Phase Modes

- `AFK`: can proceed without user decisions.
- `HITL`: requires design review, manual approval, credentials, product judgment, or unresolved trade-offs.
- `Mixed`: use only when splitting the phase would be worse than noting a checkpoint.

Mode is guidance. User override phrases like "run phases 1-5 unattended" or "treat this as HITL" take precedence.

## Plan Updates

- Check completed task boxes as work lands.
- Check acceptance criteria only after evidence proves them.
- Mark a phase complete only when all required acceptance criteria are checked.
- Add required discovered tasks under the same phase.
- Add useful but non-required work to `Discovered Follow-Ups`.
- Add `Progress Notes` only when they materially help a future handoff or review.

## Phase Completion Gate

Before marking a phase complete, run a focused self-review against the Plan phase, related Docs, tests, and actual diff.

Use a fresh subagent whenever the harness provides subagents. Give it proper review instructions and avoid biasing it with desired findings. Same-agent self-review is fallback only.

Self-review checks:

- every task was implemented or deliberately moved
- every acceptance criterion is satisfied by evidence
- tests cover intended behavior at the right level
- no obvious plan drift, overbuild, or missing docs updates
- verification commands were run and read

If AFK/unattended: fix self-review findings, update the Plan, rerun verification, then check completion.

If HITL/risky: do not mark complete. Report findings, options, recommendation, reasoning, and trade-offs.

Use the Phase Self-Review template in [templates.md](references/templates.md).

## Final Review Escalation

Phase self-review is mandatory per implemented phase.

Run or request broader final review when:

- more than one phase was implemented
- shared architecture or public contracts changed
- user requested AFK/unattended work
- implementation had plan drift or self-review fixes
- verification was incomplete or partially manual
