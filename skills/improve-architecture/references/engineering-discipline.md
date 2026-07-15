# Engineering Discipline

Apply these as proportionate heuristics. Repo conventions govern; specialized stack guidance may refine them without silently overriding the repo.

- Start from user- and caller-visible behavior. Model load-bearing data, types, invariants, and access patterns before adding logic.
- Subtract before adding. Prefer direct flows and fewer concepts, branches, layers, and mutable states when they preserve clarity and behavior.
- Keep canonical ownership local. Parse untrusted input into valid internal data at boundaries, retaining raw input only for a named need.
- Split independent ownership before serializing shared state. Coordinate only around a real shared invariant.
- Build a script, codemod, harness, or other lever only when repetition, reruns, or verification difficulty repay its cost.
- Make retryable externally observable mutations idempotent, or define an explicit duplicate/restart strategy.
- For repo-owned internal APIs, migrate callers and remove the old path in the same wave only when compatibility is unnecessary.
- Verify real behavior through the most stable useful surface. Revisit the model when flags, casts, branches, or patches accumulate.
- When the same correction recurs, encode the cheapest reliable guardrail without expanding the current task into unrelated tooling.
