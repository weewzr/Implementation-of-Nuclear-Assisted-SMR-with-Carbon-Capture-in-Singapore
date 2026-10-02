# W4 Main Research Prompt — Design Basis and Provenance

## Instruction

Continue as Main Research for the existing CN4252 project.

Repository: `weewzr/Implementation-of-Nuclear-Assisted-SMR-with-Carbon-Capture-in-Singapore`

Governing workflow: `docs/PROGRESSIVE_MANUSCRIPT_WORKFLOW.md`

Completed phases:
- W1 — Architecture and narrative spine: COMPLETE
- W2 — Introduction and research question: COMPLETE
- W3 — Proposed system and physical explanation: COMPLETE

Final verified W3 HEAD: `ec13ad86bd9c`

W3 verification:
- Research CI `37007582204` — PASS
- Paper/reproducibility CI `37007582124` — PASS

This instruction authorises **ONLY W4 — DESIGN BASIS AND PROVENANCE**.

Do **not** proceed automatically to W5.

## Boundaries

This is a bounded manuscript-development phase, not a new research programme, model redevelopment, broad literature review, numerical-provenance audit from scratch, independent review, or optimisation exercise.

Do not restart the project; repeat W0–W3; reopen Reviews 1–7; begin Review 08; change verified numerical results without scientific cause; re-optimise assumptions; invent missing process states; relabel project assumptions as literature facts; relabel source-model values as first-principles derivations; or proceed automatically to W5.

The scientific/model state remains frozen unless W4 exposes a genuine scientific discrepancy.

## W4 purpose

W3 established **what the proposed plant physically is**.

W4 must establish:

- What exact numerical design basis defines the calculation?
- Where does each important input come from?
- Why is that value used?
- What is source-backed?
- What is derived?
- What is assumed?
- What is a model output?
- What is inside or outside the system boundary?

W4 bridges:

**Physical system → numerical design basis → provenance → system boundary → model formulation.**

The reader should understand the evidence and accounting basis before the mathematical model is developed in W5.

## 1. Recover current state

Before editing, inspect at minimum:

- `docs/PROGRESSIVE_MANUSCRIPT_WORKFLOW.md`
- `STATUS.md`
- `PROJECT_BRIEF.md`
- `RESEARCH_FRAMING.md`
- `docs/CN4252_PROBLEM_STATEMENT.md`
- `docs/ASSIGNMENT_REQUIREMENTS.md`
- `paper/main.tex`
- W2 Introduction
- W3 Proposed System
- current Design Basis/System Boundary section
- current Model Formulation section
- current Heat Integration section
- current Hydrogen section
- current Carbon/CCS section
- current Techno-economic section
- `paper/sections/B_traceability.tex`
- `paper/REQUIREMENTS.md`
- `paper/references.bib`
- `results/final_design/SOURCE_REGISTER.md`
- `results/final_design/ASSUMPTIONS.md`
- `results/final_design/FINAL_DESIGN_RESULTS.md`
- `results/final_design/SANITY_CHECKS.md`
- `results/final_design/MASTER_NUMBER_PROVENANCE_REGISTER.md` if present
- `results/REACTOR_SELECTION_DERIVATION.md`
- `results/COST_DERIVATION_DEEP_FEASIBILITY.md`
- `results/CO2_DERIVATION_DEEP_FEASIBILITY.md`
- `results/FINAL_ORIGINAL_PLAN_TRACEABILITY.md`
- relevant material/energy-balance records
- `literature/MANUSCRIPT_CLAIM_AUDIT.md`
- `literature/NUCLEAR_FEASIBILITY_EVIDENCE_MATRIX.md`
- `model/src/final_design.rs`
- `model/src/deployment.rs`
- relevant Rust data generators

Use the current repository as authoritative.

A detailed numerical provenance pass has already been completed. Do not recreate another giant register. Use the existing provenance infrastructure to make the **submission-facing manuscript** communicate the design basis clearly.

## 2. Required provenance classification

Every material design value introduced in W4 must be recognisable as one of:

A. SOURCE-BACKED VALUE  
B. SOURCE-MODEL RESULT  
C. SOURCE-DERIVED VALUE  
D. PROJECT-DERIVED VALUE  
E. PROJECT SCREENING ASSUMPTION  
F. CANONICAL PROJECT MODEL OUTPUT

