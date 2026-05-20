# Plan Prototype Reference Improvements Plan

## Goal

Improve `plan-prototype` now from reference research, without waiting for several days of new dogfooding.

## Related Docs

- `docs/workflow.md`
- `GLOSSARY.md`
- `plans/plan-prototype-dogfooding.md`

## Phase Checklist

- [ ] Phase 1: Consolidate reference-backed target behavior
- [ ] Phase 2: Update prototype guidance
- [ ] Phase 3: Prepare dogfooding loop

## Verification Strategy

- Test/verification cadence: Acceptance gate.
- Targeted tests: run generated-reference checks if shared references change.
- Integration checks: run `npx skills add . --list` after skill changes.
- Type/lint/build: run `mise run check`.
- Token checks: refresh README token counts after installed skill text changes.
- Manual checks: inspect prompt shape against visual, logic, diagram, and option-comparison prototype scenarios.
- Final review triggers: review for accidental coupling to Superpowers' server, one local browser tool, or Matt's exact route/switcher shape.

## Out of Scope

- Building a Superpowers-style visual companion server.
- Building `wflow` prototype scaffolding or reusable prototype generators in this pass.
- Requiring every planning task to produce a prototype.
- Requiring browser output for every UI-related question.
- Adding repo-local app scaffolding or prototype scripts.
- Treating prototype code as production-ready implementation.
- Folding invocation-policy decisions into this Plan.

## Research Notes

- Current repo decision: `plan-prototype` stays tooling-neutral and uses existing project tooling when possible.
- Current repo decision: prototype output answers one planning question, then gets deleted or absorbed.
- Matt Pocock reference signal: the question chooses the branch. Logic/state questions want a runnable terminal probe; UI appearance questions want several radically different variants that are easy to compare.
- Matt Pocock reference signal: UI prototypes are best judged inside the real surrounding page/flow when possible, with real density and data, not isolated blank routes.
- Matt Pocock reference signal: logic prototypes should keep the validated logic behind a portable pure interface while the TUI/shell remains throwaway.
- Matt Pocock implementation model: the skill instructs the agent to use the host project's existing runtime and task runner. It does not ship a scaffolder; it tells the agent to create a throwaway route, URL-param switcher, or one-command terminal app inside the project.
- Superpowers visual companion signal: use visual output only when the user understands better by seeing than reading; a UI topic can still require terminal discussion if the question is conceptual.
- Superpowers visual companion signal: browser or visual surface is display; terminal remains the primary conversation channel. Visual feedback can supplement, but should not block conversation.
- Superpowers visual companion signal: use a small number of options, scale fidelity to the question, include the question on the screen, use real content when it affects judgment, and clear or retire stale visuals when the discussion returns to terminal.
- Superpowers implementation model: the project ships a browser companion with scripts, a server, HTML fragments, and event files. That is real infrastructure, not just prose guidance.
- Repo-fit decision: adopt the decision heuristics, showing/feedback guidance, variant quality bar, cleanup rules, and terminal-primary feedback model; do not adopt a persistent companion server, mandatory visual loop, or `wflow` scaffolding yet.
- Initial inspection surfaces: terminal/text, screenshot or image artifact, local URL or browser route, one-command TUI/script, generated diagram file, and file-path artifact. Each prototype should state which surface the user should inspect before asking for feedback.

## Phase 1: Consolidate Reference-Backed Target Behavior

Mode: HITL

### Goal

Lock the target behavior for the first improved `plan-prototype` pass.

### Context

The current skill already has the right broad categories, but it is terse. It does not yet make the "show it vs discuss it" decision explicit, and it does not strongly distinguish visual comparison, logic probes, diagrams, and conceptual option canvases.

### Test/Verification Cadence

- Cadence: Acceptance gate
- Target behavior: accepted prototype guidance decisions
- Test surface: Plan, current `plan-prototype`, Matt prototype reference, Superpowers visual companion reference
- Narrowest useful command: manual prompt-shape inspection
- Reason if not fast red/green: this phase chooses instruction behavior; implementation verification happens in later phases.

### Tasks

- [ ] Decide whether `plan-prototype` should always offer to show a prototype when a visual artifact is produced.
- [ ] Decide the initial supported inspection surfaces: terminal/text, screenshot/image, local URL/browser route, one-command TUI/script, diagram file, and file-path artifact.
- [ ] Decide how explicitly the skill should separate visual output from conceptual terminal discussion.
- [ ] Decide whether diagrams and option canvases are separate modes or subtypes under visual prototypes.
- [ ] Decide how much of Matt's URL/search-param switcher pattern belongs in portable guidance.
- [ ] Decide how to express "logic module portable, shell throwaway" without over-prescribing architecture.
- [ ] Decide that future `wflow` scaffolding is a follow-up unless dogfooding shows repeated manual scaffold pain.
- [ ] Confirm the dogfooding Plan should record both explicit and implicit invocation attempts, while invocation-policy remains the owner for broader routing conclusions.

### Acceptance Criteria

- [ ] Adopted and rejected reference behaviors are explicit.
- [ ] The target behavior stays provider/tool neutral.
- [ ] The user has approved the initial improvement scope.
- [ ] Plan checklist is updated with completed work and newly discovered tasks.

