# Workflow skills

## The loop

The loop, its routes and the project slots live in [`use-workflow`](../skills/use-workflow/SKILL.md). The vocabulary lives in [`workflow-language.md`](../skills/_shared/workflow-language.md).

## Packaging contract

Each skill's `skill.toml` declares the references it ships:

```toml
[references]
shared = [
  "workflow-language",
]
owned = []
```

A shared name maps to `skills/_shared/<name>.md` and is generated at `skills/<skill>/references/<name>.md`, so each skill installs on its own. An owned reference lives only in its skill's `references/` directory. A reference that one skill needs is owned by that skill.

`wflow refs sync` writes the generated copies. `wflow refs verify` checks manifest names, package safety, generated equality, owned files, links from `SKILL.md` to declared references, and the `agents/openai.yaml` metadata.

To change a reference, edit the shared file or the owned one, keep the manifest lists sorted, then run `mise run sync-references` and `mise run check`.

To remove a shared reference, delete `skills/_shared/<name>.md`, drop it from every `skill.toml`, fix the links, and delete every `skills/*/references/<name>.md` by hand before syncing.

## Boundaries

- Explanation, diagnosis and review change nothing unless the owner asks for a fix.
- Planning does not start implementation.
- A plan never holds current truth; docs, code, commits and PRs stay coherent after plans are removed.
- A throwaway probe does not become production code without the owner's go-ahead.
