# Workflow Language

Use this compact hierarchy consistently:

- **Roadmap**: optional ephemeral progress/prioritization view over thin Plan links. A project may have one main Roadmap and multiple scoped Roadmaps. Ordering is `Loose`, `Strict`, or `Mixed`.
- **Plan**: ephemeral artifact for one implementation outcome. It owns optional Milestones, Phases, Tasks, acceptance criteria, verification, and Doc Delta.
- **Milestone**: optional related-Phase grouping inside a Plan.
- **Phase**: coherent implementation and verification scope. Every implementation Phase ends with an independent fresh-context review of the whole Phase scope.
- **Task**: concrete work inside a Phase.
- **Review checkpoint**: Plan-defined review after high-impact Tasks or Task groups, in addition to the end-of-Phase review.

Do not use **slice** as a hierarchy term. A vertical slice is only an implementation or testing technique.

Other artifacts:

- **Target-State Doc**: durable canonical truth about current behavior, concepts, or structure.
- **Draft Target-State Doc**: explicitly non-current proposed doc under `docs/_drafts/`.
- **Doc Delta**: phase-tagged Plan checkboxes for future changes to existing canonical docs.
- **Handoff**: bounded continuation prompt or file. Its lifecycle is **Immediate** or **Durable**; storage format does not determine durability.
- **ADR**: record for a surprising, trade-off-heavy, hard-to-reverse architectural decision.

Plans and Roadmaps are ephemeral even when retained temporarily. Canonical docs, source, code comments, commits, and PRs must remain coherent after they are removed and must not refer to workflow-relative identifiers such as `Phase 0`, `P0`, `Milestone 0`, or `M0`. Workflow artifacts may use the hierarchy terms.
