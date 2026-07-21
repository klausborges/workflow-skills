---
name: simplify-work
description: Simplify an already-bounded code or documentation area without changing intended behavior. Use for deletion, direct-flow refactors, wrapper or branch reduction, clearer data models, reduced mutable state, or concrete simplification findings; use improve-architecture when the opportunity or design is still unclear.
---

# Simplify Work

Make the selected area easier to understand, change, and verify while preserving intended behavior.

Use [engineering-discipline.md](references/engineering-discipline.md). Accept a direct scope, handoff, review finding, or Plan as input.

## Boundaries

- Read repo instructions and relevant current docs; inspect the workspace so preexisting changes are not mistaken for task work.
- Confirm the bounded surface, intended behavior, callers, constraints, and proof before editing.
- Do not scan the whole repo, create a Plan or handoff, run an interview, or produce HTML by default.
- Do not use line count as the goal. Prefer fewer concepts, branches, layers, mutable states, and duplicated rules when that reduces reader load.
- Preserve unrelated dirty work and preexisting drafts or prototypes. Clean only task-created artifacts whose disposition is clear.
- If the change requires new product behavior, changing a declared compatibility contract, or unresolved architecture placement, stop for owner judgment or route discovery to `improve-architecture`.

## Simplify

1. Establish a focused behavior check. Add characterization coverage only when existing evidence cannot protect consequential behavior.
2. Trace the relevant caller, data, component, or control flow and find the accidental structure carrying the cost.
3. Prefer deletion, direct data flow, canonical ownership, and types or data models that remove invalid states and spread-out branching.
4. Keep each change small enough to reason about and verify. Avoid unrelated cleanup and speculative abstraction.
5. Run focused checks as the work proceeds, then the relevant regression or acceptance gate. Read the results.

For Plan-backed work, select the requested or next unchecked Phase, follow its checkpoints, and update Tasks or acceptance criteria only after evidence supports them. Apply only that Phase's Doc Delta and referenced-draft work; after fresh review and fixes, update the Plan/docs before marking it complete. Leave Plan or Roadmap cleanup to final review. Without a Plan, do not invent Plan mechanics.

## Close the scope

Self-check behavior preservation and whether the result materially lowers reader load. Send the entire scope, not only the latest patch, to an independent fresh-context reviewer through `review-work`; verify accepted claims against primary evidence, fix authorized blocking findings, and rerun affected checks. Report the simplification, behavior evidence, and residual risk concisely.
