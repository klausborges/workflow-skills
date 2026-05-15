---
name: review-work
description: Review implementation work against a Plan, Target-State Docs, acceptance criteria, tests, and code quality expectations. Use when the user asks for review, adversarial review, multi-aspect/subagent review, compliance review, architecture/test/library/performance/security focused review, or review-fix planning.
---

# Review Work

Review implementation work with evidence and technical judgment.

Use workflow terms from [workflow-language.md](references/workflow-language.md).
Use [document-conventions.md](references/document-conventions.md), [research-ladder.md](references/research-ladder.md), and [architecture-language.md](references/architecture-language.md) when relevant.

## Default Style

Default to balanced review with adversarial checks.

- Start from the Plan, related Target-State Docs, and acceptance criteria.
- Treat implementation as untrusted until verified.
- Prioritize real bugs, missing requirements, test gaps, and maintainability risks.
- Use adversarial depth for high-risk surfaces or when requested.

Supported overrides include "quick review", "review this hard", "adversarial review", "paranoid review", "compliance review against the plan", "focus on architecture", and "focus on tests".

## Multi-Aspect Review

Use when the user asks for multiple review angles or subagent review.

1. Infer a useful aspect set from the change and ask for confirmation.
2. If the user specified aspects, suggest any missing useful ones.
3. Dispatch or simulate focused reviewer passes per aspect when useful.
4. Reconcile duplicated or conflicting findings.
5. Verify findings against code, docs, tests, and source documentation where needed.
6. Report only findings that survive reconciliation and verification.
7. Offer or write a `Review-Fix Handoff` for accepted findings.

Possible aspects:

- code quality
- Plan/Doc/acceptance compliance
- library/API correctness
- performance risks
- test coverage and quality
- security or data-loss risks
- architecture and maintainability

## Output

Lead with findings, ordered by severity. Include file/line references when possible.

For each finding:

- severity
- evidence
- why it matters
- recommended fix

After findings, include:

- verification run or skipped
- residual risks
- optional recommendation for `improve-architecture` when architecture friction is real
