# Workflow Plugin Experiments Plan

## Goal

Create ignored experiment-ready package scaffolds for OpenCode, Pi, and Factory Droid to compare how workflow skills should integrate with plugin, hook, and package systems.

## Related Docs

- `docs/workflow.md`
- `GLOSSARY.md`

## Phase Checklist

- [ ] Phase 0: Research and grill harness mechanics
- [ ] Phase 1: Define experiment harness and comparison doc
- [ ] Phase 2: Build OpenCode experiment
- [ ] Phase 3: Build Pi experiment
- [ ] Phase 4: Build Factory Droid experiment
- [ ] Phase 5: Compare and recommend durable packaging strategy

## Verification Strategy

- Targeted tests: run each package's local validation command if available.
- Integration checks: install/load each experiment in its harness only when the harness is available locally.
- Type/lint/build: keep experiment checks isolated from the main repo unless promoted.
- Manual checks: verify skill discovery, explicit invocation, implicit invocation or disable controls, and hook/context behavior.
- Final review triggers: review before promoting anything from `.experiments/` into publishable repo content.

## Out of Scope

- Shipping official plugins in the first public release.
- Replacing Skills CLI installation for Claude Code or Codex.
- Supporting Windows-specific plugin behavior.
- Adding a full visual companion or browser automation framework.
- Publishing to plugin marketplaces.

## Research Notes

- Superpowers uses OpenCode plugin hooks to inject bootstrap context and register skill paths. Decision: test whether this repo needs any bootstrap or just skill registration.
- Factory Droid plugins can package skills, commands, hooks, MCP config, and metadata through `.factory-plugin/plugin.json`. Decision: test a real `.factory-plugin` scaffold.
- Pi can load skills from packages and extensions; extensions can intercept events, register tools, and adjust context. Decision: test a package with skills plus one minimal extension.
- OpenCode supports skill permissions and JS/TS plugins. Decision: test skill registration and a minimal context hook, not a full methodology bootstrap.
- This Plan is not implementation-ready until Phase 0 completes. Plugin and hook APIs should be checked against current first-party docs and minimal live examples before scaffolding.

## Phase 0: Research And Grill Harness Mechanics

Mode: HITL

### Goal

Research current plugin, package, skill, and hook mechanics for OpenCode, Pi, and Factory Droid, then grill the experiment shape before building scaffolds.

### Context

The purpose is to understand harness differences, not to prematurely publish plugin packages. APIs and conventions may have shifted since the local reference snapshots.

### Tasks

- [ ] Research current first-party docs for OpenCode skills, plugins, hooks/events, permissions, and package install behavior.
- [ ] Research current first-party docs for Pi skills, packages, extensions, resource discovery, disable controls, and event interception.
- [ ] Research current first-party docs for Factory Droid skills, plugins, hooks, commands, marketplaces, and install scopes.
- [ ] Check official or high-signal example repos for each harness where docs leave ambiguity.
- [ ] Identify which mechanics are essential to test: skill discovery, explicit invocation, implicit invocation controls, hook/context injection, permissions, and cleanup.
- [ ] Grill the user on whether bootstrap/context injection is desirable or whether plain skill registration should remain the default.
- [ ] Update this Plan with confirmed experiment scaffolds before Phase 1 starts.

### Acceptance Criteria

- [ ] The comparison categories are based on current docs, not stale reference snapshots.
- [ ] Each harness has a minimal experiment scope and a clear non-goal list.
- [ ] The user has approved which mechanics each experiment should prove.
- [ ] Phase 1 tasks reflect researched mechanics rather than assumptions.
- [ ] Plan checklist is updated with completed work and newly discovered tasks.

## Phase 1: Define Experiment Harness And Comparison Doc

Mode: AFK

### Goal

Create a contained place for experiments and a comparison structure.

### Context

Experiments should live under ignored `.experiments/` first. Durable conclusions can move into `docs/` after the comparison.

### Tasks

- [ ] Confirm `.experiments/` is ignored or add it to `.gitignore`.
- [ ] Create `.experiments/workflow-plugins/`.
- [ ] Create a comparison note template covering install command, package shape, skill discovery, explicit invocation, implicit invocation controls, hooks/context injection, permissions, update story, and promotion recommendation.
- [ ] Define the shared sample skill or skill path each experiment will expose.
- [ ] Define success criteria for each harness before building scaffolds.

### Acceptance Criteria

- [ ] Experiments are excluded from publishable repo content.
- [ ] The comparison captures harness differences rather than trying to normalize them too early.
- [ ] Each experiment has clear install/load/verify steps.
- [ ] Implementation self-review completed.
- [ ] Plan checklist is updated with completed work and newly discovered tasks.

### Notes

This is research infrastructure. Avoid polishing it into a product prematurely.

## Phase 2: Build OpenCode Experiment

Mode: Mixed

### Goal

Create an OpenCode plugin experiment that proves skill registration and minimal context behavior.

