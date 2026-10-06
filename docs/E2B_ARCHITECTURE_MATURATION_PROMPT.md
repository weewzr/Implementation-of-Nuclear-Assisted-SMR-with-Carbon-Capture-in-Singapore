# E2B — Architecture Innovation and FOAK-to-NOAK Threshold Recovery

Continue as Main Research. Read:
- docs/POST_SCREENING_ENGINEERING_CLOSURE_WORKFLOW.md
- results/E1_ENGINEERING_ECONOMIC_MODEL.md
- results/E2_INCREMENTAL_NUCLEAR_BENEFIT.md
- docs/CN4252_BROAD_ASSIGNMENT_REQUIREMENTS.md
- docs/CN4252_PROBLEM_STATEMENT.md

E2 is closed at 1a62ffbe4070 with Research CI 37435613524 PASS and Paper CI 37435613520 PASS.

Execute E2B ONLY. Stop before E3/W5/E8/review.

## Evidence to preserve
E1 central dedicated nuclear case: ~S$141.278m/y net incremental cost, ~S$154.0/tCO2e, FAIL. At 917,138.90 tCO2e/y avoided, S$100/t permits ~S$91.714m/y, so ~S$49.564m/y annual burden must be removed before unresolved additions.

E2: matched SMR+CCS alone gives ~0.461 MtCO2e/y and already passes the abatement threshold; under its harmonisation it is ~S$97/tCO2e. Nuclear is therefore not required for the abatement threshold and cannot be credited with all CCS benefit.

## Objective — FORWARD DESIGN FIRST

The primary E2B objective is to construct one or more **physically coherent future deployment configurations from independent engineering and economic evidence**, calculate their performance forward, and only then compare the resulting abatement cost with the CN4252 S$100/tCO2e criterion.

The required logic is:

**evidence-supported future architecture
→ reactor/process sizing
→ independently sourced deployment-stage CAPEX/O&M
→ physical utilization/allocation
→ integration + NG + CCS + T&S costs
→ annual represented cost
→ lifecycle abatement
→ forward-calculated S$/tCO2e
→ only then compare with S$100/tCO2e.**

Do NOT select CAPEX, O&M, utilization, learning rate, allocation, NG price, CCS price or any other assumption because it makes the answer fall below S$100/tCO2e.

The existing back-calculated S$100/t threshold is a **secondary diagnostic and plausibility check only**. It must not be the design method.

Find whether a physically credible future nuclear-assisted architecture and/or evidence-supported FOAK→NOAK maturation can satisfy BOTH CN4252 thresholds without tuning assumptions backwards.

No invented revenue, customers, electricity output, equipment data or arbitrary learning rates. Adverse results must remain.

## Track A — Architecture innovation
Test source-supported options:
1. reactor/module right-sizing closer to 176.8 MWth while retaining required temperature and process interface;
2. multiple H2 trains / higher utilization of a 600 MWth-class reactor, with heat balance, H2, NG, CCS and abatement scaling;
3. real industrial heat sharing only where a compatible service exists; if no exact customer/demand exists calculate the required break-even co-product demand/value instead;
4. cogeneration only with a source-backed heat/electricity architecture or validated conversion model—no inference from 423.2 MWth;
5. alternative HTGR module arrangements already supported by literature.

For every architecture report installed thermal capacity, useful heat, utilization, H2 service, abatement, represented annual cost, S$/tCO2e, unresolved costs and evidence maturity.

Do not allocate reactor cost away from H2 unless another real service/user bears that cost.

## Track B — FOAK→NOAK maturation
Conduct targeted primary/authoritative research on FOAK/NOAK advanced-reactor/HTGR economics, standardisation, modular construction, replication/fleet effects, construction duration, O&M maturation and learning.

Prioritise IAEA, OECD-NEA, DOE/INL/ORNL, JAEA and peer-reviewed nuclear economics. Vendor figures require explicit classification.

Classify evidence as OBSERVED / MODELLED / PROJECTED / TARGET / VENDOR CLAIM.

Calculate:
- maximum annual net cost at S$100/t;
- required reduction from E1;
- required % reduction in nuclear annual burden;
- break-even annual nuclear burden;
- break-even nuclear CAPEX at stated O&M;
- break-even O&M at stated CAPEX;
- useful CAPEX/O&M threshold combinations.

If using learning curves, state equation, learning rate/progress ratio, technology applicability and number of doublings required. Do not import unrelated learning rates without justification.

Construct evidence-backed FOAK / early-commercial / NOAK cases only where source support exists. For each show CAPEX, O&M, integration, allocation/utilization, unresolved allowance, net annual cost, S$/tCO2e, PASS/FAIL and evidence maturity.

## Track B5 — Forward-calculated preferred deployment point

After collecting the FOAK/NOAK evidence, define at least one **preferred mature/future deployment configuration independently of the S$100/t threshold**.

For the preferred case specify before calculating its abatement cost:
- reactor technology/module arrangement;
- number of modules;
- H2 train count/capacity;
- process-heat allocation;
- reactor utilization;
- any genuine co-product/service and its physical basis;
- deployment maturity (FOAK/early-commercial/NOAK);
- source-backed CAPEX;
- source-backed O&M;
- integration/IHX treatment;
- NG price/basis;
- CCS cost treatment;
- T&S treatment;
- availability;
- unresolved-cost treatment;
- cost year/currency.

