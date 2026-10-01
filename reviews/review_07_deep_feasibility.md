# Independent Review 07 — Deep Feasibility

## Repository state reviewed

Authoritative current HEAD independently recovered:

`5a975568dec6ce8aae6e6028766b05f1ec5ab08c`.

The integrated scientific/manuscript commit
`c6a811b4756541e7aa2e3536c6804765f27a5c19` was also inspected. The two later
commits modify closure/status evidence only and do not change the scientific
implementation or manuscript.

Current GitHub Actions at current HEAD:
- Research CI **36808010838 — PASS**.
- Paper/reproducibility CI **36808010823 — PASS**.
- current canonical artifact **11137759804**.
- `main.pdf`: **27 pages**.

The exact current artifact was downloaded, rendered and inspected page by page.
No clipping, overlapping figure/table content, broken equations, accidental
blank pages or broken glyphs were found. The current build converges and passes
the repository's unresolved-citation/reference integrity gate.

## Review scope

This review follows the Deep Feasibility instruction and does not repeat Reviews
1-6 or closed FDV2 work. Previously verified mathematics were reopened only
where the new deep-feasibility extensions introduced new equations,
sensitivities, source interpretations or manuscript claims.

The unavailable second professor/research transcript is treated as explicitly
waived, not as recovered evidence.

## Overall independent conclusion

The deeper feasibility research substantially strengthens the project.

The central result remains scientifically supportable **within its declared
screening boundary**:
- H2 production ~97,946 t/y;
- direct avoided CO2 ~862,094 t/y;
- lifecycle avoided ~917,139 tCO2e/y;
- candidate lifecycle intensity ~1.95 kgCO2e/kgH2;
- process heat 176.8 MWth;
- project electricity revenue S$0/MWh;
- central screening abatement cost ~S$3.725/tCO2e.

No new primary evidence found in this review invalidates those canonical
quantities.

The deeper evidence does, however, expose three material corrections required
before submission closure:
1. the new **availability economic sensitivity** is not physically consistent
   with plant downtime because installed/fixed economic burdens shrink with
   operating hours in the current function;
2. the new **CCS partial-delivery sensitivity** mixes plant resizing and
   storage/capture unavailability in one economic boundary;
3. the repository contains technically specific future-work plans, but the
   active manuscript contains **no Future Work section**, so several unresolved
   feasibility questions are not converted into the decision-oriented next
   analyses required by this phase.

These findings do **not** overturn the central quantitative result or the
CONDITIONAL MODEL PASS.

---

# A. Deep-feasibility question register

The register through approximately DF-40 is broad without becoming an arbitrary
question catalogue.

Material domains captured include:
- reactor family selection and outlet temperature;
- process temperature and duty;
- IHX capacity/material/lifetime;
- helium flow, pressure class, pressure drop and circulator uncertainty;
- availability, redundancy and backup heat;
- bidirectional reactor/process transients;
- air ingress, water ingress and loss-of-heat-sink sequences;
- TRISO and mechanistic source term;
- EPZ;
- siting and nuclear/chemical co-location;
- cooling/heat rejection;
- waste/spent fuel;
- tritium;
- fuel supply;
- human factors, I&C, cyber and physical security;
- Singapore regulatory readiness;
- CCS;
- economics/lifecycle;
- V&V and uncertainty.

No missing generic research question was found whose addition alone would
materially change the current scientific conclusion. The major unresolved
questions now genuinely require project/site/vendor/regulator inputs or
higher-fidelity modelling.

A small status-maintenance defect remains: DF-07/DF-09 still contain language
from before the implemented availability/gas-backup screens. See DFR-m01.

# B. Professor feedback / slide traceability

The available professor material has been used appropriately as a **question
generator**, not as scientific authority.

The research responds to the professor's substantive themes:
- risk as frequency plus consequence;
- EPZ justification;
- accident analysis;
- population/siting;
- cooling;
- waste;
- underground/offshore possibilities;
- security;
- high-temperature industrial heat;
- simulation/V&V.

The manuscript does not import the professor's several-hundred-metre example as
a Singapore EPZ.

