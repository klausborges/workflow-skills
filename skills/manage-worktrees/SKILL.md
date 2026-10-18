---
name: manage-worktrees
description: Create, reuse and remove git worktrees. Use when work needs an isolated checkout, or when stale worktrees need cleanup.
---

# Manage worktrees

Placement follows git-conventions. Reuse the current worktree when already inside one. The manual fallback is `git worktree add .worktrees/<flattened-branch> <branch>` once `.worktrees/` is ignored, which checks out an existing branch as is; add `-b` before `<branch>` only for a new branch.

Remove with `git worktree remove <path>` and no `--force`; its refusal is the check. A branch counts as merged when the forge says so, since a squash merge leaves the tip unreachable. Never run `git clean`.

## Gotchas

- A pre-commit tool that stashes unstaged changes shares one stash across worktrees: disable its stash in parallel worktrees.
