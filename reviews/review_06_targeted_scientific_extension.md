# Independent Review 06 — Targeted Scientific Extension

## Repository state reviewed

Authoritative current HEAD reviewed:
`3a1311d260bfde523de1afef86056558862f4ab9`.

The targeted-extension scientific/layout milestone at
`86d7e617ef69d3e84b241b8dda9eb611d5efe3ee` was inspected together with its
subsequent evidence/status closure commits. No subsequent scientific change was
found that invalidates the extension state.

Current independent CI check:
- Research CI 36726475287: PASS.
- Paper/reproducibility CI 36726475326: PASS.
- current manuscript artifact: 11103057397.
- current PDF: 19 pages.

The exact current artifact was downloaded, rendered and inspected page by page.

## Scope

Independent review of the targeted scientific extension only:
- scientific-paper quality;
- nuclear literature and evidence maturity;
- nuclear and process safety;
- complete CN4252 feasibility;
- Singapore context;
- interpretation of the screening economics;
- CO2 and cost ledgers;
- Rust visualisation/provenance;
- accessibility;
- implementation roadmap;
- reproducibility.

Reviews 1-5 and closed FDV2 mathematics were not repeated. The verified
quantitative foundation was reopened only where the extension itself created a
new contradiction; none was found.

## 1. Scientific-paper quality

**Assessment: sufficiently strong for the CN4252 scientific/engineering scope.**

The active manuscript now has a coherent paper sequence:
research question -> technical background -> system boundary -> final
literature-supported design/method -> nuclear feasibility/safety -> integrated
configuration -> results -> carbon/cost ledgers -> comparators -> Singapore
feasibility/roadmap -> limitations -> discussion -> conclusions ->
reproducibility/traceability.

This is materially better than an assembly of independent technical sections.

The scientific-paper benchmark was actually applied:
- source evidence precedes transferred claims;
- evidence maturity is stated;
- equations are introduced at the point of physical/economic need;
- Results separates model outputs from interpretation;
- Discussion and Limitations bound the claims rather than repeat the numerical
  pass;
- Conclusions distinguish numerical threshold compliance from deployment
  feasibility.

Paragraph-level audit found no citation dumping or systematic unsupported
conclusion-first writing. The safety section is particularly disciplined.

## 2. Nuclear-literature rigour

**Assessment: sufficiently rigorous for the stated screening scope.**

The evidence matrix and active manuscript distinguish:
- DEMONSTRATED: HTTR high-temperature operation and safety-test precedent;
- EXPERIMENTAL: AGR/TRISO retention and failure evidence;
- CONSTRUCTED/TESTED: HTTR 10-MW helium-helium IHX;
- DESIGNED: GTHTR300C architecture;
- MODELLED: INL nuclear-assisted reforming state;
- PROJECT ASSUMPTION/SCREEN: lifecycle/economic extensions and unresolved
  project-scale engineering.

Independent source checks support the main transfers:
- HTTR/HTGR safety evidence is not used to claim a demonstrated 600-MWth plant;
- AGR/TRISO evidence is correctly described as strong retention with retained
  failure/release mechanisms, not perfect containment;
- HTTR IHX evidence genuinely addresses Hastelloy XR, >900 C service, creep,
  creep-fatigue, seismic, thermal hydraulics and inspection;
- NRC/IAEA coupling evidence genuinely supports separation, inventory/layout,
  intermediate-loop and chemical-to-nuclear propagation concerns.

No important nuclear citation was found supporting a materially stronger claim
than its source.

## 3. Nuclear safety

**Assessment: adequate for CN4252 screening scope; project safety remains
correctly NOT DEMONSTRATED.**

The manuscript addresses:
- low power density / graphite thermal inertia;
- negative temperature feedback;
- decay-heat-level response in HTTR LOFC evidence;
- TRISO retention and failure mechanisms;
- graphite;
- helium;
- depressurisation / air ingress;
- water/steam ingress;
- graphite oxidation;
- fission-product transport;
- IHX pressure-boundary role;
- coupled nuclear/chemical transients;
- chemical-to-nuclear propagation;
- separation/layout;
- emergency planning and regulatory acceptance.

It explicitly rejects "risk free" / impossible-release reasoning and says the
screening study is not a site-specific nuclear safety case.

