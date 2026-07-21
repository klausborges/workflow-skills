---
name: use-workflow
description: Lightweight workflow router. Use for substantial planning, implementation, simplification, handoff, review, diagnosis, explanation, architecture discovery, or prototyping work in repos that follow this workflow; honor explicit skill requests and repo-local instructions.
---

# Use Workflow

Route work to the smallest fitting workflow. Honor repo-local instructions and explicit user requests. When they materially conflict, surface the conflict with a recommendation rather than silently choosing. Do not turn a clear low-risk task into a project.

Use the repo Glossary when present; otherwise use [workflow-language.md](references/workflow-language.md).

## Routing

- Explain a code area or flow -> `explain-codebase`
- Diagnose a bug, failure, flake, unexpected output, or regression -> `diagnose-issue`
- Explicit planning request, or material ambiguity, coordination, cross-cutting scope, irreversibility, or risk -> `plan-work`
- Execute an already-decided scope from a Plan, handoff, findings, brief, docs task, fix, or refactor -> `implement-work`
- Write a continuation prompt or file -> `write-handoff`
- Review a Plan, change, round of fixes, or non-code artifact -> `review-work`
- Discover architecture improvement opportunities -> `improve-architecture`
- Simplify an already-bounded area without changing intended behavior -> `simplify-work`
- Answer a design question with a disposable mockup or logic prototype -> `plan-prototype`
- Write a commit message or commit changes -> `commit-work` when installed
- Open or update a PR, deliver stacked PRs, or run the stack merge loop -> `manage-pr` when installed
- Set up an isolated worktree or clean up stale ones -> `manage-worktrees` when installed
- Answer a coding-standards question or ground a structure/verification decision -> `coding-standards` when installed

Clear low-risk work may remain direct; use `implement-work` when execution discipline or continuation context materially helps.

Concrete architecture review belongs to `review-work`. Use `simplify-work` for an explicitly chosen behavior-preserving simplification, `implement-work` for other decided changes, and `improve-architecture` when the opportunity or interface shape still needs discovery.

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
