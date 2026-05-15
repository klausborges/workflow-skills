# workflow skills

personal agent workflow skills for planning, implementation, handoff, review, architecture improvement, and small planning prototypes. not a framework, not a ceremony generator, mostly a set of rails i kept wanting back.

[![skills.sh](https://skills.sh/b/klausborges/workflow-skills)](https://skills.sh/klausborges/workflow-skills)

inspired by [matt pocock's skills](https://github.com/mattpocock/skills), [superpowers](https://github.com/obra/superpowers), and [vercel labs skills](https://github.com/vercel-labs/skills).

the goal is not to mirror those repos. this repo keeps the pieces that fit a concise, explicit, handoff-friendly workflow.

fork it if your planning/review language is different. the useful parts should feel like shop tools, not house rules.

## after install

ask `explain how the workflow skills work`.

if the agent does not pick it up, be explicit: `use explain-workflow to show me how this workflow works`.

then try a concrete prompt:

- `use plan-work to plan this feature`
- `use review-work to review this change`
- `use write-handoff to prepare a handoff`
- `use improve-architecture to find deepening opportunities`

## skills

| skill                                                            | purpose                                                                                                                |
| ---------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| [`explain-workflow`](./skills/explain-workflow/SKILL.md)         | explain how to use the workflow skills for a concrete situation.                                                       |
| [`use-workflow`](./skills/use-workflow/SKILL.md)                 | route substantial workflow work to the right skill.                                                                    |
| [`plan-work`](./skills/plan-work/SKILL.md)                       | research, ask questions, write target-state docs, and create an ephemeral plan.                                        |
| [`implement-plan`](./skills/implement-plan/SKILL.md)             | implement approved plan phases with verification, plan updates, and self-review.                                       |
| [`write-handoff`](./skills/write-handoff/SKILL.md)               | write fresh-context handoffs for implementation, continuation, research, refactor, bugfix, review, or review-fix work. |
| [`review-work`](./skills/review-work/SKILL.md)                   | review work against plans, docs, acceptance criteria, tests, and risk.                                                 |
| [`improve-architecture`](./skills/improve-architecture/SKILL.md) | find architecture improvements without turning review into broad refactoring.                                          |
| [`plan-prototype`](./skills/plan-prototype/SKILL.md)             | use throwaway visual or logic prototypes to answer planning questions.                                                 |

target-state docs live in [`docs/`](./docs/). canonical shared language and templates live in [`skills/_shared/`](./skills/_shared/) and are generated into each skill's `references/` directory so individual skills remain installable on their own.

## install

list local skills:

```bash
npx skills add . --list
```

install all skills into this repo for claude code and codex:

```bash
mise run install
```

install globally for claude code and codex:

```bash
mise run install:global
```

install into another repo from this checkout:

```bash
cd /path/to/other/repo
npx skills add /path/to/this/repo --skill '*' -a claude-code codex -y
```

install one skill:

```bash
npx skills add . --skill plan-work -a claude-code codex -y
```

## github install

install from GitHub:

```bash
npx skills add klausborges/workflow-skills --skill '*' -a claude-code codex -y
```

direct github urls also work:

```bash
npx skills add https://github.com/klausborges/workflow-skills --skill plan-work -a claude-code codex -y
```

## skills.sh

once indexed, the collection should be available at [`skills.sh/klausborges/workflow-skills`](https://skills.sh/klausborges/workflow-skills).

## validate

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

## credits

this repo borrows workflow ideas from [matt pocock's skills](https://github.com/mattpocock/skills), [superpowers](https://github.com/obra/superpowers), and [vercel labs skills](https://github.com/vercel-labs/skills). the structure is intentionally smaller and easier to fork.

## license

MIT. see [LICENSE](./LICENSE).
