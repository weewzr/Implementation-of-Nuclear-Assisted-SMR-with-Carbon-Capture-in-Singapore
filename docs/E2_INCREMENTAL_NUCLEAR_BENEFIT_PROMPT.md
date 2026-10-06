# E2 Main Research Prompt — Incremental Nuclear-Benefit Attribution

## Authority

Continue as Main Research for the existing CN4252 project.

Read first:
- `docs/POST_SCREENING_ENGINEERING_CLOSURE_WORKFLOW.md`
- `results/E1_ENGINEERING_ECONOMIC_MODEL.md`
- `results/COST_METHOD_RECONCILIATION.md`
- `docs/CN4252_BROAD_ASSIGNMENT_REQUIREMENTS.md`
- `docs/CN4252_PROBLEM_STATEMENT.md`
- relevant historical comparator/review records.

E1 is CLOSED at commit `77a9dac65776`.

E1 CI:
- Research CI `37433667258` — PASS
- Paper/reproducibility `37433667172` — PASS

This instruction authorises **E2 only — Matched Comparator and Incremental Nuclear-Benefit Attribution**.

Do not begin E3, W5, E8 manuscript reintegration, or independent review automatically.

## Governing question

CN4252 allows any solution. Therefore the project must answer:

**What does nuclear process heat actually add beyond SMR-H2 + CCS alone?**

Do not assume the integrated nuclear configuration deserves credit for all abatement produced by CCS.

E2 must decompose the benefit and cost of:

1. conventional unabated SMR-H2 baseline;
2. matched SMR-H2 + CCS without nuclear heat;
3. HTGR-assisted SMR-H2 + CCS.

Use the same H2 service and the most consistent feasible boundaries.

## 1. Recover comparator evidence

Inspect all repository evidence relevant to:
- IEAGHG Case 1A / conventional SMR+CCS comparator;
- INL conventional and nuclear-assisted process cases;
- historical Gate-5 comparator work;
- Review 03 / Review 07 comparator findings;
- lifecycle ledgers;
- E1 economics;
- source-stream reconstruction;
- natural-gas and process-heat differences.

Do not invent a matched comparator if source evidence is insufficient.

Where exact matching is impossible, explicitly state the normalization/mapping required and uncertainty introduced.

## 2. Freeze common H2 service

All three cases must be compared at the same hydrogen service.

Use the canonical 130 MMSCFD / approximately 97,946 tH2/y service unless evidence requires a different normalized basis.

Any comparator reported at a different production scale must be normalized transparently.

Document:
source scale → normalization equation → common scale.

## 3. Define three physical cases

### Case A — Unabated conventional SMR-H2
Define:
- NG consumption;
- fired reformer heat;
- direct CO2;
- upstream NG;
- electricity/auxiliary treatment;
- lifecycle emissions;
- represented cost.

### Case B — Conventional SMR-H2 + CCS, no nuclear heat
Use the strongest applicable source-backed comparator.

Define:
- same H2 service;
- NG/feed/fuel treatment;
- capture topology;
- capture rate/stream;
- residual direct CO2;
- capture energy;
- upstream emissions;
- T&S burden;
- represented cost.

Do not quietly give Case B the nuclear-assisted Case-6 lower NG consumption if that reduction depends on nuclear heat.

### Case C — HTGR-assisted SMR-H2 + CCS
Use the current project candidate:
- same H2 service;
- 34.0 MMSCFD candidate NG where applicable;
- 176.8 MWth nuclear process heat;
- CCS;
- verified lifecycle model;
- E1 central economic treatment.

## 4. Emissions attribution

Calculate consistently:

A → B:
**abatement attributable to adding CCS and associated conventional-process changes**

B → C:
**incremental abatement attributable to nuclear heat integration**

A → C:
**total integrated abatement**

Separate:
- direct CO2;
- upstream NG;
- nuclear lifecycle burden;
- electricity/auxiliary burden;
- T&S burden.

Avoid double counting.

The identity should be checked:

(A-B abatement) + (B-C abatement) = (A-C abatement)

within declared normalization/boundary consistency.

If source differences prevent exact closure, quantify/explain the mismatch.

## 5. Cost attribution

Using E1 economics as the controlling nuclear-assisted cost framework, construct the strongest comparable cost treatment for A and B.

Calculate where defensible:

A → B:
- incremental annual cost of CCS-only step;
- incremental tCO2e avoided;
- incremental S$/tCO2e for CCS step.

B → C:
- incremental annual cost of nuclear integration;
- incremental tCO2e avoided;
- incremental S$/tCO2e for nuclear step.

A → C:
- total integrated cost/abatement result.

Do not force a nuclear incremental cost metric if the comparator cost boundary is not sufficiently matched; if blocked, state exactly why and calculate the strongest bounded range/threshold possible.

## 6. Nuclear-value falsification

E2 must explicitly test:

