---
name: plan-work
description: Plan substantial feature or engineering work before implementation. Use when the user has a fuzzy idea, feature request, architecture change, product/workflow change, or multi-step task that needs research, grilling, a concise design brief, Target-State Docs, and an ephemeral Plan.
---

# Plan Work

Turn a rough idea into approved docs and an ephemeral implementation Plan.

Use workflow terms from [workflow-language.md](references/workflow-language.md).
Use the document conventions and templates in [document-conventions.md](references/document-conventions.md) and [templates.md](references/templates.md).

## Process

1. Read repo-local instructions and existing docs.
2. Use the Research Ladder before serious technical recommendations.
3. Interview one question at a time. Each question should include a recommended answer and trade-offs.
4. Present a concise Design Brief for approval before writing files.
5. If changes are requested, loop back through research/grilling.
6. Write or update needed Target-State Docs first. Do not defer needed durable docs into a Plan task.
7. If doc writing reveals ambiguity, stop and ask before finalizing the Plan.
8. Write the Plan in `plans/`, referencing related Docs.
9. Run Plan self-review.
10. Present the Plan path and short summary. Do not auto-start implementation.

Create directories lazily.

## Design Brief

Use the Design Brief template from [templates.md](references/templates.md).

Keep it concise. It is an approval checkpoint, not the durable doc.

## What Belongs Where

- Plan: sequence, phase checklists, acceptance criteria, verification, temporary implementation notes.
- Target-State Doc: durable behavior, flows, constraints, concepts, long-term out-of-scope.
- Glossary: canonical terms and relationships only.
- ADR: rare hard-to-reverse, surprising, real trade-off decisions.

## Plan Self-Review

Before reporting the Plan ready for review, check:

- no placeholders or unresolved TODOs
- no contradictions between brief, Docs, and Plan
- needed Target-State Docs exist before the Plan when durable docs are required
- durable behavior, constraints, and terminology do not live only in the Plan
- Target-State Docs and code examples do not reference Plan files, Plan phases, or Plan-only decisions
- phase granularity is reasonable
- acceptance criteria cover the target state
- related Docs are referenced correctly
- no code-heavy implementation blocks

Fix issues inline before handing the Plan back to the user.
