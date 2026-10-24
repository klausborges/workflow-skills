---
name: write-handoff
description: Write a lane brief, a handoff with its kickoff, a compaction prompt, a resumption record or a night-shift report. Use before spawning a lane, when work must move to another session or person, or when asked for a "compaction prompt" or a "night shift".
---

# Write handoff

Read [handoff-template.md](references/handoff-template.md) before writing a brief or a handoff, and [workflow-language.md](references/workflow-language.md) for the loop terms.

## Briefs

Without goal, scope, worktree with its actual parent, gate, and REPORT, don't spawn. For a lane that doesn't write, the worktree names what to read (a path or base..head), and the gate may read "none, because read-only". Point at files; never paste session history.

Briefs and handoffs live in the project's slot for them; the default is a gitignored folder in the repo, such as `plans/.local/`.

## Outputs

- **Handoff**: the brief plus a copy-pastable kickoff prompt, neither committed.
- **Compaction prompt**: under 300 words, fenced, last in the message. It keeps what was decided, quoted exactly; where things stand, with heads, PRs and merge order; what is open; and details that would be hard to reconstruct.
- **Resumption record**: worktree and branch, head, the worktrees the session added that are still open, next gate, open findings, pending rulings, and receipts outside git. Write it on the plan surface, else in the PR body or chat, at close and at every handoff or compaction.

Continue first, then compact. Write a handoff only when something must travel to another session, worktree or person.

## Unattended runs

First record what the owner approved for the run, quoted. The morning report leads with merge-readiness or what blocks it, then a PR table with heads and executed test counts, rulings needed with recommendations, defaults taken, and what went wrong. It ends with the compaction prompt.

## Gotchas

- A brief copied from the previous brief copies its mistakes: write each from the template.
- Never-lines that have mattered: no network unless the brief needs it; never print secret values; name excluded paths and real-data stores; commit no binaries.
- A session scratchpad or /tmp gets wiped: briefs, research and handoffs go in the durable slot.
- A paraphrased ruling drifts: quote it with its source, and record silence as pending, never as a go.
- A compacted session waits for input: a one-line nudge starts the next turn.
