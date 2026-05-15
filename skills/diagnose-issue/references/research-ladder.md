# Research Ladder

Research before serious technical recommendations, but keep findings scoped and synthesized.

Use better tools when available or requested by the prompt, `AGENTS.md`, or local project instructions:

- docs MCPs or documentation tools
- Exa
- `gh` CLI
- `gh_grep` MCP
- other project-specific research tools

Do not require those tools. Fall back to sane defaults:

- first-party docs
- `llms.txt` when available as a docs index
- official repositories and official examples
- CLI help such as `tool --help`
- local docs and existing project examples

Behavior:

- Check cheap local/first-party sources first when useful.
- Use official examples and real repo examples when API usage or integration patterns matter.
- Use broader search tools when official docs are incomplete, behavior is uncertain, or current information matters.
- Summarize findings into decisions, constraints, and trade-offs.
- Avoid long research dumps.
- Keep research notes in the Plan unless they describe durable target-state behavior.
