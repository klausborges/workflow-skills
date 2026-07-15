---
name: explain-codebase
description: Explain how an unfamiliar codebase area fits together by mapping its purpose, entry points, modules, callers, flows, tests, and important terms. Read-only.
---

# Explain Codebase

Map the requested code area without changing it.

## Process

1. Read repo instructions and relevant current docs, Glossary, or ADRs.
2. Infer the smallest useful scope when the user did not name one.
3. Inspect entry points, key modules, callers, tests, and important data/control flow.
4. Read relevant ADRs before using history. Inspect targeted history only when the user asks or current evidence cannot explain a consequential design choice.
5. Explain ownership and flow at the user's level; distinguish verified evidence, inference, and unknowns.

Prefer a compact shape:

- Purpose
- Entry points and key modules
- Caller/data/control flow
- Relevant tests or checks
- Important terms
- Unknowns or assumptions

Use focused file references, not a search dump. Do not edit, diagnose, review, or refactor unless the user switches workflows. If evidence reveals a likely bug or architecture friction, note it briefly and recommend `diagnose-issue` or `improve-architecture` as a separate next step.