- Does CCS alone already exceed 0.25 MtCO2e/y?
- What fraction of total integrated lifecycle abatement is achieved before nuclear heat is added?
- How much additional lifecycle abatement does nuclear heat contribute?
- How much additional annual represented cost does the nuclear step add under E1?
- What is the incremental S$/tCO2e of the nuclear step?
- Does nuclear improve or worsen the integrated cost effectiveness?
- Is nuclear necessary to meet the CN4252 abatement threshold?
- Is nuclear necessary to meet the cost threshold?
- Does nuclear create benefits not captured by the two thresholds that remain relevant to feasibility/effectiveness?

Do not defend nuclear if the matched evidence says CCS provides most of the benefit.

## 7. Attribution percentages

Where boundary consistency permits, report:

- % of total A→C lifecycle abatement attributable to A→B CCS step;
- % attributable to B→C nuclear step;
- NG reduction attributable to nuclear step;
- direct-emissions reduction attribution;
- lifecycle burden added by nuclear.

Do not report percentages that imply false precision when comparator mismatch is material.

## 8. Singapore interpretation

Explain what the decomposition means for the assignment.

If conventional SMR+CCS already passes >0.25 MtCO2e/y, state that plainly.

Then ask whether the additional nuclear step is justified by:
- extra abatement;
- economics;
- feed/fuel reduction;
- strategic/energy-security considerations;
- other evidence-supported benefits

versus:
- capital burden;
- safety;
- siting;
- licensing;
- implementation complexity.

Do not introduce unsupported strategic claims.

## 9. Alternative pathways

Do not turn E2 into a full technology-ranking exercise.

However, preserve the original falsification question:
if a matched non-nuclear comparator already performs strongly, the final manuscript must not imply nuclear is automatically preferred.

State where evidence is insufficient for matched comparisons with electrolysis/other pathways.

## 10. Deterministic implementation

Implement E2 calculations reproducibly in Rust where they are project-derived.

Create:
- comparator input/provenance structure;
- normalization;
- emissions attribution;
- cost attribution;
- identity checks;
- sensitivity/range if needed.

Add tests for:
- common H2 normalization;
- attribution closure;
- no double counting;
- headline incremental results.

## 11. Durable outputs

Create at minimum:
- `results/E2_INCREMENTAL_NUCLEAR_BENEFIT.md`;
- machine-readable comparator inputs/provenance;
- machine-readable emissions attribution;
- machine-readable cost attribution;
- Rust implementation/tests.

A useful table should show:

Metric | Unabated SMR | SMR+CCS | Nuclear-assisted SMR+CCS | A→B change | B→C change

for:
- H2 service;
- NG;
- direct CO2;
- lifecycle CO2e;
- annual represented cost;
- annual abatement;
- abatement cost where meaningful.

## 12. Do not rewrite final manuscript yet

E2 is analysis.

Do not broadly update the active manuscript; E8 will integrate E1–E7 coherently.

You may update status/provenance/reproducibility records required for E2.

## 13. Acceptance test

E2 passes only if the project can answer, with evidence:

**How much of the integrated solution's benefit comes from CCS, and how much comes from nuclear heat?**

and:

**What additional cost is paid for the additional nuclear benefit?**

If exact attribution is impossible because source cases are not matched, the output must make that limitation explicit and provide the strongest defensible bounded answer rather than fake precision.

## 14. Verification

At completion:
- run Rust tests;
- run Research CI;
- run Paper/reproducibility CI if triggered;
- verify E1 central economics remain unchanged;
- verify historical screening evidence remains preserved;
- verify emissions attribution does not double count captured/avoided CO2;
- verify common H2 service.

## 15. Report and STOP

Report:

PHASE: E2 — Incremental Nuclear-Benefit Attribution

STARTING HEAD:

COMMON H2 SERVICE:

CASE A — UNABATED SMR:

CASE B — SMR+CCS:

CASE C — NUCLEAR-ASSISTED SMR+CCS:

A→B LIFECYCLE ABATEMENT:

B→C INCREMENTAL NUCLEAR LIFECYCLE ABATEMENT:

A→C TOTAL LIFECYCLE ABATEMENT:

CCS SHARE OF TOTAL ABATEMENT:

NUCLEAR SHARE OF TOTAL ABATEMENT:

A→B INCREMENTAL ANNUAL COST:

B→C INCREMENTAL NUCLEAR ANNUAL COST:

CCS-STEP S$/tCO2e:

NUCLEAR-STEP S$/tCO2e:

DOES SMR+CCS ALONE PASS 0.25 MtCO2e/y?:

IS NUCLEAR NECESSARY FOR THE ABATEMENT THRESHOLD?:

IS NUCLEAR NECESSARY FOR THE COST THRESHOLD?:

MATCHING/NORMALIZATION LIMITATIONS:

SUBSTANTIVE FINDING:

RUST FILES/TESTS:

DURABLE OUTPUTS:

RESEARCH CI:

PAPER CI:

COMMIT SHA:

RECOMMENDED NEXT ACTION:

Then STOP.

Do not begin E3 automatically.
Do not begin W5.
Do not begin E8.
Do not begin independent review.
