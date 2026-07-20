---
name: commit-work
description: Write conventional commit messages from local git evidence and commit when asked. Use when the user wants changes committed or a commit message written.
---

# Commit Work

Message shape and style follow the `git-conventions` skill.

## Workflow

1. Inspect staged changes first: `git diff --staged --stat`, then the diff itself. When nothing is staged, inspect `git status --short` and the working-tree diff, and name what you would stage.
2. Check `git log --oneline -10` for scope and phrasing precedent.
3. Write the first line as `type(scope): description`. Add a body only when the why is not obvious from the diff; wrap it near 90 columns and use repeated `-m` flags or `--file` so paragraphs stay separate.
4. Recommend a split only when the diff contains clearly unrelated changes.
5. Commit when the user asked for a commit; otherwise deliver the message and the staging suggestion.

## Guardrails

- Stage files only when the user asks.
- Run the repo's fast checks before committing when they exist; let hooks run and report a failure instead of bypassing it.
- Describe the change itself; plan and milestone context stays out of the message.

## Output

The commit SHA and first line, or the proposed message plus what to stage.
