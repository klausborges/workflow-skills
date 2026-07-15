---
name: review-work
description: Read-only review of Plans, implementation, continuation rounds, or other scoped artifacts against intended behavior, docs, acceptance criteria, tests, and risk. Use for balanced, adversarial, multi-aspect, compliance, architecture, test, library, performance, or security review.
---

# Review Work

Review the requested artifact with evidence and technical judgment. This skill is read-only unless the user explicitly asks for fixes or implementation.

Use [workflow-language.md](references/workflow-language.md), [review-language.md](references/review-language.md), [document-conventions.md](references/document-conventions.md), and, when relevant, [research-ladder.md](references/research-ladder.md) or [architecture-language.md](references/architecture-language.md).

## Establish scope

Identify the primary target, comparison point, boundary, and requested intensity before judging findings. For branch or diff review, resolve the comparison point and confirm the target scope is non-empty before expensive analysis. Ask only when the boundary cannot be discovered safely.

- **Plan Review Mode**: determine whether a Plan is coherent, executable, scoped, and verifiable. Check dependencies, Task-to-acceptance coverage, prerequisites, locked-decision contradictions, cross-Plan ownership, review placement, verification, and doc lifecycle. Do not demand implementation evidence, prose polish, exhaustive edge cases, or architecture redesign unless the Plan cannot execute safely.
- **Implementation Review Mode**: start from the concrete code, tests, docs, diff, or delivered artifact. Use relevant Plans and current docs as reference truth, not as the primary artifact.
- **Continuation Review**: focus on accepted fixes, patch notes, changed surface, unresolved risk, and settled items that should not be relitigated.
- **Independent review**: use for explicit cross-validation or the mandatory fresh-context implementation review. Inspect primary evidence before prior findings or author conclusions when practical; reconcile them afterward. It may overlap prior findings because independence is intentional.
- **Ad hoc review**: adapt the same evidence-led method to the file set, feature, branch, commit, or non-code artifact the user named.

## Review method

1. Read repo instructions and authoritative artifacts.
2. Inspect the actual target and its callers, tests, docs, or runtime behavior as risk warrants. When changes touch environment variables, secrets, ports, scripts, or setup commands, trace the affected developer run/build path as a caller-visible contract.
3. Verify assumptions with code, diffs, schemas, commands, first-party docs, or focused runtime checks.
4. Look for correctness failures, missing requirements, unsafe lifecycle behavior, doc drift, test gaps, security/data-loss risk, and material maintainability issues.
5. Reconcile duplicate or conflicting evidence before reporting.

For a fresh implementation review, include a calibrated structural-simplification pass: check for avoidable concepts, branches, mutable state, indirection, or a materially simpler design. Report only concrete, proportionate improvements; this is not a separate reviewer, fixed size threshold, or presumptive blocker.

Use blast radius as an optional lens when a change crosses shared contracts, persistence, security, concurrency, deployment, or a widely used interface. When used, trace affected callers and consumers in proportion to risk, identify the one or two invariants the change's safety depends on, and prove them through the cheapest stable real surface. Report anything short of proof as residual risk; do not perform a repo-wide impact scan by default.

Default to balanced review. Add adversarial depth, multiple lenses, subagents, research, or extra rounds only when the user asks or risk and uncertainty materially justify the cost. Infer useful lenses without asking unless the choice materially changes scope, latency, or expense.

Subagent reports are leads. The main reviewer verifies each substantive accepted claim against primary evidence. Do not recursively commission reviews of the review.

There is no finding minimum. Consolidate repeated issues and normally keep the actionable set below about 15. A clean review is valid when supported by adequate coverage.

## Mandatory Phase review

For an end-of-Phase or direct-implementation review:

- the independent reviewer must receive fresh context;
- the scope is the entire Phase or Phase-equivalent implementation, not only the latest patch;
- Plan-defined high-impact checkpoints do not replace this review;
- findings are fixed only when the user has separately authorized implementation;
- after fixes, rerun only the review coverage justified by the changed surface and residual risk.

## Docs and artifact lifecycle

For Plan-backed implementation, check only drafts and Doc Delta items owned by the current Plan. Unrelated drafts may be noted but do not block closeout.

Report doc drift as a finding; do not silently edit it. Small factual drift can be recommended as a direct fix. Product-sensitive or substantive drift blocks closeout until decided or implemented.

Do not offer Plan deletion while blocking findings, incomplete verification, owned doc work, or incomplete implementation remain. When a completed Plan is not in an active Roadmap, ask once after final review and doc sync whether to delete it. If an active Roadmap references it, report completion readiness and leave Plan/Roadmap mutation to a separately authorized workflow step; Roadmap closeout gates cleanup. Never delete automatically.

## Output

Lead with verified findings ordered by severity. Each finding includes tight file/line or command evidence, why it matters, concrete impact, and a recommended fix. Distinguish clear defects, reasonable trade-offs, and owner-policy decisions.

For substantive reviews, report a compact Review Ledger:

- Scope and verification
- Review Signal: `High`, `Medium`, or `Low`
- Recommendation
- Residual risk

Add Plan, doc, round, reconciliation, or next-review fields only when applicable. For a trivial ad hoc check, a concise conclusion with scope and verification is enough. Do not produce `N/A` scaffolding or long transcripts. When no findings survive, say so and state coverage and limitations.
