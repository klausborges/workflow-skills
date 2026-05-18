---
name: review-work
description: Review implementation work against a Plan, Target-State Docs, acceptance criteria, tests, and code quality expectations. Use when the user asks for review, repeated review rounds, adversarial review, multi-aspect/subagent review, compliance review, architecture/test/library/performance/security focused review, or review-fix planning.
---

# Review Work

Review implementation work with evidence and technical judgment.

Use workflow terms from [workflow-language.md](references/workflow-language.md).
Use [document-conventions.md](references/document-conventions.md), [research-ladder.md](references/research-ladder.md), and [architecture-language.md](references/architecture-language.md) when relevant.

## Default Style

Default to balanced review with adversarial checks.

- Start from the Plan, related Target-State Docs, and acceptance criteria.
- Check whether related Target-State Docs still match the implemented behavior.
- Treat implementation as untrusted until verified.
- Prioritize real bugs, missing requirements, doc drift, test gaps, and maintainability risks.
- Use adversarial depth for high-risk surfaces or when requested.

Supported overrides include "quick review", "review this hard", "adversarial review", "paranoid review", "compliance review against the plan", "focus on architecture", and "focus on tests".

## Review Boundary

Establish the review boundary before judging findings.

- Plan-backed review: use the Plan, selected phase when provided, related Target-State Docs, acceptance criteria, tests, and current changed files unless the user gives a narrower diff range.
- Explicit ad hoc target: use what the user pointed at as the boundary, even without a Plan.
- Ambiguous or too-broad ad hoc review: ask what to review, such as a file set, feature area, branch, commit, comparison point, or current uncommitted diff.
- Non-code review: keep the same evidence-led shape, adapted to the artifact.

Do not silently choose a broad boundary for substantial reviews.

## Multi-Aspect Review

Use when the user asks for multiple review angles or subagent review.

1. Infer a useful aspect set from the change and ask for confirmation.
2. If the user specified aspects, suggest any missing useful ones.
3. Dispatch or simulate focused reviewer passes per aspect when useful.
4. Reconcile duplicated or conflicting findings.
5. Verify findings against code, docs, tests, and source documentation where needed.
6. Report only findings that survive reconciliation and verification.
7. Recommend the follow-up route: direct patch, Plan-backed `implement-plan`, or `Review-Fix Handoff` only when fresh-context transfer is needed.

Possible aspects:

- code quality
- Plan/Doc/acceptance compliance
- library/API correctness
- performance risks
- test coverage and quality
- security or data-loss risks
- architecture and maintainability

## Review Signal And Rounds

Track Review Signal for every review, including ad hoc reviews.

Before judging findings, establish the scope, round when inferable, review mode, and finding threshold. If this is a repeated round, focus on the delta since accepted fixes plus unresolved high-risk surfaces.

Signal levels:

- `High`: blocking findings, missed requirements, doc drift, failed or missing verification, likely regression, incorrect tests, unsafe closeout, or user-visible correctness risk.
- `Medium`: valid findings remain but are localized and likely patchable; patch and consider one more scoped review.
- `Low`: remaining findings are minor, speculative, stylistic, already-deferred, or better routed to follow-up or architecture work; suggest closeout unless the user requests another pass.

Repeated-round rules:

- Round 1 may be broad.
- Round 2 and later require concrete evidence and closeout relevance: correctness, docs, tests, verification, regression risk, or user-visible behavior.
- One low-signal round after significant patches can justify closeout; offer one final narrow round only when the changed surface remains risky.
- Two consecutive low-signal rounds should strongly recommend closeout.
- Do not keep widening scope unless new evidence appears.

Subagents or clean-context reviewers are preferred for final closeout. If unavailable, say the same-agent review is weaker and recommend a fresh session or handoff for high-risk final review unless the user explicitly accepts same-context review.

## Finding Classification

Classify surviving findings after verification:

- `blocking`: prevents closeout.
- `important`: should be patched before closeout, but is localized.
- `defer/follow-up`: valid, but outside this review boundary or not required for closeout.
- `architecture recommendation`: real architecture friction; recommend `improve-architecture` instead of review-fix work unless it blocks the current target.
- `rejected/noisy`: reviewed and not accepted as a finding.

Findings should have concrete evidence, why it matters, and a recommended fix. For regression-risk findings, require a specific affected behavior, plausible failure path, and evidence from changed tests, touched consumers, public contracts, or verification gaps.

## Follow-Up

Verify reviewer claims before patching. Push back with evidence when a claim is wrong, and ask when an item is unclear enough that partial implementation would risk the wrong fix.

When accepted findings should be fixed in the same context, patch them directly if the scope is small and clear. If the review is tied to an existing Plan or planned checklist, route the fix work through `implement-plan` and keep the Plan updated.

After accepted findings are patched, run targeted verification and decide whether another scoped review is needed based on Review Signal and changed surface risk.

Use `write-handoff` only when the user wants a fresh-context transfer, delegation prompt, or saved review-fix brief. Do not treat a `Review-Fix Handoff` as the default next step after every review.

## Final Doc Sync And Plan Cleanup

Use when review is tied to a Plan and implementation appears complete.

1. Confirm the relevant Plan phases and acceptance criteria are complete, or report incomplete implementation as a blocking finding.
2. Check related Target-State Docs against the implemented behavior.
3. Patch small, factual doc drift directly when implementation evidence is clear.
4. Report substantive, ambiguous, or product-sensitive doc drift as a blocking finding and recommend a decision or Plan-backed implementation fix.
5. Do not ask to delete a Plan while blocking findings, incomplete implementation, incomplete verification, or doc drift remain.
6. After full implementation and review pass with Target-State Docs current, ask the user whether to delete the completed Plan. Never delete a Plan automatically.

## Output

Lead with findings, ordered by severity. Include file/line references when possible.

For each finding:

- classification/severity
- evidence
- why it matters
- recommended fix

After findings, include:

- review metadata: scope, round when inferable, review mode, and finding threshold
- verification run or skipped
- Target-State Doc sync status
- Plan cleanup prompt status when reviewing completed Plan-backed work
- Review Signal: high, medium, or low
- next recommendation: patch, rerun scoped review, close out, or route to `improve-architecture`
- residual risks
