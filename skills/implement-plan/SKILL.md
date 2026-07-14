---
name: implement-plan
description: Implement selected Phases from an approved Plan with risk-matched verification, Plan and doc updates, Plan-defined review checkpoints, and mandatory fresh-context end-of-Phase review.
---

# Implement Plan

Implement the selected Plan scope and keep its evidence current.

Use [workflow-language.md](references/workflow-language.md), [document-conventions.md](references/document-conventions.md), and [phase-review-template.md](references/phase-review-template.md).

## Before changing code

1. Read repo instructions, the Plan, related current docs, relevant source/tests, referenced drafts, and Phase-tagged Doc Delta items.
2. Select the requested Phase(s), or the next unchecked Phase when the user asks to continue.
3. Identify unresolved decisions, high-impact review checkpoints, target behavior, the narrowest useful verification, and any required doc work.
4. Research only implementation gaps that can materially change the approach.
5. Ask only for real ambiguity, credentials, product judgment, risky authority, or an explicit human checkpoint. Otherwise proceed.

Canonical docs must remain current truth. Update a draft or Doc Delta when unresolved target state changes; update canonical docs as the corresponding behavior lands.

## Verification cadence

Use the fastest meaningful feedback loop for each behavior:

- **Fast red/green** for cheap focused unit, integration, component, or browser behavior.
- **Same-Phase loop** for setup-heavy or medium-cost checks.
- **Acceptance gate** for slow end-to-end, full-suite, or visual checks, backed by cheaper logic tests where useful.
- **Manual/observational fallback** only when automation is impractical; record the reason, evidence, and residual risk.

Test through public or intentionally stable interfaces when practical. Heavy tests change cadence, not the need for evidence. Do not claim success until fresh commands or observations have run and their results were read.

## Execute and track

- Work through the selected Phase without absorbing adjacent cleanup.
- Follow Plan-defined independent review checkpoints after high-impact Tasks or Task groups. Low-impact Tasks do not need individual immediate review.
- Check a Task or acceptance criterion only after evidence supports it.
- Add required discoveries to the current Phase; route useful non-required work to `Discovered Follow-ups`.
- Apply only the current Phase's Doc Delta and referenced-draft work. Sync docs at Phase closeout or handoff, earlier only when downstream work needs the contract.
- Keep progress notes only when a fresh worker or reviewer would otherwise lose important state.
- Preserve preexisting drafts, prototypes, and unrelated dirty work. Clean up only artifacts created by this task whose disposition is already clear.

## Phase completion

Before marking a Phase complete:

1. Self-check the whole Phase against its tasks, acceptance criteria, docs, tests, actual diff, and durable-artifact boundary.
2. Run the relevant verification and read its output.
3. Send the entire Phase scope to an independent fresh-context reviewer using `review-work`. Do not bias the reviewer toward a desired result.
4. Verify every accepted reviewer claim against code, docs, diffs, or commands.
5. Fix authorized blocking findings, rerun affected verification, and repeat a scoped fresh review only when the changes or residual risk justify it.
6. Update the Plan and related docs, then mark the Phase complete.

Use [phase-review-template.md](references/phase-review-template.md) for the compact review record. Do not recursively review the review.

At a Milestone boundary, add an integration review only when cross-Phase acceptance or risk warrants it. The end of each Phase already has mandatory independent review.

When all Phases are complete, leave Plan or Roadmap cleanup to final review. Do not ask about deletion during implementation.