The missing second transcript is explicitly waived and no scientific claim is
attributed to it.

# C. Reactor selection derivation

**PASS.**

The manuscript now derives the reactor requirement from the chemical process:

INL process
-> 871 C reformer outlet
-> 900 C secondary-helium supply
-> positive 29 C reported-location temperature difference
-> 25 C INL primary/secondary difference
-> 925 C reactor-outlet case
-> 176.8 MWth duty
-> reactor-family screen
-> GTHTR300C-class design basis.

The manuscript correctly warns that 900-871 C is **not** a local reformer
pinch calculation.

The helium energy-balance cross-check is internally consistent:
176.8 MW / [78.49 kg/s * (900-466) K]
= ~5.19 kJ/kg-K.

# D. Reactor technology screen

**PASS.**

- LWR/LWR-SMR: not rejected as a technology; rejected only as the direct
  ~900-C heat basis without temperature augmentation.
- HTTR: correct temperature demonstrator, insufficient project scale.
- HTR-PM: relevant operating/safety/licensing comparator, but its operating
  steam-cycle plant is not the selected 900-C secondary-helium reformer
  architecture.
- GTHTR300C: retained because the design family explicitly combines 600 MWth,
  ~950 C reactor outlet and high-temperature process-heat/cogeneration
  architecture.

The manuscript does not describe GTHTR300C as commercially operating.

Independent primary/source checks support the distinction:
- JAEA HTTR: 30 MWth, 950 C, 50-day high-temperature full-power operation;
- Kunitomi et al. 2007: conceptual GTHTR300C 600 MWth / 950 C and 170 MWth IHX;
- JAEA design variants include a higher-hydrogen branch around 370 MWth.

# E. Demonstrated / designed / modelled terminology

**PASS.**

Maturity is generally disciplined:
- HTTR high-temperature operation = demonstrated/operated;
- HTTR transient tests = experimental operating-reactor evidence;
- HTTR 10-MW IHX = constructed/tested precedent;
- GTHTR300(C) = design architecture;
- INL Case 6 = source-modelled process;
- project lifecycle/economics/sensitivities = project-derived screening;
- Singapore EPZ/project safety = unresolved.

No material maturity inflation was found.

# F. Japanese HTTR evidence

**PASS.**

Independent checks support:
- 30 MWth HTTR;
- 950 C outlet;
- 50 continuous days at full-power/high-temperature conditions in 2010;
- stable high-temperature heat-supply evidence;
- current programme still developing reactor-to-hydrogen/process-heat coupling;
- 2024 heat-load variation experiment: approximately 11 C inlet perturbation,
  reactor power ~90% to ~88%, outlet temperature nearly unchanged under the
  tested condition.

The manuscript correctly treats this as relevant experimental precedent, not
validation of the 600-MWth project.

# G. 176.8 MWth process heat

**PASS for provenance and present claim strength.**

176.8 MWth is a direct INL TEV-961 source-model result for the selected
nuclear-assisted process state, not a value invented from the project's
constant-Cp helium calculation.

The project independently reconciles it with source helium state/flow but does
not pretend to reproduce proprietary Aspen equipment internals.

Annual calculations consistently use the selected availability basis.

# H. 170 MWth IHX versus 176.8 MWth project duty

**PASS.**

Independent arithmetic:
- 176.8 - 170 = **6.8 MWth**;
- 176.8 / 170 = **1.040**.

The project duty is therefore ~4.0% above the published conceptual 170-MWth
GTHTR300C IHX.

The manuscript explicitly states that the 170-MW exchanger does **not** already
cover the project duty and treats larger/parallel/redesigned exchanger options
as future component qualification.

# I. 370/371 MWth branch meaning

**PASS.**

The manuscript now distinguishes:
- ~370/371 MWth **source process-heat branch/economic architecture**;
from
- ~170 MWth **reference physical conceptual IHX duty**.

It no longer calls the 370-MW branch one demonstrated exchanger.

# J. IHX materials / lifetime

**PASS at screening level.**

