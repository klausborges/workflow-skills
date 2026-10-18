---
name: review-work
description: Review a plan, a change, a confirmation round or a whole stack, read-only, with evidence. Use for a review step or review pair, a fresh read of a stack, or when asked to "review this" or "confirm the fix".
---

# Review work

As the main session, run the loop in use-workflow and hand each reviewer a brief. As a reviewer, read the brief's intent, then the primary evidence, then any prior findings. Read [engineering-discipline.md](references/engineering-discipline.md) when judging tests or evidence, and [workflow-language.md](references/workflow-language.md) for the loop terms.

## Kinds

- **Plan or brief**: can a builder execute it as written? Report only the questions a builder would still need answered.
- **Change**: the whole diff against its actual parent, base and head named. A later commit voids the review for that delta only.
- **Confirmation round**: the same model line gets its own findings and the fix diff, and confirms or reopens each one. Review only the new diff.
- **Fresh read**: one reviewer per model line, none of whom saw any slice, reads the whole stack against main.

The slice route uses two model lines, blind: separate report paths, neither reads the other's report, and neither hears the main session's lean. When a line is unavailable, stop and ask the owner; never substitute.

## Report

Start with the count and the verdict, and say plainly when there is nothing to fix. Then:

- base and head; what you ran and what you only read;
- findings, each labeled defect (shown failing, or a concrete failure path), risk (credible but not shown), or preference; file and line, why it is wrong, how to show it fails;
- pre-existing issues, labeled as such;
- declined to judge, with the reason;
- what this review does not qualify.

## Gotchas

- A reviewer told what not to flag, or how severe something is, ships the flaw. The brief states intent and the reasoning behind each choice in the owner's words, never verdicts.
- Rounds stop converging: after the second, most findings are old code. Flag the class of bug once so every instance gets fixed, never repeat a posted finding, and from round three block only on regressions the fix introduced or loss at real scale. After three rounds, ask whether the approach is wrong.
- A probe or mutation in the lane's worktree corrupts its state: run it in a detached temporary worktree or an export.
- Test and defensive-code findings tend to grow scope. Without a failure path they are preferences.
- A real race without a deterministic reproduction is still a finding.
- Gate config in the diff is code: a new suppression, skipped test, or loosened threshold needs the owner.
- A green gate counts only on the head under review, with its executed test count.
- Describe a security finding by mechanism and file:line, never with a working payload.
