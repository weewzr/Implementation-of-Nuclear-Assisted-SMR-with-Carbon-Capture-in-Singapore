# Top-down readability refinement

## Scope
Compilation order read from `paper/main.tex`: abstract; nomenclature; introduction; proposed system; design basis; model formulation; heat integration; hydrogen; carbon/CCS; energy performance; techno-economics; sensitivity; CN4252 assessment; deployment; conclusions; reproducibility and traceability. Active nested inputs are lifecycle formulation, economic formulation, limitations, nuclear safety and Singapore context.

## Before -> after examples
| Active section | Before | Refinement |
|---|---|---|
| Abstract | Dense but correct summary of thresholds and future architecture | Retained because problem/intervention/results/qualifications are already explicit |
| Nomenclature | Immediate symbol table with little orientation | Added **How to read this paper**, maturity categories, MWth vs MWe and CO2 vs CO2e |
| Introduction | Assignment thresholds lead technical motivation | Added methane as feedstock vs fuel explanation and central intervention question |
| Proposed system | Figures introduced before a simple complete process story | Added left-to-right methane/steam -> reformer -> WGS -> capture -> PSA explanation |
| Design basis | Detailed source hierarchy before comparison principle | Added equal-hydrogen-service and boundary-first explanation |
| Model formulation | Source states before whole calculation chain | Added process -> annual flows -> annual economics map and limits |
| Heat integration | Temperature cascade before whole heat condition | Added Q_required <= Q_available and temperature-driving-force warning |
| Hydrogen | Source rate and annualization before overall identity | Added N * mass rate * annual hours / 1000 with definitions |
| Carbon and lifecycle | Detailed component ledger | Added E_baseline - E_candidate and captured-versus-avoided distinction |
| Energy | Already clarified residual heat | Added 600/370/353.6 MWth mental model |
| Economics | Superseded one-train screen dominated first impression | Added exact two-train candidate/baseline equations, complete represented mature cost table and worked AC before historical screen |
| Sensitivity | Legacy figures were already labelled | Added AC numerator/denominator direction-of-effect explanation |
| CN4252 assessment | Detailed results first | Added two-threshold versus deployment-readiness framing |
| Deployment | Gate sequence without immediate simple thesis | Added 'a promising equation is not a construction permit' explanation |
| Conclusion | Technical architecture first | Added plain-language conditional verdict first |
| Reproducibility | Build command before purpose | Added inputs -> model -> ledger -> LaTeX explanation |
| Traceability | Table without reader orientation | Added purpose as evidence-reading map |
| Nested lifecycle | Component equations first | Added lifecycle subtraction overview |
| Nested economics | Source prices and older burden first | Added pointer to controlling parent ledger |
| Nested limitations | Limitations list first | Added model-consistency vs experimental-validation distinction |
| Nested safety | Safety mechanisms in detail | Added initiator -> barriers -> safe-state -> consequence framework |
| Nested Singapore | Infrastructure details first | Added NG/H2/CO2/cooling/logistics overview |
| README | Results table first | Added two-minute seven-question orientation and manuscript reading path |

## Glossary coverage checklist
- [x] SMR = steam methane reforming; distinguish small modular reactor
- [x] HTGR = high-temperature gas-cooled reactor
- [x] CCS/CCUS = carbon capture and storage/utilization
- [x] IHX = intermediate heat exchanger
- [x] WGS = water-gas shift
- [x] PSA = pressure-swing adsorption
- [x] EPZ = emergency planning zone, not assigned a project radius
- [x] FOAK, BOAK and 10-OAK = deployment/maturation stages, not observed plant prices
- [x] CAPEX and OPEX = capital and operating expenditure
- [x] CO2 vs lifecycle CO2e, MWth vs MWe

## Equation and units QA
The earlier `results/FULL_MANUSCRIPT_EQUATION_AUDIT.md` remains the full grouped numerical ledger. New top-down equations were checked dimensionally:
- $M_{H2,y}=N\dot m_{H2}(8760A)/1000$: (kg/h)*(h/y)*(t/1000kg)=t/y.
- $A_y=E_{baseline,y}-E_{candidate,y}$: tCO2e/y.
- $AC=(C_{candidate,y}-C_{baseline,y})/A_y$: S$/tCO2e.
- $Q_{required}\le Q_{available}$: both MWth at suitable temperatures.
- Mature 10-OAK ledger: two-train baseline ~S$538.230m/y; candidate ~S$616.814m/y; difference ~S$78.583m/y using unrounded model; 1.834278 MtCO2e/y avoided; ~S$42.84/t.
- Historical one-train and projected two-train results remain separately labelled.

## Motivated high-school-reader walkthrough
A reader should be able to answer:
1. **Plant:** methane and steam form hydrogen; WGS and PSA refine it; CCS removes process CO2.
2. **Nuclear:** high-temperature reactor heat replaces gas-fired reformer heat, not the methane feedstock.
3. **Carbon:** capture is a physical CO2 stream; avoided emissions are baseline-minus-candidate lifecycle emissions.
4. **Two trains:** 2 x 176.8 = 353.6 MWth fits the ~370 MWth process branch; this is the best-supported architecture, not a global optimum.
5. **Annual cost:** candidate minus baseline annual represented costs divided by avoided tonnes.
6. **Why FOAK fails:** the first reactor cost burden exceeds the S$100/t threshold; projected replication learning reduces annual nuclear cost.
7. **Why Singapore remains conditional:** no approved site, complete nuclear/chemical safety demonstration, site-specific EPZ or contracted CCS chain.

## Remaining pedagogical and scientific limits
Detailed chemical stream tables, thermodynamic state conventions, INL/IEAGHG empirical assumptions, lifecycle factors and the long historical one-train economic bridge remain technically demanding. They are preserved for auditability rather than hidden. A student should use the new overview equations first and return to the detailed substitutions afterward. The numerical maturity cases are not vendor estimates; complete project-cost and safety closure remain future work.

## Verification status
The final Rust tests, LaTeX citation/reference build, and full-page PDF inspection must be checked at the final commit. A green earlier build does not verify the present changes.
