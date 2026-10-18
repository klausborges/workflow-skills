# Handoff template

Read when writing a brief or a handoff. A brief missing a required field is not ready to hand over. Every other field may read "none, because ...".

```md
# Brief: outcome

GOAL (required):
SCOPE (required): what is in, what is out.
WORKTREE (required): path, branch, and the actual parent it was created from; for a lane that doesn't write, what to read (a path or base..head).
GATE (required): the repo's real command, the host, what counts as green; or "none, because read-only".
ACCEPTANCE: observable results that close the goal.
WRONG IMPLEMENTATIONS: plausible wrong versions a test must tell apart.
INVARIANTS:
DOCS TOUCHED:
MAY:
MAY NOT:
STOP AND REPORT: conditions where the lane reports instead of proceeding.
NEVER:
SKILLS: skills to load.
PREREQUISITES: host, install steps, local copies of material the lane can't reach.
READ FIRST: files and pointers.
REPORT (required): what comes back and where (a file when the lane can write, final output otherwise), verdict first.
```