Hastelloy XR, high-temperature creep, creep-fatigue, pressure boundary,
inspection and permeation are treated as genuine design constraints.

The 10-MW HTTR IHX is used as constructed precedent. Commercial/project
176.8-MWth lifetime qualification is explicitly not demonstrated.

# K. Helium hydraulics

**PASS at screening level.**

INL Case 6: 78.49 kg/s, 900 C supply.
GTHTR300C reference: ~81 kg/s, ~5.15 MPa, 900 C supply.

78.49/81 = **0.969**.

This is useful flow-scale compatibility evidence. The project correctly refuses
to infer project pressure drop, pipe diameter or circulator power from similar
mass flow alone.

# L. Tritium / radiological isolation

**PASS.**

The manuscript no longer treats the IHX as perfect radiological isolation.

HTTR measured non-zero secondary-helium tritium; Hastelloy-XR permeation and
oxide-film mitigation are represented as finite transport/mitigation mechanisms.

The project appropriately refuses to scale HTTR tritium concentrations directly
to the 600-MWth project.

# M. Availability / reliability

## Emissions/throughput screen: PASS

The no-backup screen scales operating quantities linearly with effective annual
availability. Because lifecycle avoided emissions are linear in this defined
screen, the threshold is:

Acrit = 0.85 * 250,000 / 917,138.896
= **0.2317**, or approximately **23.2% availability**.

This arithmetic is correct.

The manuscript correctly states that this is an assignment-threshold screen,
not evidence that an industrial hydrogen plant is operationally acceptable at
23.2% availability.

## Economic sensitivity: correction required

See **DFR-M01**. The current availability function also scales reactor
source-product burden and throughput-scaled CCS CAPEX with availability. That
does not represent the fixed installed-cost consequences of downtime.

# N. Gas backup

**PASS as a lower-bound operating sensitivity.**

At 50% nuclear availability:
- backup service hours = (0.85-0.50)*8760 = **3066 h/y**;
- backup fuel = approximately **2.280 PJ/y** using 85.6% heater efficiency;
- backup direct CO2 = approximately **114.6 kt/y**;
- backup upstream burden = approximately **26.2 ktCO2e/y**;
- nuclear lifecycle burden reduced during backup hours by ~1.50 ktCO2e/y;
- lifecycle avoided = approximately **0.778 MtCO2e/y**;
- backup fuel cost = approximately **S$34.20m/y**;
- abatement cost = approximately **S$48.35/tCO2e**.

The code retains the base reactor economic burden and adds only backup variable
fuel cost. It explicitly excludes backup CAPEX, fixed O&M, staffing, startup and
integration.

Therefore it is correctly labelled a **lower-bound operating robustness
screen**, not a full backup-plant design.

# O. CCS robustness

## Emissions arithmetic: PASS as a narrow algebraic screen

At zero delivered canonical captured stream:
- the canonical captured stream is returned to direct emissions;
- nuclear heat and lower NG throughput remain;
- transport/storage burden falls to zero.

The resulting avoided emissions are approximately **0.388 MtCO2e/y**, above
the CN4252 0.25-Mt threshold.

The manuscript correctly says this does not prove CCS unnecessary and reports
the much higher direct candidate emissions (~0.582 MtCO2/y).

## Economic/operability boundary: correction required

See **DFR-M02**.

# P/Q. CO2 derivation and contribution bridge

**PASS.**

The canonical bridge remains:
- direct plant emissions saved ~862.094 kt/y;
- upstream NG saved ~72.704 ktCO2e/y;
- nuclear heat lifecycle burden ~3.649 ktCO2e/y;
- incremental auxiliary electricity ~0.450 ktCO2e/y;
- CO2 T&S lifecycle burden ~13.559 ktCO2e/y;
- net lifecycle avoided ~917.139 ktCO2e/y.

Signs and units are correct. Captured CO2 is not double counted as avoided
emissions.

The Rust bridge traces directly to `final_lifecycle_ledger()`.

# R/S/T. Cost derivation, bridge and economic boundary

**PASS for the canonical central result.**