### Context

Superpowers uses an OpenCode plugin to register its skills directory and inject bootstrap context. This repo may need less.

### Tasks

- [ ] Create `.experiments/workflow-plugins/opencode/`.
- [ ] Add a minimal package/plugin entrypoint.
- [ ] Register the workflow skills path or a copied experiment skill path.
- [ ] Add a minimal context hook only if needed to prove routing behavior.
- [ ] Document `opencode.json` configuration.
- [ ] Verify skill listing/loading if OpenCode is available.

### Acceptance Criteria

- [ ] The experiment can expose workflow skills to OpenCode.
- [ ] The experiment documents how to deny or permit skills through OpenCode skill permissions.
- [ ] The experiment avoids unconditional heavy bootstrap unless evidence says it is needed.
- [ ] Install, verify, and cleanup steps are documented.
- [ ] Implementation self-review completed.
- [ ] Plan checklist is updated with completed work and newly discovered tasks.

### Notes

Pay attention to whether plain Skills CLI install is already good enough for OpenCode.

## Phase 3: Build Pi Experiment

Mode: Mixed

### Goal

Create a Pi package experiment with skills plus one minimal extension.

### Context

Pi packages can declare skills and extensions. Extensions can intercept lifecycle events, register tools, and inject or transform context.

### Tasks

- [ ] Create `.experiments/workflow-plugins/pi/`.
- [ ] Add `package.json` with `pi.skills` and `pi.extensions` entries.
- [ ] Add a minimal extension proving lifecycle/context or tool interception.
- [ ] Expose a small workflow skill set or sample skill.
- [ ] Document global and project install/load options.
- [ ] Verify skill and extension loading if Pi is available.

### Acceptance Criteria

- [ ] The experiment proves Pi can package workflow skills and extension behavior together.
- [ ] The experiment documents `--no-skills`, explicit `--skill`, package filtering, and relevant disable controls.
- [ ] The extension is minimal and does not become a workflow implementation by itself.
- [ ] Install, verify, and cleanup steps are documented.
- [ ] Implementation self-review completed.
- [ ] Plan checklist is updated with completed work and newly discovered tasks.

### Notes

Pi may be the best place to test deeper customization, but keep the first experiment small.

## Phase 4: Build Factory Droid Experiment

Mode: Mixed

### Goal

Create a Factory Droid plugin experiment with skills and a minimal hook.

### Context

Droid plugins can include `.factory-plugin/plugin.json`, `skills/`, `commands/`, `hooks/`, and MCP config. Plugin hooks use `${DROID_PLUGIN_ROOT}`.

### Tasks

- [ ] Create `.experiments/workflow-plugins/droid/`.
- [ ] Add `.factory-plugin/plugin.json`.
- [ ] Add a minimal `skills/` payload or references to copied workflow skills.
- [ ] Add `hooks/hooks.json` with a harmless proof hook if needed.
- [ ] Document marketplace-free local install options if available, otherwise document the nearest test path.
- [ ] Verify skill discovery and hook behavior if Droid is available.

### Acceptance Criteria

- [ ] The experiment proves Droid plugin shape for workflow skills.
- [ ] The experiment documents `disable-model-invocation` and `user-invocable` behavior.
- [ ] The hook proof is harmless and easy to remove.
- [ ] Install, verify, and cleanup steps are documented.
- [ ] Implementation self-review completed.
- [ ] Plan checklist is updated with completed work and newly discovered tasks.

### Notes

Droid's plugin model is likely the richest packaging target. Do not let that pull the main workflow into Droid-only assumptions.

## Phase 5: Compare And Recommend Durable Packaging Strategy

Mode: HITL

### Goal

Turn experiment results into a recommendation without promoting unstable scaffolds.

### Context

The output should answer what belongs in this repo's publishable shape, what remains Skills CLI-only, and what should become separate plugin packages later.

### Tasks

- [ ] Fill the comparison note for OpenCode, Pi, and Droid.
- [ ] Identify common concepts that belong in durable docs.
- [ ] Identify harness-specific mechanics that should stay in experiment docs or future package docs.
- [ ] Recommend whether to promote any package scaffolds into the repo.
- [ ] Recommend whether the first stable plugin target should be OpenCode, Pi, Droid, or none yet.

### Acceptance Criteria

- [ ] The comparison clearly distinguishes skills, plugins, hooks, commands, packages, and install/update behavior by harness.
- [ ] The recommendation does not require plugin packaging for ordinary Skills CLI users.
- [ ] Any promoted files have a follow-up implementation plan.
- [ ] Implementation self-review completed.
- [ ] Plan checklist is updated with completed work and newly discovered tasks.

## Discovered Follow-Ups

- Package Codex or Claude plugins only after OpenCode/Pi/Droid experiments clarify what benefit plugin packaging adds.
- Consider a future installer only if plugin experiments show Skills CLI cannot express needed behavior.