Use existing canonical terminology where already defined.

## 3. W4 reader test

After W4, a technically trained reader should be able to answer:

1. What plant/service is being modelled?
2. What hydrogen production basis is used?
3. What conventional baseline is used?
4. What nuclear-assisted candidate is used?
5. What process operating conditions define the candidate?
6. What reactor design basis is used?
7. Why is that reactor basis compatible with the process requirement?
8. What values came directly from INL?
9. What values came from JAEA/JAERI/Nishihara?
10. Which values are project calculations?
11. Which values are project assumptions?
12. What availability basis is used?
13. What lifecycle boundary is used?
14. What economic boundary is used?
15. What does the model explicitly not include?
16. Which numbers should not be interpreted as physical project outputs?
17. What calculation comes next and why?

If these are unclear before W5, W4 is incomplete.

## 4. Common service / comparison basis

Make the comparison basis explicit.

Clearly establish:
- H2 product = 130 MMSCFD [source + local citation]
- approximately 29,000 lb/h [source + local citation]
- annual project H2 ≈ 97,946 t/y at declared availability [project-derived]

Explain why fixing a common H2 service matters: baseline and candidate must be compared while providing the same hydrogen service, not made favourable merely by producing less hydrogen.

Do not present the annualised project result as though INL directly reported it if it is calculated by this project.

## 5. Conventional baseline design basis

Clearly establish the baseline used for comparison.

At minimum identify and locally cite/define where relevant:
- conventional SMR-H2 service
- baseline natural-gas use = 52.5 MMSCFD
- baseline direct CO2 = 3,205 short ton/day
- common H2 production basis
- relevant process electricity basis if used
- fired reformer heat concept
- capture/no-capture status of the controlling baseline

Explain what the baseline represents physically. Do not mix different literature baselines without explicitly explaining the mapping.

## 6. Candidate chemical-process design basis

Clearly establish the INL candidate process basis.

Audit and present, where active in the model:
- candidate natural gas = 34.0 MMSCFD
- reformer outlet = 871 °C
- reformer pressure = 31.7 bar if active/relevant
- steam/carbon ratio = 3.0 mol/mol
- methane conversion = 78.1% if active/relevant
- PSA H2 recovery = 88%
- H2 product = 130 MMSCFD / approximately 29,000 lb/h
- process heat = 176.8 MWth
- process electricity = 17.3 MWe
- captured CO2 = 1,927 short ton/day
- residual emitted CO2 = 142 short ton/day
- secondary-He supply = 900 °C
- secondary-He return ≈ 466 °C
- secondary-He source flow = 78.49 kg/s
- reactor-outlet source case = 925 °C

For each material value show:

**Value + unit + physical meaning + evidence class + local citation + why used.**

Do not re-derive these in W4 unless a short relationship is necessary to explain the design basis. W5 and later sections contain the calculations.

## 7. Reactor design basis

Clearly establish that the project uses a **GTHTR300C-class / GTHTR300C-informed screening architecture**, not an exact operating GTHTR300C plant.

Relevant source architecture includes:
- 600 MWth reactor thermal rating
- high-temperature helium architecture
- approximately 950 °C JAEA design precedent where relevant
- approximately 900 °C secondary-helium process-heat precedent
- approximately 370/371 MWth high-H2 process-heat branch
- approximately 87–88 MWe complementary source product where relevant to economic allocation
- approximately 170 MWth reference physical IHX

Use established primary JAEA/JAERI/Nishihara citations locally.

Preserve these distinctions explicitly:

- **600 MWth** = source reactor design rating
- **176.8 MWth** = project process-heat requirement from INL source model
- **423.2 MWth** = project-derived remaining reactor thermal capacity
- **370/371 MWth** = source process-heat branch / architecture
- **170 MWth** = reference physical IHX duty
- **87–88 MWe** = source architecture/economic product

Do not imply:
- 423.2 MWth = electricity
- 370/371 MWth = one physical IHX
- 88 MWe = project electricity export
- the 170 MWth physical IHX is already qualified for the 176.8 MWth project duty

## 8. Availability / annualisation basis

If the controlling value remains A = 0.85, identify:
- meaning
- source/provenance
- why it is used
- what it annualises
- what it does not prove about real project reliability

Distinguish **design-study availability basis** from **demonstrated Singapore project availability**.

