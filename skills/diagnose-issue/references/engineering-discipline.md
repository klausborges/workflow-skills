# Engineering discipline

Read before designing, testing or reporting on code, and when judging someone else's.

When rules conflict, decide in this order:

1. Correctness, safety and debuggability.
2. The repo's conventions.
3. These standards, for new code and for the full behavior being refactored.
4. Contain an incompatible existing pattern at its nearest boundary instead of copying it into new code.

## Design

- Make invalid states unrepresentable: `{ completed: bool, completed_at?: time }` admits contradictions, `Open | Done(at)` does not.
- Preserve evidence the code already established, parse truly unknown input once at its boundary, and never fabricate validity with an assertion or a cast.
- An invariant has one owner that constructs valid values and makes legal transitions; callers don't re-check it or mutate around it.
- Code and APIs are evergreen unless the repo declares a compatibility contract: migrate the callers you control and delete the old path in the same change, with no aliases or shims. Stored data is protected: a schema, queue or persisted-format change ships with a verified migration.

## Tests

- A test proves behavior, not a count: choose scenarios a plausible wrong implementation fails, and assert exact values.
- Expected values come from the requirement or a hand calculation, never copied from the implementation.
- Prefer the real path through real layers to mocks of internal code.
- A reviewer's probe is an investigation, not a demand for a permanent test.
- Timing tests order by observable transitions, not sleeps, and never loosen a production timeout.
- A new lint, type or test suppression, a module mock, or substantial new end-to-end infrastructure needs the owner's go-ahead. Propose it in the report and carry on with the rest.

## Evidence

- The real check is the project's suite, green as the repo defines it. A syntax-only run or a check that failed to start does not count; when no real check can run, say which and why.
- A green gate names base, head, host and executed test count. Zero executed tests proves nothing.
- Keep the first failure verbatim; a passing rerun does not replace it.
- Check a dependency's guarantee in its pinned source.
- Reports keep ran, inferred and unverified apart, and implemented, reviewed, pushed, merged and verified apart.
