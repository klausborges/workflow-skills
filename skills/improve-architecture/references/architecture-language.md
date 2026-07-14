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

- A public or intentionally stable interface is the primary behavioral test surface; test internals only when they carry distinct risk.
- Use the deletion test: if removing a module merely spreads its complexity across callers, it was earning its keep.
- One adapter may still justify a seam for volatility, ownership, or testing isolation. Ask what variation or boundary the seam protects.
- Do not expose internal seams only for tests.
