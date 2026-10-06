# Cost-Method Reconciliation

## Scope and decision

This record executes `docs/COST_METHOD_RECONCILIATION_PROMPT.md`. It reconciles the current canonical abatement-cost screen with the user-supplied CN4119/CAPCOST/DFP/CCEP costing materials **before any change to the Rust cost model or headline result**.

**Decision:** the attachments materially improve the **methodological standard and completeness test**, but the current repository does **not** contain the equipment design inputs required for a defensible bottom-up replacement of the canonical cost model. A complete CAPCOST/DFP/CCEP-style recost would require a separate cost-model redevelopment phase. Therefore this reconciliation does **not** change the Rust model, canonical cost ledger, S$3.725/tCO2e headline result, or CONDITIONAL MODEL PASS.

The current S$3.725/tCO2e value remains usable only as a **screening abatement-cost bridge**, not a Total Capital Investment (TCI), Cost of Manufacture (COM), discounted-cash-flow profitability analysis, or bankable Singapore project estimate.

## 1. Attachment inventory and costing logic

### 1.1 `CAPCOST_2017_rev2.xlsm`

**Type:** Turton-style module-costing workbook with manufacturing-cost and cash-flow sheets. VBA was not executed.

**Observed workbook structure:**
- `Equipment Summary`: equipment-specific purchased cost, bare-module cost, base-equipment cost and base bare-module cost. Inputs vary by equipment and include size, pressure, material of construction, area, power, volume, etc. Example cells: exchanger inputs/outputs at rows 32--35; pump at row 49--50; towers at rows 61--62; vessels at rows 67--68.
- `Utilities Summary`: equipment utility use and annual utility cost.
- `COM Summary`: revenue, raw materials, utilities, waste treatment, operating labour, COM without depreciation, working capital and project/economic inputs.
- `Cash Flow Analysis`: construction-period investment, depreciation, tax, discounted/non-discounted cash flow, NPV, discounted cash-flow rate of return and payback.
- `Monte Carlo Simulation`: parameter variation around FCI, product price, raw-material price, etc.
- `User Options`: hours/y, utility prices, equipment efficiencies and labour cost.
- `Equipment Cost Data`: embedded equipment correlations/factors.

**Representative formula provenance:**
- `Equipment Summary!H75`: total module cost = total bare-module cost × 1.18.
- `Equipment Summary!H76`: grass-roots cost = total module cost + 0.5 × base bare-module cost.
- `COM Summary!C37`: COMd = 0.18 FCIL + 2.76 COL + 1.23(CUT+CWT+CRM) in this workbook version.
- `COM Summary!C40:C43`: working-capital factor model.
- `Cash Flow Analysis!J50:J63`: discounted annual cash flows.

**Inputs required:** equipment-specific geometry/capacity, pressure, MOC, power/area/volume; CEPCI; utilities; raw-material rates/prices; waste; labour; FCI/land/working-capital/economic assumptions.

**Project applicability:** useful as a future chemical-process costing framework, but current project design information is insufficient to populate it consistently for the whole proposed plant.

### 1.2 `CCEP-Version 1-Read this first.xlsx`

**Type:** NUS Capital Cost Estimation Program documentation.

**Observed logic:** based on Seider, Seader and Lewin (2010). The user selects equipment and enters relevant equipment data; CCEP calculates equipment purchase cost and bare-module cost.

**Inputs required:** equipment identity and equipment-specific design data.

**Project applicability:** potentially useful for conventional chemical equipment if required design variables are available.

### 1.3 `CCEP-Version 1-Use this next.xls`

**Type:** legacy CCEP calculation workbook.

**Inspection status:** workbook is encrypted and cannot be read safely without a password/decryption step. No password was provided; no password guessing or macro execution was attempted.

**What can be supported from the companion file:** it is the CCEP platform described above and is based on Seider et al. correlations.

**Decision:** do not cite hidden cells/formulas from this workbook until it can be opened legitimately. The companion documentation is sufficient to classify the method, but not to adopt a numerical correlation from the encrypted workbook.

### 1.4 `DFP-Version 1 (Feb 2011).xlsm`

**Type:** NUS Detailed Factorial Program based on Sinnott & Towler (2009). VBA was not executed.

**Observed logic:**
- `DFP!B2`: user CEPCI.
- equipment blocks for agitators, boilers, compressors, dryers, evaporators, exchangers, furnaces, pumps/drivers, tanks, towers and user-added equipment.
- `DFP!B55:B58`: total ISBL and total fixed capital; `B58 = B56 × 1.82`.
- `Equipment Cost Data`: U.S. Gulf Coast January-2007 cost correlations `C_p^0=a+bS^n` with valid size ranges and equipment-specific data.

