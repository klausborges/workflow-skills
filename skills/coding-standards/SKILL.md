---
name: coding-standards
description: Generic engineering standards for designing, implementing, refactoring, or reviewing code. Use when deciding how to structure or verify new code, when a standards question comes up, or when another skill needs the user's coding standards.
---

# Coding Standards

Canonical standards live in [engineering-discipline.md](references/engineering-discipline.md). Read it and apply the rules as proportionate heuristics, not laws.

## Applying the standards

- Apply the standards to new code and to the full behavior being refactored. Contain incompatible existing patterns at the nearest boundary rather than copying them into new code; leave unrelated old code unchanged.
- Stack-specific standards live in sibling skills that link back here. When none exists, apply the generic standards in the stack's idiom.
- Record a deliberate exception in a comment or ADR only when it is surprising, trade-off-heavy, or hard to reverse.
