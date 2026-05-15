---
name: explain-codebase
description: Explain how an unfamiliar codebase area fits together. Use when the user asks to understand code, map a flow, explain relevant modules, callers, entry points, tests, or how a feature area works.
---

# Explain Codebase

Map an unfamiliar codebase area without changing it.

Use workflow terms from [workflow-language.md](references/workflow-language.md).
Use repo Glossary and ADRs when present.

## When To Use

- The user asks "explain this area", "how does this work", or "how does this fit together".
- The user wants a map of a feature, module, flow, call path, or subsystem.
- The user is preparing for planning, review, debugging, handoff, or architecture work and needs orientation first.

Do not use this as a substitute for `improve-architecture`; this skill explains first and recommends deeper architecture work only when concrete friction is visible.

## Process

1. Read repo-local instructions and relevant docs.
2. Identify the requested area or infer the smallest useful scope.
3. Inspect entry points, key modules, callers, tests, and important data/control flow.
4. Use project vocabulary from Glossary/docs when available.
5. Explain the map at the right level of abstraction.
6. List unknowns or assumptions separately.

## Output Shape

Prefer concise sections:

- **Purpose**: what this area is for.
- **Entry Points**: commands, routes, handlers, public functions, or user flows that enter it.
- **Key Modules**: the important files/modules and what each owns.
- **Callers And Flow**: how control/data moves through the area.
- **Tests And Checks**: useful tests or verification commands, if present.
- **Terms**: glossary/domain terms that matter.
- **Unknowns**: unclear parts, missing docs, or assumptions.

Use file references when helpful, but do not dump every file touched by a search.

## Guardrails

- Do not edit files.
- Do not start implementation, review, diagnosis, or refactoring unless the user asks to switch workflows.
- Do not propose broad refactors as the main output.
- If concrete architecture friction appears, name it briefly and suggest `improve-architecture` as a next step.
- If the explanation reveals a bug or failing behavior, suggest `diagnose-issue` as a next step.