Do not redo the reliability sensitivity analysis in W4.

## 9. Lifecycle design basis

Define the lifecycle boundary before W7 calculates it.

Identify categories included:
- direct plant CO2
- upstream natural-gas emissions
- nuclear lifecycle proxy
- incremental auxiliary electricity
- CO2 transport/storage lifecycle burden

For material factors used later, identify value, unit, provenance class and source/assumption.

Do not perform the full lifecycle calculation here. Explain what is included, why, and important exclusions/limitations.

## 10. Economic design basis

Define the economic boundary before later calculations.

Identify represented categories:
- baseline NG expenditure
- candidate NG expenditure
- selected reactor/source economic burden
- CCS capital/annualisation
- integration/site screening allowance
- CO2 transport/storage tariff
- project electricity revenue = zero

For material economic assumptions identify value, units, cost year/basis where relevant, classification, justification and source where applicable.

Do not perform the complete cost derivation in W4. The reader should understand what will be counted before seeing the arithmetic.

## 11. Project electricity boundary

This must remain unambiguous.

The project has no validated off-design turbine/internal-load model for the changed heat split.

Therefore:

- NO EXACT PROJECT GROSS MWe IS CLAIMED.
- NO EXACT PROJECT NET MWe IS CLAIMED.
- NO EXACT PROJECT EXPORT MWe IS CLAIMED.
- CONTROLLING PROJECT ELECTRICITY REVENUE = S$0/MWh.

The source 87–88 MWe quantity may be used where appropriate as part of the published source architecture/economic burden. It is not a project electricity-output claim.

## 12. Assumption register in the paper

The manuscript should contain a compact presentation of assumptions that materially control the final result.

Do not dump the entire repository assumptions file into the paper.

Prioritise assumptions affecting:
- annualisation
- lifecycle emissions
- economic result
- CN4252 threshold result

For each important assumption provide:
- parameter
- value
- unit
- classification
- justification/source
- where used

A project assumption must never visually look like an externally established fact.

## 13. Design-basis table

Evaluate current tables.

Prefer one compact principal design-basis/provenance table rather than several duplicative tables.

A useful structure may be:

**ID | Parameter | Value | Unit | Evidence class | Source/justification | Role in model**

Group logically into Process, Reactor/Heat, Lifecycle and Economic if useful.

Keep the table readable. Detailed number-level provenance remains in the repository/appendix.

## 14. Local citations

Maintain the strict citation standard already established.

For externally sourced design values:

**Value → citation beside value/row.**

For project assumptions:

**Value → explicit project-assumption label + justification.**

For project-derived values:

**Cite inputs, not the arithmetic result as though it came from literature.**

Example:
- 600 MWth GTHTR300C design basis \cite{...}
- 176.8 MWth INL process duty \cite{...}
- therefore 423.2 MWth remaining capacity is project-derived.

## 15. Why each value is used

W4 must not become a telephone directory of numbers.

For every principal parameter answer briefly: **Why does this number matter?**

Examples:
- 871 °C → source reformer operating condition
- 900 °C → secondary-He supply condition
- 176.8 MWth → process thermal requirement
- 600 MWth → selected reactor architecture scale
- 130 MMSCFD → common H2 service
- 52.5 vs 34.0 MMSCFD → source NG comparison
- 1,927 short ton/day → captured process CO2
- 142 short ton/day → candidate residual direct CO2
- 85% → annualises design-study rates

The reader should see a design basis, not merely a data table.

## 16. Formal system boundary

Clearly distinguish:

### A. Chemical process boundary
NG/steam → reforming → WGS → capture → PSA → H2.

### B. Nuclear process-heat boundary
reactor → primary helium → IHX → secondary helium → process heat.

### C. Direct-emissions boundary
baseline/candidate source direct CO2.

### D. Lifecycle extension
upstream NG + nuclear proxy + auxiliary electricity + CO2 T&S burden.

### E. Economic boundary
represented baseline/candidate annual costs.

### F. Deployment boundary
actual Singapore site/licensing + detailed nuclear safety + actual CCS route/storage + bankable FOAK economics remain outside demonstrated model closure.

## 17. Boundary diagram/table

If an existing system-boundary visual/table already communicates this well, refine it rather than adding another figure.

Do not add a decorative diagram merely because W4 concerns boundaries.

Any boundary figure should answer:
- What is counted?
- What is not counted?
- What crosses the boundary?

