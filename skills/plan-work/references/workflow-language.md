# Workflow language

Read when a brief, plan or report uses one of these terms and the exact meaning matters. A repo's own glossary, `docs/glossary.md` when present, wins for its domain terms.

## Plans

- **Roadmap**: an ordered view over Plans: Loose (advisory), Strict (only the earliest open Plan is eligible) or Mixed (only named blockers hold).
- **Plan**: one outcome and its Phases. Not canonical: it never holds current truth, and the host decides whether a finished Plan is deleted, archived or kept.
- **Phase**: one coherent change, the unit of delivery and review. A **Milestone** is an optional label on Phases, never a level.
- **Task**: concrete work inside a Phase; a doc change is a Task naming the doc.
- **Candidate**: a Plan whose outcome is not committed to yet.

Durable text (docs, code, commits, PRs) never cites a positional id such as "Phase 2" or "M0"; a stable tracker id such as #55 is fine.

## The loop

- **brief**: the input to one run of the loop: one scope, one worktree, one PR or stack, with or without a Plan.
- **slice**: an informal word for a bounded chunk of work; never a level, a field or a heading.
- **lane**: a delegated worker executing one brief.
- **gate**: the project's check command on an exact state, reported with base, head, host and executed test count.
- **plan surface**: where the repo keeps plans.
- **review pair**: two reviewers on different models. **Blind**, **confirmation round** and **fresh read** are defined in review-work.
- **lean pass**: a read for code tacked on until something worked; see improve-architecture.
- **comments pass**: once per stack, over code the stack changed and nothing else, comments that restate the code or explain a workaround go, with the code fixed where needed. Existing why-comments stay. Its changes go back through review, like lean-pass and fresh-read fixes.
- **unqualified surface**: what the gate and reviews did not exercise.
- **compaction prompt**: the text handed to the summarizer before context is compacted.
- **resumption record**: what a new session needs to pick the work up; see write-handoff.
