---
name: plan-work
description: Turn unclear or cross-cutting work into a plan of decisions, phases and a gate. Use when the owner asks for a plan, or when ambiguity, coordination across slices, a shared contract or stored data leaves the approach open.
---

# Plan work

Read [workflow-language.md](references/workflow-language.md) for Plan, Phase and Roadmap, and [planning-templates.md](references/planning-templates.md) before writing to the plan surface.

Unless the owner asked for a plan, when the change touches one or two files and the approach is obvious, say so and stop. Inside authorized implementation, hand control to the brief instead.

Understand the intent before sizing the work. A plan records decisions, slices, acceptance, the gate, the review pair and what stays unqualified, never code. A plan several times longer than the ask is a transcript.

Planning ends at the plan. Building starts from a brief after the owner's go-ahead on it.

The plan surface is a project slot: an issue, a `plans/` file or a page. In a repo outside the owner's accounts and orgs (listed in the owner's global instructions), planning stays in chat, and nothing (issues, PRs, comments, reports) is created there unless the owner asks.

The blind plan review (use-workflow step 1) asks only the questions a builder would still need answered.

When a design question needs evidence, build the smallest throwaway probe, visual or logic, inside the real project. Report the answer and what it means for the design, then delete the probe; it becomes production code only with the owner's go-ahead.

## Gotchas

- Facts recalled from memory or from skill text go stale: take prices, API shapes and contracts in a plan or brief from live sources.
- A plan long enough to need scrolling goes unread: when the owner will read it, make it a page.