**Inputs required:** equipment-specific size/power/area/flow/mass, process type, MOC, quantity and CEPCI.

**Project applicability:** useful for future preliminary chemical-plant FCI estimation; not sufficient for nuclear-grade IHX/reactor island and not usable without missing equipment design data.

### 1.5 `CN4119-0-Second Part Intro.pptx`

**Role:** course context. It places costing/economics after PFD, reaction/separation design, heat-exchanger design/heat integration and process synthesis. This reinforces that equipment costing requires upstream design definition rather than labels alone.

### 1.6 `CN4119-1-Cost Estimation and Economic Analysis-Intro.pdf`

**Role:** scope/learning framework. Distinguishes capital/operating cost estimation from subsequent engineering economic analysis and profitability.

**Project implication:** the current abatement-cost metric is not itself a full profitability analysis.

### 1.7 `CN4119-2-Capital Cost Estimation.pdf`

**Core methods:**
- estimate class/accuracy depends on available design detail (pp. 4--5);
- equipment scaling `C/C_r=(S/S_r)^n` and six-tenths screening rule (pp. 7--12, 22--23);
- historical-cost escalation with a suitable cost index such as CEPCI (pp. 14--17);
- capital hierarchy: purchased equipment → direct/indirect project expense → bare-module cost → total-module cost → grass-roots cost (pp. 18--19);
- Turton/Guthrie module costing: `C_BM=C_P^0(B_1+B_2F_PF_M)` with purchase-cost, pressure and material correlations and validity ranges (pp. 24--35);
- `C_TM=1.18\sum C_BM` and grass-roots extension (p. 41);
- FCI is total-module cost or grass-roots cost depending on project context; terminology differs across references (p. 49).

**Project implication:** a bottom-up estimate needs equipment size, MOC and pressure. Those inputs are absent for most of the current proposed system.

### 1.8 `CN4119-3-Capital Cost Estimation-Scanned Appendix.pdf`

**Role:** correlation constants and factor tables for Turton-style costing:
- purchase-cost coefficients and valid size ranges;
- pressure-factor coefficients/ranges;
- material factors;
- bare-module coefficients/factors.

**Project implication:** these tables cannot be applied responsibly until the required equipment design variables are known and within correlation ranges.

### 1.9 `CN4119-4-Manufacturing Cost Estimation.pdf`

**Core methods:**
- COM = direct manufacturing + fixed manufacturing + general expenses (p. 3);
- direct costs include raw materials, waste, utilities, operating labour, supervision, maintenance, supplies, laboratory and royalties (p. 4);
- fixed costs include depreciation, taxes/insurance and plant overhead (p. 5);
- general expenses include administration, distribution/selling and R&D (p. 6);
- simplified formulas: `COM=0.28 FCI+2.73 COL+1.23(CRM+CWT+CUT)`; `COM_d=0.18 FCI+2.73 COL+1.23(CRM+CWT+CUT)` (p. 8);
- operating-labour correlation requires a count of fluid/solids processing steps (p. 11);
- utilities require duty/power/flow and current utility prices (pp. 14--20);
- annual raw-material/utility costs require operating time/stream factor (pp. 18--20).

**Project implication:** current economics do not constitute a complete COM/COMd calculation.

### 1.10 `CN4119-5-Engineering Economic Analysis.pdf`

**Core methods:** time value of money, discounted cash flows, depreciation, taxation, working capital, project construction/operation timing and profitability. Working capital is commonly 15--20% of FCI as a rule-of-thumb starting point (p. 16), with project-specific treatment required.

**Project implication:** the current CRF annualisation is a screening annual-cost comparison, not NPV/IRR/payback/profitability analysis. A full DCF model would require revenue/product-price, construction schedule, tax/depreciation, working capital, salvage/land and financing assumptions not currently defined for this project.

### 1.11 `Evaluating Capital Cost Estimation Programs.PDF`

**Type:** scanned Feng & Rangaiah (2011) comparison of CapCost, DFP, CCEP, EconExpert and Aspen PEA. The repository scan has no embedded text; the matching published article was cross-checked to interpret the scan.

**Methodological conclusion relevant here:** these are preliminary/study cost tools; equipment-level estimates can differ substantially because correlations, material/pressure factors and installation factors differ. Whole-plant estimates can be closer, but consistency requires using one method/program across design alternatives. Out-of-range equipment often requires multiple units or lower-bound treatment and must not be extrapolated casually.

