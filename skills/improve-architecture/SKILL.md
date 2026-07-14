---
name: improve-architecture
description: Discover and shape architecture improvement opportunities in hard-to-understand, hard-to-test, shallow, scattered, or over-coupled code. Use review-work for concrete change review and direct implementation for already-decided scoped refactors.
---

# Improve Architecture

Find concrete architecture friction and shape only the decision still needed.

Use [architecture-language.md](references/architecture-language.md) and the repo Glossary/ADRs when present.

## Boundaries

- Use this skill to discover opportunities or clarify an unresolved module/interface/seam choice.
- Use `review-work` to judge architecture in a concrete change, PR, or Phase.
- When the user has already chosen a bounded refactor, implement it directly or through `implement-plan`; do not force candidate selection, an interview, a Plan, or a handoff.
- Review may recommend this skill but should not start it automatically.

## Discovery

1. Map the relevant modules, callers, interfaces, seams, and flows.
2. Identify evidence-backed friction: knowledge scattered across callers, shallow pass-through layers, leaky implementation details, hard-to-test behavior, change amplification, or unclear ownership.
3. Present a small ranked set of candidates only when the user has not already selected the area.
4. For the chosen candidate, clarify only load-bearing constraints, migration, verification, and caller impact.
5. Return the smallest useful next artifact: recommendation, interface choice, scoped implementation direction, Plan request, or handoff. Do not create a Plan or handoff by default.

Candidate summaries should name the area, observed friction, opportunity, benefits, risks, and evidence.

## Interface or seam choice

When placement is genuinely unresolved, compare two or three materially different options by caller impact, hidden complexity, migration cost, testability, volatility, and ownership. Recommend one. Skip this branch when placement is obvious or unchanged.

Treat architecture vocabulary as heuristics. A single adapter can justify a seam when it isolates volatility, ownership, or testing; ask what boundary it protects. Prefer public or intentionally stable interfaces as behavioral test surfaces without exposing internals only for tests.

Do not propose broad rewrites, relitigate ADRs without new friction, or make ordinary feature work speak architecture jargon unnecessarily.