### Grill Question

Recommended answer: treat visual output as something the agent should show or make inspectable when it exists, but do not require a browser/server unless that is the natural project tool.

Trade-off: this improves ergonomics for mockups and diagrams without turning `plan-prototype` into Superpowers' visual companion. The cost is that agents need to state the inspection path clearly: screenshot, local URL, file path, command, or terminal interaction.

Question: should `plan-prototype` require an explicit "handoff surface" for every prototype artifact, meaning the agent must tell the user exactly how to inspect it before asking for feedback?

## Phase 2: Update Prototype Guidance

Mode: AFK

### Goal

Update `plan-prototype` and only the durable docs needed to reflect the agreed behavior.

### Context

Most of the behavior belongs in `skills/plan-prototype/SKILL.md`. `docs/workflow.md` should stay concise unless the new behavior changes durable workflow policy.

### Test/Verification Cadence

- Cadence: Acceptance gate
- Target behavior: installed `plan-prototype` guidance
- Test surface: `skills/plan-prototype/SKILL.md`, `docs/workflow.md` if changed, README token table if counts change
- Narrowest useful command: `mise run check`
- Reason if not fast red/green: skill-instruction changes are verified through review, prompt-shape inspection, package discovery, and token-count refresh.

### Tasks

- [ ] Add question-first mode selection: visual, logic, diagram, option canvas, or text-only discussion.
- [ ] Add "show or inspect" guidance for prototype artifacts.
- [ ] Add a compact inspection-surface list with examples and implementation expectations.
- [ ] Add explicit terminal-primary feedback guidance when visual output is used.
- [ ] Strengthen visual prototype guidance around real context, real content when relevant, small option counts, and structurally different variants.
- [ ] Strengthen logic prototype guidance around one-command run, portable core logic, visible state, and throwaway shell.
- [ ] Strengthen cleanup guidance: delete, absorb, or keep temporarily with an explicit reason and capture of the answered question.
- [ ] Update `docs/workflow.md` only if needed.
- [ ] Refresh README token counts if skill text changes.
- [ ] Run repository checks and skill discovery.
- [ ] Run or prompt for local/global skill install according to repo instructions after review.

### Acceptance Criteria

- [ ] The skill tells agents when to show a prototype versus keep discussion in terminal.
- [ ] The skill explains how users inspect visual and logic prototypes.
- [ ] The skill distinguishes guidance-only building blocks from future `wflow` scaffold support.
- [ ] The skill remains tooling-neutral and avoids a mandatory visual companion server.
- [ ] Prototype cleanup/absorption is explicit.
- [ ] README token counts are current if changed.
- [ ] `npx skills add . --list` discovers the expected skills.
- [ ] `mise run check` passes.
- [ ] Implementation self-review completed.
- [ ] Plan checklist is updated with completed work and newly discovered tasks.

## Phase 3: Prepare Dogfooding Loop

Mode: AFK

### Goal

Keep `plans/plan-prototype-dogfooding.md` ready for real usage over the next few days.

### Context

After the research-backed improvement lands, the dogfooding Plan should collect real sessions rather than block the first improvement.

### Tasks

- [ ] Update the dogfooding Plan to assume the improved skill exists.
- [ ] Add an evidence template for each prototype session.
- [ ] Require at least one visual or diagram prototype and one logic/state/data/API prototype when possible.
- [ ] Include prompt style in each example: explicit skill call, natural prompt, or ambiguous prompt.
- [ ] Tell agents to mirror invocation surprises into `plans/invocation-policy-dogfooding.md` without making invocation policy part of prototype dogfooding.

### Acceptance Criteria

- [ ] Dogfooding is split from the initial improvement.
- [ ] Dogfooding examples can distinguish skill guidance issues from project/tooling constraints.
- [ ] The dogfooding Plan is ready to use after this improvement lands.
- [ ] Plan checklist is updated with completed work and newly discovered tasks.

## Discovered Follow-Ups

- Consider a tiny reusable prototype evidence template in shared references only if several dogfooding Plans need it.
- Consider a concrete visual companion or local-server integration only after multiple real sessions show tooling-neutral guidance is insufficient.
- Consider `wflow prototype` scaffolding only after dogfooding shows repeated manual setup friction across projects.

## Plan Self-Review

Status: PASS

### Findings

- No blocking findings. The Plan separates immediate reference-backed improvements from later dogfooding evidence, so the first skill update is not blocked on several days of usage.
- No blocking findings. The Plan adopts reference heuristics without importing Matt's exact route/switcher implementation or Superpowers' persistent browser companion server.
- No blocking findings. Durable docs are not changed in this planning split; Phase 2 owns `docs/workflow.md` updates only if the behavior becomes durable workflow policy.

### Plan Updates

- Tasks checked: none.
- Acceptance criteria checked: none.
- Tasks added/moved: split dogfooding evidence collection into `plans/plan-prototype-dogfooding.md`.

### Verification

- Researched local Matt Pocock prototype references: `prototype/SKILL.md`, `UI.md`, and `LOGIC.md`.
- Researched local Superpowers visual companion references: `brainstorming/SKILL.md`, `visual-companion.md`, and visual brainstorming design notes.
- `git diff --check`: passed.
- `mise run check`: passed.
