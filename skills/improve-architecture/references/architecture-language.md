# Architecture Language

Use these terms only when they clarify a real design decision.

- **Module**: interface plus implementation.
- **Interface**: what a caller must know, including invariants, ordering, errors, configuration, and relevant performance behavior.
- **Depth**: useful behavior hidden behind a smaller interface.
- **Seam**: place where behavior can vary without editing the caller.
- **Adapter**: concrete implementation at a seam.
- **Leverage**: capability callers gain from depth.
- **Locality**: change, knowledge, bugs, and verification concentrated in one place.

Heuristics, not laws:

- Stress-test load-bearing terms, relationships, and invariants with one or two concrete scenarios against current code/docs.
- Start from caller-visible usage and sketch only the call, component, data, or control flow needed to resolve the decision, including relevant effects and the stable proof surface.
- Minimize reader load: reduce the layers a change must trace and the hidden or mutable state it must remember.
- Prefer separate ownership before coordinating shared state; serialize only around a real shared invariant.
- Parse untrusted input at the boundary into valid internal data, preserving raw input only for a named need.
- A public or intentionally stable interface is the primary behavioral test surface; test internals only when they carry distinct risk.
- Use the deletion test: if removing a module merely spreads its complexity across callers, it was earning its keep.
- One adapter may still justify a seam for volatility, ownership, or testing isolation. Ask what variation or boundary the seam protects.
- Do not expose internal seams only for tests.
