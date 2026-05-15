---
name: plan-prototype
description: Build or plan a throwaway prototype to answer a planning question. Use when the user wants mockups, UI variants, diagrams, option canvases, interaction flows, logic/state/data/API prototypes, or to try a design before committing to implementation.
---

# Plan Prototype

Use throwaway prototypes to answer one planning question before committing to implementation.

Use workflow terms from [workflow-language.md](references/workflow-language.md).

Prototype output should be deleted or absorbed once it has answered the question.

## Pick The Mode

- Visual prototype/mockup: UI variants, diagrams, option canvases, interaction flows.
- Logic prototype: state/data/API model exploration, often as a tiny interactive terminal app.

Ask if the mode is unclear.

## Rules

- State the question the prototype answers.
- Mark prototype code clearly as throwaway.
- Prefer existing project tooling.
- Do not add polish, persistence, or tests unless the question requires it.
- Surface state/variants clearly so the user can inspect behavior.
- Capture the answer in the Plan, Target-State Doc, ADR, or implementation before deleting or absorbing the prototype.

## Visual Prototype

Prefer several structurally different variants, not color/copy tweaks.

When possible, put variants inside an existing page/flow so they run against real density, data, navigation, and constraints.

Use a simple switcher or URL parameter so variants are easy to compare and share.

## Logic Prototype

Use for state machines, data models, API shape, and business logic that needs to be felt through examples.

Keep the core logic behind a small portable interface. The TUI/shell is throwaway; the validated logic shape may inform real code later.

Prefer in-memory state. Avoid real persistence unless persistence is the question.
