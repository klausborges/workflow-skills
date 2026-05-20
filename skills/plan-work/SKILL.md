---
name: plan-work
description: Plan substantial feature or engineering work before implementation. Use when the user has a fuzzy idea, feature request, architecture change, product/workflow change, or multi-step task that needs research, grilling, a concise design brief, target-state artifacts, and an ephemeral Plan.
---

# Plan Work

Turn a rough idea into approved target-state artifacts and an ephemeral implementation Plan.

Use workflow terms from [workflow-language.md](references/workflow-language.md).
Use the document conventions and templates in [document-conventions.md](references/document-conventions.md) and [templates.md](references/templates.md).

## Process

1. Read repo-local instructions and existing docs.
2. Use the Research Ladder before serious technical recommendations.
3. Interview one question at a time. Each question should include a recommended answer and trade-offs.
4. Present a concise Design Brief for approval before writing files.
5. If changes are requested, loop back through research/grilling.
6. Write needed target-state artifacts first. Use canonical Target-State Docs only for currently true durable information, `docs/_drafts/` for substantial new proposed docs, and Plan Doc Delta for future edits to existing docs.
7. If doc writing or Doc Delta shaping reveals ambiguity, stop and ask before finalizing the Plan.
8. Write the Plan in `plans/`, referencing related Docs.
9. Run Plan self-review.
10. Present the Plan path and short summary. Do not auto-start implementation.

Create directories lazily.

## Design Brief

Use the Design Brief template from [templates.md](references/templates.md).

Keep it concise. It is an approval checkpoint, not the durable doc.

## What Belongs Where

- Plan: sequence, phase checklists, acceptance criteria, verification, temporary implementation notes, and Doc Delta for future edits to existing docs.
- Target-State Artifact: planning-time durable intent captured as current canonical docs, draft docs, or Plan Doc Delta.
- Target-State Doc: current durable behavior, flows, constraints, concepts, long-term out-of-scope.
- Draft Target-State Doc: substantial new proposed durable doc under `docs/_drafts/`, deleted after promotion, merge, abandonment, or explicit deferment.
- Glossary: canonical terms and relationships only.
- ADR: rare hard-to-reverse, surprising, real trade-off decisions.

## Doc Lifecycle

During planning, keep canonical docs truthful about current behavior.

- Update canonical docs before implementation only for currently true clarifications, stale-current-doc fixes, stable background, or canonical terminology already valid today.
- Create `docs/_drafts/<topic>.md` only for substantial new durable docs that are likely to become canonical docs.
- Do not create draft docs for small future edits to existing docs.
- Put future edits to existing docs in the Plan's `Doc Delta` section.
- Use checkbox items in `Doc Delta`. Each item should carry an inline phase tag such as `(Phase 2)`; items without a tag default to the final implementation phase.
- Define referenced draft docs by listing them under `Doc Delta` > `Draft Docs`. Unlisted files in `docs/_drafts/` are unrelated to the Plan.
- Do not require YAML/frontmatter for draft docs.

## Plan Self-Review

Before reporting the Plan ready for review, check:

- no placeholders or unresolved TODOs
- no contradictions between brief, Docs, and Plan
- needed target-state artifacts exist before the Plan when durable docs are required
- durable behavior, constraints, and terminology live in canonical docs, draft docs, or Doc Delta rather than only in ad hoc Plan prose
- existing canonical docs were not silently updated with future-only behavior
- Doc Delta uses checkbox items for future edits to existing docs
- Target-State Docs and code examples do not reference Plan files, Plan phases, or Plan-only decisions
- phase granularity is reasonable
- acceptance criteria cover the target state
- related Docs are referenced correctly
- no code-heavy implementation blocks

Fix issues inline before handing the Plan back to the user.
