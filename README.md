# workflow skills

personal agent workflow skills for codebase explanation, diagnosis, planning, implementation, handoff, review, architecture improvement, and small planning prototypes. not a framework, not a ceremony generator, mostly a set of rails i kept wanting back.

inspired by [matt pocock's skills](https://github.com/mattpocock/skills), [superpowers](https://github.com/obra/superpowers), and built to work with [vercel labs skills](https://github.com/vercel-labs/skills).

fork it if your planning/review language is different. the useful parts should feel like shop tools, not house rules.

## after install

ask `explain how the workflow skills work`.

if the agent does not pick it up, be explicit: `use explain-workflow to show me how this workflow works`.

then try a concrete prompt:

- `use plan-work to plan this feature`
- `explain how this code path works`
- `diagnose this failing test`
- `use review-work to review this change`
- `use write-handoff to prepare a handoff`
- `use improve-architecture to find deepening opportunities`

## skills

| skill                                                            | purpose                                                                                                                |
| ---------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| [`explain-workflow`](./skills/explain-workflow/SKILL.md)         | explain how to use the workflow skills for a concrete situation.                                                       |
| [`explain-codebase`](./skills/explain-codebase/SKILL.md)         | explain codebase areas, modules, callers, flows, tests, and terms without editing files.                               |
| [`diagnose-issue`](./skills/diagnose-issue/SKILL.md)             | reproduce, diagnose, and fix normal bugs, failures, flaky behavior, and performance regressions.                       |
| [`use-workflow`](./skills/use-workflow/SKILL.md)                 | route substantial workflow work to the right skill.                                                                    |
| [`plan-work`](./skills/plan-work/SKILL.md)                       | research, ask questions, write target-state docs, and create an ephemeral plan.                                        |
| [`implement-plan`](./skills/implement-plan/SKILL.md)             | implement approved plan phases with verification, plan updates, and self-review.                                       |
| [`write-handoff`](./skills/write-handoff/SKILL.md)               | write fresh-context handoffs for implementation, continuation, research, refactor, bugfix, review, or review-fix work. |
| [`review-work`](./skills/review-work/SKILL.md)                   | review work against plans, docs, acceptance criteria, tests, and risk.                                                 |
| [`improve-architecture`](./skills/improve-architecture/SKILL.md) | find architecture improvements without turning review into broad refactoring.                                          |
| [`plan-prototype`](./skills/plan-prototype/SKILL.md)             | use throwaway visual or logic prototypes to answer planning questions.                                                 |

target-state docs live in [`docs/`](./docs/). canonical shared language and templates live in [`skills/_shared/`](./skills/_shared/) and are generated into each skill's `references/` directory so individual skills remain installable on their own.

## future work

these active plans are kept here as a public roadmap and examples of the workflow in use. completed plans should disappear before they become stale context.

- [`plan-prototype-dogfooding`](./plans/plan-prototype-dogfooding.md): clarify when prototypes should be shown, discarded, or absorbed.
- [`review-work-dogfooding`](./plans/review-work-dogfooding.md): tune review behavior from real project reviews.
- [`invocation-policy-dogfooding`](./plans/invocation-policy-dogfooding.md): decide whether `use-workflow` stays implicit and how provider controls should be documented.
- [`setup-quality-gates`](./plans/setup-quality-gates.md): research and plan an opinionated local quality-gate setup skill.
- [`wflow-minijinja-templating`](./plans/wflow-minijinja-templating.md): plan template rendering only if provider-specific generated files become necessary.
- [`workflow-plugin-experiments`](./plans/workflow-plugin-experiments.md): compare plugin, hook, and package experiments for OpenCode, Pi, and Factory Droid.

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

| workflow area        | local tokens | matt pocock tokens | superpowers tokens |
| -------------------- | -----------: | -----------------: | -----------------: |
| planning             |        1,994 |              2,916 |             14,948 |
| implementation       |        2,830 |              2,092 |             10,152 |
| review               |        1,814 |              1,052 |              3,210 |
| diagnosis            |        1,541 |              1,947 |             14,015 |
| architecture         |        1,226 |              3,832 |             13,186 |
| prototype            |          651 |              3,525 |             13,186 |
| codebase explanation |          782 |                 89 |                n/a |
| handoff              |        1,666 |                160 |                n/a |
| workflow routing     |        1,227 |              3,281 |              3,106 |
| workflow explanation |          910 |                n/a |              3,106 |
| total                |       14,641 |             22,894 |             88,909 |

## credits

this repo borrows selectively from [matt pocock's skills](https://github.com/mattpocock/skills) and [superpowers](https://github.com/obra/superpowers). matt's repo is a useful model for small focused skills, while superpowers is the heavier workflow loop this trims down. [vercel labs skills](https://github.com/vercel-labs/skills) shaped the packaging target through the skills CLI.

## license

MIT. see [LICENSE](./LICENSE).
