---
name: diagnose-issue
description: Diagnose bugs, failing tests, build failures, flaky behavior, unexpected output, or performance regressions. Read-only unless the user explicitly asks for a fix.
---

# Diagnose Issue

Find the cause with evidence before changing behavior. Diagnosis is read-only unless the user explicitly asks to fix or implement.

Use [research-ladder.md](references/research-ladder.md) when external docs or tool behavior matter. Use repo Glossary and ADRs when present.

## Process

1. Build the fastest useful feedback loop: focused test, build, CLI call, request, browser check, trace replay, benchmark, or small disposable harness.
2. Reproduce the reported symptom when feasible and capture the exact failure. For flaky behavior, record a baseline reproduction rate and raise it enough to compare changes. If reproduction is unavailable, distinguish conclusions from hypotheses and request only the missing evidence that matters.
3. Read full errors, logs, stack traces, recent diffs, dependency/config changes, environment differences, callers, and nearby working examples as relevant.
4. Keep ranked falsifiable hypotheses only while uncertainty remains. When direct evidence explains the cause, proceed to verification instead of manufacturing alternatives.
5. Instrument the boundary that separates plausible causes. Minimize a reproducible symptom by removing inputs, steps, config, or callers one at a time until only load-bearing elements remain. Trace bad values to their source, measure performance, and wait on real async conditions rather than arbitrary sleeps.
6. State the most supported cause and the evidence that rules in or out meaningful alternatives.

## When fixes are authorized

- Prefer a failing regression path before the fix when a useful seam exists.
- Apply the smallest change that addresses the cause.
- If a public contract, migration, persistence, auth/security behavior, production state, or performance trade-off needs owner judgment, stop and recommend options before mutating it.
- If repeated changes fail, stop stacking patches. Reassess the mental model, feedback loop, and architecture using the new evidence.
- Re-run the original feedback loop and relevant regression checks; read the results before claiming success.
- Before delivery, send the entire scope, not only the latest patch, to an independent fresh-context reviewer through `review-work`; verify accepted claims against primary evidence, fix authorized blocking findings, and rerun affected checks.
- Remove only temporary instrumentation and scratch artifacts created by this task. Preserve preexisting drafts, fixtures, and unrelated work.

If the correct fix lacks a safe test or architecture seam, report that constraint and route unresolved design to `improve-architecture` or `plan-work`. Route an already-understood behavior-preserving cleanup to `simplify-work` rather than hiding it in the bugfix.

## Output

Keep the report compact:

- Cause and evidence
- Fix, only if authorized
- Verification performed
- Limitations, residual risk, or recommended next action

Do not turn the diagnosis into a lab transcript or claim certainty beyond the evidence.
