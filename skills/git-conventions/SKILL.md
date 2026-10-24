---
name: git-conventions
description: 'Remotes, commits, PRs and stacks, and worktrees for the owner''s repos. Use for development work in a git repository: committing, opening or updating PRs and stacks, restacking after a merge or when the owner says "merged #N", and creating, reusing or removing worktrees.'
---

# Git conventions

## Remotes

Remotes are named by host: `github` for GitHub, `forge` for the personal Forgejo instance. Set up both when possible; the second is a backup mirror. Each repo picks one primary remote, which owns CI and PRs; the default is `github`.

## Commits

The first line is `type(scope): description`, scope optional. Types: `feat`, `fix`, `docs`, `ci`, `build`, `chore`, `refactor`, `test`, `perf`, `revert`. Add a body only when the why is not obvious from the diff. PR titles take the same shape.

Messages and PR text describe the change, never plans, milestones or phases. Branch names may keep stack prefixes such as `m2/prng`, since the forge deletes remote branches at merge.

No attribution or co-author trailers anywhere, including inside briefs.

Stage only when asked; otherwise name what you would stage. Let hooks run, and fix or report what they catch.

## PRs and stacks

PRs go to the repo's primary remote, by the name the repo gives it; never assume `origin`, which a repo may define as canonical.

The owner merges. Agents never merge PRs, including `gh stack merge`; "merge what's needed" means leave it ready. PRs open as drafts and are marked ready once reviewed and complete, since a draft blocks the merge by itself.

### Body

- `Closes #N` when there is an issue.
- Parent branch and head SHA.
- Review: base and head, with each model line's verdict, or "single model line" when the owner ruled one.
- Gate: the command on that head, with its executed test count.
- Unqualified: what the gate and the reviews did not exercise.

For images and video, reference the file by path in the body and pass the same path with `--attach`; when an upload fails, the PR is still created and the command exits non-zero.

### Stacks

`gh stack` is the default stack tool, also for unrelated changes done in sequence: `gh stack init` or `gh stack add`, `gh stack push --remote <primary>`, `gh pr create --draft --base <parent>` per branch, then `gh stack link` bottom to top.

Restacking the stack's own branches needs no go-ahead. When anything under the stack changes, including after "merged #N":

1. Record the trunk tip and each branch's head.
2. Run `gh stack rebase --remote <primary>`.
3. On each branch, run `git range-diff <old-parent-tip>..<old-tip> <new-parent>..<branch>` with the heads from step 1, and show that only the base moved.
4. Gate the new top head.
5. Run `gh stack push --remote <primary>`.

A conflict, or a range-diff showing more than a base move, means more changed than the base: stop and report. `gh stack sync` stays out because it pushes before the range-diff and the gate. Where the repo forbids force pushes over reviewed work, merge the parent into the children instead.

### PR gotchas

- Adding commits to a PR already handed over changes what the owner reviewed: it gets nothing more except a restack whose range-diff shows only the base moved. Restacking and adding PRs to the stack stay fine.
- A PR left in an old merged stack can't be relinked until it is unstacked, which is the owner's call: list it as a blocker.
- Independent PRs can each merge cleanly and still conflict together: check the combined merge of all open heads, not only pairs.
- Attribution trailers reach main through squash bodies: strip them before handing over.

## Worktrees

Use the harness's native worktree first. Manual ones live at `<repo>/.worktrees/<branch>` with slashes flattened to dashes, in a gitignored directory. Never nest a worktree. The manual fallback is `git worktree add .worktrees/<flattened-branch> <branch>` once `.worktrees/` is ignored, which checks out an existing branch as is; add `-b` before `<branch>` only for a new branch.

A worktree the host (an app or orchestrator that created the session's checkout) made for the session belongs to the host: work in it, and never remove, move or re-point it. Worktrees you add for lanes and spikes are yours to clean up.

Reuse the current worktree when already inside one.

### Removal

When the forge says a branch's PR merged, remove its worktree if clean, with `git worktree remove <path>` and no `--force`; its refusal is the check. Trust the forge, since a squash merge leaves the tip unreachable. At the close of a slice, remove the merged worktrees you added, and name the ones you keep in the resumption record. Local branches of merged PRs stay until a final closeout, when the owner says they're no longer needed. Never run `git clean`.

To sweep, for each worktree under the repo's worktree folders read its PR's state from the forge and `git status --porcelain`. Remove the merged and clean ones, and list the rest for the owner.

### Worktree gotchas

- A pre-commit tool that stashes unstaged changes shares one stash across worktrees: disable its stash in parallel worktrees.
- A test that spawns git inside a hook inherits `GIT_DIR` and `GIT_WORK_TREE`. A `git init` there writes `core.worktree` into the main repo's config and points the main checkout at the worktree. Strip `GIT_*` from the spawned environment, and after a failed hook run check `git config --get core.worktree` in the main repo.

## Writing style

Plain sentences, sentence-case headings, no em dashes, no decorative emoji.
