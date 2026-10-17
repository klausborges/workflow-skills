---
name: manage-pr
description: Open and update GitHub PRs and gh stacks, restack after a merge, and leave work ready for the owner to merge. Use when a PR or a stack is to be opened or updated, or when the owner says "merged #N".
---

# Manage PR

Titles and style follow git-conventions. PRs go to the repo's primary remote, by the name the repo gives it; never assume `origin`, which a repo may define as canonical.

The owner merges. Agents never merge PRs, including `gh stack merge`; "merge what's needed" means leave it ready. PRs open as drafts and are marked ready once reviewed and complete, since a draft blocks the merge by itself.

## Body

- `Closes #N` when there is an issue.
- Parent branch and head SHA.
- Review: base and head, with each model line's verdict, or "single model line" when the owner ruled one.
- Gate: the command on that head, with its executed test count.
- Unqualified: what the gate and the reviews did not exercise.

For images and video, reference the file by path in the body and pass the same path with `--attach`; when an upload fails, the PR is still created and the command exits non-zero.

## Stacks

`gh stack` is the default stack tool, also for unrelated changes done in sequence: `gh stack init` or `gh stack add`, `gh stack push --remote <primary>`, `gh pr create --draft --base <parent>` per branch, then `gh stack link` bottom to top.

Restacking the stack's own branches needs no go-ahead. When anything under the stack changes, including after "merged #N":

1. Record the trunk tip and each branch's head.
2. Run `gh stack rebase --remote <primary>`.
3. On each branch, run `git range-diff <old-parent-tip>..<old-tip> <new-parent>..<branch>` with the heads from step 1, and show that only the base moved.
4. Gate the new top head.
5. Run `gh stack push --remote <primary>`.

A conflict, or a range-diff showing more than a base move, means more changed than the base: stop and report. `gh stack sync` stays out because it pushes before the range-diff and the gate. Where the repo forbids force pushes over reviewed work, merge the parent into the children instead.

## Gotchas

- Adding commits to a PR already handed over changes what the owner reviewed: it gets nothing more except a restack whose range-diff shows only the base moved. Restacking and adding PRs to the stack stay fine.
- A PR left in an old merged stack can't be relinked until it is unstacked, which is the owner's call: list it as a blocker.
- Independent PRs can each merge cleanly and still conflict together: check the combined merge of all open heads, not only pairs.
- Attribution trailers reach main through squash bodies: strip them before handing over.