The central cost bridge reconciles:
- NG expenditure saved ~S$94.831m/y;
- reactor source-product burden ~S$76.572m/y;
- CCS annualised capital ~S$10.683m/y;
- integration/site allowance ~S$2.857m/y;
- T&S ~S$8.135m/y;
- project electricity revenue = **S$0/y**;
- net annual incremental cost ~S$3.416m/y.

3.416m / 917,139 ~= **S$3.725/tCO2e**.

The manuscript clearly calls this a screening-model abatement cost and lists
unrepresented/partly represented FOAK, financing, licensing, security, EPC,
contingency, waste, decommissioning, insurance, schedule, site and CCS-contract
costs.

# U/V. Chinese HTR-PM EPZ evidence and transferability

**PASS.**

Independent evidence confirms the project's key distinction:

1. Ding et al. 2018 HTR-PM research:
   design-specific technical analysis supports EPZ at the exclusion-area /
   several-hundred-metre scale.

2. Chinese implemented HTR-PM emergency plan:
   MEE documentation cites **3 km inner plume, 7 km outer plume and 30 km
   ingestion zones**.

3. Other Chinese small-reactor examples demonstrate scalable EPZ methodology,
   not a universal HTR-PM/Singapore radius.

4. Singapore project:
   **NO CALCULATED EPZ**.

The manuscript does not transfer the Chinese numerical distances to Singapore.

# W/X. EPZ methodology and mechanistic source term

**PASS.**

The repository uses the defensible chain:

accident sequence
-> frequency
-> radionuclide inventory
-> fuel release
-> graphite/core retention
-> primary-circuit transport / plate-out / dust
-> confinement release
-> atmospheric dispersion
-> dose
-> protective action
-> EPZ.

AGR evidence is nuclide-specific; TRISO is not treated as one perfect retention
factor. Cs, Ag, Sr/Eu and Kr behaviour are distinguished.

AVR dust/plate-out experience is retained as counter-evidence/mechanism evidence
without numerically transferring pebble-bed contamination to the prismatic
project.

HTR-STAC provides external precedent for a multi-module HTR source-term analysis
with V&V; the project correctly states that it has **not calculated a project
source term**.

# Y. Accident phenomena

**PASS for hazard completeness / claim strength.**

The selected-design accident register includes:
- LOFC;
- depressurisation;
- air ingress;
- water ingress;
- loss of ultimate heat sink;
- IHX leak;
- secondary blowdown;
- process trip;
- reactor trip;
- chemical releases/fire/explosion;
- external industrial hazards;
- coastal flooding;
- loss of electrical power;
- CCS interruption;
- spent-fuel events;
- security/external events.

The register is explicitly not a PRA and contains no invented frequencies or
off-site doses.

# Z. Nuclear / chemical interface

**PASS at screening level.**

The hazard register covers H2, methane/syngas, CO, steam, CO2, amine, fire,
explosion, toxic/asphyxiant release, pressure failure, common power, external
industrial hazards, flooding and security.

Separation distance remains site-specific and is not invented.

# AA/AB. Process trip and loss of nuclear heat

**PASS in evidence classification.**

Japanese mock-up/component and HTTR reactor-level heat-load experiments support
the process-load-loss direction.

The opposite direction — reactor trip / loss of nuclear heat to the reformer —
remains correctly unresolved. The repository identifies steam/carbon
management, coking, feed isolation, depressurisation, thermal trajectory and
restart criteria as requiring a project dynamic safe-state model.

The project does not claim to possess that model.

# AC/AD. Jurong and alternative siting

**PASS.**

The Devanand et al. 2019 Jurong/J-Park work is correctly described as a
preliminary academic optimisation/screening tool, not nuclear site approval.

Jurong, a future western island, other coastal sites, underground and
floating/offshore concepts remain options. No site is selected.

# AE. Cooling / heat rejection

**PASS.**

The project correctly states:

**423.2 MWth remaining reactor thermal capacity != 423.2 MWth cooling duty.**

An exact heat-rejection duty requires a coherent project power/heat cycle and
state points.

