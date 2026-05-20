# Skill Workflow Design

This glossary defines the language for this agent skill workflow. It exists to keep planning, implementation, handoff, and review concepts precise across skills and agents.

## Language

**Target-State Doc**:
A canonical durable document describing current desired behavior, concepts, or structure of a system after implementation has landed. Normal Target-State Docs should not describe unimplemented future behavior unless they are explicitly historical/reference.
_Avoid_: Spec, implementation plan, code example dump, references to Plan files or phases, future-only behavior during planning

**Target-State Artifact**:
A planning-time artifact that captures desired durable behavior before implementation. It may be a canonical Target-State Doc update for currently true information, a Draft Target-State Doc, or a Plan Doc Delta.
_Avoid_: Treating every target-state artifact as current docs, putting durable behavior only in informal chat

**Draft Target-State Doc**:
A non-current draft under `docs/_drafts/` for substantial new durable docs that are likely to become canonical docs. Draft docs may describe proposed target state before implementation and are deleted after promotion, merge, abandonment, or explicit deferment.
_Avoid_: Canonical doc, permanent spec, unrelated scratchpad

**Doc Delta**:
A Plan section that records future changes needed for existing canonical docs. Doc Delta items use phase-tagged checkboxes and are resolved as implementation phases land.
_Avoid_: Separate doc-delta file, unchecked prose note, Plan reference inside canonical docs

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

**Plan Review Mode**:
A review mode for ephemeral Plan artifacts that checks whether the Plan is executable, coherent, and verifiable before implementation. It focuses on dependencies, task-to-acceptance coherence, prerequisites, locked-decision contradictions, cross-plan ownership, and verification strategy.
_Avoid_: Prose polishing, exhaustive edge-case review, architecture redesign, implementation review

**Implementation Review Mode**:
A review mode for code, tests, docs, and concrete implementation diffs. It uses Plans, Target-State Docs, acceptance criteria, and tests as reference truth, while judging the implemented behavior and changed surface.
_Avoid_: Treating attached Plans as the primary artifact when the user asked to review implementation

**Continuation Review**:
A repeated review round that focuses on accepted fixes, patch notes, changed surface, unresolved high-risk areas, and items that should not be relitigated. Same-context reviewers can infer some continuation context; fresh-context reviewers need it supplied through the prompt or a handoff.
_Avoid_: Cold broad re-review by accident, duplicate findings, widening scope without new evidence

**Review Ledger**:
A compact record of review round, mode, scope, Review Signal, accepted findings, rejected or deferred items, and whether another review is recommended.
_Avoid_: Long review transcript, unverified finding dump

**Review-Continuation Handoff**:
A handoff for a fresh-context reviewer to run a continuation or explicitly independent next review round. It carries the Review Ledger, prior findings, patch notes, rejected or deferred items, doc-lifecycle state when relevant, unresolved risks, and intended review mode. Ambiguous "fresh review" requests should be clarified instead of silently becoming continuation or independent review.
_Avoid_: Ambiguous "fresh review" prompt, defaulting every follow-up to a full re-review

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

- A **Plan** may be standalone or reference one or more related **Target-State Docs**, **Draft Target-State Docs**, or other target-state artifacts.
- Completed **Plans** should not be required by live docs or code after cleanup. Durable docs and code should not depend on Plan files, Plan phases, or Plan-only decisions.
- A **Scoped Doc Directory** groups **Target-State Docs** when a feature, app, or monorepo area is too large for a single doc.
- During planning, **Target-State Artifacts** capture durable intent without making canonical docs describe unimplemented behavior.
- **Doc Delta** is owned by the **Plan** and is resolved during implementation or final review; canonical docs should not reference Doc Delta items or Plan phases.
- A **Plan** may produce one or more **Handoffs** during implementation or review.
- **Implementation** and **Handoff** are separate workflows, even when both reference the same **Plan**.
- **Plan Review Mode** reviews the Plan itself; **Implementation Review Mode** reviews concrete implementation against the Plan and durable docs.
- **Continuation Review** is the default shape for repeated rounds unless the user explicitly asks for independent cross-validation.
- **Review Signal** helps decide whether another review round is worth running: repeated rounds should tighten the finding threshold and can close out before findings reach zero when remaining findings are low signal.
- A **Review Ledger** gives future same-context or fresh-context reviewers enough round state to avoid relitigating settled findings.
- A **Review-Continuation Handoff** uses the **Review Ledger** plus patch notes to make fresh-context continuation possible.
- **Feedback-Calibrated Test Discipline** is part of **Implementation**: test cost changes cadence, not the expectation that behavior changes get meaningful test or verification evidence.
- An **ADR** supplements a **Target-State Doc** or **Plan** only when the decision meets the ADR threshold.
- A **Glossary** should contain domain/workflow language only, not implementation details.
- A root **Glossary** is the default; scoped glossaries live under `docs/<scope>/GLOSSARY.md`; a **Glossary Map** exists only when discovery becomes hard.
- ADRs default to `docs/adr/`, with scoped ADRs under `docs/<scope>/adr/` only when the decision is clearly local to that scope.

## Example dialogue

> **Dev:** "Should this behavior live in the **Plan**, a **Draft Target-State Doc**, or a canonical **Target-State Doc**?"
> **Domain expert:** "If it is current durable truth, put it in the canonical doc. If it is future durable behavior, draft it or record a **Doc Delta** until implementation lands. If it only guides the work, keep it in the **Plan**."

## Flagged Ambiguities

- "spec" can imply spec-driven development or code-heavy permanent design docs. Resolved: use **Target-State Doc** as the canonical term; "doc" or "docs" may be shorthand in conversation.