The NRC NGNP PIRT independently confirms that coupled chemical/nuclear safety is
less mature than standalone HTGR safety and that separation, inventories,
layout and intermediate-loop failures require explicit treatment.

## 4. Process safety / nuclear-chemical interface

**Assessment: adequate for scope.**

The active paper identifies hydrogen, methane, CO, hot/high-pressure
syngas/steam, reformer fire/explosion, solvent/capture equipment and compressed
CO2 hazards. It requires future PHA and site-specific propagation/layout work.

Primary/secondary helium separation and IHX isolation are explained clearly.
The absence of detailed HAZOP/LOPA/QRA is stated as a limitation rather than
silently treated as completed work.

## 5. Complete CN4252 answer

**Assessment: substantially complete.**

The official assignment requires >0.25 MtCO2e/y, <S$100/tCO2e, explanation of
abatement, an implementation roadmap, and evaluation of feasibility,
effectiveness, accuracy and presentation.

The manuscript now addresses:
- numerical abatement;
- cost;
- thermodynamic/process feasibility;
- heat-source compatibility;
- project scale;
- nuclear safety;
- chemical/process safety;
- nuclear technology maturity;
- Singapore nuclear readiness/regulation;
- siting/emergency-planning dependency;
- CCS transport/storage;
- natural-gas dependence;
- economic bankability limitations;
- staged implementation.

The SUPPORTED / CONDITIONAL / UNRESOLVED / NOT DEMONSTRATED categories are
evidence-based and map cleanly to the literature/source maturity.

## 6. Singapore feasibility

**Assessment: current and appropriately bounded.**

Independent current-source checks confirm:
- Singapore has not made a nuclear deployment decision;
- INIR Phase 1 begins in 2027;
- safety, reliability, affordability and environmental sustainability are
  official decision criteria;
- EMA explicitly notes the special safety demands of a small, densely populated
  city-state;
- Singapore lacks suitable domestic geological CO2 storage and is pursuing
  cross-border CCS;
- CCS cost/value-chain details remain under study.

The manuscript does not invent a site or deployment date.

## 7. Economic result

**Assessment: appropriately bounded.**

The ~S$3.725/tCO2e value is clearly described as a **modelled screening
abatement cost**, not a real all-in Singapore project quotation.

The manuscript explicitly identifies important unpriced/incompletely priced
items:
- project-specific FOAK premium;
- financing structure;
- Singapore site/construction premium;
- licensing/security;
- EPC/contingency;
- emergency-planning infrastructure;
- detailed 176.8-MWth IHX qualification;
- waste/decommissioning;
- insurance/liability;
- schedule risk;
- negotiated cross-border CCS infrastructure/contracts.

The controlling boundary retains **S$0/MWh project electricity revenue**.

No new evidence was found requiring the closed FDV2 arithmetic to be reopened.

## 8. CO2 ledger

**Assessment: PASS.**

Rust chain:
`final_design.rs::final_lifecycle_ledger`
-> `final_design_lifecycle_ledger_csv.rs`
-> generated lifecycle ledger
-> manuscript Table 2.

The ledger correctly distinguishes direct CO2 from lifecycle CO2e.

Reconciliation:
- direct plant emissions saved ~862,094 tCO2e/y;
- upstream NG saved ~72,704;
- nuclear heat added ~3,649;
- incremental auxiliary electricity added ~450;
- transport/storage lifecycle burden added ~13,559;
- net lifecycle emissions saved ~917,139 tCO2e/y.

862,094 + 72,704 - 3,649 - 450 - 13,559 ~= 917,140 tCO2e/y,
with the 1-t/y difference due only to displayed integer rounding.

Captured CO2 is not counted a second time as avoided emissions.

The SAVED / ADDED / NET presentation is understandable to a non-specialist.

## 9. Cost ledger

**Assessment: PASS.**

Rust chain:
`final_design.rs::final_cost_ledger`
-> `final_design_cost_ledger_csv.rs`
-> generated cost ledger
-> manuscript Table 3.

Displayed ledger:
- NG expenditure saved: ~S$94.831m/y;
- full selected reactor economic burden added: ~S$76.572m/y;
- CCS capital annualisation: ~S$10.683m/y;
- integration/site allowance: ~S$2.857m/y;
- CO2 T&S: ~S$8.135m/y;
- project electricity revenue: S$0/y;
- net annual cost change: ~S$3.416m/y.

