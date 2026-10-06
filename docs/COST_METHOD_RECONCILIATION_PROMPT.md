# Cost Method Reconciliation — Main Research Instruction

## Purpose

The user has supplied engineering-costing and economic-analysis attachments that may improve the clarity and/or rigor of the current techno-economic analysis.

This is a **bounded scientific/economic reconciliation phase before further manuscript progression**.

Do NOT begin W5 or later manuscript phases until this task is complete and coordinator-checked.

The objective is not to force the current approximately S$3.725/tCO2e result to survive. The objective is to determine whether the new materials provide a technically applicable, better-supported cost method for this project.

## Attachments to inspect

Inspect the actual contents of:

- `attachments/CAPCOST_2017_rev2.xlsm`
- `attachments/CCEP-Version 1-Read this first.xlsx`
- `attachments/CCEP-Version 1-Use this next.xls`
- `attachments/DFP-Version 1 (Feb 2011).xlsm`
- `attachments/CN4119-0-Second Part Intro.pptx`
- `attachments/CN4119-1-Cost Estimation and Economic Analysis-Intro.pdf`
- `attachments/CN4119-2-Capital Cost Estimation.pdf`
- `attachments/CN4119-3-Capital Cost Estimation-Scanned Appendix.pdf`
- `attachments/CN4119-4-Manufacturing Cost Estimation.pdf`
- `attachments/CN4119-5-Engineering Economic Analysis.pdf`
- `attachments/Evaluating Capital Cost Estimation Programs.PDF`

Also inspect the current canonical economics, including at minimum:

- `results/COST_DERIVATION_DEEP_FEASIBILITY.md`
- `results/final_design/ASSUMPTIONS.md`
- `results/final_design/SOURCE_REGISTER.md`
- `results/final_design/MASTER_NUMBER_PROVENANCE_REGISTER.md` if present
- relevant economics files/directories
- `model/src/final_design.rs`
- relevant cost-ledger Rust generators
- active techno-economic manuscript section
- current W4 design-basis/economic-boundary text
- Review 07 and relevant prior economic reviews/resolutions

## 1. First understand the supplied methods

Do not simply extract formulas.

For each supplied costing tool/lecture/resource determine:

- intended purpose;
- method type;
- cost-estimate class/accuracy if stated;
- equipment/process types covered;
- required inputs;
- cost-year basis;
- currency basis;
- scaling/capacity rules;
- installation factors;
- material/pressure/temperature factors;
- bare-module or purchased-equipment treatment;
- indirect costs;
- working capital;
- fixed capital;
- total capital;
- manufacturing/operating costs;
- depreciation/tax/financing assumptions where relevant;
- annualisation/economic metrics;
- limitations and intended applicability.

Distinguish teaching notation from empirically calibrated correlations.

## 2. Map the current economic model

Reconstruct the existing canonical cost boundary explicitly.

At minimum map:

- baseline natural-gas expenditure;
- candidate natural-gas expenditure;
- natural-gas saving;
- Nishihara reactor/source economic burden;
- heat-product cost;
- source electricity-product burden where represented;
- CCS CAPEX scaling from IEAGHG;
- CCS annualisation/CRF;
- integration/site allowance;
- CO2 transport/storage tariff;
- electricity revenue = zero;
- net annual incremental cost;
- lifecycle avoided-emissions denominator;
- current abatement cost.

For each current term identify:

SOURCE / ASSUMPTION / DERIVATION / MODEL OUTPUT.

## 3. Reconciliation classification

For each useful method/equation/notation in the new attachments classify it as one of:

A. **PRESENTATION IMPROVEMENT ONLY**
Same economics, clearer notation/derivation.

B. **VALID REPLACEMENT METHOD**
A better-supported applicable method for an existing cost term.

C. **VALID ADDITIONAL COST CATEGORY**
A legitimate currently omitted cost that can be estimated with available data.

