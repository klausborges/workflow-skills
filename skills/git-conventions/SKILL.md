---
name: git-conventions
description: Shared git conventions for personal repos covering remote naming, primary remote choice, conventional commit shape, worktree placement, and commit/PR writing style. Use when setting up or renaming remotes, choosing where to push or open PRs, deciding where a worktree lives, or when another git skill points here.
---

# Git Conventions

## Remotes

- Name remotes by host: `github` for GitHub, `forge` for the personal Forgejo instance. Never `origin`.
- Set up both remotes when possible; the second acts as a backup mirror.
- Each repo picks one primary remote at setup. The primary owns CI and pull requests, and is what "origin" means in conversation. Default primary: `github`.

## Commits and PR titles

First line is `type(scope): description`, scope optional. Types: `feat`, `fix`, `docs`, `ci`, `build`, `chore`, `refactor`, `test`, `perf`, `revert`.

## Worktrees

Worktrees are optional. Manual ones live at `<repo>/.worktrees/<branch>` with slashes flattened to dashes, and the directory stays gitignored. Native harness worktree homes (for example Claude Code's `.claude/worktrees/`) are equally fine. Keep project folders free of bare worktree siblings.

## Writing style

- Plain sentences with periods and commas; sentence-case headings.
- Write for the repo's durable history: describe the change itself, never plans, milestones, or phases. Branch names may keep stack prefixes like `m2/prng` since branches are deleted at merge.
- Skip emdashes and decorative emoji.
