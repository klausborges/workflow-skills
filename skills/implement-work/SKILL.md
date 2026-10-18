---
name: implement-work
description: Execute a decided brief, red before green, and return its report. Use when a lane gets an implementation brief, a fix brief, or accepted findings to apply.
---

# Implement work

As the main session, run the loop in use-workflow. As a lane, execute the brief and return its report.

Read [engineering-discipline.md](references/engineering-discipline.md) before writing code or tests, and [workflow-language.md](references/workflow-language.md) when the brief's terms need their exact meaning.

Execute the brief's scope. Adjacent work goes on the tracker item that owns it, described by mechanism.

Red first: see new behavior fail on an assertion before making it pass; a missing API or a type error is not red. A changed test says why, with an assertion as strong or stronger. A new guard is seen failing on a prohibited example.

After two failed attempts at the same invariant, stop patching and report what you tried; a new reviewer or a new reproduction doesn't reset the count.

The lane's work ends at REPORT. Close is use-workflow step 7.

## Gotchas

- Skipping hooks to show red leaves a commit the gate never checked: check the old files out into the worktree instead.
- A lane's full gate duplicates the main session's: run targeted checks, and leave the final head's gate to the main session unless the brief says otherwise.
- Gate output piped through head or tail hides the failure and the count: read it whole.
- A server the owner will look at dies with the session: run it detached and save its PID.
- Committed screenshots bloat history: attach them to the PR.
