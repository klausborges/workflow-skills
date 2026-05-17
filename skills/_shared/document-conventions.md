# Document Conventions

Create directories lazily.

- `plans/`: ephemeral implementation Plans.
- `docs/`: durable Target-State Docs.
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

Target-State Docs include `Out of Scope` only when exclusions matter long-term.

Target-State Docs avoid code examples by default. Tiny snippets are allowed only for durable contracts such as public type shape, state machine table, event payload schema, or config shape.

Target-State Docs and code comments should not reference Plan files, Plan phases, or Plan-only decisions. They must stay coherent after a completed Plan is deleted.

Do not create `.out-of-scope/` by default. Use Plans, Target-State Docs, or ADRs.
