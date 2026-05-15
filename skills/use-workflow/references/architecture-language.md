# Architecture Language

Use this vocabulary when it clarifies planning, review, or architecture improvement.

**Module**: anything with an interface and an implementation.

**Interface**: everything a caller must know to use a module correctly, including invariants, ordering, error modes, configuration, and performance characteristics.

**Implementation**: code inside a module.

**Depth**: leverage at the interface. A deep module puts lots of behavior behind a small interface; a shallow module exposes nearly as much complexity as it hides.

**Seam**: a place where behavior can be altered without editing in that place.

**Adapter**: concrete thing satisfying an interface at a seam.

**Leverage**: what callers get from depth.

**Locality**: what maintainers get from depth: change, bugs, knowledge, and verification concentrated in one place.

Principles:

- The interface is the test surface.
- Use the deletion test: if deleting a module makes complexity vanish, it was pass-through; if complexity reappears across callers, it was earning its keep.
- One adapter means a hypothetical seam. Two adapters means a real seam.
- Do not expose internal seams just because tests use them.