Singapore once-through seawater, seawater cooling tower and NEWater options are
treated as option evidence, while coastal flood, sea-level, fouling, marine
thermal discharge and external hazards remain site-dependent.

# AF. Waste / spent fuel

**PASS at CN4252 level.**

The evidence correctly avoids importing an LWR spent-fuel-pool requirement into
all HTGRs.

Dry interim storage has HTGR design/operating precedent, but final disposal,
graphite waste, C-14/Cl-36, confinement/ventilation, safeguards and national
policy remain deployment gates.

# AG. Fuel supply

**PASS.**

The selected GTHTR300-class fuel is correctly recognised as HALEU-class LEU /
TRISO (~14% average U-235 class, design-dependent zoning).

Japanese HTGR fabrication experience is distinguished from a qualified
commercial GTHTR300 project supply chain.

Singapore procurement, safeguards, transport and vendor qualification remain
unresolved dependencies.

# AH. Human factors / I&C / cyber / security

**PASS at screening level.**

Passive/inherent safety is not portrayed as eliminating operators,
maintenance, procedures, I&C, cyber protection, physical security or safety
culture.

These remain appropriately project/regulator-specific.

# AI. Singapore regulatory readiness

**PASS and current.**

Independent current official evidence confirms:
- no Singapore nuclear deployment decision;
- capability building is ongoing;
- INIR Phase 1 starts in 2027;
- the Milestones/INIR framework evaluates 19 infrastructure issues;
- safety, reliability, affordability and environmental sustainability are
  decision criteria;
- Singapore's dense-city-state context makes safety a particularly demanding
  consideration.

The manuscript describes readiness as under development, not permanently
impossible.

# AJ. CCS infrastructure

**PASS in policy interpretation.**

Singapore lacks suitable domestic geological storage and is pursuing
cross-border CCS.

Government agreements/cooperation are correctly distinguished from an
operating, contracted storage service for this project.

The project's S$15/t T&S value remains visibly a screening assumption.

# AK. V&V matrix

**PASS.**

The repository explicitly distinguishes:
- verification;
- validation;
- uncertainty;
- extrapolation.

Rust tests are not called physical validation.

The matrix correctly limits the Rust model's validity and identifies unmodelled
PRA, source term, EPZ, QRA, site selection and bankable FOAK economics.

# AL. Figures

**PASS visually and reproducibly, subject to the sensitivity-model findings.**

Current 27-page artifact inspection:
- availability/abatement plot: readable, correct threshold and units;
- CCS robustness plot: readable and correctly labelled as screening;
- CO2 bridge: signs/units visually clear;
- cost bridge: saved vs added contributions clear;
- reactor selection and thermal-capacity visuals: readable;
- feasibility tables: readable;
- no clipping/overlap found.

Rust chain is deterministic:
canonical model
-> Rust generators
-> generated TikZ/CSV
-> LaTeX
-> PDF.

The figures faithfully display the implemented models. DFR-M01/M02 concern the
scientific boundary of two sensitivity models, not plotting fidelity.

# AM/AN. Scientific-paper quality and two-reader test

**PASS with one major completeness correction (DFR-M03).**

The manuscript reads as an engineering paper, not as a pasted research register.

Reader A can understand:
- why high-temperature helium is selected;
- the 925->900->871 C chain;
- why 176.8 MWth matters;
- what the IHX does;
- why 170 versus 176.8 MWth matters;
- where carbon and money move;
- why the numerical thresholds pass;
- why EPZ is unknown;
- why Chinese distances cannot be copied;
- why deployment remains conditional.

Reader B can trace source -> evidence class/assumption -> equation/model ->
V&V -> generated figure/table -> result -> limitation -> CN4252 interpretation.

However, the active manuscript has **no Future Work section**. The repository
contains excellent future-work definitions, but a submission reader does not
receive them in a consolidated decision-oriented form.

# AO. CN4252 requirements

**PASS.**

The manuscript answers more than the two numerical thresholds:
- >0.25 MtCO2e/y;
- <S$100/tCO2e;
- technical feasibility;
- potential effectiveness;
- nuclear/process safety limitations;
- Singapore context;
- siting/cooling/CCS/regulatory dependencies;
- implementation roadmap;
- accuracy/evidence maturity.

