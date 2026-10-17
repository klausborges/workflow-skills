---
name: git-conventions
description: Remote names, commit shape, worktree placement and writing style for the owner's repos. Use when naming remotes, choosing where to push, writing a commit or PR title, or placing a worktree.
---

# Git conventions

## Remotes

Remotes are named by host: `github` for GitHub, `forge` for the personal Forgejo instance. Set up both when possible; the second is a backup mirror. Each repo picks one primary remote, which owns CI and PRs; the default is `github`.

## Commits and PR titles

The first line is `type(scope): description`, scope optional. Types: `feat`, `fix`, `docs`, `ci`, `build`, `chore`, `refactor`, `test`, `perf`, `revert`. Add a body only when the why is not obvious from the diff.

Messages and PR text describe the change, never plans, milestones or phases. Branch names may keep stack prefixes such as `m2/prng`, since branches are deleted at merge.

No attribution or co-author trailers anywhere, including inside briefs.

## Worktrees

Use the harness's native worktree first. Manual ones live at `<repo>/.worktrees/<branch>` with slashes flattened to dashes, in a gitignored directory. Never nest a worktree.

## Writing style

Plain sentences, sentence-case headings, no em dashes, no decorative emoji.