## 18. Do not invent missing stream states

Do not fabricate:
- WGS inlet/outlet temperatures if not adequately traced
- detailed PSA feed/tail-gas state points
- CO2 compressor discharge pressure
- project helium pressure drop
- detailed exchanger wall temperatures
- local reformer pinch
- project electricity output

If unavailable, **NOT AVAILABLE / NOT MODELLED** is preferable to fabricated precision.

## 19. Source transferability

When combining evidence, make the layering explicit:

- **INL** → chemical/process operating basis
- **JAEA/JAERI/Nishihara** → reactor/process-heat architecture and source economics
- **HTTR** → experimental/demonstrated high-temperature evidence
- **PROJECT** → integration and screening calculations

Do not imply these sources describe one already-built plant. This is a constructed screening architecture assembled from compatible evidence layers.

## 20. First-principles honesty

Do not describe the model as wholly first-principles.

The design basis includes:

**Literature inputs + source-model results + design-study values + project assumptions + engineering balances + project derivations.**

Do not invent derivations for hard-coded/source-derived values merely to make the paper appear more fundamental.

## 21. Transitions

W3 asks: **What is the proposed system?**

W4 answers: **What numbers and boundaries define it?**

Avoid re-explaining the entire physical process.

W4 should end by creating the next question:

**Given the defined process states, reactor basis, assumptions and boundaries, how are the governing equations used to test thermal compatibility and calculate the system?**

That is W5. Do not perform W5 automatically.

## 22. W4 acceptance test

W4 passes only if a reader can take every principal design input and answer:

- What is it?
- What is its value?
- What are its units?
- Where does it apply?
- Where did it come from?
- What evidence class is it?
- Why is it used?
- Is it source-backed or assumed?
- What calculation will use it?
- What does it not mean?

The reader must also understand:
- what is inside the model
- what is outside the model
- what is lifecycle
- what is economic
- what is deployment-level and unresolved

## 23. Scientific discrepancy rule

If W4 reveals conflicting source values, stale design parameters, wrong citations, incorrect provenance classification, source/model mismatch, unit inconsistency, manuscript/Rust mismatch or unsupported assumptions, do not hide them.

Classify as:
- PRESENTATION / PROVENANCE ERROR, or
- SUBSTANTIVE SCIENTIFIC DISCREPANCY.

Correct bounded presentation/provenance errors.

If a substantive discrepancy could alter the verified result, **STOP AND REPORT IT**. Do not silently modify the science.

## 24. Verification

After W4:

1. Build the canonical manuscript.
2. Run Research CI.
3. Run Paper/reproducibility CI.
4. Verify zero undefined citations.
5. Verify zero undefined references.
6. Inspect the exact generated PDF.
7. Inspect design-basis/provenance tables.
8. Inspect W3→W4 transition.
9. Inspect W4→W5 transition.
10. Check table wrapping.
11. Check citations beside relevant source values.
12. Confirm canonical scientific numbers remain unchanged.

Do not perform W5.

## 25. Report and stop

At completion report:

PHASE: W4 — Design basis and provenance

STARTING HEAD:

FILES CHANGED:

WHAT WAS IMPROVED:

COMMON H2 SERVICE BASIS:

BASELINE DESIGN BASIS:

CANDIDATE PROCESS DESIGN BASIS:

REACTOR DESIGN BASIS:

AVAILABILITY BASIS:

LIFECYCLE BOUNDARY:

ECONOMIC BOUNDARY:

PROJECT ELECTRICITY BOUNDARY:

PRINCIPAL ASSUMPTIONS:

DESIGN-BASIS TABLE STATUS:

SOURCE / MODEL / ASSUMPTION DISTINCTIONS:

MISSING VALUES DELIBERATELY NOT INVENTED:

SCIENTIFIC DISCREPANCIES FOUND:

SCIENTIFIC MODEL CHANGED?: YES/NO

CANONICAL NUMBERS CHANGED?: YES/NO

CITATION/REFERENCE STATUS:

RESEARCH CI:

PAPER CI:

PDF INSPECTION:

REMAINING ISSUES:

SINGLE RECOMMENDED NEXT PHASE:

COMMIT:

Then **STOP**.

Do not begin W5 automatically.  
Do not begin Review 08.  
Do not reopen broad scientific research.
