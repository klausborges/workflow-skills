---
name: write-handoff
description: Write a fresh-context handoff prompt or file. Use when the user asks for a handoff, prompt for another agent/model/tool, continuation brief, implementation brief, research brief, refactor brief, bugfix brief, review brief, or review-fix brief tied to a Plan, Docs, current work, or review findings.
---

# Write Handoff

Create a handoff that lets a fresh agent succeed without inheriting this context.

Use workflow terms from [workflow-language.md](references/workflow-language.md).
Use the Handoff template in [templates.md](references/templates.md).

## Handoff Types

- `Implementation Handoff`: implement selected Plan phase(s).
- `Continuation Handoff`: continue partially completed work with fresh context.
- `Research Handoff`: investigate options, prototype lightly if useful, and return a recommendation.
- `Refactor Handoff`: change implementation direction or clean up wrong work.
- `Bugfix Handoff`: fix a discovered issue.
- `Review Handoff`: review implementation with extra context.
- `Review-Fix Handoff`: address accepted review findings.
- `Review-Continuation Handoff`: run a continuation or explicitly independent next review round with fresh context.

Use the same base template for all types. Adjust `Task`, `Current State`, and `Completion Criteria`.

## Review Handoff Intent

Use when the handoff target is a reviewer.

- `delta`, `continuation`, or `another review`: write a Review-Continuation Handoff focused on accepted fixes, patch notes, changed surface, unresolved high-risk areas, and do-not-relitigate items.
- `independent` or `cross-validation`: write a review handoff for an explicitly independent pass, and label the expected cost and duplicate-finding risk.
- Ambiguous `fresh review` or `fresh context`: ask whether the user wants continuation/delta review or independent cross-validation before writing the handoff.
- If prior Review Signal was low and the user still asks for another review handoff, write it if requested, but label it optional/low ROI.

Review-Continuation Handoffs include:

- prior findings and patch notes
- accepted, rejected, and deferred items
- Review Signal, scope, round, and intended next review mode
- next focus and remaining risk surface
- do-not-relitigate items
- optional/low-ROI label when prior Review Signal was low
- Target-State Doc sync status, current-Plan Doc Delta status, and referenced draft-doc status when Plan-backed

## Durability

- `Immediate Handoff`: same-day/current-codebase continuation. Concrete files and current state are fine.
- `Durable Handoff`: later/unknown agent, or reusable prompt for repeated work across models/tools. Prefer behavior, acceptance criteria, and scope boundaries over brittle paths.

Default to Immediate unless the user asks for durable or the handoff is written to a file.

File handoffs default to Durable.

## Plan Updates

Do not add "handoff prepared" notes for ordinary chat handoffs.

Update the Plan only when tracked work changes:

- after partial implementation, check completed tasks/criteria already verified
- add required discovered tasks under the current phase
- add useful but non-required bugfix/refactor/review work to `Discovered Follow-Ups`
- for file handoffs, add a sparse pointer only if needed to avoid losing the file

## Quality Bar

- Include a `Read First` list with the Plan, related Docs, and key files.
- Do not repeat Docs/Plans except for facts that are extremely important or easy to miss.
- Include constraints, verification, and completion criteria.
- Be explicit about what is out of scope.
