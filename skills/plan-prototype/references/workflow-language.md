# Workflow Language

Use these terms consistently.

**Plan**: ephemeral phased implementation artifact in `plans/`. After full implementation and final review, ask before deleting the completed Plan.
Avoid: spec, permanent documentation.

**Target-State Doc**: canonical durable document under `docs/` describing current behavior, concepts, or structure that should remain true after implementation.
Avoid: implementation plan, code dump, references to Plan files or phases, future-only behavior during planning.

**Target-State Artifact**: planning-time artifact that captures desired durable behavior before implementation. It may be a current canonical doc update, a Draft Target-State Doc, or Plan Doc Delta.
Avoid: treating every artifact as current docs.

**Draft Target-State Doc**: non-current draft under `docs/_drafts/` for substantial new durable docs likely to become canonical docs.
Avoid: permanent spec, small existing-doc edit.

**Doc Delta**: Plan section recording future changes needed for existing canonical docs. Items are phase-tagged checkboxes resolved as implementation phases land.
Avoid: separate doc-delta file, unchecked prose note, Plan reference inside canonical docs.

**Glossary**: `GLOSSARY.md` or scoped `docs/<scope>/GLOSSARY.md`; canonical terms, relationships, and flagged ambiguities only.
Avoid: context dump, implementation notes.

**Handoff**: copyable prompt or file that gives a fresh agent enough context to continue, implement, research, refactor, fix, or review work.
Avoid: status summary.

**Durable Handoff**: handoff for later/unknown agents or repeated work across models/tools.

**Review Signal**: actionable, verified review value remaining in a review round. High signal blocks closeout or materially affects correctness, docs, tests, verification, regression risk, or user-visible behavior. Medium signal is valid but localized and patchable. Low signal is minor, speculative, stylistic, already-deferred, or better routed to follow-up or architecture work.
Avoid: treating every nit as a reason to keep reviewing, stopping before blockers are resolved.

**Plan Review Mode**: review mode for ephemeral Plan artifacts. Checks whether implementation can start safely and verifiably: dependencies, task-to-acceptance coherence, prerequisites, locked-decision contradictions, cross-plan ownership, and verification strategy.
Avoid: prose polishing, exhaustive edge-case review, architecture redesign, implementation review.

**Implementation Review Mode**: review mode for code, tests, docs, and concrete diffs. Uses Plans, Target-State Docs, acceptance criteria, tests, and changed files as evidence.
Avoid: treating an attached Plan as the primary artifact when the user asked to review implementation.

**Continuation Review**: repeated review round focused on accepted fixes, patch notes, changed surface, unresolved high-risk areas, and items that should not be relitigated. Same-context reviewers can infer some state; fresh-context reviewers need it supplied.
Avoid: accidental cold broad re-review, duplicate findings, widening scope without new evidence.

**Review Ledger**: compact review record with round, mode, scope, Review Signal, accepted findings, rejected or deferred items, and whether another review is recommended.
Avoid: long transcript, unverified finding dump.

**Review-Continuation Handoff**: handoff for a fresh-context reviewer to run a continuation or explicitly independent next review round. It carries the Review Ledger, prior findings, patch notes, rejected or deferred items, doc-lifecycle state when relevant, unresolved risks, and intended review mode. Ambiguous "fresh review" requests should trigger a clarifying question instead of silently choosing continuation or independent review.
Avoid: ambiguous fresh-review prompt, defaulting every follow-up to full re-review.

**Feedback-Calibrated Test Discipline**: choose the fastest meaningful feedback loop for each behavior change. Cheap tests use red/green vertical slices; slower UI/E2E checks use same-phase or acceptance-gate verification. Fallback requires an explicit reason and replacement evidence.
Avoid: blanket red/green TDD for heavy suites, skipping tests because they are expensive.

**ADR**: concise record of a hard-to-reverse, surprising, trade-off-driven decision.

**Research Ladder**: portable source preference for technical investigation.
