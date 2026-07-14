---
name: plan-prototype
description: Build the smallest throwaway visual or logic prototype needed to answer a planning question before production implementation.
---

# Plan Prototype

Answer one planning question with the smallest disposable experiment.

Use [workflow-language.md](references/workflow-language.md) when the result feeds a Plan or durable doc.

## Scope

- State the question and the evidence that would answer it.
- Choose a visual prototype for UI, diagrams, flows, or option canvases; choose a logic prototype for state, data, API, or business-rule questions.
- Ask about mode only when the question does not make it clear.
- Compare multiple variants only when comparison is the question. Do not add a switcher, persistence, polish, abstractions, or tests unless they materially improve the answer.
- Prefer existing project tooling and in-memory state. Mark prototype code as throwaway.

Put a visual prototype in the real page/flow only when real density, navigation, or data is necessary to answer the question. Keep logic behind a small interface only when portability is itself useful to the experiment.

## Closeout

Report the answer, supporting observations, uncertainty, and recommended design consequence. Record it in an existing Plan, doc, or ADR only when that owning artifact exists and the conclusion belongs there; chat may be sufficient.

Delete or absorb only prototype artifacts created by this task after their disposition is clear. Preserve preexisting prototypes and unrelated drafts. Do not turn a useful experiment into production implementation without explicit authorization.
