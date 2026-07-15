---
name: explain-workflow
description: Explain how to invoke and use the workflow skills for a concrete situation without starting the work unless the user asks.
---

# Explain Workflow

Explain the workflow from the user's point of view. Use [workflow-language.md](references/workflow-language.md).

This skill is a guide, not a router. Give the relevant skill, an exact next prompt, and what to expect; do not start the work unless asked.

## Invocation

- Explicit: `use plan-work to plan this feature` or `use review-work to review this diff`.
- Natural: `plan this feature`, `diagnose this failure`, `write a handoff`, or `review this change`.
- Correction: name the skill when the agent chose the wrong workflow.

## Recipes

- Uncertain or high-risk work, or an explicit planning request -> `plan-work`. Planning can stay in chat when requested; otherwise a saved ephemeral Plan is the default.
- Already-decided work from a Plan, handoff, findings, direct brief, docs task, fix, or refactor -> `implement-work`. Plan tracking applies only when a Plan governs the work; verification and independent fresh-context review always apply.
- Tiny clear implementation -> work directly; it still gets a fresh-context review before delivery.
- Code orientation -> `explain-codebase`.
- Root-cause investigation -> `diagnose-issue`; fixes require explicit authorization.
- Plan, implementation, continuation, or artifact review -> `review-work`; review is read-only unless fixes are separately requested.
- Continuation prompt or file -> `write-handoff`; file/chat storage is separate from Immediate/Durable lifecycle.
- Architecture opportunity discovery -> `improve-architecture`; concrete change review belongs to `review-work`.
- Bounded behavior-preserving simplification -> `simplify-work`.
- Disposable design experiment -> `plan-prototype`.
- Unsure but substantial -> `use-workflow`.

Keep the answer short unless the user asks for the full map. If the question is “how do I use this for X?”, explain and give the next prompt rather than silently starting X.
