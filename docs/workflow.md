# Workflow Skills

## Purpose

This repo defines a small set of agent workflow skills for explaining the workflow, explaining codebase areas, diagnosis, planning, implementation, handoff, review, architecture improvement, and planning prototypes.

The skills are intended to be portable across repos and agent tools while preserving a concise, explicit workflow. They borrow selectively from Matt Pocock's agent skills, Superpowers, and Vercel Labs Skills without requiring those projects at runtime.

## Target State

- The repo exposes installable skills under `skills/`.
- `explain-workflow` explains how to use the workflow skills for concrete situations without starting the work by default.
- `explain-codebase` explains unfamiliar codebase areas, modules, callers, flows, tests, and terms without editing files.
- `diagnose-issue` builds a feedback loop, reproduces the issue, finds root cause, and fixes normal bugs after the verification path is clear.
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

Installable skills must be self-contained. The canonical files in `skills/_shared/` are generated into each skill's `references/` directory according to each skill's metadata.

## Reference Metadata

Each installable skill has a `skill.toml` file that declares the reference files shipped with that skill:

```toml
[references]
shared = [
  "workflow-language",
]
owned = []
```

Reference names are bare names. A shared reference named `workflow-language` maps to `skills/_shared/workflow-language.md` and is generated into `skills/<skill>/references/workflow-language.md`.

`skill.toml` is the only reference artifact contract. `wflow` does not parse `SKILL.md` links; this intentionally avoids Markdown path parsing and keeps verification focused on the files that will be packaged with each skill.

To add an existing shared reference to a skill:

1. Add the bare reference name to `[references].shared` in that skill's `skill.toml`.
2. Run `mise run sync-references`.
3. Run `mise run check`.
4. Optionally link the generated local file from `SKILL.md`, for example `references/workflow-language.md`.

To add a skill-owned reference:

1. Create `skills/<skill>/references/<name>.md`.
2. Add the bare reference name to `[references].owned`.
3. Use a name that does not exist in `skills/_shared/`; shared-name collisions are rejected.
4. Run `mise run check`.
5. Optionally link it from that skill's `SKILL.md`.

`wflow refs sync` may create, update, and prune generated shared references. It must not delete skill-owned references. `wflow refs verify` fails when metadata, package files, generated shared files, skill-owned files, or stale temporary reference artifacts disagree. Verification reports independent validation failures together when it can; for example, invalid `skill.toml` does not suppress package-file safety checks for the same skill.

## User/System Flows

### Explanation

1. The user asks how to use the workflow, how to invoke a skill, or how the workflow applies to a concrete situation.
2. The agent answers from the user's point of view with the relevant skill, what to ask next, and what to expect.
3. If the request might also mean "start the work", the agent explains first and gives the exact next prompt instead of starting by default.

### Codebase Explanation

1. The user asks how a codebase area, feature, flow, module, or call path works.
2. The agent reads local docs, relevant source, callers, entry points, and tests.
3. The agent returns a read-only map of purpose, entry points, key modules, flow, tests/checks, terms, and unknowns.
4. If concrete architecture friction appears, the agent may recommend `improve-architecture` as a next step without starting it automatically.

### Diagnosis

1. The user reports a bug, failing test, build failure, flaky behavior, unexpected output, or performance regression.
2. The agent builds or identifies a fast feedback loop and reproduces the reported symptom.
3. The agent forms falsifiable hypotheses, instruments narrowly, and identifies root cause before changing behavior.
4. The agent fixes normal bugs once root cause and verification path are clear.
5. The agent stops before risky fixes, public behavior changes, production-sensitive work, performance trade-offs, or missing architecture seams.

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
- Codebase explanation is read-only by default.
- Diagnosis requires root-cause evidence before fixes.

## Out of Scope

- Recreating Superpowers' full visual companion loop by default.
- Treating PRDs, GitHub issues, commits, or worktrees as required workflow artifacts.
- Creating `.out-of-scope/` as a default convention.
- Keeping reference clone contents in the published repo.
