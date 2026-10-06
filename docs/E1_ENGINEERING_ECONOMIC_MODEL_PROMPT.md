# E1 Main Research Prompt — Engineering-Economic Model Replacement

## Authority

Continue as Main Research for the existing CN4252 project.

Read first:
- `docs/POST_SCREENING_ENGINEERING_CLOSURE_WORKFLOW.md`
- `results/COST_METHOD_RECONCILIATION.md`
- `docs/CN4252_BROAD_ASSIGNMENT_REQUIREMENTS.md`
- `docs/CN4252_PROBLEM_STATEMENT.md`

E0 is CLOSED at commit `2f01f96d97fa`.

E0 CI:
- Research CI `37431207435` — PASS
- Paper/reproducibility `37431207469` — PASS

This instruction authorises **E1 only — Engineering-Economic Model Replacement**.

Do not begin E2, W5, manuscript reintegration, or independent review automatically.

## Governing decision from E0

E0 established:

1. the historical approximately S$3.725/tCO2e result is internally reproducible under its declared differential-screening boundary;
2. it is too favourable/incomplete to remain the intended final-report controlling economic answer;
3. a complete CAPCOST/DFP/CCEP bottom-up plant estimate is not defensible with current equipment-design inputs;
4. stronger source-specific nuclear/integration evidence already exists in the repository;
5. a substantive replacement/addition economic model is warranted.

Therefore E1 must build the **strongest defensible engineering-economic replacement model supported by current evidence**, without inventing missing equipment design data.

The historical S$3.725/tCO2e result must remain preserved as audit history until E8, but E1 must produce the candidate controlling economic result/range for later manuscript replacement.

## 1. Recover all relevant economic evidence

Inspect at minimum:
- `results/COST_METHOD_RECONCILIATION.md`
- `results/COST_DERIVATION_DEEP_FEASIBILITY.md`
- `economics/` including `htgr_cost_overlap_v1.md` and `integration_cost_margin_v1.md`
- `results/final_design/ASSUMPTIONS.md`
- `results/final_design/SOURCE_REGISTER.md`
- `results/final_design/MASTER_NUMBER_PROVENANCE_REGISTER.md` if present
- `model/src/final_design.rs`
- current cost-ledger generators/tests
- active TEA manuscript section for context only
- Nishihara/JAEA/JAEA-IAEA nuclear economics already collected
- IAEA TECDOC-1682 integration/IHX evidence
- General Atomics intermediate-loop cross-check
- IEAGHG CCS economic basis
- uploaded CAPCOST/DFP/CCEP/CN4119 materials

Do not rely on generic textbook memory where the repository contains the actual source/method.

## 2. Freeze comparison and accounting boundaries

Before calculation define explicitly:
- common H2 service;
- availability basis;
- baseline conventional SMR-H2 boundary;
- candidate HTGR-assisted SMR-H2+CCS boundary;
- target currency;
- target cost year;
- escalation/index convention;
- FX convention;
- real/nominal convention;
- annualisation convention;
- treatment of common equipment;
- treatment of project electricity;
- rule for replacing versus retaining old aggregate cost blocks.

Preserve:
**project electricity revenue = zero**
unless a validated project power-output model is independently established. Do not infer export MWe from remaining thermal capacity.

## 3. Build a replacement/addition/double-counting ledger

Every cost block must be classified:

- RETAIN
- REPLACE
- ADD
- COMMON/CANCELS
- UNRESOLVED — MISSING INPUT
- NOT APPLICABLE
- DOUBLE-COUNT RISK

For each block record:
scope | method/equation | raw input | unit | provenance | original cost year/currency | normalization | annualisation | uncertainty/evidence class | double-count status.

No material cost may enter the model without this chain.

## 4. Nuclear/source economics — replace the favourable bridge

Evaluate the strongest applicable nuclear economic evidence already in the repository, without mixing incompatible scopes.

At minimum test:
- historical Nishihara/JAEA source-product economics;
- modern IAEA HTGR-200 source CAPEX/O&M case(s);
- modern MHR-T source CAPEX/O&M case(s);
- dedicated-reactor allocation;
- shared/cogeneration thermal-share allocation where physically/economically defensible.

For each nuclear case document:
- reactor size/technology;
- included capital scope;
- O&M scope;
- original currency/year;
- escalation basis;
- FX basis;
- capacity/availability basis;
- allocation rule;
- annualised burden.

Do not cherry-pick the cheapest source case.

Use a defensible central case and expose meaningful alternative cases/ranges.

If modern source cases make the economics adverse, retain that result.

## 5. Integration / IHX / secondary-helium-loop cost

E0 identified stronger source-specific evidence than the generic 10% allowance.