# AP. Future Work

**NOT YET ADEQUATE IN THE ACTIVE MANUSCRIPT.**

The repository contains specific future analyses in the question register,
accident register, hazard register and saturation record.

But `paper/main.tex` contains no active Future Work section.

For the major unresolved questions, the final paper should expose a compact
decision-oriented table with:
- QUESTION;
- NEXT ANALYSIS / EXPERIMENT;
- REQUIRED INPUT;
- EXPECTED OUTPUT;
- DECISION ENABLED.

At minimum:
1. project 176.8-MWth IHX qualification;
2. reactor-trip -> reformer safe-state dynamics;
3. mechanistic source term/PRA -> dispersion/dose -> Singapore EPZ;
4. chemical/nuclear separation/QRA;
5. project heat-rejection/cooling design;
6. tritium transport/radiological classification;
7. site comparison;
8. CCS contract/outage design;
9. bankable FOAK economics.

See DFR-M03.

# AQ. Overall scientific conclusion

The current **CONDITIONAL MODEL PASS remains justified**.

Evidence supports:
- numerical threshold pass under the declared model;
- conditional engineering feasibility at screening level;
- unresolved deployment feasibility.

No finding in this review requires changing the canonical 97,946 tH2/y,
917,139 tCO2e/y or S$3.725/tCO2e central screening result.

The new sensitivity-model findings require correction of robustness/economic
interpretation only.

# AR. Reproducibility

**PASS.**

Current authoritative evidence:
- Research CI 36808010838: PASS;
- Paper/reproducibility CI 36808010823: PASS;
- current artifact 11137759804;
- main.pdf: 27 pages;
- exact current artifact rendered and visually inspected: PASS.

The documented build regenerates Rust outputs and manuscript assets before
LaTeX compilation.

---

# Findings

## DFR-M01 — MAJOR

**Exact file / section:**
`model/src/final_design.rs::availability_sensitivity`;
`model/src/bin/final_design_reliability_summary.rs`;
any generated availability-cost output/interpretation.

**Issue:**
The no-backup availability function is scientifically valid for the linear
throughput/emissions screen, but its economic output is not a physically
consistent plant-downtime sensitivity.

**Evidence:**
When availability `a` falls, the function:
- scales the Nishihara reactor heat/electric product burden with `hours=a*8760`;
- calls `ccs_capex_sgd(captured)`, causing installed CCS capital to shrink with
  annual captured throughput;
- scales integration cost partly through the throughput-scaled CCS capital.

A real plant that experiences lower availability does not automatically become
a smaller installed reactor/CCS plant.

**Why it matters:**
The calculated S$/t at low availability can be artificially favourable because
fixed/installed burdens disappear with downtime. This does not affect the
23.2% **emissions threshold**, which is correctly linear, but it affects any
economic conclusion drawn from the no-backup availability sensitivity.

**Required correction:**
Either:
1. restrict the no-backup availability sensitivity explicitly to
   throughput/emissions and remove its economic outputs; or
2. construct a fixed-versus-variable cost decomposition in which installed
   reactor/CCS/integration capital/fixed burdens remain installed while only
   justified variable terms scale with operating hours.

Do not invent unavailable fixed/variable fractions.

**Changes:**
- central quantitative result: **NO**;
- feasibility interpretation: **YES, robustness economics only**;
- safety interpretation: **NO**;
- manuscript: **YES if availability cost is discussed/shown**.

---

## DFR-M02 — MAJOR

**Exact file / section:**
`model/src/final_design.rs::ccs_capture_sensitivity`;
`literature/CCS_AVAILABILITY_ROBUSTNESS_EVIDENCE.md`;
Section 7.3 robustness discussion.

**Issue:**
The CCS sensitivity combines two different physical questions:
- alternative capture-plant sizing/capture fraction;
- temporary or structural cross-border storage/capture availability.

At reduced `f`, the function scales CCS CAPEX down with captured throughput
while holding process electricity fixed at the canonical source operating
point.

