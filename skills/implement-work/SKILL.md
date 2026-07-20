---
name: implement-work
description: Implement an already-decided bounded scope from a Plan, handoff, review findings, direct brief, docs task, fix, or refactor. Use for implementation or continuation work; use diagnose-issue first when the cause is unknown.
---

# Implement Work

Execute the authorized outcome without imposing a Plan-shaped process on other inputs.

Use [engineering-discipline.md](references/engineering-discipline.md). For Plan-backed work, also use [workflow-language.md](references/workflow-language.md), [document-conventions.md](references/document-conventions.md), and [phase-review-template.md](references/phase-review-template.md).

## Establish the work scope

1. Read repo instructions, the governing input, relevant current docs, source, tests, and any referenced artifacts.
2. Identify the authorized outcome, boundaries, acceptance evidence, doc impact, and unresolved decisions. Research only gaps that can materially change the implementation.
3. If a Plan governs the work, select the requested Phase(s), or the next unchecked Phase when asked to continue, and note its review checkpoints and Doc Delta.
4. Ask only for real ambiguity, credentials, product judgment, risky authority, or an explicit human checkpoint. Otherwise proceed.

Do not create a Plan, interview loop, handoff, worktree, commit, or PR unless the input or authorized workflow calls for it.

## Execute and verify

- Work through the bounded scope without absorbing adjacent cleanup.
- Use the fastest meaningful feedback loop: focused red/green checks for cheap behavior, same-scope checks for setup-heavy behavior, and acceptance gates for slow end-to-end, full-suite, or visual behavior. Use manual evidence only when automation is impractical and record the residual risk.
- Verify through public or intentionally stable interfaces when practical. Read fresh command or observation results before claiming success.
- Keep canonical docs current as behavior lands. Preserve unrelated dirty work and preexisting drafts or prototypes.
- Follow Plan-defined high-impact review checkpoints. Low-impact Tasks can wait for the end-of-Phase review.
- For Plan-backed work, check Tasks and acceptance criteria only after evidence supports them; apply only the selected Phase's Doc Delta and route non-required work to `Discovered Follow-ups`.
- Without a Plan, do not invent Plan bookkeeping. Treat the authorized work as one review scope and keep only state another worker or reviewer needs.

## Close the scope

1. Self-check the entire authorized scope against its input, docs, tests, actual diff, and durable-artifact boundary.
2. Run the relevant verification and read its output.
3. Send the entire scope, not only the latest patch, to an independent fresh-context reviewer through `review-work`; verify accepted claims against primary evidence, fix authorized blocking findings, and rerun affected checks.
4. Repeat only the review coverage justified by the changed surface or residual risk.
5. Update governing artifacts. For Plan-backed work, use [phase-review-template.md](references/phase-review-template.md), finish the selected Phase's Plan/doc updates, and only then mark it complete.

Do not recursively review the review. Add Milestone-level integration review only when cross-Phase acceptance or risk warrants it. Leave Plan or Roadmap cleanup to final review.
