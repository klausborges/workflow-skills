# Workflow Language

This glossary is the canonical vocabulary for the workflow skills.

## Work hierarchy

**Roadmap**: An ephemeral progress and prioritization view over thin links to Plans. A project may have a main Roadmap and any number of scoped Roadmaps. Ordering is `Loose`, `Strict`, or `Mixed`.

`Loose` order is advisory. `Strict` makes the earliest incomplete Plan the only eligible Plan, with no override; reorder or restructure the Roadmap before proceeding otherwise. `Mixed` enforces named cross-Plan blockers while other unblocked Plans remain independently eligible. Ordering selects Plans, not Phases, and never authorizes work. Dependencies that must hold across Roadmaps belong in named cross-Plan constraints, not local list order.

**Plan**: An ephemeral implementation artifact for one outcome. A Plan owns its milestones, phases, tasks, acceptance criteria, verification strategy, and Doc Delta. It may appear in more than one Roadmap and remains authoritative there.

**Milestone**: An optional grouping of related Phases inside a Plan. Use it only when it improves navigation or marks a meaningful integration boundary.

**Phase**: A coherent implementation and verification scope inside a Plan. Every implementation Phase closes with an independent fresh-context review of the whole Phase scope.

**Task**: A concrete unit of work inside a Phase.

**Review checkpoint**: A Plan-defined independent review after a high-impact Task or related Task group. Low-impact Tasks do not require individual review. This is separate from the mandatory end-of-Phase review.

Do not use **slice** as a workflow hierarchy term. A vertical slice is only an implementation or testing technique.

## Documentation and handoffs

**Target-State Doc**: A durable canonical document describing behavior, concepts, or structure that is true now. It must remain coherent after workflow artifacts are removed.

**Target-State Artifact**: Planning-time material that captures durable intent: a current-doc correction, Draft Target-State Doc, or Plan Doc Delta.

**Draft Target-State Doc**: A clearly non-current draft under `docs/_drafts/` for a substantial proposed durable document. Promote, merge, defer, abandon, or delete it when its owning work closes.

**Doc Delta**: Phase-tagged Plan checkboxes for future changes to existing canonical docs. Items without an explicit Phase are invalid.

**Handoff**: A copyable prompt or file that gives another agent enough context to continue a bounded task.

**Immediate handoff**: A handoff for the next known worker or near-term continuation. It may be stored in chat or a file.

**Durable handoff**: A reusable or later-use handoff for an unknown worker or tool. It favors stable behavior, acceptance criteria, and scope over volatile status and paths. Storage format does not determine durability.

**ADR**: A concise record of a surprising, trade-off-heavy, hard-to-reverse architectural decision.

**Glossary**: A durable language document containing canonical terms and relationships, not implementation notes.

## Review

**Plan Review Mode**: Read-only review of whether a Plan is coherent, executable, scoped, and verifiable.

**Implementation Review Mode**: Read-only review of code, tests, docs, or another concrete change against its intended behavior and relevant Plans or docs.

**Continuation Review**: A later round focused on accepted fixes, changed surface, unresolved risks, and settled items that should not be relitigated.

**Review Signal**: The actionable verified value remaining: `High` blocks closeout, `Medium` is valid but localized, and `Low` is minor, speculative, deferred, or better routed elsewhere.

**Review Ledger**: A compact review record. Its core is scope, verification, Review Signal, recommendation, and residual risk. Add Plan, doc, round, or reconciliation fields only when applicable.

**Review-continuation handoff**: A fresh-context handoff carrying the prior Review Ledger, accepted findings, patch notes, settled items, unresolved risks, and intended next review scope.

## Supporting language

**Research Ladder**: A source preference that starts with cheap local and first-party evidence, then escalates only when consequential uncertainty remains.

**Feedback-calibrated test discipline**: Match verification cadence to feedback cost. Cheap behavior uses a narrow red/green loop; expensive behavior may use same-Phase or acceptance-gate evidence.

## Lifecycle boundaries

- Plans and Roadmaps are ephemeral even when retained temporarily or committed for coordination.
- A completed Plan outside an active Roadmap may be offered for deletion once after final review and doc sync. Never delete it automatically.
- An active Roadmap gates cleanup of linked Plans. Record completion in the Plan and Roadmap through an authorized workflow step, keep the link, and ask once about cleanup when that Roadmap closes.
- If a Plan is deleted, replace its Roadmap link with a checked plain-text one-line outcome.
- Canonical docs, source code, code comments, commit messages, and PR titles or bodies must not depend on workflow files or identifiers such as `Phase 0`, `P0`, `Milestone 0`, or `M0`. Describe the behavior, component, or outcome instead.
- Roadmaps, Plans, Handoffs, and Review Ledgers may use workflow hierarchy terms.
- Review and diagnosis are read-only unless the user explicitly authorizes fixes or implementation.
