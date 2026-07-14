# Planning Templates

Use only the sections the work needs.

## Design Brief

```md
## Design Brief

### Goal and target state

### Recommended approach

### Decisions and constraints

### Acceptance criteria

### Docs impact

### Open questions or risks
```

## Current Target-State Doc

Canonical docs describe current truth, not proposed behavior.

```md
# Name

## Purpose

## Current behavior and contracts

## Key concepts or flows

## Constraints

## Out of scope
```

## Roadmap

```md
# Roadmap: Scope

Ordering: Loose | Strict | Mixed

## Plans

- [ ] [Plan outcome](./plan.md) — one-line result

## Candidates

- One-line idea not yet shaped into a Plan.

## Cross-plan constraints

- Only dependencies or shared constraints that affect ordering.
```

## Plan

```md
# Outcome

## Goal

## Related docs

## Phase checklist

- [ ] Phase 1: Outcome

## Verification strategy

## Out of scope

## Phase 1: Outcome

### Goal

### Tasks

- [ ] Concrete change.
- [ ] Relevant verification.

### Acceptance criteria

- [ ] Observable behavior or contract is true.
- [ ] Phase verification passes.
- [ ] Relevant docs and Doc Delta items are current.
- [ ] Independent fresh-context review covers the full Phase scope.

### Review checkpoints

- Optional checkpoints after high-impact Tasks or Task groups.

## Doc Delta

Include only when existing durable docs need future changes.

- [ ] (Phase N) `docs/path.md`: required current-truth update.

## Discovered follow-ups

- Useful work outside this Plan's required scope.
```

Milestones are optional headings that group related Phases. Keep research notes only when they carry a decision, constraint, or source needed to execute the Plan.
