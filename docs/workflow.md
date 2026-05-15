# Workflow Skills

## Purpose

This repo defines a small set of agent workflow skills for explaining the workflow, planning, implementation, handoff, review, architecture improvement, and planning prototypes.

The skills are intended to be portable across repos and agent tools while preserving a concise, explicit workflow. They borrow selectively from Matt Pocock's agent skills, Superpowers, and Vercel Labs Skills without requiring those projects at runtime.

## Target State

- The repo exposes installable skills under `skills/`.
- `explain-workflow` explains how to use the workflow skills for concrete situations without starting the work by default.
- `use-workflow` routes substantial work to the most specific workflow skill without forcing itself into small direct tasks.
- `plan-work` turns unclear feature or workflow ideas into Target-State Docs and an ephemeral Plan.
- `implement-plan` executes approved Plan phases, keeps checklist state current, verifies work, and runs phase self-review.
- `write-handoff` creates fresh-context prompts or files for implementation, continuation, research, refactor, bugfix, review, or review-fix work.
- `review-work` reviews implementation against Plans, Target-State Docs, acceptance criteria, tests, and risk.
- `improve-architecture` remains explicit-invocation by default and shapes targeted architecture improvements.
- `plan-prototype` supports throwaway visual or logic prototypes that answer a planning question, then get deleted or absorbed.

## Key Concepts

Canonical workflow language lives in the root `GLOSSARY.md`.

Shared skill references live in `skills/_shared/`:

- `workflow-language.md` defines portable workflow terms for installed skills.
- `document-conventions.md` defines where Plans, Target-State Docs, ADRs, and glossaries belong.
- `templates.md` defines reusable Design Brief, Plan, Handoff, and Phase Self-Review shapes.
- `research-ladder.md` defines source preferences without hard-coding one research tool.
- `architecture-language.md` defines optional architecture vocabulary.

Installable skills must be self-contained. The canonical files in `skills/_shared/` are generated into each skill's `references/` directory, and skill instructions should link to those local generated copies.

## User/System Flows

### Explanation

1. The user asks how to use the workflow, how to invoke a skill, or how the workflow applies to a concrete situation.
2. The agent answers from the user's point of view with the relevant skill, what to ask next, and what to expect.
3. If the request might also mean "start the work", the agent explains first and gives the exact next prompt instead of starting by default.

### Planning

1. The user explains an idea, requirements, and constraints.
2. The agent reads local context, researches as needed, and asks one focused question at a time.
3. The agent proposes a concise Design Brief with recommendation, trade-offs, constraints, acceptance criteria, and docs needed.
4. After approval, the agent writes or updates Target-State Docs first.
5. The agent writes an ephemeral Plan with phases, tasks, acceptance criteria, verification strategy, and out-of-scope.
6. The agent self-reviews the Plan before handing it back.

### Implementation

1. The agent reads the Plan, related Target-State Docs, and relevant source.
2. The agent identifies selected phases or infers the next unchecked phase.
3. The agent asks questions only for real ambiguity, HITL work, or explicit override confirmation.
4. The agent implements with pragmatic red/green TDD by default unless the user opts out.
5. The agent updates Plan tasks and acceptance criteria only when work is verified.
6. Before completing a phase, the agent runs focused self-review, using a fresh subagent when the harness provides one.

### Handoff

1. The agent identifies the handoff type and durability.
2. The handoff starts with a `Read First` list.
3. The handoff includes current state, task, constraints, implementation notes, verification, completion criteria, and out-of-scope.
4. File handoffs default to durable handoffs.
5. Ordinary chat handoffs do not add Plan notes unless tracked work changes.

### Review

1. The agent starts from the Plan, Target-State Docs, acceptance criteria, and tests.
2. Findings lead, ordered by severity.
3. Multi-aspect review is explicit, inferred from the change when requested, and reconciled before reporting.
4. Review may recommend `improve-architecture` when architecture friction is real, but should not start it automatically.

## Constraints

- Plans are ephemeral; durable target state belongs in `docs/`.
- Target-State Docs should avoid code examples except for stable contracts that prose cannot describe clearly.
- ADRs are rare and reserved for hard-to-reverse, surprising, trade-off-heavy decisions.
- Skills should use stronger tools when available or requested, but must remain portable and fall back to first-party docs, `llms.txt`, official examples, CLI help, and local examples.
- The workflow should not require commits, worktrees, issue publishing, or subagent-driven implementation by default. Subagents are preferred for self-review when the harness provides fresh-context subagents.
- Handoffs are first-class workflow artifacts, not summaries.

## Out of Scope

- Recreating Superpowers' full visual companion loop by default.
- Treating PRDs, GitHub issues, commits, or worktrees as required workflow artifacts.
- Creating `.out-of-scope/` as a default convention.
- Keeping reference clone contents in the published repo.