D. **POTENTIALLY RELEVANT BUT INSUFFICIENT INPUT DATA**
Method is valid, but project lacks required sizing/design inputs.

E. **NOT APPLICABLE**
Wrong technology/equipment/project stage/boundary.

F. **WOULD DOUBLE COUNT**
Already represented elsewhere in the current model.

Do not use a method merely because it appears more detailed.

## 4. Equipment-by-equipment applicability

Determine whether the attachments can defensibly estimate costs for any of:

- reformer;
- WGS equipment;
- heat exchangers;
- IHX;
- helium circulator/compressor;
- piping;
- PSA;
- amine absorber/stripper;
- reboiler/condenser;
- CO2 compressor;
- pumps;
- steam/utilities;
- storage vessels;
- reactor/nuclear island;
- turbine/power system;
- CO2 conditioning;
- other balance-of-plant.

For each item state whether sufficient design variables exist.

Do NOT invent:
- vessel dimensions;
- exchanger area;
- pressure drop;
- materials;
- compressor ratios;
- stage counts;
- detailed temperatures/pressures;
- equipment counts

merely to make CAPCOST/CCEP run.

If the required input is unavailable, classify the estimate as not currently calculable.

## 5. Nuclear-cost applicability

Be especially careful with nuclear costs.

Generic chemical-process equipment correlations must NOT be used to estimate an entire nuclear reactor/nuclear island unless the source explicitly supports that application.

Determine whether the existing Nishihara/JAEA source economics remain the more defensible screening basis for the reactor.

If the new material helps estimate only non-nuclear process equipment, say so.

Avoid double-counting equipment already embedded in the source reactor/process-heat economic architecture.

## 6. CCS-cost applicability

Compare any applicable new chemical-equipment costing with the existing IEAGHG CCS CAPEX basis.

Do not add absorber/stripper/compressor costs separately if they are already included in the IEAGHG reference CAPEX unless the old aggregate is deliberately replaced with a new bottom-up estimate.

Any replacement must be complete enough to avoid partial-boundary bias.

## 7. Cost-year and currency discipline

For every adopted monetary input/method record:

- original currency;
- original cost year;
- escalation/index method;
- target cost year;
- FX date/basis;
- capital versus annual cost;
- nominal versus real treatment where relevant.

No cost may silently jump across cost years or currencies.

## 8. CAPEX hierarchy

Where supported, distinguish clearly:

PURCHASED EQUIPMENT COST
→ INSTALLED / BARE-MODULE COST
→ DIRECT PERMANENT INVESTMENT
→ INDIRECT COSTS
→ FIXED CAPITAL INVESTMENT
→ WORKING CAPITAL
→ TOTAL CAPITAL INVESTMENT

only if the selected method actually supports those definitions.

Do not mix terminology from different methods without reconciliation.

## 9. OPEX / manufacturing-cost hierarchy

Where applicable distinguish:

- raw materials;
- utilities;
- operating labour;
- supervision;
- maintenance;
- operating supplies;
- laboratory;
- overhead;
- property/insurance;
- waste/disposal;
- catalyst/solvent replacement;
- feedstock;
- CO2 T&S;
- other fixed/variable costs.

Identify which are already represented and which are omitted.

Avoid double counting natural gas or CCS costs already explicitly modelled.

## 10. Economic-analysis notation

Use the supplied teaching material to improve notation where technically appropriate.

Every material economic result should have a clear chain:

SYMBOL
→ DEFINITION
→ EQUATION
→ INPUTS
→ UNITS
→ SOURCE/ASSUMPTION
→ SUBSTITUTION
→ RESULT
→ INTERPRETATION.

If the CN4119 notation is clearer than current notation and mathematically equivalent, adopt the clearer notation in the economic derivation/manuscript later.

Do not change a correct model solely to imitate lecture notation.

## 11. Current S$3.725/tCO2e result

