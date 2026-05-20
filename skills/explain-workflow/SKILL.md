---
name: explain-workflow
description: Explain how to use the workflow skills for a concrete situation. Use when the user asks how to use the workflow, how to invoke workflow skills explicitly or implicitly, or how the workflow applies to features, implementation, review, handoff, architecture improvement, or prototypes.
---

# Explain Workflow

Explain the workflow from the user's point of view.

Use workflow terms from [workflow-language.md](references/workflow-language.md).

Keep answers short and practical unless the user asks for depth. This skill is a guide, not a router: do not start planning, review, handoff, implementation, architecture improvement, or prototyping unless the user clearly asks to switch into that work.

## Invocation

- Explicit: name the skill when you know the workflow, such as "use `plan-work` to plan this feature" or "use `review-work` to review this diff."
- Implicit: describe the job naturally, such as "plan this feature", "review this change", "write a handoff", or "find architecture improvements."
- Correction: if the agent picks the wrong workflow, name the skill you want.

## Quick Recipes

- New feature or fuzzy idea: use `plan-work`. Expect focused questions, a Design Brief, target-state artifacts for durable behavior, and an ephemeral Plan if you approve the direction.
- Approved Plan or next phase: use `implement-plan`. Expect code changes, verification, Plan updates, and phase self-review.
- Codebase explanation: use `explain-codebase`. Expect a read-only map of purpose, entry points, modules, callers, flows, tests, terms, and unknowns.
- Diagnosis: use `diagnose-issue`. Expect reproduction, root-cause investigation, a focused fix when safe, and verification.
- Review: use `review-work`. Expect findings first, ordered by severity, with evidence and recommended fixes.
- Handoff: use `write-handoff`. Expect a fresh-context prompt or file with read-first context, task, constraints, verification, and out-of-scope.
- Architecture improvement: use `improve-architecture`. Expect targeted improvement candidates before any redesign or refactor plan.
- Mockup or throwaway exploration: use `plan-prototype`. Expect a disposable prototype that answers one planning question.
- Unsure but substantial: use `use-workflow`. Expect the agent to pick the most specific workflow skill.

## Ambiguous Requests

If the user asks "how do I use the workflow for X?", explain the likely workflow and give the exact next prompt. Do not start the workflow unless the user clearly asks.

Example:

```text
for auth, use `plan-work`. ask: "use plan-work to plan auth." the agent will ask focused questions, draft a design brief, then write target-state artifacts and a plan after you approve the direction.
```

## Output Shape

Prefer:

- one short answer
- the relevant skill name
- what to ask next
- what to expect

Avoid long lists unless the user asks for a full map.
