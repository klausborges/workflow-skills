# Templates

## Design Brief

```md
## Design Brief

### Goal

What we are trying to achieve.

### Target State

User/system behavior that should be true after implementation.

### Approach

Recommended approach and why.

### Key Decisions

Important decisions already made, including rejected alternatives if useful.

### Constraints

Technical, product, workflow, or compatibility constraints.

### Acceptance Criteria

High-level criteria the final Plan phases must cover.

### Docs Needed

Whether Target-State Docs or ADRs should be created/updated.

### Open Risks

Known uncertainty or follow-up research.
```

## Target-State Doc

```md
# Feature or System Name

## Purpose

Why this exists.

## Target State

What should be true after implementation.

## Key Concepts

Durable behavior or concept details that exceed Glossary definitions.

## User/System Flows

Important flows, states, or interactions.

## Constraints

Durable technical, product, workflow, or compatibility constraints.

## Out of Scope

Only when long-term exclusions matter.
```

## Plan

```md
# Name

## Goal

One sentence describing the outcome.

## Related Docs

- Optional links to Target-State Docs.

## Phase Checklist

- [ ] Phase 1: Name

## Verification Strategy

- Targeted tests:
- Integration/UI checks:
- Type/lint/build:
- Manual checks:
- Final review triggers:

## Out of Scope

- Work not included in this Plan.

## Research Notes

- Source: finding -> decision or constraint.

## Phase 1: Name

Mode: AFK | HITL | Mixed

### Goal

Brief outcome for this phase.

### Context

Related docs, assumptions, constraints, and prior decisions.

### Tasks

- [ ] Implement or change one concrete part of the system.
- [ ] Add or update relevant tests/checks.

### Acceptance Criteria

- [ ] Observable behavior or developer-facing contract is true.
- [ ] Relevant verification passes.
- [ ] Implementation self-review completed.
- [ ] Plan checklist is updated with completed work and newly discovered tasks.

### Notes

Concise load-bearing implementation details only.

## Discovered Follow-Ups

- Useful but non-required work discovered during implementation.
```

## Handoff

```md
# Handoff: Goal

Type: Implementation | Continuation | Research | Refactor | Bugfix | Review | Review-Fix
Durability: Immediate | Durable

## Read First

- `plans/<plan>.md`
- `docs/<related-doc>.md`
- Relevant source/test files

## Current State

What has already happened, including completed phases/tasks.

## Task

Exactly what the next agent should do.

## Constraints

What must not change, decisions already made, style/test constraints.

## Implementation Notes

Only context not obvious from the plan/docs/code.

## Verification

Commands and manual checks expected.

## Completion Criteria

What must be true before the agent reports done.

## Out of Scope

Adjacent work not included.
```

## Phase Self-Review

```md
## Phase Self-Review

Status: PASS | FIXED | NEEDS_HITL

### Findings

- [severity] Finding, with file/test evidence.

### Plan Updates

- Tasks checked:
- Acceptance criteria checked:
- Tasks added/moved:

### Verification

- `command`: result

### Recommendation

Only needed for NEEDS_HITL.
```
