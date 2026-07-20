# Roadmap: Workflow skills

Ordering: Loose

## Plans

- [ ] [Improve plan-prototype references](./plan-prototype-reference-improvements.md) — refine prototype guidance from external reference workflows.
- [ ] [Dogfood plan-prototype](./plan-prototype-dogfooding.md) — exercise the prototype flow in real planning sessions.
- [ ] [Dogfood invocation policy](./invocation-policy-dogfooding.md) — decide implicit routing and provider-control guidance.
- [ ] [Plan setup quality gates](./setup-quality-gates.md) — shape an opinionated local quality-gate setup skill.
- [ ] [Evaluate MiniJinja templating](./wflow-minijinja-templating.md) — add rendering only if provider-specific generated files require it.
- [ ] [Run workflow plugin experiments](./workflow-plugin-experiments.md) — compare plugin, hook, and package approaches.
- [x] Add git workflow skills — `git-conventions`, `commit-work`, and `manage-pr` landed with remote renames across local repos.

## Candidates

- Forgejo PR skill: `fj`/API support for the forge instance once it matures.
- `fix-ci`: repair PR CI one failure at a time with `gh pr checks` as source of truth.
- `pr-follow-up`: reviewer comments and rebase loops beyond stack resync.
- `verify-this`: falsifiable baseline/treatment verification for concrete claims.
- `cli-for-agents`: design or review CLIs for non-interactive agent use.
- Canvas-style review skill: revisit after surveying what each harness offers.