S$3.416m / 917,139 tCO2e/y ~= S$3.725/tCO2e.

The ledger is consistent with the canonical zero-credit final model.

## 10. Rust figures and supervisor-code lesson

**Assessment: PASS with one provenance-description clarification noted below.**

The supervisor repositories were not merely named. Independent inspection of
the public OUTRAM PARK code confirms the relevant pattern:
- backend state stores engineering data;
- plotting code converts state into explicit plot-point vectors;
- `egui_plot::Plot`, `Line` and `PlotPoints` are used;
- axes carry engineering units;
- display code is separated from simulation/state logic.

The CN4252 extension adopts the useful architectural principle rather than
copying unrelated reactor-transient code:
canonical model -> deterministic data/generator -> publication visual.

The Rust-generated thermal-capacity figure is actually included in the
manuscript and derives 600, 370, 176.8 and 423.2 MWth from canonical constants.

The lifecycle and cost tables are likewise Rust-generated from canonical ledger
functions.

The Plotters threshold SVG is a reproducible generated asset, but the active
manuscript's threshold illustrations are separate TikZ figures rather than that
SVG. This is not a scientific defect because their values are simple canonical
results, but `results/FIGURE_PROVENANCE.md` should avoid implying that every
submission threshold visual is the Plotters SVG.

## 11. Accessibility

### Reader 1 — technically curious high-school student
**PASS overall.**

The manuscript now explains:
- what SMR does;
- why carbon remains;
- what CCS changes;
- what nuclear heat changes and does not change;
- HTGR, TRISO, helium and IHX at conceptual level;
- why the primary and chemical circuits are separated;
- where lifecycle emissions are saved/added;
- where annual costs are saved/added;
- why both CN4252 numerical thresholds pass;
- why deployment remains conditional.

The ledgers and simple process schematics materially improve accessibility.

### Reader 2 — university engineering marker
**PASS.**

The repository supports the chain:
source -> evidence class/assumption -> equation/model -> deterministic result ->
generated table/figure -> interpretation -> assignment requirement.

## 12. Implementation roadmap

**Assessment: PASS.**

The roadmap appropriately uses decision gates rather than invented dates:
1. evidence/capability;
2. Singapore nuclear readiness;
3. CCS-chain qualification;
4. vendor/component qualification;
5. non-nuclear integration demonstration;
6. nuclear safety/licensing case;
7. FOAK project decision;
8. construction/commissioning/operation.

Dependencies include regulation, siting, CCS, component qualification,
demonstration, licensing, financing and project development.

## 13. Reproducibility

**Assessment: PASS.**

Current GitHub evidence:
- Research CI 36726475287: PASS.
- Paper/reproducibility CI 36726475326: PASS.
- current PDF artifact 11103057397.
- PDF: 19 pages.

The current artifact was rendered and inspected. No clipping, overlapping
figures/tables, blank pages or broken glyphs were observed. The feasibility
table, roadmap, nuclear-safety section and ledger tables are readable.

The paper build runs Rust tests, regenerates canonical data/figures, compiles
LaTeX/BibTeX and rejects unresolved final citations/references.

## 14. Verified quantitative foundation

**Assessment: remains valid.**

The targeted extension did not introduce a changed model logic, contradictory
primary evidence, arithmetic defect or broken provenance that requires reopening
the closed FDV2 result.

Preserved values:
- ~97,946 tH2/y;
- ~862,094 t/y direct avoided CO2;
- ~917,139 tCO2e/y lifecycle avoided;
- ~1.95 kgCO2e/kgH2 candidate lifecycle intensity;
- 176.8 MWth process heat;
- S$0/MWh project electricity revenue;
- ~S$3.725/tCO2e screening abatement cost.

## Findings

### TE-R01 — MINOR — accident-mechanism paragraph needs direct source citations

**File/section:** `paper/sections/05_nuclear_feasibility_safety.tex`,
"Accident mechanisms retained".

**Issue:** The paragraph identifies depressurisation/air ingress, graphite
oxidation, water/steam ingress, loss of heat sink, leakage and fission-product
transport, but unlike the surrounding nuclear-safety subsections it has no
inline primary/authoritative citation.

**Evidence:** These mechanisms are technically credible and consistent with
HTGR safety literature; the issue is manuscript claim provenance, not the
substance of the statements.

