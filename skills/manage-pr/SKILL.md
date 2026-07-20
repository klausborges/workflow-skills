---
name: manage-pr
description: Create and update GitHub pull requests, including stacked PRs and the stack merge loop. Use when the user wants a PR opened or updated, work delivered as a stack, or says "let's get the PRs merged" or "merged #N".
---

# Manage PR

Titles, style, and remote naming follow the `git-conventions` skill. PRs live on the repo's primary remote; this skill supports GitHub through `gh`. An explicit repo convention (PR template, title check, contributing doc) wins over these defaults.

## Create or update

1. Inspect first: `git log <base>..HEAD --oneline`, `git diff <base>...HEAD --stat`, and `gh pr view` for existing PR state.
2. Title: `type(scope): description`.
3. Body from the template below through `--body-file`; pass explicit flags so `gh` never prompts.
4. Push the branch to the primary remote first (`git push -u <primary> <branch>`; repos usually have two remotes, and an unset upstream makes `gh` prompt).
5. Create ready for review and self-assigned: `gh pr create --assignee @me --base <base> --title <title> --body-file <file>`.
6. Fill Verification only with commands actually run and their results.

Template (omit Notes when empty):

    ## Summary

    - 2-5 plain bullets: what changed and why.
    - Stacked on #N. (stacked PRs only)

    ## Verification

    - Commands run and their results.

    ## Notes

    - Risk, migration order, or review pointers.

## Stacks

When work spans several PRs, chain branches bottom-up: the first PR bases on the default branch, each later PR bases on the previous branch. Keep every PR independently green and order the stack so it reads as an argument.

## Merge loop

On "let's get the PRs merged": map the stack bottom-up, record every branch's current tip, hand back the bottom PR's link, and wait. On each "merged #N" (any merge method, often squash):

1. Fetch the primary remote and fast-forward the local default branch.
2. Rebase the next branch onto the fresh default using the recorded tips: `git rebase --onto <default> <old-base-tip> <branch>`. Then rebase each remaining branch onto its rewritten parent the same way: `git rebase --onto <parent> <old-parent-tip> <branch>`. Record the new tips.
3. On conflict, stop and report; a half-resolved rebase never gets pushed.
4. Confirm each branch's content survived: `git range-diff <old-base-tip>..<old-tip> <new-base>..<branch>` shows only the base change. Push with `--force-with-lease` only then.
5. Retarget the next PR (`gh pr edit <next> --base <default>`), reply with its link, and wait for the next "merged #N". Resync this way even when the merge was not a squash. After the last PR, confirm the stack is complete and offer local cleanup (a branch checked out in a worktree needs `git worktree remove` first).

## Guardrails

- Merging is the user's action; this skill never merges.
- Resolve the primary remote by name; a remote called `origin` does not exist in these repos.
- Keep plan and milestone context out of titles and bodies.

## Output

The PR URL, or during the merge loop the next PR's link and remaining stack.
