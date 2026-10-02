# W4 CI Repair — Main Research Instruction

Continue as Main Research for the existing CN4252 project.

This is a bounded repair instruction following W4 — Design Basis and Provenance.

## Current state

W4 manuscript content was committed at:

`7f6d297b1c2c` — Develop W4 design basis and provenance

Verification at that W4 commit:
- Research CI `37008756249` — PASS
- Paper/reproducibility CI `37008756293` — FAILURE

W4 is therefore **not yet accepted**.

## Task

Diagnose and fix **only the Paper/reproducibility CI failure introduced/exposed by the W4 state**.

Inspect the failed workflow logs and determine the actual cause before editing.

Make the smallest technically correct repair required to restore the canonical paper/reproducibility workflow.

## Boundaries

Do NOT:

- begin W5;
- rewrite W4 broadly;
- restart W0–W4;
- reopen scientific research;
- reopen Reviews 1–7;
- begin Review 08;
- modify the verified Rust scientific model unless the CI evidence proves that is genuinely necessary;
- change canonical numerical results merely to make CI pass;
- change assumptions or conclusions;
- suppress a real error without fixing its cause;
- weaken reproducibility checks;
- disable tests or CI checks;
- proceed automatically to the next manuscript phase.

The scientific/model state remains frozen.

If the failure reveals a substantive scientific discrepancy rather than a build/presentation/reproducibility defect, STOP and report it explicitly instead of silently changing the science.

## Required verification after repair

After the bounded fix:

1. build the canonical manuscript;
2. run/allow Research CI;
3. run/allow Paper/reproducibility CI;
4. require both workflows to PASS;
5. verify zero undefined citations;
6. verify zero undefined references;
7. verify bibliography convergence;
8. inspect the exact generated PDF as required by the W4 workflow;
9. inspect the W4 design-basis/provenance table for wrapping/clipping;
10. confirm canonical scientific values remain unchanged;
11. confirm no W5 work was performed.

## Report and stop

Report:

W4 CI REPAIR:

FAILURE ROOT CAUSE:

FILES CHANGED:

REPAIR MADE:

SCIENTIFIC MODEL CHANGED?: YES/NO

CANONICAL NUMBERS CHANGED?: YES/NO

RESEARCH CI:

PAPER/REPRODUCIBILITY CI:

CITATION/REFERENCE STATUS:

PDF INSPECTION:

RESULTING COMMIT SHA:

W4 READY FOR COORDINATOR RECHECK?: YES/NO

Then STOP.

Do NOT begin W5 automatically.
