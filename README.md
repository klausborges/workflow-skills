# workflow skills

personal agent workflow skills for codebase explanation, diagnosis, planning, implementation, simplification, handoff, review, architecture improvement, and small planning prototypes. not a framework, not a ceremony generator, mostly a set of rails i kept wanting back.

inspired by [matt pocock's skills](https://github.com/mattpocock/skills), [superpowers](https://github.com/obra/superpowers), and built to work with [vercel labs skills](https://github.com/vercel-labs/skills).

fork it if your planning/review language is different. the useful parts should feel like shop tools, not house rules.

## after install

ask `explain how the workflow skills work`.

if the agent does not pick it up, be explicit: `use explain-workflow to show me how this workflow works`.

then try a concrete prompt:

- `use plan-work to plan this feature`
- `use implement-work to implement this handoff`
- `explain how this code path works`
- `diagnose this failing test`
- `use review-work to review this change`
- `use write-handoff to prepare a handoff`
- `use improve-architecture to find deepening opportunities`
- `use simplify-work to simplify this module without changing behavior`

## skills

| skill                                                            | purpose                                                                                 |
| ---------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| [`explain-workflow`](./skills/explain-workflow/SKILL.md)         | explain how to invoke the workflow without starting work by default.                    |
| [`explain-codebase`](./skills/explain-codebase/SKILL.md)         | map code areas, modules, callers, flows, tests, and terms without editing.              |
| [`diagnose-issue`](./skills/diagnose-issue/SKILL.md)             | find evidence-backed causes; apply fixes only when explicitly authorized.               |
| [`use-workflow`](./skills/use-workflow/SKILL.md)                 | route work to the smallest fitting workflow.                                            |
| [`plan-work`](./skills/plan-work/SKILL.md)                       | shape uncertain or substantial work, with an optional saved Plan.                       |
| [`implement-work`](./skills/implement-work/SKILL.md)             | implement bounded work from Plans or other inputs with evidence and fresh review.       |
| [`write-handoff`](./skills/write-handoff/SKILL.md)               | write concise fresh-context handoffs with lifecycle separate from storage.              |
| [`review-work`](./skills/review-work/SKILL.md)                   | read-only review of Plans, changes, continuation rounds, or other scoped artifacts.     |
| [`improve-architecture`](./skills/improve-architecture/SKILL.md) | discover evidence-backed architecture improvements without forcing a refactor workflow. |
| [`simplify-work`](./skills/simplify-work/SKILL.md)               | simplify a bounded area while preserving intended behavior.                             |
| [`plan-prototype`](./skills/plan-prototype/SKILL.md)             | answer one planning question with a minimal throwaway visual or logic prototype.        |
| [`git-conventions`](./skills/git-conventions/SKILL.md)           | shared remote naming, primary remote choice, commit shape, and commit/PR style.         |
| [`commit-work`](./skills/commit-work/SKILL.md)                   | conventional commit messages from local evidence; commits on request.                   |
| [`manage-pr`](./skills/manage-pr/SKILL.md)                       | create/update GitHub PRs, stacked PRs, and the stack merge loop.                        |
| [`manage-worktrees`](./skills/manage-worktrees/SKILL.md)         | create, reuse, and clean up git worktrees consistently.                                 |
| [`coding-standards`](./skills/coding-standards/SKILL.md)         | front door to the shared engineering standards, with room for stack-specific siblings.  |

target-state docs live in [`docs/`](./docs/). canonical shared references live in [`skills/_shared/`](./skills/_shared/) and are generated into each skill's `references/` directory so individual skills remain installable on their own.

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

## reference comparison

rough `o200k_base` token counts for equivalent workflow areas. equivalent skills are approximate because the repos split workflow ideas differently.

the goal is to use references deliberately and keep installed skills small enough to load well, not to turn the repo into benchmaxxing theater.

local counts cover each skill's `SKILL.md` plus packaged references; `skill.toml` is excluded.

| workflow area        | local tokens | matt pocock tokens | superpowers tokens |
| -------------------- | -----------: | -----------------: | -----------------: |
| planning             |        2,286 |              2,916 |             14,948 |
| implementation       |        2,276 |              2,092 |             10,152 |
| review               |        2,879 |              1,052 |              3,210 |
| diagnosis            |          750 |              1,947 |             14,015 |
| architecture         |        1,376 |              3,832 |             13,186 |
| simplification       |        1,077 |                n/a |                n/a |
| prototype            |          766 |              3,525 |             13,186 |
| codebase explanation |          257 |                 89 |                n/a |
| handoff              |        1,413 |                160 |                n/a |
| workflow routing     |        1,197 |              3,281 |              3,106 |
| git workflow         |        1,779 |                792 |              1,896 |
| workflow explanation |          893 |                n/a |              3,106 |
| coding standards     |          713 |                n/a |                n/a |
| total                |       17,662 |             23,686 |             90,805 |

## credits

this repo borrows selectively from [matt pocock's skills](https://github.com/mattpocock/skills), [superpowers](https://github.com/obra/superpowers), [cursor's pstack, thermos, and team-kit plugins](https://github.com/cursor/plugins), and [dmmulroy's skills](https://github.com/dmmulroy/skills). their implementation, simplification, domain-modeling, and review ideas are adapted here as concise heuristics rather than imported workflows. [vercel labs skills](https://github.com/vercel-labs/skills) shaped the packaging target through the skills CLI.

## license

MIT. see [LICENSE](./LICENSE).
