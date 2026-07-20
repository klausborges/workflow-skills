# Document Conventions

Create directories only when needed.

- `plans/roadmap.md`: optional main Roadmap.
- `plans/<scope>-roadmap.md`: optional scoped Roadmap; use a subdirectory only when it improves navigation.
- `plans/<name>.md`: ephemeral Plan.
- `docs/`: current durable Target-State Docs.
- `docs/_drafts/`: explicitly non-current Draft Target-State Docs.
- `GLOSSARY.md` or `docs/<scope>/GLOSSARY.md`: canonical language.
- `docs/adr/` or, when clearly local, `docs/<scope>/adr/`: ADRs.

Roadmaps contain a checkbox, Plan link, and one-line outcome per active Plan. They may also list one-line candidates and cross-Plan dependencies. They do not repeat Plan milestones, phases, tasks, acceptance criteria, or research. A Plan may appear in multiple Roadmaps and stays authoritative.

For a completed Plan not linked from an active Roadmap, ask once after final review and doc sync whether to delete it. An active Roadmap gates linked-Plan cleanup: when work-tracking mutation is authorized, mark the Plan and Roadmap item complete; otherwise report the required update. Ask once about the Roadmap and completed Plans when the Roadmap closes. Never delete automatically. If a Plan is deleted, replace its Roadmap link with a checked plain-text one-line outcome.

Canonical Target-State Docs describe current durable truth. During planning, update them only for currently true clarifications or corrections. Use a Draft Target-State Doc for a substantial proposed new doc; use the Plan's Doc Delta for future changes to existing docs. Every Doc Delta item must name its Phase explicitly.

Resolve relevant draft docs and Doc Delta items as their Phase lands. Sync docs at Phase closeout or handoff; update earlier only when downstream work needs the contract.

Target-State Docs include `Out of Scope` only when exclusions matter durably and avoid code examples except for stable contracts that prose cannot express clearly.
