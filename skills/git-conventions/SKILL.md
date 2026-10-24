---
name: git-conventions
description: 'Use for any development work in a git repository: committing, opening or updating PRs and stacks, restacking after a merge or when the owner says "merged #N", and creating, reusing or removing worktrees.'
---

# Git conventions

## Remotes

Remotes are named by host: `github` for GitHub, `forge` for the personal Forgejo instance. Set up both when possible; the second is a backup mirror. Each repo picks one primary remote, which owns CI and PRs; the default is `github`.

## Commits

The first line is `type(scope): description`, scope optional. Types: `feat`, `fix`, `docs`, `ci`, `build`, `chore`, `refactor`, `test`, `perf`, `revert`. Add a body only when the why is not obvious from the diff. PR titles take the same shape.

Messages and PR text describe the change, never plans, milestones or phases. Branch names may keep stack prefixes such as `m2/prng`, since the PR host deletes remote branches at merge.

No attribution or co-author trailers anywhere, including inside briefs.

Stage and commit only when asked; otherwise name what you would stage. Let hooks run, and fix or report what they catch.

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

Reuse the current worktree when inside one; otherwise prefer the harness's native worktree. Manual ones live at `<repo>/.worktrees/<branch>`, slashes flattened to dashes, with `.worktrees/` gitignored: `git worktree add .worktrees/<flattened-branch> <branch>`, adding `-b` before `<branch>` only for a new branch. Never nest a worktree.

A worktree you start in, made by the host (the app or orchestrator that made your checkout) or the harness, belongs to it: work in it, and never remove, move or re-point it. Worktrees you add, natively or by hand, are yours.

### Removal

Remove only worktrees you added, with `git worktree remove <path>` and no `--force`. It refuses on changes but silently deletes ignored files: first check `git -C <path> status --porcelain --ignored` and move anything besides build output and dependencies (briefs, `.env`, notes). A detached HEAD's commits go with the worktree, so check that HEAD is on a branch.

- At merge: once the primary remote shows a PR merged (not `git branch --merged`: a squash merge leaves the tip unreachable), remove the worktree you added for its branch.
- Stacks: a worktree that switches between a stack's branches goes only once none of them has an open PR.
- At the end of the work: remove the merged ones you added; name those you keep in your handoff notes.
- Sweep: for each worktree in `.worktrees/`, read its PR state and `git status --porcelain --ignored`. Remove the merged, clean ones you added (across sessions: named in your own handoff notes), or all of them when the owner asked for the sweep; list the rest with their state.
- Local branches of merged PRs stay until the owner says they're no longer needed.
- Never run `git clean`.

### Worktree gotchas

- A pre-commit tool that stashes unstaged changes shares one stash across worktrees: disable its stash in parallel worktrees.
- Plain git gives hooks `GIT_DIR` and `GIT_INDEX_FILE`; some hook runners add `GIT_WORK_TREE`. With both, a scratch `git init` in a test writes `core.worktree` into the main repo's config. Strip `GIT_*` from spawned git, and if the main repo then has `core.worktree`, run `git config --unset core.worktree`.

## Writing style

Plain sentences, sentence-case headings, no em dashes, no decorative emoji.
