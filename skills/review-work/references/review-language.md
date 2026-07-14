# Review Language

- **Plan Review Mode**: review whether a Plan is coherent, executable, scoped, and verifiable.
- **Implementation Review Mode**: review a concrete change against intended behavior and relevant Plans, docs, criteria, and tests.
- **Continuation Review**: focus a later round on accepted fixes, changed surface, unresolved risks, and settled items that should not be relitigated.
- **Review Signal**: actionable verified value remaining. `High` blocks closeout; `Medium` is valid but localized; `Low` is minor, speculative, deferred, or better routed elsewhere.
- **Review Ledger**: compact review record. Core fields are scope, verification, Review Signal, recommendation, and residual risk. Add Plan, doc, round, or reconciliation fields only when they apply.
- **Review-continuation handoff**: fresh-context prompt carrying the prior Review Ledger, accepted findings, patch notes, settled items, unresolved risks, and intended next scope.

Review is read-only unless the user explicitly asks for fixes or implementation. There is no finding minimum. Use extra lenses, agents, or rounds only when risk, uncertainty, or the user request justifies them.