**Project implication:** do not mix selected CAPCOST/DFP/CCEP equipment numbers opportunistically into the current ledger. A future bottom-up recost should select one coherent chemical-equipment method and apply it consistently.

## 2. Current canonical cost model

Canonical chain:
`model/src/final_design.rs::final_cost_ledger()`
→ generated cost ledger
→ active manuscript TEA.

Current annual bridge:
1. baseline natural gas: ~S$269.115m/y;
2. candidate natural gas: ~S$174.284m/y;
3. natural-gas saving: ~S$94.831m/y;
4. full Nishihara/JAEA source-product economic burden: ~S$76.572m/y;
5. annualised scaled IEAGHG CCS capital: ~S$10.683m/y;
6. annualised 10% integration/site allowance: ~S$2.857m/y;
7. CO2 T&S tariff: ~S$8.135m/y;
8. project electricity revenue: S$0/y;
9. net incremental annual cost: ~S$3.416m/y;
10. lifecycle avoided: ~917,139 tCO2e/y;
11. abatement cost: ~S$3.725/tCO2e.

### Boundary interpretation

This is a **differential screening annual-cost model**. It is not:
- a bottom-up purchased-equipment/bare-module estimate;
- a complete FCI/TCI estimate;
- a complete COM/COMd estimate;
- a discounted cash-flow profitability model;
- a bankable FOAK Singapore project estimate.

Common conventional SMR equipment costs are implicitly treated as common/cancelling unless separately represented. Nuclear common cost is represented through published source-product economics rather than equipment build-up. CCS is represented through an aggregate IEAGHG capital anchor rather than individual absorber/stripper/compressor costing.

## 3. Cost-component completeness and double-counting matrix

| Component | Current status | Attachment method applicability | Required project inputs available? | Reconciliation |
|---|---|---|---|---|
| Baseline/candidate natural gas | represented | manufacturing-cost raw material/fuel framework | yes: flow + screening price | retain current explicit differential term |
| Conventional/common SMR train | largely cancels by comparison boundary | CAPCOST/DFP/CCEP possible | no detailed equipment sizes | do not invent; future bottom-up differential check |
| Nuclear reactor/island | represented by Nishihara source-product economics | generic chemical CAPCOST/DFP inappropriate | no nuclear equipment design basis for generic correlations | retain nuclear source economics |
| IHX | only through source architecture/integration allowance; no project equipment estimate | generic exchanger correlation structurally possible but nuclear-grade service outside ordinary correlation intent | no project area, pressure drop, wall/material design, count | do not use generic exchanger cost; separate nuclear/component redevelopment needed |
| Secondary-He piping/circulator | not separately represented | compressor/piping methods could apply in principle | no pressure drop, diameter, circulator power or detailed routing | missing; cannot calculate |
| Reformer modification / helium-heated reformer | not separately represented | furnace/heater correlations not physically equivalent | no qualified geometry/area/MOC; 176.8 MWth exceeds DFP box-furnace range | missing; do not map to conventional furnace correlation |
| WGS reactor | common/source process but no differential equipment cost | reactor/vessel correlations possible | no volume/MOC/design pressure | cannot calculate |
| PSA | not separately bottom-up costed | no clear standard PSA package in DFP; user-added/vendor approach needed | no bed size/cycle/compressor design | cannot calculate |
| Amine absorber/stripper | represented inside aggregate IEAGHG CCS CAPEX | tower correlations possible | no diameter/height/packing/trays/design pressure | adding bottom-up tower costs now would double count aggregate CCS CAPEX |
| CCS reboiler/condenser | represented inside aggregate IEAGHG CCS CAPEX | exchanger correlations possible | no area/pressure/MOC | double-count risk if added separately |
| CO2 compressor/conditioning | aggregate CCS/T&S boundary; detailed split unclear | compressor correlations possible | no power/staging/pressure ratio | cannot add without scope audit; potential double count |
| Pumps | not separately differential | pump correlations possible | no pump duties/heads/flows | cannot calculate |
| Process electricity | physical 17.3 MWe source value exists; economic treatment is embedded in conservative full source burden rather than a separate utility bill | manufacturing utility costing possible | candidate demand exists; baseline anchor is only a project assumption; supply boundary unresolved | do not add separately without redesigning reactor/electricity allocation; otherwise double count |
| Cooling water/steam/other utilities | not separately represented as differential COM | utility costing framework applicable | detailed duties/flows absent | missing; cannot calculate |
| Operating labour | not separately represented | CN4119 NOL/COL method applicable | full equipment/process-step count and Singapore labour basis absent | missing |
| Maintenance/repairs, overhead, insurance, admin | not separately represented | COM/COMd factor method applicable once FCI/COL known | no complete FCI/COL | missing |
| Working capital | not represented | CAPCOST/CN4119 DCF framework applicable | no complete FCI/CRM/COL and no project financing boundary | not required for current annual abatement screen; needed for profitability model |
| Land/site | only 10% integration/site screening allowance | grass-roots/DCF framework possible | no selected site/land basis | current allowance remains explicit proxy |
| T&S | represented by S$15/t screening tariff | not an equipment-correlation problem | captured tonnes yes; contracted tariff no | retain as explicit screening assumption |
| Waste/spent fuel/decommissioning/security/licensing | not separately represented | outside ordinary chemical-equipment programs | project inputs absent | remain explicit FOAK/deployment omissions |