Treat the current result as a hypothesis to be checked, not a protected answer.

Determine:

- which costs it currently includes;
- which costs it omits;
- whether those omissions are intentional screening boundaries;
- whether the new material enables any omitted cost to be estimated credibly;
- whether any existing cost term is mis-specified;
- whether a more complete applicable cost boundary changes the numerator materially.

If a better-supported applicable method changes the result, allow the result to change.

If the new methods cannot be applied because design inputs are missing, retain the current result but strengthen its limitation/interpretation.

Do not manufacture inputs simply to produce a new number.

## 12. Required outputs of this reconciliation

Create a durable repository record, e.g.:

`results/COST_METHOD_RECONCILIATION.md`

It should include:

### A. Attachment-method inventory
Resource | Method | Required inputs | Intended use | Applicability

### B. Current-cost-boundary map
Current term | Value | Source/assumption | What it represents

### C. Reconciliation matrix
New method/category | Classification A–F | Reason | Data available? | Action

### D. Equipment-costability matrix
Equipment | Method candidate | Required data | Data available | Can estimate now?

### E. Double-counting map
Potential new cost | Existing represented term | Risk | Treatment

### F. Recommended economic model
Clearly state what should:
- remain;
- be replaced;
- be added;
- remain unresolved.

## 13. Implementation rule

Do **not** automatically rewrite the economic model during the initial comparison.

First complete the reconciliation and determine whether changes are:
- presentation-only;
- bounded economic-model improvements;
- or substantive economic-model redevelopment.

If only notation/presentation improves, implement that bounded improvement and verify.

If a small number of cost terms can be validly improved without changing the scientific boundary materially, implement them carefully with tests and provenance.

If the new method would materially change the economic model/boundary or headline abatement cost, STOP after the reconciliation and report the proposed change for coordinator approval before altering the canonical result.

## 14. Spreadsheet integrity

The supplied spreadsheets/macros are reference calculation tools, not automatically canonical.

Inspect formulas/logic where possible.

Do not execute untrusted macros blindly.

Cross-check any adopted equation against accompanying lecture/documentation.

Record workbook/sheet/cell provenance for any numerical value or correlation adopted.

## 15. Scientific discrepancy rule

If the reconciliation shows the existing cost model contains:
- arithmetic error;
- unit error;
- currency error;
- cost-year error;
- double counting;
- omitted cost that can now be credibly quantified;
- invalid use of source economics;

treat this as a substantive economic finding.

Do not hide it to preserve the previous result.

## 16. Stop condition and report

At completion report:

COST METHOD RECONCILIATION:

ATTACHMENTS INSPECTED:

METHODS IDENTIFIED:

CURRENT COST BOUNDARY RECONSTRUCTED:

PRESENTATION-ONLY IMPROVEMENTS:

VALID REPLACEMENT METHODS:

VALID ADDITIONAL COSTS:

METHODS BLOCKED BY MISSING DATA:

NOT-APPLICABLE METHODS:

DOUBLE-COUNTING RISKS:

NUCLEAR COST TREATMENT:

CCS COST TREATMENT:

RECOMMENDED ECONOMIC MODEL:

DOES CURRENT S$3.725/tCO2e REMAIN TECHNICALLY DEFENSIBLE UNDER ITS DECLARED BOUNDARY?:

WOULD A BETTER-SUPPORTED MODEL CHANGE THE HEADLINE RESULT?:

SUBSTANTIVE ECONOMIC DISCREPANCY FOUND?:

FILES CREATED/CHANGED:

SCIENTIFIC MODEL CHANGED?: YES/NO

CANONICAL ECONOMIC RESULT CHANGED?: YES/NO

RESEARCH CI:

PAPER CI IF APPLICABLE:

COMMIT SHA:

RECOMMENDED NEXT ACTION:

Then STOP.

Do not begin W5 or another manuscript phase automatically.
Do not begin Review 08.
