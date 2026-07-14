---
name: use-workflow
description: Lightweight workflow router. Use for substantial planning, implementation, handoff, review, diagnosis, explanation, architecture discovery, or prototyping work in repos that follow this workflow; honor explicit skill requests and repo-local instructions.
---

# Use Workflow

Route work to the smallest fitting workflow. Honor repo-local instructions and explicit user requests. Do not turn a clear low-risk task into a project.

Use the repo Glossary when present; otherwise use [workflow-language.md](references/workflow-language.md).

## Routing

- Explain a code area or flow -> `explain-codebase`
- Diagnose a bug, failure, flake, unexpected output, or regression -> `diagnose-issue`
- Explicit planning request, or material ambiguity, coordination, cross-cutting scope, irreversibility, or risk -> `plan-work`
- Execute an approved Plan -> `implement-plan`
- Write a continuation prompt or file -> `write-handoff`
- Review a Plan, change, round of fixes, or non-code artifact -> `review-work`
- Discover architecture improvement opportunities -> `improve-architecture`
- Answer a design question with a disposable mockup or logic prototype -> `plan-prototype`

A task being new, multi-step, or nontrivial is not by itself a reason to require planning. When the requested implementation is clear and bounded, implement directly.

Concrete architecture review of a change, PR, or Phase belongs to `review-work`. An explicitly decided scoped refactor may be implemented directly or through `implement-plan`; use `improve-architecture` when the opportunity or interface shape still needs discovery.

If several skills apply, choose the most specific. If the user names a skill, use it.

## Cost and authority

- Match questions, research, tools, artifacts, delegation, and verification to actual uncertainty and risk.
- Consolidate owner questions and include a recommendation; do not interview for facts the repo can answer.
- Research only gaps that can materially change the result, starting with local and first-party evidence.
- Review and diagnosis are read-only unless the user explicitly asks for fixes or implementation.
- Do not create a Plan, Roadmap, durable doc, handoff file, worktree, commit, PR, or issue unless requested or naturally required by the authorized workflow.
- Every implementation Phase ends with an independent fresh-context review of the whole Phase scope. A direct implementation without a Plan is one Phase-equivalent review scope and receives the same review before delivery.
- Plans may add review checkpoints after high-impact Tasks or Task groups. Low-impact Tasks do not need individual immediate reviews.
- Do not recursively review a review. Verify delegated claims against primary evidence and stop when the requested scope is closed.

Keep workflow state in workflow artifacts. Durable delivery artifacts must not depend on Plan or Roadmap files or workflow-relative identifiers.