## 4. Double-counting audit

### 4.1 Nuclear source economics versus CAPCOST/DFP
Do **not** add generic reactor/IHX/turbine equipment costs on top of the Nishihara heat + electricity source-product burden. The current model deliberately charges the full published source-product burden while giving zero project electricity revenue. A bottom-up nuclear recost would be a replacement model, not an additive patch.

### 4.2 Aggregate CCS CAPEX versus individual amine equipment
Do **not** add CAPCOST/DFP absorber, stripper, reboiler, condenser or CO2-compressor costs without first replacing/decomposing the IEAGHG aggregate CCS capital term. Otherwise the same capture plant would be costed twice.

### 4.3 Process electricity
The current model's source electricity-product burden is an economic allocation used to close the full reactor source burden, not a project-export claim. Adding a separate 17.3-MWe utility charge without redefining which source burden supplies/allocates that electricity would create an inconsistent boundary. This needs a future energy/economic allocation model, not a one-line addition.

### 4.4 Integration/site allowance
The 10% integration/site allowance is intentionally broad. A future explicit IHX/secondary-loop/reformer-retrofit/site build-up must state whether it **replaces** all or part of this allowance; it cannot simply be stacked on top without a scope reconciliation.

## 5. Equipment-by-equipment costing decision

Classification:
- **A** attachment method can be used directly;
- **B** method usable after escalation/currency conversion;
- **C** usable only as sensitivity/cross-check;
- **D** unusable until missing design variables are supplied;
- **E** not appropriate for this equipment;
- **F** already covered elsewhere / double-count risk.

| Equipment/system | Class | Reason / missing input |
|---|---|---|
| Natural-gas consumption | A for operating-cost structure | flow and screening price already available; current method already explicit |
| Generic chemical heat exchanger | D | need area, shell/tube pressures and MOC |
| Nuclear IHX | D/E/F | need area/pressure/material/lifetime design; ordinary chemical correlation not nuclear-grade; source economics already represent architecture |
| Helium circulator | D/E | need shaft power/pressure rise/efficiency and nuclear-grade equipment basis |
| Secondary-He piping | D | need diameter, length, pressure class, insulation/material |
| Helium-heated reformer | D/E | need actual reformer heat-transfer geometry/material; conventional fired-heater correlation is not equivalent |
| WGS reactor | D | need volume, MOC, design pressure |
| PSA package | D/E | no sufficient bed/package design; vendor/user-added basis required |
| Amine absorber | D/F | need tower geometry/packing/pressure; currently inside aggregate CCS CAPEX |
| Amine stripper | D/F | same |
| Capture reboiler/condenser | D/F | need areas/pressures/MOC; aggregate CCS term already exists |
| CO2 compressor | D/F | need power/stages/pressure ratio and CCS scope split |
| Pumps | D | need power/head/flow/MOC |
| Storage/buffer vessels | D | need volume/pressure/MOC |
| Steam/cooling utilities | D | need actual utility duties/flows |
| Reactor/nuclear island | E/F | generic chemical equipment methods inappropriate; retain nuclear-specific source economics |
| Turbine/generator/power system | E/F | no validated project off-design power system; source economics already carry source-product burden |
| Site/offsites | D/F | grass-roots factors are possible only after coherent equipment build-up; current 10% allowance is a proxy |
| Operating labour | D | requires complete PFD equipment-step count and labour basis |
| COM/COMd | D | requires FCI, COL, utilities, waste and raw-material cost boundary |
| DCF/NPV/IRR/payback | D | requires complete TCI/working capital, product revenue, tax/depreciation, construction schedule and project finance assumptions |