**Evidence:**
For `f<1`:
- uncaptured canonical stream is emitted;
- T&S burden/cost scales with `f`;
- `ccs_capex_sgd(captured)` shrinks installed CCS capital;
- auxiliary/process electricity does not change.

A storage-chain outage would not erase already-installed capture CAPEX.
Conversely, a genuinely smaller capture plant would generally change solvent,
compression and electricity requirements.

**Why it matters:**
The **emissions-only** zero-delivered-stream result (~0.388 MtCO2e/y) is useful
as a narrow algebraic robustness bound. The economic/turndown interpretation is
not physically self-consistent.

**Required correction:**
Split the concepts:
1. an **emissions-only delivered-storage/capture robustness screen**, clearly
   holding the source process fixed and making no cost/operability claim; and/or
2. a separate design capture-fraction sensitivity only if defensible energy and
   installed-cost scaling can be sourced.

For an outage interpretation, installed CAPEX must not disappear.

**Changes:**
- central quantitative result: **NO**;
- feasibility interpretation: **YES, CCS robustness only**;
- safety interpretation: **NO**;
- manuscript: **YES, robustness wording/figure caption if necessary**.

---

## DFR-M03 — MAJOR

**Exact file / section:**
`paper/main.tex` and active `paper/sections/*.tex`.

**Issue:**
The active manuscript has no Future Work section even though the deep
feasibility phase identifies multiple decision-critical unresolved analyses.

**Evidence:**
Repository research records define technically specific next work for EPZ,
source term, IHX qualification, reformer safe-state dynamics, tritium, cooling,
site hazards and FOAK economics. None is consolidated into an active Future
Work section.

**Why it matters:**
The Deep Feasibility objective is not merely to label questions unresolved but
to show exactly what would resolve them. Without this, the paper's
"conditional" conclusion is scientifically honest but less actionable and does
not fully expose the research programme created by the extension.

**Required correction:**
Add a compact Future Work section/table containing, for each major unresolved
item:
QUESTION -> NEXT ANALYSIS/EXPERIMENT -> REQUIRED INPUT -> EXPECTED OUTPUT ->
DECISION ENABLED.

Do not turn the full DF register into the manuscript.

**Changes:**
- central quantitative result: **NO**;
- feasibility interpretation: **NO change to current classification, but
  improves closure**;
- safety interpretation: **NO**;
- manuscript: **YES**.

---

## DFR-m01 — MINOR

**Exact file / section:**
`literature/DEEP_FEASIBILITY_QUESTION_REGISTER.md`, especially DF-07 and DF-09.

**Issue:**
Some status text predates the completed Rust sensitivities. DF-07 still reads
as unresolved despite the implemented no-backup threshold screen; DF-09 still
contains pre-implementation wording about the quantitative gas-backup case.

**Evidence:**
Current Rust implements and tests both sensitivities; the manuscript reports
their bounded results.

**Required correction:**
Update statuses to distinguish:
- **screening calculation answered/implemented**;
- **physical reliability/backup design unresolved**.

**Changes:** repository traceability only.

---

## DFR-m02 — MINOR

**Exact file / section:**
`STATUS.md`.

**Issue:**
Earlier paragraphs still call the second professor transcript a source-recovery
blocker, while later closure text records the user's explicit waiver.

**Required correction:**
Make the current status internally consistent: unavailable + waived, not an
active blocker.

**Changes:** repository/status clarity only.

---

## DFR-P01 — PRESENTATION

**Exact file / section:**
Section 7.3 availability figure/caption.

**Issue:**
The dotted gas-backup curve and solid no-backup curve use the same x-axis label
"Effective annual availability (%)", although the dotted curve's x coordinate is
specifically **nuclear availability while process-service hours remain at 85%**.

**Why it matters:**
A high-school reader can reasonably read both curves as the same definition of
availability.

**Required correction:**
Clarify in the axis/caption/legend that the dotted series uses nuclear-source
availability with gas backup maintaining 85% process-service hours.

**Changes:** presentation only.

---

