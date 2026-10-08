# Full active-manuscript editorial audit

## Compilation scope
`paper/main.tex` includes 17 top-level section files in order: abstract, nomenclature, introduction, proposed system, design basis, model formulation, heat integration, hydrogen, carbon/CCS, energy, economics, sensitivity, CN4252 assessment, deployment, conclusions, reproducibility and traceability. Nested active sources: `07_lifecycle_formulation`, `09_economic_formulation`, `11_limitations`, `05_nuclear_feasibility_safety` and `10_singapore`. Other numbered legacy sections in the repository are dormant and should not be mistaken for the compiled narrative.

| Section | Narrative assessment | Actual edit / retained boundary |
|---|---|---|
| 00 Abstract | Already gives the controlling two-train architecture, FOAK failure and conditional maturity; concise but information-dense | Retained to avoid unneeded repetition |
| 00 Nomenclature | Distinguishes steam methane reforming from small modular reactors; supports model units | Retained; mathematical audit cross-checks symbol meanings |
| 01 Introduction | Connects Singapore problem, chemistry, nuclear intervention and falsifiable threshold question | Retained source-backed thesis; no ungrounded superiority language |
| 02 Proposed system | Three figures explain chemical process, reactor reference and conceptual integration | Retained original user artwork; current Figure 3 is conceptual/one-train orientation, not proof of two-train equipment |
| 03 Design basis | Makes per-train source vs project two-train derivation explicit and separates source branch from physical IHX | Retained E8 algebra and evidence hierarchy |
| 04 Model formulation | Correctly admits literature-anchored screening rather than a fully solved kinetic simulation | Retained methodological honesty |
| 05 Heat integration | **One-train 29.5%/423.2 MWth equations could read as current final design** | **Edited** to identify them explicitly as historical Case-6 benchmark and point to preferred 353.6/246.4 MWth boundary |
| 06 Hydrogen | Source annualisation clearly traced to lb/h and 85% | Retained; two-train doubling located in design basis |
| 07 Carbon/CCS | Detailed one-train source ledger and lifecycle bridge; risk of confusing historical scale with preferred architecture | Preserved original generated Rust ledger and explicit per-train basis; two-train synthesis in Section 11 |
| 08 Energy performance | Previously presented 423.2 MWth one-train residual as final | **Corrected in preceding commit** to preferred two-train 353.6 MWth heat and conditional 246.4 MWth residual |
| 09 Economics | Historical S$3.725/t differential preceded controlling stage-specific results; rounded mature operands produced a misleading exact equality | **Edited** historical heading/warning and approximate rounded-ledger equality; FOAK/BOAK/10-OAK table remains controlling |
| 10 Sensitivity | Historical one-train backup/CCS plots could be read as preferred two-train predictions | **Edited in preceding commit** with explicit historical one-train scope and no cross-architecture extrapolation |
| 11 CN4252 assessment | Clearly distinguishes E2 comparator from proposed nuclear solution and separates current from future results | Retained; numerical consistency checked |
| 12 Deployment | E7 gate sequence has falsification branches; avoids invented dates | **Edited after PDF inspection:** long displayed roadmap overflowed right margin; replaced with nine-item numbered list |
| 13 Conclusions | Reports both adverse FOAK economics and conditional future maturity | Retained; no deployment or site approval asserted |
| A Reproducibility | Points to Rust, canonical LaTeX and generated ledgers | Retained; CI must be verified on final HEAD |
| B Traceability | Maps assignment criteria to paper sections and numbers | Retained; historical data not silently erased |
| 07_lifecycle_formulation | Defines carbon-boundary method without re-solving source flowsheet | Retained |
| 09_economic_formulation | Gives cost/abatement ratio and model boundaries | Retained |
| 11_limitations | Lists missing site, source-term, costing and dynamic validation | Retained |
| 05_nuclear_feasibility_safety | Explicit evidence maturity, tritium and propagation; no numerical EPZ | Retained |
| 10_singapore | Jurong candidate context, normal/safety cooling distinction and CCS infrastructure | Retained |

## Scientific storytelling
The manuscript's defensible arc is: established methane-reforming chemistry -> fired-heat emissions -> high-temperature nuclear intervention -> literature-supported per-train process state -> project-derived two-train architecture -> explicit balances -> FOAK economic failure and conditional mature recovery -> siting/safety/commercial gates. Strong rhetoric is constrained by the evidence hierarchy: **a numerical projected pass is not deployment feasibility**.

## Corrections and remaining issues
The material current-facing corrections were README replacement, two-train energy section, historical sensitivity labelling, historical economic framing, rounded-ledger equality and the heat-integration benchmark warning. No numerical model assumptions were changed.

Open scientific uncertainties are commercial-scale IHX/loop qualification, detailed dynamic transients, QRA/PRA/source term, site/EPZ, contracted storage/offtake and observed mature cost learning. These are not editorial defects that can be solved by rewriting. Final PDF page-by-page visual inspection and both CI results must be recorded separately; do not infer layout quality from source review alone.

## PDF QA finding and repair
The first final-HEAD manuscript artifact compiled successfully but visual inspection of all 43 rendered pages identified a **severe right-margin overflow on PDF page 29**: the displayed E7 roadmap arrows were wider than the text block. Replaced that display with a nine-item numbered gate list in `12_deployment_constraints.tex`; the next PDF build must be visually rechecked. The original 43-page PDF is not accepted as final layout QA.
