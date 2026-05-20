# Document Conventions

Create directories lazily.

- `plans/`: ephemeral implementation Plans.
- `docs/`: durable Target-State Docs.
- `docs/_drafts/`: non-current Draft Target-State Docs for substantial new proposed docs.
- `docs/<scope>/`: scoped Target-State Docs when a feature/app/domain needs multiple docs.
- `GLOSSARY.md`: root Glossary.
- `docs/<scope>/GLOSSARY.md`: scoped Glossary.
- `GLOSSARY-MAP.md`: optional map only when multiple scoped glossaries make discovery hard.
- `docs/adr/`: default ADR location.
- `docs/<scope>/adr/`: scoped ADRs only when clearly local.

Plans may be standalone or include `Related Docs`.

Plans are temporary work artifacts. After implementation is complete, final review passes, and related Target-State Docs are current, ask the user whether to delete the completed Plan. Never delete a Plan automatically.

Users may commit Plans when useful for history, but live docs and code should not depend on completed Plans staying in the workspace.

Plans always include `Out of Scope`.

When durable docs are affected, Plans include a `Doc Delta` section. Use checkbox items under `Draft Docs` and `Existing Docs`. Each item carries an inline phase tag such as `(Phase 2)`; items without a tag default to the final implementation phase (the last numbered Phase N, not Final Review And Cleanup). A Doc Delta item is resolved when checked or removed as obsolete.

Referenced draft docs are draft paths listed under the current Plan's `Doc Delta` > `Draft Docs`. Unlisted files in `docs/_drafts/` are unrelated to the current Plan's closeout.

Canonical Target-State Docs under `docs/` should describe current durable truth. They may be updated during planning only for currently true clarifications, stale-current-doc fixes, stable background, or canonical terminology already valid today.

Draft Target-State Docs may describe proposed target state before implementation. Create them only for substantial new durable docs that are likely to become canonical docs. Do not use draft docs for small future edits to existing docs; use Plan Doc Delta instead.

Draft docs are pending work artifacts. During implementation, promote or merge referenced draft docs as behavior lands. Delete draft docs after promotion, merge, abandonment, or explicit deferment. Record abandonment or deferment as a checked `Draft Docs` Doc Delta item with an inline reason.

Target-State Docs include `Out of Scope` only when exclusions matter long-term.

Target-State Docs avoid code examples by default. Tiny snippets are allowed only for durable contracts such as public type shape, state machine table, event payload schema, or config shape.

Target-State Docs and code comments should not reference Plan files, Plan phases, Doc Delta items, or Plan-only decisions. They must stay coherent after a completed Plan is deleted.

Do not create `.out-of-scope/` by default. Use Plans, Target-State Docs, or ADRs.
