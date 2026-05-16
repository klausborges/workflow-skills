# Reference Signal Improvements Plan

## Goal

Evaluate useful signals from Matt Pocock's skills and Superpowers without bloating the workflow skills.

## Related Docs

- `README.md`
- `docs/workflow.md`
- `GLOSSARY.md`
- `plans/review-work-dogfooding.md`
- `plans/plan-prototype-dogfooding.md`

## Phase Checklist

- [x] Phase 1: Route reference signals to the right owner
- [ ] Phase 2: Add seam/interface option exploration to architecture improvement
- [ ] Phase 3: Evaluate explain-workflow selection guidance

## Verification Strategy

- Targeted tests: run generated-reference drift checks after skill/reference changes.
- Integration checks: run `npx skills add . --list` after skill changes.
- Type/lint/build: run `mise run check`.
- Manual checks: compare updated skill behavior against the reference comparison table and a small real prompt.
- Final review triggers: review for imported terminology drift, duplicated dogfooding plans, and reference-driven bloat.

## Out of Scope

- Importing "spec" as workflow vocabulary.
- Recreating Superpowers' full workflow loop.
- Adding issue-tracker, worktree, branch-finishing, or general delegation workflows.
- Changing `diagnose-issue` unless later dogfooding finds a concrete gap.
- Updating `plan-prototype` or `review-work` directly from this Plan; those signals belong to their existing dogfooding Plans.
- Creating a new Target-State Doc for this synthesis pass.

## Research Notes

- The README reference comparison table records rough `o200k_base` token counts for local, Matt Pocock, and Superpowers equivalents.
- `diagnose-issue` already has root-cause-first rules, stop signs, feedback loops, hypothesis ranking, narrow instrumentation, failed-fix escalation, and verification-before-completion language. Decision: no immediate change.
- Matt Pocock's `review` skill uses a Standards/Spec framing. Repo decision: avoid "spec"; route the useful part to `plans/review-work-dogfooding.md` as Plan/Target-State Doc/acceptance compliance.
- Matt Pocock's `prototype` skill has stronger logic-vs-UI branch guidance. Decision: route investigation to `plans/plan-prototype-dogfooding.md`.
- Matt Pocock's `design-an-interface` skill suggests generating materially different interface designs before choosing one. Decision: adapt this as seam/interface option exploration inside `improve-architecture`, not as a new skill yet.
- `explain-workflow` already has explicit/implicit invocation, Quick Recipes, ambiguous request behavior, and output shape guidance. Decision: evaluate before adding any selection matrix.

## Phase 1: Route Reference Signals To The Right Owner

Mode: AFK

### Goal

Keep the comparison signals visible without duplicating existing dogfooding Plans.

### Context

Some signals belong to active dogfooding Plans rather than a new synthesis Plan. This phase records routing and patches those Plans with the relevant research notes and tasks.

### Tasks

- [x] Patch `plans/review-work-dogfooding.md` with the Matt review signal using repo vocabulary.
- [x] Patch `plans/plan-prototype-dogfooding.md` with the Matt prototype signal.
- [x] Record `diagnose-issue` as considered but already covered for now.
- [x] Confirm no new Target-State Doc is needed.
- [x] Keep the README comparison table factual and separate from plan decisions.

### Acceptance Criteria

- [x] Review and prototype signals have a clear owner Plan.
- [x] No new "spec" workflow terminology is introduced.
- [x] `diagnose-issue` remains unchanged unless future dogfooding finds a gap.
- [x] This Plan does not duplicate dogfooding tasks already tracked elsewhere.
- [x] Plan checklist is updated with completed work and newly discovered tasks.

## Phase 2: Add Seam/Interface Option Exploration

Mode: AFK

### Goal

Teach `improve-architecture` to compare materially different seam/interface options before planning a candidate that depends on a new or changed seam.

### Context

`improve-architecture` already maps modules, callers, seams, and flows, then presents deepening candidates. The reference signal is useful when the selected candidate depends on where a seam lives or what callers must know about an interface.

### Tasks

- [ ] Update `skills/improve-architecture/SKILL.md` with a conditional seam/interface option branch after the user picks a candidate.
- [ ] Require 2-3 materially different options only when seam or interface placement is central to the candidate.
- [ ] Compare options by caller impact, hidden complexity, migration cost, testability, and fit with current Glossary/ADRs.
- [ ] Keep broad rewrites and standalone `design-interface` skill creation out of scope.
- [ ] Run reference sync or verification if metadata/references change.
- [ ] Run repository checks.

### Acceptance Criteria

- [ ] `improve-architecture` can ask for or present multiple seam/interface options before a focused improvement plan.
- [ ] The branch is conditional and does not burden every architecture improvement.
- [ ] Existing architecture vocabulary remains consistent with `architecture-language.md`.
- [ ] `mise run check` passes.
- [ ] Implementation self-review completed.
- [ ] Plan checklist is updated with completed work and newly discovered tasks.

## Phase 3: Evaluate Explain-Workflow Selection Guidance

Mode: Mixed

### Goal

Decide whether `explain-workflow` needs a compact selection table or whether Quick Recipes already cover the use case.

### Context

`explain-workflow` should stay short and practical, not become a second README. The reference comparison suggests stronger skill-selection guidance may be useful, but the current skill already covers explicit and implicit invocation plus common recipes.

### Tasks

- [ ] Compare current `explain-workflow` output against the README skills list and Quick Recipes.
- [ ] Decide whether a compact "if you want X, use Y" table would reduce confusion or duplicate existing guidance.
- [ ] If useful, add only the smallest selection guidance that preserves the skill's short-answer default.
- [ ] If not useful, record no-op rationale in this Plan.
- [ ] Run repository checks if the skill changes.

### Acceptance Criteria

- [ ] `explain-workflow` remains concise and does not become a second README.
- [ ] Users can still ask naturally, such as "explain how the workflow skills work."
- [ ] Explicit and implicit invocation guidance remains clear.
- [ ] If no change is made, the no-op decision is documented.
- [ ] Plan checklist is updated with completed work and newly discovered tasks.

## Plan Self-Review

Status: PASS

### Findings

- No blocking findings. The Plan routes review/prototype signals to existing dogfooding Plans and keeps direct ownership narrow.
- No blocking findings. The Plan avoids importing "spec" despite the reference source using that framing.
- No blocking findings. The architecture work is conditional and uses existing seam/interface vocabulary.

### Plan Updates

- Tasks checked: Phase 1 tasks.
- Acceptance criteria checked: Phase 1 acceptance criteria.
- Tasks added/moved: review and prototype signals moved to their dogfooding Plans.

### Verification

- Read `GLOSSARY.md`, `docs/workflow.md`, existing dogfooding Plans, current skill bodies, and README reference comparison.
- No code checks run; this is planning and plan-doc updates only.

## Discovered Follow-Ups

- Consider a separate `design-interface` skill only if repeated architecture dogfooding shows the seam/interface branch deserves standalone invocation.