**No attachment-derived equipment correlation is adopted numerically in the canonical model in this reconciliation.** Therefore there is no new numerical workbook-cell formula to reproduce in Rust.

## 6. Methodology improvements that can be adopted without changing the model

The following are **presentation/method-definition improvements**, not new cost numbers:

1. **Estimate-class statement:** describe the current economics as a screening/study-level differential annual-cost estimate, not a definitive capital estimate.
2. **Capital hierarchy vocabulary:** when a future bottom-up estimate is developed, distinguish purchased equipment, bare-module, total-module/grass-roots FCI, working capital and TCI.
3. **Operating-cost vocabulary:** distinguish raw materials, utilities, waste, labour, fixed manufacturing cost and general expenses; do not call the current ledger a complete COM.
4. **Cost-year discipline:** use a suitable equipment/plant cost index such as CEPCI for future chemical-equipment correlations, with explicit base/current index and currency conversion. Do not silently replace the existing source-specific escalation in the present model.
5. **Single-method consistency:** a future chemical-equipment build-up should use one coherent program/correlation family across alternatives rather than cherry-picking the lowest cost from CAPCOST, DFP and CCEP.
6. **Validity-range discipline:** do not extrapolate equipment correlations outside stated size/pressure ranges without an explicit engineering justification or multiple-unit treatment.
7. **Profitability separation:** CRF annualisation/abatement cost and DCF profitability are different questions. NPV/IRR/payback should not be implied until the necessary project cash-flow inputs exist.

## 7. What the attachments reveal about current cost completeness

### Supported/represented
- differential natural-gas expenditure;
- nuclear source-product economic burden from nuclear-specific literature;
- aggregate CCS capital annualisation;
- broad integration/site allowance;
- T&S tariff;
- conservative zero electricity revenue.

### Missing or only proxied relative to a full chemical-engineering TEA
- equipment-level chemical plant FCI for changed/new equipment;
- explicit nuclear/process IHX and secondary-He-loop project cost;
- reformer retrofit/replacement cost;
- capture-plant O&M decomposition;
- process utilities as an economic ledger;
- operating labour;
- maintenance/repairs;
- overhead/insurance/admin;
- working capital;
- construction schedule and interest during construction;
- depreciation/tax/salvage/land;
- full DCF/NPV/IRR/payback;
- Singapore FOAK/EPC/licensing/security/waste/decommissioning/insurance/schedule costs.

The attachments therefore **reinforce**, rather than remove, the manuscript's existing statement that S$3.725/tCO2e is not an all-in Singapore project quotation.

## 8. Impact on the canonical headline result

A complete attachment-based recost would require enough new design/economic assumptions that it constitutes **substantive model redevelopment**. It could materially change the numerator of the abatement-cost equation and therefore the S$3.725/tCO2e headline result. The direction cannot be quantified honestly from the current project inputs.

Accordingly:
- no speculative CAPCOST/DFP/CCEP values are inserted;
- no canonical result is changed;
- no manuscript headline is changed;
- no W5 work is started.

## 9. Required future inputs for a defensible bottom-up recost

At minimum:
- equipment list tied to the final PFD;
- reformer heat-transfer geometry/area and design pressure/material;
- WGS reactor size/design pressure/material;
- PSA package sizing or vendor correlation;
- amine absorber/stripper dimensions, packing/trays and design pressures;
- reboiler/condenser duties and areas;
- CO2 compressor suction/discharge states, staging and power;
- pump duties/heads/flows;
- secondary-He pressure, pressure drop, piping/circulator design and material;
- project IHX area/pressure/material/count and qualification concept;
- utility duties/flows;
- operating-labour basis;
- consistent CEPCI/base year and FX;
- explicit definition of which current aggregate/source cost blocks are replaced;
- if profitability is required: H2 revenue basis, construction schedule, land, working capital, tax/depreciation, discount rate, salvage and project life.

## 10. Reconciliation conclusion

**Methodology improvement can be adopted now; numerical model change cannot.**

The attachments show a more complete chemical-engineering cost-estimation framework than the current differential screen, but they do not supply the missing project equipment design variables. The defensible current action is therefore to preserve the canonical screening model and strengthen its methodological classification/completeness record.

A future cost-model redevelopment should be a separate gated task:
equipment/design-input inventory → select one coherent costing method → reproduce correlations outside spreadsheets → validate against workbook/reference cases → define replacement/double-counting boundary → recalculate FCI/COM/annualised cost → regression-test → only then reconsider the headline abatement cost.