Explicitly evaluate:
- IAEA TECDOC-1682 preliminary GTHTR300C IHX + secondary-He-loop construction-cost anchor of USD69 million for the ~170 MW process-heat architecture;
- General Atomics intermediate-loop cross-check already recorded in the repository.

Determine:
- original cost year/currency;
- equipment/scope included;
- whether scaling from ~170 MW to the 176.8 MW project duty is technically justified;
- scaling exponent/method if used;
- escalation;
- FX;
- annualisation;
- uncertainty.

If adopted, state exactly which part of the old 10% integration/site allowance it REPLACES.

Do not stack the new integration cost on top of the old broad allowance without scope reconciliation.

If residual site/integration categories remain outside the source anchor, identify them separately and justify any retained allowance.

## 6. Chemical-process equipment costing

Use CAPCOST/DFP/CCEP/Turton/Sinnott methods **only where the required project design variables actually exist** and correlation validity ranges are satisfied.

For each candidate equipment item:
- identify method;
- required inputs;
- available inputs;
- validity range;
- result if calculable.

Potential candidates include:
- conventional chemical heat exchangers;
- pumps;
- vessels;
- WGS equipment;
- CO2 compression/conditioning;
- capture equipment only if replacing aggregate IEAGHG CCS CAPEX.

Do not invent area, pressure, material, diameter, volume, power, stage count or equipment count.

Do not use generic chemical correlations for:
- reactor/nuclear island;
- nuclear-grade IHX;
- nuclear helium circulator;
- safety-class nuclear equipment

unless the source method explicitly supports that service.

## 7. CCS scope

Choose one coherent CCS treatment.

Either:
A. retain/update the aggregate IEAGHG capture-capital basis;

or:
B. replace it with a sufficiently complete bottom-up capture-system estimate.

Do not combine aggregate IEAGHG CAPEX with separately costed absorber/stripper/reboiler/compressor equipment already inside that aggregate.

Keep CO2 T&S separate.

If no stronger contracted Singapore/cross-border tariff exists, retain a clearly identified uncertainty/sensitivity range rather than pretending the screening tariff is contractual.

## 8. OPEX / manufacturing-cost completeness

Using the CN4119 manufacturing-cost framework, determine which terms can actually be quantified from current data:

- natural-gas feed/fuel;
- utilities;
- waste;
- operating labour;
- maintenance/repairs;
- operating supplies;
- supervision/laboratory;
- insurance/taxes/overhead;
- solvent/catalyst replacement;
- fixed O&M.

Calculate only supported terms.

For each unsupported material term state:
- missing input;
- whether omission likely biases baseline, candidate, or both;
- whether it can reasonably cancel in a differential comparison.

Do not call the result a complete COM/COMd unless it actually satisfies that boundary.

## 9. Capital hierarchy

For every newly calculated capital item identify its level:

purchased equipment
→ bare module / installed
→ total module / grass-roots / FCI
→ working capital
→ TCI.

Do not combine values from different hierarchy levels as though equivalent.

If DCF/NPV/IRR/payback cannot be supported because revenue, construction schedule, tax, depreciation, financing, working capital, salvage or other inputs are missing, do not invent them.

CN4252 requires abatement cost, not necessarily a complete investor profitability model.

## 10. Cost-year / currency discipline

For every adopted cost:
- original year;
- original currency;
- index/escalation source;
- target year;
- FX basis;
- resulting normalized value.

Use consistent normalization.

Do not silently mix historical JPY, USD, EUR and SGD values.

Create machine-readable provenance for all normalized economic inputs.

## 11. Falsification cases

E1 must expose at least defensible economic cases such as:

- central stronger-source case;
- dedicated-reactor case;
- shared/cogeneration allocation case where defensible;
- integration-cost lower/central/upper evidence range;
- NG-price sensitivity;
- CCS/T&S sensitivity;
- other supported dominant uncertainties.

Do not create arbitrary optimistic/pessimistic ranges without provenance.

The purpose is to determine whether <S$100/tCO2e survives stronger economics.

## 12. Required equations/results

Calculate reproducibly:
- annual baseline represented cost;
- annual candidate represented cost;
- annual NG saving;
- annual nuclear/source burden;
- annual integration/IHX/secondary-loop burden;
- annual CCS burden;
- annual T&S burden;
- other supported differential costs;
- net annual incremental cost;
- lifecycle avoided-emissions denominator;
- S$/tCO2e.

Where a cost category is unresolved, do not hide it. Report the result as conditional on that unresolved category and, where possible, calculate the break-even omitted annual cost that would push the project to S$100/tCO2e.

A useful falsification quantity is:

**maximum additional unresolved annual cost consistent with the CN4252 S$100/tCO2e ceiling.**

Calculate this explicitly from the verified avoided-emissions denominator.

