---
name: improve-architecture
description: The lean pass, sized by risk, and simplifications the owner asks for. Use for a "lean pass", when a slice or a milestone closes, or when the owner says "simplify X".
---

# Improve architecture

As the main session, run the loop in use-workflow and size the pass here. As a lane, execute the brief and return its report. Read [architecture-language.md](references/architecture-language.md) when naming a finding, and [engineering-discipline.md](references/engineering-discipline.md) before judging tests or applying a change.

The owner's purpose for the pass: to "look with a bit more clarity to ensure we didn't just tack things on until something worked, without zooming out a bit and ensuring the code is actually well architected and lean, my testing preferences, comments close to zero to avoid explanation of workarounds and hacks to mask problem areas".

## Sizing

- **Milestone close, or a change to a shared contract**: four reads, named by role: two independent architecture reads on the primary model line, one on the second line with the first two withheld, and one on the tests.
- **Normal slice**: one blind read per model line.
- **Lighter route**: none.

The main session compares the reads and is never one of them. Findings are reported, the owner rules, and accepted fixes land as their own PR on top of the stack.

## Discovery

Start from caller-visible usage. Name friction as scattered knowledge, pass-through layers, unowned rules, invalid states, workarounds explained by comments, or defensive code and tests with no failure they guard. A finding names the caller, the rule, the owner it proposes, and how unchanged behavior is shown. A no-change verdict needs evidence too.

## Apply

A direct "simplify X" from the owner is applied, not only reported. Preserve behavior, add a characterization check only when existing evidence can't protect it, and set no line-count goal.

## Gotchas

- A lean pass on a lean-pass PR never ends: that PR gets none.
- Test advice from a lean read can be wrong: check it by mutation before relaying it.
- Input for a named next consumer looks like churn: it stays.
- A tidy-up that touches stored data risks loss: stored data outranks tidiness.
