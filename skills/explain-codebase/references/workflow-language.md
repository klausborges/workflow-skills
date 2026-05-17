# Workflow Language

Use these terms consistently.

**Plan**: ephemeral phased implementation artifact in `plans/`. After full implementation and final review, ask before deleting the completed Plan.
Avoid: spec, permanent documentation.

**Target-State Doc**: durable document under `docs/` describing behavior, concepts, or structure that should remain true after implementation.
Avoid: implementation plan, code dump, references to Plan files or phases.

**Glossary**: `GLOSSARY.md` or scoped `docs/<scope>/GLOSSARY.md`; canonical terms, relationships, and flagged ambiguities only.
Avoid: context dump, implementation notes.

**Handoff**: copyable prompt or file that gives a fresh agent enough context to continue, implement, research, refactor, fix, or review work.
Avoid: status summary.

**Durable Handoff**: handoff for later/unknown agents or repeated work across models/tools.

**Feedback-Calibrated Test Discipline**: choose the fastest meaningful feedback loop for each behavior change. Cheap tests use red/green vertical slices; slower UI/E2E checks use same-phase or acceptance-gate verification. Fallback requires an explicit reason and replacement evidence.
Avoid: blanket red/green TDD for heavy suites, skipping tests because they are expensive.

**ADR**: concise record of a hard-to-reverse, surprising, trade-off-driven decision.

**Research Ladder**: portable source preference for technical investigation.