Explain why each value is selected from engineering/source evidence **without reference to whether it will pass S$100/t**.

Then run the deterministic model forward and report:
- annual H2;
- lifecycle emissions/abatement;
- annual represented baseline cost;
- annual represented candidate cost;
- net incremental annual cost;
- **forward-calculated S$/tCO2e**;
- CN4252 PASS/FAIL.

The preferred future value must be the value later intended for the final report if the scenario is sufficiently defensible.

Do not report a deliberately selected value such as S$99/t merely because it passes.

If the independently defined preferred future case is below S$100/t, preserve the resulting calculated value exactly with appropriate precision.

If it remains above S$100/t, retain the failure and investigate other independently motivated architectures rather than tuning this case.

## Track C — Combined cases
Only combine individually defensible improvements. Test a small number of coherent scenarios such as right-sized+NOAK, high-utilization multi-train+NOAK, or real shared-heat+NOAK.

Do not stack every optimistic assumption.

## Forward-result hierarchy

E2B should ideally produce three independently defined deployment-stage results:

1. **FOAK/current-source case** — forward-calculated;
2. **early-commercial/replicated case** — forward-calculated if evidence supports it;
3. **preferred mature/NOAK case** — forward-calculated.

The final report should eventually present a specific future S$/tCO2e value only if it comes from this forward chain.

The back-calculated break-even CAPEX/O&M/annual burden remains useful for:
- checking how far a failing case is from the threshold;
- testing plausibility of a passing case;
- defining roadmap gates.

It is not the headline future cost estimate.

## Threshold and roadmap logic
Every scenario must show:
- annual H2;
- lifecycle abatement;
- >0.25 MtCO2e/y PASS/FAIL;
- net incremental annual cost;
- S$/tCO2e;
- <S$100/t PASS/FAIL;
- unresolved annual costs;
- remaining margin to S$100/t;
- deployment stage/evidence maturity.

A future NOAK PASS is acceptable as an implementation-roadmap target but must not be described as current economics.

If a credible passing case exists, convert it into measurable roadmap gates:
FOAK demonstration → validate cost/construction/availability → replication/standardisation → achieve annual nuclear burden threshold X → achieve utilization Y → secure CCS capacity → Singapore deployment decision.

## Comparator discipline
Because SMR+CCS alone already performs strongly, every nuclear passing scenario must explain what nuclear adds relative to SMR+CCS: additional abatement, reduced fired heat/NG dependence, real co-product service or another evidence-backed benefit. Do not claim strategic benefits without evidence.

## Reproducibility
Implement project-derived scenario and threshold calculations in Rust. Create machine-readable scenario/provenance, break-even CAPEX/O&M and utilization outputs. Add tests for threshold identities and unsupported-credit prevention.

Create results/E2B_ARCHITECTURE_MATURATION.md.

Do not rewrite the active manuscript yet; E8 will integrate the result.

## Acceptance

E2B does NOT pass merely because a break-even calculation shows what assumptions would be needed to reach S$100/t.

At least one preferred future configuration must be specified independently and forward-calculated. If no evidence-supported future configuration passes, state that honestly.

For any claimed passing case, verify:
- its assumptions were selected before observing the threshold result;
- source evidence supports the deployment-stage costs;
- physical utilization/allocation closes;
- no unsupported revenue or cost shifting is used;
- unresolved costs are visible;
- the result remains below S$100/t after the full represented ledger for that scenario.

Answer:
1. Can architecture redesign reach <S$100/t?
2. Can credible FOAK→NOAK maturation reach it?
3. What exact cost/utilization conditions are required?
4. Are they evidence-supported or aspirational?
5. At what deployment stage, if any, are both thresholds met?
6. Is that case meaningful relative to SMR+CCS alone?
7. What conditions enter the implementation roadmap?

If none passes defensibly, say so.

Run Rust tests and both CI workflows as applicable. Preserve E1/E2 and historical adverse cases.

## Report and STOP
Report:
PHASE: E2B
STARTING HEAD:
E1 CENTRAL GAP TO S$100/t:
ARCHITECTURES TESTED:
RIGHT-SIZING:
MULTI-TRAIN/UTILIZATION:
HEAT-SHARING:
COGENERATION:
FOAK/NOAK SOURCES:
REQUIRED COST REDUCTION:
BREAK-EVEN CAPEX:
BREAK-EVEN O&M:
FOAK FORWARD-CALCULATED RESULT:
EARLY-COMMERCIAL FORWARD-CALCULATED RESULT:
PREFERRED NOAK/MATURE CONFIGURATION:
PREFERRED NOAK/MATURE FORWARD-CALCULATED S$/tCO2e:
WAS THE PREFERRED CASE DEFINED INDEPENDENTLY OF THE S$100/t THRESHOLD?: YES/NO
NOAK RESULT:
COMBINED SCENARIOS:
ANY CASE PASSES BOTH THRESHOLDS?:
STRONGEST DEFENSIBLE PASSING CASE:
EVIDENCE MATURITY:
NUCLEAR JUSTIFICATION VS SMR+CCS:
ROADMAP CONDITIONS:
RUST FILES/TESTS:
DURABLE OUTPUTS:
RESEARCH CI:
PAPER CI:
COMMIT:
RECOMMENDED NEXT ACTION:

Then STOP. Do not begin E3, W5, E8 or review.
