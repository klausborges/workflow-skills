---
name: write-handoff
description: Write a bounded fresh-context prompt or file for continuation, implementation, research, refactor, bugfix, review, or review-fix work.
---

# Write Handoff

Give the next worker enough stable context to complete one bounded job without recreating the owning Plan or docs.

Use [workflow-language.md](references/workflow-language.md), [review-language.md](references/review-language.md) for review work, and [handoff-template.md](references/handoff-template.md).

## Choose intent and lifecycle

Common types are implementation/continuation, research, refactor/bugfix, review, review-continuation, and review-fix. Use the smallest type that describes the next job.

Choose lifecycle from expected use:

- **Immediate**: next known worker or near-term continuation; current paths and volatile status may help.
- **Durable**: later or unknown worker, or reusable work across tools; emphasize stable behavior, acceptance criteria, and scope boundaries.

Default to Immediate unless the expected lifecycle clearly requires Durable. A handoff in chat or a file can be either; file storage does not imply durability.

## Build the handoff

1. Read the owning Plan, docs, Review Ledger, current diff, or source only as needed for this task.
2. State the exact task, current state, scope, constraints, completion conditions, and verification.
3. Link only authoritative files the next worker actually needs. Do not copy whole Plans/docs or generate a replacement Plan.
4. Include volatile implementation notes only when they are not discoverable and materially reduce rework.
5. Keep adjacent ideas out of scope or in the owning Plan's follow-ups.

Create a file only when the user asks for one or the authorized workflow clearly needs a stored artifact. Use the repository's established handoff location when present; otherwise ask or return copyable text.

## Review handoffs

- For a continuation/delta round, carry accepted findings, patch notes, changed surface, settled items, Review Signal, unresolved risks, and intended next scope.
- For explicit independent cross-validation, say that overlap is possible and avoid anchoring the reviewer to desired findings.
- If `fresh review` could materially mean continuation or independent cross-validation, consolidate that choice with any other owner questions rather than silently choosing.
- If prior Review Signal was Low, note the likely low return without refusing an explicitly requested round.

## Owning artifact updates

Do not add bookkeeping just because a handoff exists. Update a Plan only when verified Task state, required work, or follow-ups changed. Add a sparse pointer to a handoff file only when it prevents the artifact from being lost.

Use [handoff-template.md](references/handoff-template.md), omit unused sections, and keep the result self-contained and concise.
