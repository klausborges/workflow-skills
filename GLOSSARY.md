# Skill Workflow Design

This glossary defines the language for this agent skill workflow. It exists to keep planning, implementation, handoff, and review concepts precise across skills and agents.

## Language

**Target-State Doc**:
A durable document describing the desired behavior, concepts, or structure of a system after implementation.
_Avoid_: Spec, implementation plan, code example dump, references to Plan files or phases

**Plan**:
An ephemeral phased implementation artifact used to guide and track work toward an accepted target state. After full implementation and final review, the agent asks before deleting the completed Plan.
_Avoid_: Spec, target-state doc, permanent documentation

**Scoped Doc Directory**:
A directory under `docs/` for Target-State Docs belonging to one feature, app, or monorepo area.
_Avoid_: Specs folder, plan folder

**Handoff**:
A copyable prompt or file that gives a fresh agent enough context to continue, implement, refactor, fix, or review work.
_Avoid_: Summary, status update

**Durable Handoff**:
A handoff written for later or unknown agents, or as a reusable prompt for repeated work across models/tools, emphasizing behavior, acceptance criteria, and scope boundaries over brittle current file paths.
_Avoid_: Same-day status note

**Research Handoff**:
A handoff that asks a fresh agent to investigate options, prototype lightly if useful, and return a recommendation rather than production implementation.
_Avoid_: Implementation handoff, research dump

**Multi-Aspect Review**:
A review mode that evaluates the same work through multiple focused lenses, then reconciles and verifies the findings before reporting.
_Avoid_: Unreconciled subagent dump, default review

**Review Signal**:
The amount of actionable, verified review value remaining in a review round. High signal means findings block closeout or materially affect correctness, docs, tests, verification, regression risk, or user-visible behavior. Medium signal means valid findings remain but are localized and likely patchable. Low signal means remaining findings are minor, speculative, stylistic, already-deferred, or better routed to follow-up or architecture work.
_Avoid_: Treating every nit as a reason to keep reviewing, stopping before blocking findings are resolved

**Implementation**:
The act of changing code and tests for selected Plan phase work, with verification and checklist updates.
_Avoid_: Handoff, review

**Feedback-Calibrated Test Discipline**:
A testing and verification practice that chooses the fastest meaningful feedback loop for each behavior change. Cheap tests use red/green vertical slices; medium or heavy tests use same-phase or acceptance-gate verification; fallback requires an explicit reason and replacement evidence.
_Avoid_: Blanket red/green TDD for every test type, skipping tests because they are expensive

**Research Ladder**:
A portable source preference for technical investigation that uses stronger tools when available and falls back to first-party docs, `llms.txt`, official examples, CLI help, and local examples.
_Avoid_: Hard-coded tool dependency, web search first

**Glossary**:
A durable language document containing canonical terms, relationships, and flagged ambiguities for a project or scoped area.
_Avoid_: Context dump, scratchpad, implementation notes

**Glossary Map**:
An optional root document that lists multiple scoped glossaries and when to use each one.
_Avoid_: Required setup artifact, generic docs index

**ADR**:
A concise record of a hard-to-reverse, surprising, trade-off-driven architectural decision.
_Avoid_: Design note, decision log for obvious choices

## Relationships

- A **Plan** may be standalone or reference one or more related **Target-State Docs**.
- Completed **Plans** should not be required by live docs or code after cleanup. Durable docs and code should not depend on Plan files, Plan phases, or Plan-only decisions.
- A **Scoped Doc Directory** groups **Target-State Docs** when a feature, app, or monorepo area is too large for a single doc.
- A **Plan** may produce one or more **Handoffs** during implementation or review.
- **Implementation** and **Handoff** are separate workflows, even when both reference the same **Plan**.
- **Review Signal** helps decide whether another review round is worth running: repeated rounds should tighten the finding threshold and can close out before findings reach zero when remaining findings are low signal.
- **Feedback-Calibrated Test Discipline** is part of **Implementation**: test cost changes cadence, not the expectation that behavior changes get meaningful test or verification evidence.
- An **ADR** supplements a **Target-State Doc** or **Plan** only when the decision meets the ADR threshold.
- A **Glossary** should contain domain/workflow language only, not implementation details.
- A root **Glossary** is the default; scoped glossaries live under `docs/<scope>/GLOSSARY.md`; a **Glossary Map** exists only when discovery becomes hard.
- ADRs default to `docs/adr/`, with scoped ADRs under `docs/<scope>/adr/` only when the decision is clearly local to that scope.

## Example dialogue

> **Dev:** "Should this behavior live in the **Plan** or a **Target-State Doc**?"
> **Domain expert:** "If it should remain true after implementation, put it in the **Target-State Doc**. If it only guides the work, put it in the **Plan**."

## Flagged Ambiguities

- "spec" can imply spec-driven development or code-heavy permanent design docs. Resolved: use **Target-State Doc** as the canonical term; "doc" or "docs" may be shorthand in conversation.
