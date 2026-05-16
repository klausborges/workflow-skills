---
name: use-workflow
description: Lightweight umbrella workflow router. Use for substantial feature, implementation, handoff, review, diagnosis, explanation, architecture, or planning work in repos that follow this workflow; establishes vocabulary and routes to plan-work, implement-plan, write-handoff, review-work, diagnose-issue, explain-codebase, improve-architecture, or plan-prototype while honoring repo-local instructions.
---

# Use Workflow

Route substantial work to the right workflow skill without making the user name it.

Honor repo-local `AGENTS.md`, local docs, and project conventions over these defaults. Do not interrupt small direct tasks.

## Vocabulary

Use the repo's `GLOSSARY.md` when present. If missing, use the conventions in [workflow-language.md](references/workflow-language.md).

Key terms:

- **Plan**: ephemeral phased implementation artifact in `plans/`.
- **Target-State Doc**: durable doc under `docs/`.
- **Handoff**: fresh-context prompt or file.
- **Glossary**: canonical terms only, not a context dump.
- **ADR**: rare decision record for hard-to-reverse, surprising trade-offs.

## Routing

- "Explain this code", "how does this work", codebase map, feature/module/flow orientation -> `explain-codebase`
- Bug, failing test, build failure, flaky behavior, unexpected output, performance regression, or "debug/diagnose this" -> `diagnose-issue`
- Fuzzy idea, new feature, architecture/product/workflow change, or multi-step task -> `plan-work`
- Approved Plan, "implement phase N", or planned checklist execution -> `implement-plan`
- Patch accepted review findings -> direct scoped implementation; use `implement-plan` when the review fix belongs to an existing Plan.
- Prompt for another agent/model/tool, continuation brief, or "handoff" -> `write-handoff`
- "Review this", adversarial review, multi-aspect review, test/library/performance/security review -> `review-work`
- "Improve architecture", "find deepening opportunities", hard-to-test/scattered/shallow code, architecture refactor -> `improve-architecture`
- Mockups, option visuals, diagrams, throwaway logic/state/API prototypes -> `plan-prototype`

If several apply, choose the most specific skill. If the user explicitly names a skill, use that skill.

## Defaults

- Plan before nontrivial implementation.
- Use the Research Ladder from [research-ladder.md](references/research-ladder.md) before serious technical recommendations.
- Create durable docs only when information should remain useful after the Plan is deleted.
- Keep handoffs first-class; do not treat them as implementation summaries.
- After review, prefer the user's requested follow-up: patch accepted findings, route Plan-backed fixes through `implement-plan`, or write a handoff only when transferring context.
- Do not commit, create worktrees, publish issues, or start implementation unless requested or clearly part of the repo's local workflow.
