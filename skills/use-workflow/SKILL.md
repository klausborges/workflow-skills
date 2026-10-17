---
name: use-workflow
description: The owner's work loop and its routes. Use when orchestrating a slice, a stack or a project takeover, when choosing a route, or when asked how to use these skills for something.
---

# Use workflow

Read [workflow-language.md](references/workflow-language.md) when a term below needs its exact meaning. A request to explain how to use the skills gets the exact next prompt and starts nothing.

## The loop

Per brief:

- **1 Brief** (write-handoff). On the slice route, two model lines review it blind before the first PR, Plan or not.
- **2 Implement** (implement-work), in a lane.
- **3 Review** (review-work): a review pair, blind.
- **4 Lean pass** (improve-architecture), sized there.
- **7 Close**, by the main session: acceptance reconciled with the brief; the gate run on the final head, with base, head, host and executed count; the unqualified surface named; a resumption record written.

Once per stack, on the top branch, before it is offered to merge:

- **5 Comments pass**.
- **6 Fresh read** (review-work).

## Routes

- **Slice**: every step.
- **Lighter**: one reviewer and the gate, for a change that stays in one module and touches no shared contract, no stored data and no new dependency.
- **Takeover**: read-only fan-out (explain-codebase); findings and rulings written to the plan surface; no code changes.
- **Explain**: the next prompt; start nothing.

The route is proposed in the plan or brief, and the owner's go-ahead on it covers it. It is never downgraded mid-loop. Lean-pass and fresh-read findings go to the owner, who reads them before any fix goes back through step 3. Comments-pass changes go straight back through step 3. An authorization not given is recorded as pending, never inferred.

## Roles

A lane owns one brief. Reviewers don't approve their own work or decide product. The owner approves anything that can't be undone, and merges.

## Project slots

A project's instructions fill these: the gate command and its host, the plan surface, the actions that need the owner's go-ahead, the stack tool, the surfaces that stay unqualified, and who runs the gate (the main session by default).

## Gotchas

- A compaction or an account crossover leaves stale state: re-check heads and PR state live before acting.
- Parallel full gates on one host time out falsely: gate one head at a time.
- Day work quietly drops steps 4 to 6: name the route in the brief.
- A locked decision that disagrees with the code is a finding for the owner, not a doc edit.
- A decision taken while the owner is away is logged as "default taken" for confirmation.
- A reviewer's suggestion can be wrong: check it before relaying it into a fix brief.
