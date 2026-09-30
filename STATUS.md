# STATUS

## DEEP FEASIBILITY / PROFESSOR-FEEDBACK PHASE — IN PROGRESS

The previously frozen state **FINAL SUBMISSION CANDIDATE — CLOSED / FROZEN** is preserved as a historical milestone at commit `2605b773a57edac8e219176d1bd172707354c524`.

The project is deliberately reopened under the user's superseding deep-feasibility instruction because genuinely new scientific requirements were identified: professor slide/transcript traceability, nuclear safety methodology, EPZ/siting, process-heat reliability, source term, V&V/uncertainty, deeper primary literature, and additional reproducible Rust analysis/figures.

This reopening does not invalidate the previously verified quantitative foundation unless new evidence or justified sensitivity work exposes a specific contradiction.

## Preserved quantitative foundation entering this phase
- H2 production: ~97,946 t/y.
- Direct avoided CO2: ~862,094 t/y.
- Lifecycle avoided: ~917,139 tCO2e/y.
- Candidate lifecycle intensity: ~1.95 kgCO2e/kgH2.
- Process heat: 176.8 MWth.
- Selected architecture: 600 MWth GTHTR300C-class reactor; 370 MWth source heat/IHX branch.
- Remaining reactor thermal capacity: 423.2 MWth; no exact project electricity output claimed.
- Previous controlling project electricity revenue: S$0/MWh.
- Previous controlling screening abatement cost: ~S$3.725/tCO2e.
- Previous CN4252 numerical result: CONDITIONAL MODEL PASS.

These are hypotheses/results to stress-test, not values to preserve by optimisation.

## New mandatory research gates
1. Recover and pair professor/research slides with transcripts.
2. Complete professor slide and feedback traceability.
3. Build deep-feasibility question register.
4. Follow primary-literature citation chains to saturation for major nuclear/process questions.
5. Separate verification, validation, uncertainty and extrapolation for each submodel.
6. Address process-heat reliability/availability and justified backup strategies.
7. Deepen accident, source-term, EPZ, nuclear/chemical interface, cooling, waste, security, human-factors and Singapore regulatory/siting analysis.
8. Extend Rust only where evidence supports quantitative work.
9. Revise manuscript and Future Work only after evidence matures.
10. Pass Rust/Research/Paper CI and exact-PDF QA before the next Independent Review.

## Project-source recovery status
Available Project Sources include the transcript `Nuclear Energy Assessment and SMR Feasibility for Singapore.txt` and 15 photographed presentation slides. These are being analysed as slide + spoken explanation pairs.

The requested second transcript, `Nuclear Reactor Safety Research and Simulation Overview.txt`, is **not currently present in the Project file inventory under that title**. Main Research will not fabricate its contents; this is a source-recovery blocker for complete professor-feedback closure unless the source becomes available.

## Governing distinction
**PROFESSOR OBSERVATION != SCIENTIFIC EVIDENCE.**

Professor observations generate engineering questions. Primary literature, authoritative technical evidence, verified modelling and explicit uncertainty determine project conclusions.

**REPOSITORY = COMPLETE SCIENTIFIC AUDIT TRAIL.**

## Current critical path
Professor slide/transcript traceability for the available presentation is complete; the second requested transcript remains a source-recovery blocker.

Completed deep-feasibility work now includes:
- HTGR process-heat demonstrated→designed→modelled scale-up evidence;
- verified Rust availability sensitivity and evidence-bounded gas-backup sensitivity implementation;
- industrial process-heat reliability/backup literature;
- mechanistic source-term / EPZ methodology and radionuclide-specific TRISO evidence;
- process–nuclear hazard register;
- HTGR spent-fuel / graphite-waste evidence;
- heat-rejection accounting boundary and Singapore siting criteria;
- Singapore nuclear regulatory/infrastructure readiness matrix.

Key unresolved critical-path questions:
1. verify the latest combined reliability/CCS Rust sensitivities and record their exact generated margins from CI;
2. deepen selected-design accident/source-term evidence only where primary citation chains still change conclusions;
3. recover the missing second professor/research transcript and slides;
4. reconcile professor traceability/question register after that source recovery;
5. integrate the now-mature process-heat, reliability, CCS, source-term/EPZ, waste, cooling/siting, regulatory, V&V and accident-register evidence into the manuscript;
6. generate evidence-backed Rust figures for the most decision-relevant sensitivities;
7. run full CI/PDF QA before the next Independent Review.

Newly completed since the previous status update:
- CCS partial-capture/storage screening sensitivity implemented in Rust with base reconciliation and monotonic tests;
- CCS availability/infrastructure evidence review;
- deep-feasibility V&V/uncertainty/extrapolation matrix;
- selected-design accident/transient sequence register;
- secondary-helium hydraulic compatibility cross-check (INL 78.49 kg/s vs GTHTR300C ~81 kg/s at 900 C; project ΔP still not claimed).

Do not begin the next Independent Review until all mandatory deep-feasibility gates are complete.

Do not begin the next Independent Review until all mandatory deep-feasibility gates are complete.
