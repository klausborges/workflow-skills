---
name: improve-architecture
description: Find and shape architecture improvement opportunities. Use when the user asks to improve architecture, find deepening opportunities, make code easier to test or understand, reduce shallow/scattered/over-coupled code, review architecture, or refactor a landed feature using Module/Interface/Seam/Adapter/Depth/Leverage/Locality thinking.
---

# Improve Architecture

Find architecture friction and shape targeted improvements.

Use workflow terms from [workflow-language.md](references/workflow-language.md).
Use [architecture-language.md](references/architecture-language.md) and repo Glossary/ADRs when present.

## When To Use

- After implementing and reviewing a Plan.
- Before final review when architecture risk is visible.
- On a feature that landed earlier and now shows friction.
- When code is hard to test, hard to understand, shallow, scattered, or over-coupled.

This skill is explicit-invocation by default. `review-work` may recommend it but should not start it automatically.

## Process

1. Read relevant Glossary and ADRs.
2. Zoom out: map relevant modules, callers, seams, and flows at a high level.
3. Explore friction:
   - understanding requires bouncing across many shallow modules
   - behavior is hard to test through a stable interface
   - changes spread across many callers
   - seams exist with only one real adapter
   - implementation details leak through public surfaces
4. Present numbered deepening candidates. Do not redesign yet.
5. Let the user pick a candidate.
6. Grill the chosen candidate: constraints, dependencies, seam placement, tests, migration path.
7. Propose a focused improvement plan or handoff.

## Candidate Format

- **Area**: module/feature in Glossary vocabulary.
- **Problem**: concrete friction.
- **Opportunity**: what could deepen or simplify.
- **Benefits**: leverage, locality, and testability.
- **Risks**: what could go wrong or invalidate the change.

## Guardrails

- Do not propose broad rewrites.
- Do not relitigate ADRs unless friction is real enough to reopen the decision.
- Do not force architecture vocabulary into ordinary feature work unless it clarifies the problem.
- Prefer improvements that support current Plan/feature goals or real maintenance friction.
