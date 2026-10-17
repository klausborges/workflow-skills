# Architecture language

Read when naming a finding or comparing interface options.

- **Module**: an interface plus its implementation. The **interface** is everything a caller must know: invariants, ordering, errors, configuration.
- **Depth**: behavior hidden behind a small interface. Deletion test: if removing a module only spreads its complexity across callers, it is deep; if removing it removes complexity, it was a pass-through.
- **Seam**: where behavior varies without editing the caller; an **adapter** plugs into it. One adapter can justify a seam that isolates volatility or ownership; a seam that exists only for tests does not.
- **Locality**: change, knowledge, bugs and verification concentrated in one place.
