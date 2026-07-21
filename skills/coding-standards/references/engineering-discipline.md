# Engineering Discipline

Apply these as proportionate heuristics. Specialized stack guidance may refine them without silently overriding the repo.

When rules pull in different directions, decide in this order:

1. Correctness, safety, and debuggability.
2. Repo conventions for generic defaults; surface material conflicts with the current request with a recommendation rather than silently choosing.
3. These standards, applied to new code and to the full behavior being refactored.
4. Contain incompatible existing patterns at the nearest boundary rather than copying them into new code; leave unrelated old code unchanged unless a broader migration is authorized.
5. Record a deliberate exception in a comment or ADR when it is surprising, trade-off-heavy, or hard to reverse.

- Start from user- and caller-visible behavior. Model load-bearing data, types, invariants, and access patterns before adding logic; make invalid states unrepresentable: `{ completed: bool, completed_at?: time }` admits contradictory values, `Open | Done(at)` does not.
- Subtract before adding. Prefer direct flows and fewer concepts, branches, layers, and mutable states when they preserve clarity and behavior.
- When a distinction carries real invariants or lifecycle rules, keep valid construction and legal transitions with one canonical owner; callers must not recreate checks or mutate around it. Model genuinely closed variants exhaustively without forcing domain wrappers or state machines where simple data suffices.
- At external boundaries, derive or validate boundary types against the authoritative schema, parse untrusted input into valid internal data, and translate into caller/domain language when meaning or shape differs. Avoid mirror types without semantic purpose; retain raw input only for a named need.
- Split independent ownership before serializing shared state. Coordinate only around a real shared invariant.
- Build a script, codemod, harness, or other lever only when repetition, reruns, or verification difficulty repay its cost.
- For retryable or multi-step externally observable mutations, verify duplicate execution and interruption at visible boundaries. Use idempotency or an explicit duplicate/restart strategy; make related writes atomic or define reconciliation/compensation.
- Unless the user or repo declares a stable compatibility contract, treat code and APIs as evergreen: migrate controlled callers and remove the old path in the same wave, without preemptive aliases, shims, deprecation periods, or legacy paths. A public symbol alone is not such a contract.
- Evergreen permits deep or breaking design improvement; it does not lower correctness, stability, or verification. Preserve durable data: database, schema, queue, and persisted-format changes require an explicit verified migration. Use an atomic migration when sufficient; use expand-migrate-contract only for overlapping versions, staged backfills, or cutovers.
- Verify real behavior through the cheapest stable useful surface that proves it. Prefer affordable real-surface integration checks—for example, an HTTP/Hurl check backed by a real database—and property tests for parsers, state machines, invariants, or idempotency when they prove more than mock-heavy unit tests; keep focused unit tests when they add distinct signal. Revisit the model when flags, casts, branches, or patches accumulate.
- Module mocks and substantial new end-to-end test infrastructure require owner approval; they add coupling, maintenance, and execution cost.
- When the same correction recurs, encode the cheapest reliable guardrail without expanding the current task into unrelated tooling.
