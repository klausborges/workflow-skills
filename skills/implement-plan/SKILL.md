---
name: implement-plan
description: Implement selected phases from an approved Plan. Use when the user asks to implement a Plan, continue planned work, work through phase(s), run work unattended, or execute checklist items with test/verification cadence, Plan updates, and phase self-review.
---

# Implement Plan

Implement approved Plan phases while keeping the Plan current.

Use workflow terms from [workflow-language.md](references/workflow-language.md).
Use [document-conventions.md](references/document-conventions.md), [templates.md](references/templates.md), and [research-ladder.md](references/research-ladder.md).

## Before Code Changes

1. Read the Plan and related Docs.
2. Run a Target-State Doc preflight: if the Plan or target state needs durable docs, confirm related Docs exist and match the intended target state before code changes.
3. If needed Docs are missing or stale, update them first. If doc writing reveals real ambiguity, stop and ask before implementation.
4. Identify selected phase(s), or infer the next unchecked phase if not specified.
5. Restate phase work and mode/override in a short checklist.
6. Research implementation-specific gaps.
7. Ask questions only for real ambiguity, HITL work, or mode override confirmation.
8. Use feedback-calibrated test discipline. If the user explicitly opts out of automated tests, record the fallback verification evidence instead.

Confirmation is not required for straightforward AFK work with no ambiguity.

## Test/Verification Cadence

Use the fastest meaningful feedback loop for each behavior change.

Before implementation or at the start of each phase, identify:

- target behavior
- test surface
- narrowest useful command
- cadence: fast red/green, same-phase test loop, acceptance gate, or manual/observational fallback
- reason when not using fast red/green

Cadences:

- `Fast red/green`: cheap unit tests, focused integration tests, or focused component/browser tests that can run in isolation. Write one behavior test, run it red, implement the smallest green change, and repeat vertically.
- `Same-phase test loop`: medium-cost or setup-heavy integration, UI, browser, or visual tests. Write or update tests during the phase and run the narrowest relevant command before phase completion.
- `Acceptance gate`: heavy E2E, full-browser, full-suite, or slow visual checks. Prefer lower-level tests for logic, then run the heavy check at module or phase closeout before claiming completion.
- `Manual/observational fallback`: only when useful automated tests are unavailable or impractical. Record why, run the strongest available check, and state residual risk.

Rules:

- Heavy tests change cadence; they are not permission to skip tests or checks.
- Test behavior through public or stable interfaces. Avoid implementation-detail tests and unnecessary mocks.
- Do not claim fixed, passing, or complete until verification commands or manual checks have run and the output or evidence has been read.

## Phase Modes

- `AFK`: can proceed without user decisions.
- `HITL`: requires design review, manual approval, credentials, product judgment, or unresolved trade-offs.
- `Mixed`: use only when splitting the phase would be worse than noting a checkpoint.

Mode is guidance. User override phrases like "run phases 1-5 unattended" or "treat this as HITL" take precedence.

## Plan Updates

- Check completed task boxes as work lands.
- Check acceptance criteria only after evidence proves them.
- Mark a phase complete only when all required acceptance criteria are checked.
- Update Target-State Docs immediately when implementation reveals durable behavior, constraints, or terminology changes.
- Keep uncertain, temporary, or non-required discoveries in Plan notes or `Discovered Follow-Ups` until they are resolved.
- Add required discovered tasks under the same phase.
- Add useful but non-required work to `Discovered Follow-Ups`.
- Add `Progress Notes` only when they materially help a future handoff or review.
- When all implementation phases are complete, require final review before Plan cleanup. Do not ask to delete the Plan from implementation.

## Phase Completion Gate

Before marking a phase complete, run a focused self-review against the Plan phase, related Docs, tests, and actual diff.

Use a fresh subagent whenever the harness provides subagents. Give it proper review instructions and avoid biasing it with desired findings. Same-agent self-review is fallback only.

Self-review checks:

- every task was implemented or deliberately moved
- every acceptance criterion is satisfied by evidence
- test/verification cadence was identified and followed
- tests or checks cover intended behavior at the right level
- fallback verification, missing automation, or skipped fast red/green has an explicit reason
- no obvious plan drift, overbuild, missing docs updates, or Target-State Doc drift
- durable docs and code remain coherent without references to Plan files or phases
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
