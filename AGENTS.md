# agent guidelines

- prefer first-party docs for libraries.
- prefer first-party docs or CLI help for tools.
- when using first-party docs, check `llms.txt` as an index when available.
- keep changes scoped to the workflow skills and supporting docs unless asked otherwise.
- after changing skills or generated skill references, and after review/approval, run `mise run install` to update installed local skills; ask whether to also run `mise run install:global` to update the global install.