# Required final verdicts

1. **Reactor selection scientifically justified?** YES, for the direct
   high-temperature-heat screening objective.
2. **Reactor-temperature selection scientifically justified?** YES at
   source-model/component-screening level.
3. **176.8 MWth correctly derived?** YES as an INL source-model duty, with
   independent consistency checks.
4. **170 MWth IHX comparison correctly interpreted?** YES.
5. **370/371 MWth branch correctly interpreted?** YES.
6. **Japanese HTTR/GTHTR evidence used appropriately?** YES.
7. **Chinese HTR-PM EPZ evidence interpreted correctly?** YES.
8. **Project correctly refuses to assign Singapore EPZ?** YES.
9. **Mechanistic source-term limitations honest?** YES.
10. **TRISO safety claims appropriately bounded?** YES.
11. **Nuclear/chemical co-location treated adequately?** YES at screening
    scope; project QRA remains unresolved.
12. **Availability sensitivity correct?** EMISSIONS/THROUGHPUT: YES.
    ECONOMIC OUTPUT AT REDUCED AVAILABILITY: **CORRECTION REQUIRED (DFR-M01)**.
13. **Gas-backup sensitivity correct?** YES as explicitly lower-bound variable
    operating sensitivity.
14. **CCS sensitivity correct?** EMISSIONS ROBUSTNESS: YES.
    ECONOMIC/OPERABILITY INTERPRETATION: **CORRECTION REQUIRED (DFR-M02)**.
15. **CO2 derivations correct and understandable?** YES.
16. **Cost derivations correct and understandable?** YES for canonical central
    case.
17. **~917,139 tCO2e/y remains supported?** YES.
18. **~S$3.725/tCO2e remains supported within screening boundary?** YES.
19. **Siting/Jurong conclusions appropriately bounded?** YES.
20. **Cooling/heat rejection treated correctly?** YES.
21. **Waste/spent-fuel treatment adequate?** YES for CN4252 scope.
22. **Human factors/security treatment adequate?** YES for screening scope.
23. **Singapore regulatory readiness represented accurately?** YES.
24. **CCS infrastructure uncertainty represented accurately?** YES; robustness
    sensitivity economics needs DFR-M02 correction.
25. **V&V terminology scientifically correct?** YES.
26. **New Rust figures reproducible and faithful?** YES to implemented models;
    DFR-M01/M02 concern model meaning, not rendering/provenance.
27. **Professor feedback adequately incorporated?** YES for the available
    material; missing second source correctly waived.
28. **CN4252 requirements fully answered?** YES at assignment/screening level.
29. **CONDITIONAL MODEL PASS remains justified?** **YES**.
30. **Ready for bounded correction pass and final submission closure?**
    **YES — bounded corrections required; no further broad research pass is
    justified.**

# Finding counts

- BLOCKER: **0**
- MAJOR: **3**
- MINOR: **2**
- PRESENTATION: **1**

# Effect on canonical result

No finding invalidates:
- 97,946 tH2/y;
- 862,094 t/y direct avoided;
- 917,139 tCO2e/y lifecycle avoided;
- 1.95 kgCO2e/kgH2;
- 176.8 MWth process duty;
- S$3.725/tCO2e central zero-electricity-credit screening result.

# Effect on final interpretation

The **CONDITIONAL MODEL PASS remains scientifically justified**.

The deeper review strengthens, rather than weakens, the reason for the word
"conditional": reactor/process temperature compatibility and the CN4252
screening thresholds are supported, while project IHX qualification, dynamic
safe-state design, source term/EPZ, site safety, regulatory readiness, cooling,
fuel/waste, security and CCS infrastructure remain real deployment gates.

# Independent review decision

**DEEP FEASIBILITY VERIFIED WITH BOUNDED CORRECTIONS REQUIRED**

Main Research should perform one bounded correction pass addressing DFR-M01,
DFR-M02, DFR-M03, DFR-m01, DFR-m02 and DFR-P01, rerun Research/Paper CI and
exact-PDF QA, and then proceed to final submission closure rather than another
broad research cycle.
