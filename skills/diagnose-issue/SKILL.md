---
name: diagnose-issue
description: Diagnose and fix bugs, failing tests, build failures, flaky behavior, unexpected output, or performance regressions. Use when the user reports something broken, failing, slow, flaky, or asks to debug/diagnose an issue.
---

# Diagnose Issue

Find root cause before fixing.

Use workflow terms from [workflow-language.md](references/workflow-language.md).
Use [research-ladder.md](references/research-ladder.md) when external docs or tool behavior matter.
Use repo Glossary and ADRs when present.

## Core Rule

Do not change production behavior until you have a root-cause hypothesis backed by evidence.

Normal bugs, failing tests, and build failures may be fixed once root cause and verification path are clear. Stop before fixing when the change is high-risk, performance-sensitive, changes public behavior, needs credentials/production data, or reveals an architectural seam problem.

## Process

1. **Build a feedback loop.**
   - Prefer a fast, deterministic, agent-runnable signal: focused test, build command, CLI invocation, HTTP request, browser check, trace replay, or small throwaway harness.
   - If the issue is flaky, raise reproduction rate with repeated runs, fixed seeds, stress, timing control, or narrower triggers.
   - If no loop is possible, stop and ask for logs, traces, repro steps, access, or permission to add temporary instrumentation.
2. **Reproduce the reported issue.**
   - Confirm the loop fails for the same symptom the user reported.
   - Capture the exact error, output, timing, or behavior.
3. **Sweep the evidence.**
   - Read the full error message, stack trace, logs, and command output.
   - Check recent diffs, commits, dependency changes, config changes, and environment differences.
   - Find nearby working examples or reference implementations and compare behavior before changing code.
4. **Form hypotheses.**
   - List 3-5 ranked falsifiable hypotheses before code changes, unless direct evidence already fully explains the root cause.
   - Each hypothesis should predict what evidence or change would prove or disprove it.
5. **Instrument narrowly.**
   - Probe the boundary that distinguishes hypotheses.
   - Change one variable at a time.
   - Tag temporary debug output with a unique prefix so cleanup is reliable.
   - When bad data appears deep in the stack, trace callers and values backward until you find the original trigger. Fix the source, not the symptom.
   - For performance regressions, measure first; prefer profiling, timing harnesses, query plans, or before/after benchmarks over logs.
   - For flaky async behavior, wait for the actual condition: state, event, file, output, DOM, or process state. Use sleeps only when testing real timing behavior, and document why.
6. **Fix with a regression path.**
   - Prefer writing or adapting a failing regression test before the fix when a correct seam exists.
   - If no correct seam exists, document that as a finding and recommend `improve-architecture` after the immediate issue is handled.
   - Apply the smallest fix that addresses root cause.
7. **Verify and clean up.**
   - Re-run the original feedback loop.
   - Run the regression test or verification command.
   - Remove temporary instrumentation, scratch fixtures, and throwaway harnesses unless they were deliberately promoted.
   - Do not report fixed, passing, or complete until fresh verification commands have run and their output has been read.
   - State the root cause and the evidence that proved the fix.

## Stop Signs

If any of these thoughts appear, stop and return to evidence gathering:

- "I'll just try this quick fix."
- "It's probably X."
- "The error output is noisy; skip it."
- "I'll add a sleep and see if the flake goes away."
- "I'll fix this symptom and investigate later."
- "The first fix failed, so I'll stack another fix on top."
- "I do not understand this, but this might work."

If two fix attempts fail, stop and re-run diagnosis with the new evidence.

If three fix attempts fail, assume the architecture or mental model is wrong. Ask before continuing.

## Stop Conditions

Stop and ask before fixing when:

- the fix changes a public API, schema, data migration, persistence behavior, auth/security behavior, or documented contract
- the issue is a performance regression and the fix needs a trade-off decision
- the issue requires production credentials, production data, or external service mutation
- three fix attempts failed and the pattern suggests the architecture is wrong
- there is no good test seam and the fix would be hard to lock down

## Output Shape

When reporting progress or completion, include:

- feedback loop used
- reproduced symptom
- root cause hypothesis and evidence
- fix summary, if a fix was applied
- regression test or verification
- cleanup performed
- residual risk or recommended next workflow

Keep the process strict, but keep the prose practical. The point is to stop guessing, not to write a lab report.
