---
name: plan-work
description: Plan substantial work when explicitly requested or when material ambiguity, coordination, cross-cutting scope, irreversibility, or risk warrants planning.
---

# Plan Work

Shape uncertain or substantial work into an executable direction without manufacturing ceremony.

Use [workflow-language.md](references/workflow-language.md), [document-conventions.md](references/document-conventions.md), and [planning-templates.md](references/planning-templates.md).

## Invocation and output

Use this skill when the user explicitly requests it or the work has material ambiguity, coordination, cross-cutting scope, irreversibility, or risk. New, multi-step, or nontrivial work alone does not require a Plan.

When explicitly invoked, follow the requested output. If the user says not to write a file, do the planning work and return it in chat. Otherwise an ephemeral saved Plan is the default. The planning process and Plan-file creation are separate decisions.

Do not start implementation.

## Process

1. Read repo-local instructions, current docs, relevant code, and existing Plans or Roadmaps.
2. Research only unresolved questions that can materially change the design; start with repo-local and first-party evidence.
3. Ask only decision-blocking questions the repo cannot answer. Consolidate related questions and provide a recommended option with trade-offs.
4. Present a concise Design Brief when a direction needs owner approval. Skip a redundant checkpoint when the direction is already explicit.
5. Produce the requested planning result. For a saved Plan, use `plans/<outcome>.md`; create a Roadmap only when multiple Plans benefit from prioritization or progress tracking.
6. Check coherence, executability, scope, verification, and doc lifecycle before delivery.

When domain terms, states, relationships, invariants, data flow, or access patterns are load-bearing or unclear, model them explicitly before shaping logic. Stress-test load-bearing terms, relationships, and invariants with one or two concrete scenarios against current code/docs. Use the repo Glossary as authority and add a compact call-stack, component-tree, or data-flow sketch only when it resolves a design decision. Do not silently change canonical terms or docs; record future changes in the authorized draft or Phase-tagged Doc Delta and surface owner decisions.

Do not manufacture downstream Tasks whose shape depends on an unresolved load-bearing decision: one whose plausible answers would materially change the data/schema model, ownership, contracts, migration, or verification.

If the verification design would add module mocks or substantial new end-to-end infrastructure, make that an explicit owner decision and explain the coupling, maintenance, and execution cost. Repo conventions remain authoritative over specialized stack guidance.

## Artifact ownership

- A Roadmap is a thin grouping of Plan links, candidates, and cross-Plan constraints. It may be main or scoped, and its ordering must be labeled `Loose`, `Strict`, or `Mixed`. Keep a Candidate coarse until its implementation outcome and blocking questions can be stated; then promote it to a Plan.
- A Plan owns optional Milestones, Phases, Tasks, acceptance criteria, verification, review placement, and Doc Delta.
- A Phase is the implementation/review scope. Every Phase includes an independent fresh-context review of its full scope.
- Add intermediate review checkpoints only after high-impact Tasks or Task groups. Low-impact Tasks can wait for the Phase review.
- A Target-State Doc contains current durable truth. Correct it during planning only when the correction is already true.
- Use a Draft Target-State Doc for a substantial proposed new durable doc.
- Put future changes to existing canonical docs in Plan Doc Delta, with an explicit Phase on every item.
- Use an ADR only for a surprising, trade-off-heavy, hard-to-reverse decision.

Before the Plan, create or update only independently useful current docs or substantial draft docs. The Plan owns future existing-doc changes, so Doc Delta is written as part of the Plan rather than as a prerequisite to it.

## Plan quality

Keep only execution-bearing context. Verify that:

- decisions, docs, Plan tasks, and acceptance criteria do not contradict one another;
- Phase boundaries are coherent and reviewable;
- verification can prove the claimed outcome;
- review checkpoints reflect risk rather than Task count;
- every Doc Delta item has an explicit Phase;
- unresolved decisions are visible, and downstream Tasks do not assume answers to load-bearing ones;
- load-bearing domain and data-model decisions are explicit when applicable;
- workflow files are not treated as permanent product documentation.

Fix clear issues inline. Return the file path when a file was requested or created, plus a short summary and any owner decisions still needed.
