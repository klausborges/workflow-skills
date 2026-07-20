# Workflow Skills

## Purpose

This repo provides portable skills for explanation, diagnosis, planning, implementation, simplification, handoff, review, architecture discovery, and disposable planning prototypes. The workflow should add only the process justified by the task's ambiguity, risk, and coordination needs.

Canonical vocabulary lives in [`GLOSSARY.md`](../GLOSSARY.md). Installed skills carry only the focused shared references they need.

## Current behavior

- `use-workflow` routes to the smallest fitting skill and leaves clear low-risk tasks direct.
- `explain-workflow` explains invocation without starting work by default.
- `explain-codebase` maps code and stays read-only.
- `diagnose-issue` finds causes with evidence and stays read-only unless the user explicitly authorizes a fix.
- `plan-work` runs when explicitly requested or when material ambiguity, coordination, cross-cutting scope, irreversibility, or risk warrants it. Planning may remain in chat; otherwise a saved ephemeral Plan is the default.
- `implement-work` executes already-decided work from a Plan, handoff, findings, direct brief, docs task, fix, or refactor. Plan mechanics are conditional; risk-matched verification and fresh-context review are not.
- `write-handoff` creates bounded continuation prompts or files. Immediate/Durable lifecycle is independent of chat/file storage.
- `review-work` reviews Plans, implementation, continuation rounds, or other scoped artifacts without mutating them unless fixes are separately authorized.
- `improve-architecture` discovers opportunities or resolves unclear interface choices. Concrete change review belongs to `review-work`; already-decided changes route to implementation or simplification.
- `simplify-work` executes a bounded behavior-preserving simplification without default repo scans, Plans, interviews, or reports.
- `plan-prototype` builds the smallest disposable experiment needed to answer one planning question.
- `coding-standards` is an optional front door to the shared engineering standards; canonical rules stay in the `engineering-discipline` reference, and stack-specific standards belong in sibling skills that link back to it.
- `git-conventions`, `commit-work`, `manage-pr`, and `manage-worktrees` are optional git skills: shared remote/commit/worktree/style conventions, conventional commit messages, GitHub PR management including stacked PRs and the stack merge loop, and consistent worktree placement and cleanup. Installing them does not make commits, PRs, or worktrees required workflow artifacts.

## Cost and authority

- Match questions, research, tools, delegation, artifacts, and verification to actual uncertainty and risk.
- Repo-local instructions govern generic defaults. Surface material conflicts with the current request with a recommendation rather than silently choosing.
- Unless the user or repo declares stable compatibility, code and APIs are evergreen: deep or breaking design improvement does not lower correctness, and durable data still requires explicit verified migration.
- Consolidate owner decisions and include recommendations. Do not interview for facts the repo can answer.
- Research only consequential gaps, starting with local and first-party evidence.
- Do not require a Plan, Roadmap, durable doc, handoff file, subagent pass, worktree, commit, or PR merely because a task is multi-step.
- Every implementation Phase ends with an independent fresh-context review of the whole Phase. Direct unplanned implementation and bounded simplification are Phase-equivalent scopes and get the same review before delivery.
- Plans place extra review checkpoints after high-impact Tasks or Task groups; low-impact Tasks can wait for the Phase review.
- The parent verifies delegated review claims against primary evidence. Reviews are not recursively reviewed.

## Artifacts and lifecycle

The hierarchy is `Roadmap > Plan > optional Milestone > Phase > Task`. “Slice” is not a hierarchy term; a vertical slice is only a testing or implementation technique.

A Roadmap is an optional thin progress/prioritization view over Plan links. Projects may have a main `plans/roadmap.md`, multiple `plans/<scope>-roadmap.md` files, or no Roadmap. The canonical Glossary defines `Loose`, `Strict`, and `Mixed` eligibility semantics. A Plan may appear in more than one Roadmap and remains authoritative. Roadmaps do not repeat Plan milestones, phases, tasks, acceptance criteria, or research.

Plans and Roadmaps are ephemeral even when committed or retained temporarily. A completed standalone Plan may be offered for deletion once after final review and doc sync. An active Roadmap gates cleanup of linked Plans; ask once when the Roadmap closes. Never delete automatically. Replace a deleted Plan link with a checked plain-text one-line outcome.

Canonical Target-State Docs describe current durable truth. During planning, correct them only when the correction is already true. Use `docs/_drafts/` for a substantial proposed new doc and the owning Plan's explicitly Phase-tagged Doc Delta for future changes to existing docs. Sync these as behavior lands, normally at Phase closeout or handoff.

Canonical docs, source, code comments, commits, and PRs must remain coherent after workflow artifacts are removed. They do not reference Plan paths or workflow-relative identifiers such as `Phase 0`, `P0`, `Milestone 0`, or `M0`. Workflow artifacts may use hierarchy terms.

## Shared references

Canonical references live under `skills/_shared/`:

- `workflow-language.md`: portable hierarchy and lifecycle boundary
- `review-language.md`: review modes, signal, and ledger
- `document-conventions.md`: artifact locations and lifecycle
- `planning-templates.md`: Design Brief, current-doc, Roadmap, and Plan shapes
- `handoff-template.md`: handoff shape
- `phase-review-template.md`: compact end-of-Phase review record
- `research-ladder.md`: conditional source preference
- `architecture-language.md`: optional architecture heuristics
- `engineering-discipline.md`: proportionate implementation and simplification heuristics

Each skill's `skill.toml` declares what it packages:

```toml
[references]
shared = [
  "workflow-language",
]
owned = []
```

A shared name maps to `skills/_shared/<name>.md` and is generated at `skills/<skill>/references/<name>.md`. A skill-owned reference lives in that directory and is declared under `owned`.

`skill.toml` is the packaging contract. `wflow refs sync` creates, updates, and prunes generated shared references without deleting owned references. `wflow refs verify` checks manifest names, package safety, generated equality, owned files, and stale temporary artifacts. Skill Markdown links and provider metadata are validated by repository formatting/check tooling rather than treated as packaging declarations.

To change references:

1. Edit the canonical shared file or create an owned reference.
2. Update the skill's sorted `shared` or `owned` list.
3. Run `mise run sync-references`.
4. Run `mise run check`.

## Workflow boundaries

- Explanation and review do not mutate by default.
- Diagnosis does not fix by default.
- Planning does not auto-start implementation.
- Implementation changes only the authorized scope and preserves unrelated dirty work.
- Simplification preserves intended behavior and does not use line count or repo-wide scanning as a default objective.
- Handoffs do not replace Plans or duplicate durable docs.
- Architecture discovery does not force a refactor Plan or handoff; decided behavior-preserving changes belong to `simplify-work`.
- Prototypes do not become production work without explicit authorization.
- Cleanup is limited to artifacts created by the current task whose disposition is clear; preexisting drafts and prototypes are preserved.

## Out of scope

- Requiring PRDs, issues, commits, worktrees, or permanent Plans.
- Creating process artifacts for their own sake.
- Recreating a heavyweight review or companion loop for low-risk work.
- Keeping reference source clones in the published repo.
