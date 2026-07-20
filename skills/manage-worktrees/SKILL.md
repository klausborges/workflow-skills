---
name: manage-worktrees
description: Create, reuse, and clean up git worktrees consistently. Use when work needs an isolated checkout, when the user asks for a worktree, or when stale worktrees need cleanup.
---

# Manage Worktrees

Placement rules follow the `git-conventions` skill. Worktrees are optional: reach for one when isolation helps (parallel agents, work beside an occupied checkout) and work in place otherwise.

## Workflow

1. Detect existing isolation first: when `git rev-parse --git-dir` and `--git-common-dir` differ and `git rev-parse --show-superproject-working-tree` prints nothing, this is already a worktree; use it and skip creation.
2. Prefer the harness's native worktree tool when one exists; it manages placement and cleanup itself.
3. Manual fallback: confirm `.worktrees/` is ignored (`git check-ignore -q .worktrees/wt`; add it to `.gitignore` and commit when missing), then `git worktree add .worktrees/<flattened-branch> <branch>` for an existing branch, adding `-b <branch>` to create one.
4. Run the repo's dependency setup in the new worktree when the work needs a build or tests.
5. Clean up once a branch merges or is abandoned: `git worktree remove <path>`, delete the branch, and `git worktree prune` for broken registrations. Audit with `git worktree list` when in doubt.

## Guardrails

- Never nest a worktree inside another worktree.
- Create worktrees only under `.worktrees/` or a native harness home; project folders stay free of bare worktree siblings.
- Removal waits until the branch tip is reachable from the remote or the user waives it.

## Output

The worktree path and branch, or the cleanup performed per `git worktree list`.
