# workflow skills

personal agent workflow skills for planning, implementation, review, the lean pass, diagnosis, codebase explanation, handoffs, ui design, and git. not a framework, not a ceremony generator, mostly a set of rails i kept wanting back.

inspired by [matt pocock's skills](https://github.com/mattpocock/skills), [superpowers](https://github.com/obra/superpowers), and built to work with [vercel labs skills](https://github.com/vercel-labs/skills).

fork it if your planning/review language is different. the useful parts should feel like shop tools, not house rules.

## after install

ask `how do I use the workflow skills for <task>?`. you get the next prompt to run, and nothing starts.

if the agent does not pick it up, be explicit: `use use-workflow to explain how to plan this feature`.

then try a concrete prompt:

- `use plan-work to plan this feature`
- `use implement-work to implement this brief`
- `explain how this code path works`
- `diagnose this failing test`
- `use review-work to review this change`
- `use write-handoff to write a compaction prompt`
- `run a lean pass on this stack`

## skills

| skill                                                            | purpose                                                             |
| ---------------------------------------------------------------- | ------------------------------------------------------------------- |
| [`use-workflow`](./skills/use-workflow/SKILL.md)                 | the work loop, its routes, and how to use these skills.             |
| [`plan-work`](./skills/plan-work/SKILL.md)                       | plans of decisions, phases and a gate, plus throwaway probes.       |
| [`write-handoff`](./skills/write-handoff/SKILL.md)               | briefs, handoffs, compaction prompts, resumption records.           |
| [`implement-work`](./skills/implement-work/SKILL.md)             | execute a decided brief, red before green.                          |
| [`review-work`](./skills/review-work/SKILL.md)                   | read-only review of plans, changes, confirmation rounds and stacks. |
| [`improve-architecture`](./skills/improve-architecture/SKILL.md) | the lean pass and owner-requested simplifications.                  |
| [`diagnose-issue`](./skills/diagnose-issue/SKILL.md)             | find a failure's cause; fix only when authorized.                   |
| [`explain-codebase`](./skills/explain-codebase/SKILL.md)         | map how a code area works, read-only.                               |
| [`coding-standards`](./skills/coding-standards/SKILL.md)         | design, test and evidence standards.                                |
| [`design-ui`](./skills/design-ui/SKILL.md)                       | tokens and a core set before screens, drawn from the product.       |
| [`git-conventions`](./skills/git-conventions/SKILL.md)           | remote names, commit shape, worktree placement, writing style.      |
| [`commit-work`](./skills/commit-work/SKILL.md)                   | conventional commit messages; commits on request.                   |
| [`manage-pr`](./skills/manage-pr/SKILL.md)                       | PRs and gh stacks, restacking after merges.                         |
| [`manage-worktrees`](./skills/manage-worktrees/SKILL.md)         | create, reuse and remove worktrees.                                 |

repo docs live in [`docs/`](./docs/). canonical shared references live in [`skills/_shared/`](./skills/_shared/) and are generated into each skill's `references/` directory so individual skills remain installable on their own.

## future work

active development planning and the public Roadmap live under [`plans/`](./plans/).

## install from github

install the collection:

```bash
npx skills add klausborges/workflow-skills --skill '*' -a claude-code codex -y
```

install one skill:

```bash
npx skills add klausborges/workflow-skills --skill plan-work -a claude-code codex -y
```

## local install

list local skills:

```bash
npx skills add . --list
```

install all local skills into this repo for claude code and codex:

```bash
mise run install
```

install local skills globally for claude code and codex:

```bash
mise run install:global
```

install local skills into another repo:

```bash
cd /path/to/other/repo
npx skills add /path/to/this/repo --skill '*' -a claude-code codex -y
```

install one local skill:

```bash
npx skills add . --skill plan-work -a claude-code codex -y
```

## skills.sh

once indexed, the collection should be available at [`skills.sh/klausborges/workflow-skills`](https://skills.sh/klausborges/workflow-skills).

[![skills.sh](https://skills.sh/b/klausborges/workflow-skills)](https://skills.sh/klausborges/workflow-skills)

## validate

local validation uses Rust/Cargo for repo maintenance checks. installed skills do not require Rust.

```bash
npx skills add . --list
mise run check
```

format owned markdown:

```bash
mise run format
```

sync generated skill references:

```bash
mise run sync-references
```

install pre-commit hooks:

```bash
mise run prek:install
```

## rust-analyzer

The repository launcher keeps rust-analyzer's Cargo work inside the mise-managed
Rust and Mr Boxington environment. Launch Helix from the repository root so its
project-local language configuration can find the launcher:

```bash
mise exec helix@25.07.1 -- hx .
```

Helix is optional and is not part of the repository toolset. By default,
rust-analyzer shares the workspace's `target/` directory. To avoid target-lock
contention with another Cargo process, give the analyzer a separate target:

```bash
WORKFLOW_SKILLS_RA_SEPARATE_TARGET=1 mise exec helix@25.07.1 -- hx .
```

The separate target uses `target/rust-analyzer` and costs extra disk space.
Switching back to the shared target does not delete its artifacts.

Other LSP clients can run `./scripts/rust-analyzer` from the repository root.
Their initialization options must set
`cargo.buildScripts.useRustcWrapper=false` so rust-analyzer does not replace
Mr Boxington's compiler wrapper. Leave `cargo.targetDir` unset (or `null`) so
the launcher controls target selection. Keep `checkOnSave=true` for normal
Cargo diagnostics, or set it to `false` when the client should not run checks
on save. Do not set `check.overrideCommand`; rust-analyzer should construct its
normal Cargo command.

## reference comparison

rough `o200k_base` token counts for equivalent workflow areas. equivalent skills are approximate because the repos split workflow ideas differently.

the goal is to use references deliberately and keep installed skills small enough to load well, not to turn the repo into benchmaxxing theater.

local counts cover each skill's `SKILL.md` plus packaged references; `skill.toml` is excluded.

| workflow area        | local tokens | matt pocock tokens | superpowers tokens |
| -------------------- | -----------: | -----------------: | -----------------: |
| planning             |        1,064 |              3,099 |              7,986 |
| implementation       |        1,344 |              1,622 |             21,924 |
| review               |        1,626 |              1,521 |              3,592 |
| diagnosis            |          686 |              2,374 |             12,050 |
| architecture         |        1,118 |              5,433 |        in planning |
| prototype            |  in planning |              3,546 |        in planning |
| codebase explanation |           94 |                n/a |                n/a |
| handoff              |        1,318 |                198 |                n/a |
| workflow routing     |        1,135 |              5,073 |              4,886 |
| git workflow         |        1,199 |                752 |              1,623 |
| coding standards     |          575 |                n/a |                n/a |
| total                |       10,159 |             23,618 |             52,061 |

matt pocock's skills are counted at 49dd158 and superpowers at v7.0.0. superpowers counts cover markdown only and exclude its scripts. superpowers' brainstorming skill covers architecture and prototyping, so it is counted once, in planning. the totals in earlier versions of this table were mis-summed.

## credits

this repo borrows selectively from [matt pocock's skills](https://github.com/mattpocock/skills), [superpowers](https://github.com/obra/superpowers), [cursor's pstack, thermos, and team-kit plugins](https://github.com/cursor/plugins), and [dmmulroy's skills](https://github.com/dmmulroy/skills). their implementation, simplification, domain-modeling, and review ideas are adapted here as concise heuristics rather than imported workflows. [vercel labs skills](https://github.com/vercel-labs/skills) shaped the packaging target through the skills CLI.

## license

MIT. see [LICENSE](./LICENSE).
