---
name: diagnose-issue
description: Find the cause of a bug, failing test, flaky check or regression, read-only unless a fix is authorized. Use when something fails and the cause is unknown.
---

# Diagnose issue

Read [engineering-discipline.md](references/engineering-discipline.md) for the evidence rules, including the first failure.

Reproduce first. A passing retry explains nothing. Measure a flaky failure's rate over repeated runs, before and after.

Fix only when authorized, with a regression test seen failing without the fix. Report instead of making these calls: a cause that implies a design change, a fix that touches stored data or a shared contract, a test you would have to weaken.

After two failed attempts at the same cause, stop patching and report what you learned.

## Gotchas

- A mutation can miss its target: a mutation red counts only when a diff against pristine shows it landed and the run reached it.
- An unexplained cause invites a story: write it as unknown.