**Why it matters:** The targeted extension explicitly set a higher
paragraph-level nuclear-literature standard. Accident/source-term claims should
meet the same citation discipline as TRISO, LOFC and IHX claims.

**Required correction:** Attach an appropriate primary/authoritative HTGR safety
source to the paragraph, preferably a JAEA/IAEA/NRC source that explicitly
covers air ingress/graphite oxidation and fission-product transport.

**Affects scientific conclusion:** No.

---

### TE-R02 — MINOR — NRC PIRT bibliography metadata is inaccurate

**File/section:** `paper/references.bib`, key `nrcngnppirt2010`.

**Issue:** The bibliography gives year 2010 and a paraphrased title. The NRC
publication page identifies NUREG/CR-6944 Volume 6 as published March 2008 and
titles it "Process Heat and Hydrogen Co-Generation PIRTs".

**Evidence:** NRC official publication record.

**Why it matters:** The cited source does support the manuscript's co-location
safety claims, so this is not a support failure. Correct metadata improves
traceability and publication quality.

**Required correction:** Correct year/title to the NRC publication record and
add a stable URL if desired.

**Affects scientific conclusion:** No.

---

### TE-R03 — MINOR — figure-provenance wording overstates use of the Plotters threshold SVG

**File/section:** `results/FIGURE_PROVENANCE.md`.

**Issue:** The register calls the Rust/Plotters CN4252 threshold SVG a
publication plot, while the active manuscript's two threshold visuals in
`07_final_results.tex` are manually authored TikZ graphics. The Rust-generated
thermal-capacity figure and Rust-generated ledgers are genuinely active.

**Evidence:** Active `paper/main.tex` -> `07_final_results.tex`; Rust
`model/src/figures.rs`; search of manuscript inputs.

**Why it matters:** Provenance documentation should distinguish generated assets
that are actually included from related reproducible assets retained for
reference.

**Required correction:** Mark the Plotters SVG as a reproducible reference asset
unless it is actually included in the manuscript, and explicitly identify the
TikZ threshold visuals as manuscript-authored presentations of canonical
numbers.

**Affects scientific conclusion:** No.

---

### TE-R04 — PRESENTATION — some nuclear/safety prose remains dense for Reader 1

**File/section:** Section 5, especially TRISO and coupled-transient paragraphs.

**Issue:** Scientifically strong paragraphs contain several specialised terms
(IPyC, SiC, creep-fatigue, source term, propagation) before the reader receives
a one-sentence plain-language summary.

**Evidence:** Exact 19-page PDF visual/readability inspection.

**Why it matters:** The project explicitly targets high-school conceptual
accessibility plus university-grade rigour.

**Required correction:** Optional final-edit pass: add one short plain-language
sentence or parenthetical definition at the start/end of the densest
subsections. Do not remove technical detail.

**Affects scientific conclusion:** No.

## No BLOCKER or MAJOR findings

No new blocker or major scientific defect was identified in the targeted
extension.

The remaining findings are bounded citation/provenance/presentation corrections.

## Required explicit review answers

1. **Nuclear literature sufficiently rigorous?** YES, with TE-R01/02 minor
   citation cleanup.
2. **Manuscript structure scientifically strong?** YES.
3. **Safety treatment adequate for CN4252 scope?** YES; project/site safety is
   correctly conditional/not demonstrated.
4. **Complete CN4252 feasibility addressed?** YES at screening/assignment level;
   unresolved real deployment dependencies are explicitly identified.
5. **CO2 ledger correct?** YES.
6. **Cost ledger correct?** YES under the verified zero-credit model boundary.
7. **~S$3.725/tCO2e appropriately bounded?** YES; explicitly screening-model,
   not bankable all-in project cost.
8. **Rust figures reproducible/scientifically faithful?** YES; TE-R03 is a
   provenance-label cleanup, not a data defect.
9. **Implementation roadmap adequate?** YES.
10. **Verified quantitative foundation remains valid?** YES.
11. **Manuscript ready for final correction/submission?** YES after the minor
    targeted corrections above; no further broad research expansion is
    scientifically justified by this review.

## Review decision

**TARGETED SCIENTIFIC EXTENSION VERIFIED — MINOR CORRECTIONS ONLY**

The project should proceed to a bounded final correction/submission pass, not
another broad scientific review.