This does not replace actual costing; it quantifies the remaining economic margin.

## 13. Rust implementation and tests

Every adopted E1 calculation must be reproduced in deterministic code rather than living only in spreadsheets/Markdown.

Implement:
- economic input structures;
- normalization;
- replacement/addition ledger;
- annualisation;
- cases/sensitivities;
- abatement-cost outputs.

Add regression/unit tests for:
- unit conversions;
- escalation/FX;
- annualisation;
- no-double-count logic where testable;
- headline case outputs;
- S$100/tCO2e break-even unresolved-cost margin.

Do not execute untrusted VBA.

## 14. Durable E1 outputs

Create/update durable records, preferably including:

- `results/E1_ENGINEERING_ECONOMIC_MODEL.md`
- machine-readable economic input/provenance CSV
- machine-readable E1 cost ledger/case outputs
- Rust source/tests
- generated quantitative figure/data only if useful

The E1 derivation record must make every material represented cost auditable:

**scope → equation/method → input → unit → provenance → cost-year/FX treatment → calculation → uncertainty/evidence class → double-counting status.**

## 15. Do not rewrite the final manuscript yet

E1 is model redevelopment, not E8 manuscript reintegration.

Do not broadly rewrite the paper.

You may update repository status/provenance/reproducibility documentation needed to record E1.

The old S$3.725/tCO2e manuscript result remains temporarily until E8 so historical/current manuscript state is not silently half-updated.

E1 must clearly flag it as superseded-for-future-final-report if a stronger result is produced.

## 16. E1 acceptance test

E1 is complete only if:

- a stronger economic boundary is explicitly defined;
- nuclear/source economics are no longer controlled solely by the favourable historical bridge;
- source-specific integration cost is tested;
- CCS double counting is prevented;
- chemical equipment costs are used only where inputs exist;
- unresolved cost categories are explicit;
- all adopted monetary values have year/currency provenance;
- calculations are deterministic and tested;
- a central/range result is produced;
- the <S$100/tCO2e threshold is honestly retested;
- the break-even unresolved annual-cost margin is calculated;
- no invented engineering inputs are used.

## 17. Scientific discrepancy rule

If E1 changes the economic conclusion, this is an expected scientific result, not a workflow failure.

Do not tune assumptions to recover a PASS.

If the strongest defensible economics exceed S$100/tCO2e, record FAIL.

If the result remains below S$100/tCO2e but unresolved omitted costs could plausibly consume the margin, classify it conditionally.

If the result remains robustly below the threshold under defensible adverse cases, retain that evidence.

## 18. Verification

At completion:
- run Rust tests;
- run Research CI;
- run Paper/reproducibility CI if repository changes trigger it;
- verify generated E1 outputs reproduce;
- verify no existing scientific emissions result was accidentally changed;
- verify historical S$3.725/tCO2e evidence remains preserved;
- verify the new E1 result is clearly identified as the candidate controlling economic result for later E8 manuscript integration.

## 19. Report and STOP

Report:

PHASE: E1 — Engineering-Economic Model Replacement

STARTING HEAD:

ECONOMIC BOUNDARY:

NUCLEAR COST CASES:

INTEGRATION/IHX COST TREATMENT:

CCS COST TREATMENT:

CHEMICAL EQUIPMENT COSTS ADOPTED:

CHEMICAL EQUIPMENT COSTS BLOCKED BY MISSING DATA:

OPEX TERMS ADOPTED:

OPEX TERMS UNRESOLVED:

COST-YEAR/CURRENCY BASIS:

DOUBLE-COUNTING CONTROLS:

CENTRAL ANNUAL BASELINE COST:

CENTRAL ANNUAL CANDIDATE COST:

CENTRAL NET INCREMENTAL ANNUAL COST:

CENTRAL ABATEMENT COST:

ECONOMIC RANGE / CASES:

S$100/tCO2e THRESHOLD RESULT:

BREAK-EVEN ADDITIONAL UNRESOLVED ANNUAL COST TO S$100/tCO2e:

DOES E1 SUPERSEDE S$3.725/tCO2e FOR THE FUTURE FINAL REPORT?: YES/NO

SUBSTANTIVE ECONOMIC DISCREPANCIES:

RUST FILES/TESTS ADDED:

DURABLE OUTPUTS:

SCIENTIFIC EMISSIONS MODEL CHANGED?: YES/NO

CANONICAL HISTORICAL SCREENING RESULT PRESERVED?: YES/NO

RESEARCH CI:

PAPER CI:

COMMIT SHA:

RECOMMENDED NEXT ACTION:

Then STOP.

Do not begin E2 automatically.
Do not begin W5.
Do not begin E8.
Do not begin independent review.
