# E2B Architecture Innovation and FOAK-to-NOAK Threshold Recovery

## Executive finding

E2B forward-designs architectures first and applies independent cost/maturation evidence second. The S$100/tCO2e threshold is evaluated only after each scenario is constructed.

The main result is:

- the current under-utilized one-train / 600 MWth architecture remains economically adverse;
- a source-supported right-sized single module cannot yet be demonstrated at the required temperature and 176.8 MWth duty;
- **two 176.8 MWth SMR-H2 trains on one 600 MWth GTHTR300C-class heat source are physically compatible with JAEA's published maximum 370 MW process-heat branch** (353.6 MWth required);
- using INL's independently published medium heat-only advanced-reactor costing method, the two-train architecture is approximately **S$137.74/tCO2e at FOAK, S$74.14/tCO2e at the medium BOAK/early-commercial basis, and S$42.84/tCO2e at the 10th-of-a-kind mature point**;
- no electricity revenue, heat-sharing customer, or unsupported co-product credit is used.

The mature S$42.84/t result is a **PROJECTED future deployment case**, not current economics. The early-commercial S$74.14/t result is a **MODELLED source-method case**. Both retain visible unresolved-cost margins.

## 1. Preserved adverse evidence and threshold diagnostic

E1 remains unchanged:
- one 600 MWth MHR-T-class module;
- 97,946 tH2/y;
- 917,138.9 tCO2e/y avoided;
- net incremental represented cost = S$141.278m/y;
- S$154.04/tCO2e;
- FAIL.

At S$100/t, the E1 maximum net annual cost is S$91.714m/y, leaving a gap of **S$49.564m/y**.

Holding all non-nuclear E1 terms fixed:
- maximum nuclear annual burden = **S$167.727m/y**;
- required reduction from E1 nuclear annual burden = **22.81%**;
- break-even nuclear CAPEX at fixed E1 O&M = **S$497.9m**;
- break-even O&M at fixed E1 CAPEX = **S$71.52m/y**.

These are diagnostics only. They were not used to choose the E2B preferred architecture or cost assumptions.

## 2. Track A — architecture innovation

### A1. Right-sizing

A right-sized reactor is attractive because the project needs only 176.8 MWth.

Two primary/authoritative candidates were checked:

1. **HTGR-200** in IAEA TECDOC 2075: 200 MWth and explicitly coupled through an intermediate circuit to steam-methane reforming, but the reported reactor outlet temperature is **850 C**. This does not meet the project's 925 C reactor-outlet source case / 900 C secondary-He supply requirement. It is therefore **not adopted** merely because its economics are favourable.

2. **HTR-Modul process-heat version** in IAEA Nuclear Energy Series P1772: 950 C outlet but **170 MWth** process-heat rating. It satisfies the temperature class but is 6.8 MWth short of the 176.8 MWth project duty. One module therefore does not close the duty.

No single independently evidenced right-sized module currently closes **both** temperature and duty. E2B therefore does not claim a right-sized pass.

### A2. Multi-train utilization of a 600 MWth reactor

JAEA's GTHTR300C design is 600 MWth and can send a **maximum 370 MW** to the secondary hydrogen-production system. The project Case-6 reformer duty is 176.8 MWth per 130 MMSCFD H2 train.

Therefore:
- one train: 176.8 MWth, 29.5% of reactor thermal rating;
- two trains: **353.6 MWth**, 58.9% of reactor thermal rating and 95.6% of the 370 MW source process-heat branch;
- three trains: 530.4 MWth, which **exceeds the 370 MW source branch** and is rejected.

The two-train architecture is therefore selected **before economics** because it is the largest integer number of existing project trains that fits the published source heat branch without inventing a new reactor or exceeding its process-heat allocation.

Forward physical scaling:
- H2 service = 260 MMSCFD;
- annual H2 = **195,892 t/y**;
- lifecycle abatement = **1,834,278 tCO2e/y**;
- NG, capture and T&S quantities scale with two identical process trains;
- one 600 MWth reactor cost is charged;
- two TECDOC-1682 IHX/secondary-loop preliminary source anchors are represented conservatively;
- no electricity/co-product revenue is credited.

The replication of the INL Case-6 chemical train is a project-derived screening architecture, not a demonstrated GTHTR300C-SMR plant. Project-specific shared BOP and integration costs remain unresolved.

### A3. Heat sharing

No specific Singapore industrial customer with a validated compatible heat demand, temperature, availability and contractual cost-bearing role is established in the current evidence.

Therefore E2B applies **zero heat-sharing credit**.

For the E1 one-train case, a real external service would have to bear at least **S$49.564m/y** of represented annual burden merely to reach S$100/t, equivalent to 22.81% of the E1 nuclear annual burden. This is a break-even diagnostic, not an assumed customer or revenue.

### A4. Cogeneration

JAEA's GTHTR300C literature supports genuine heat/electricity cogeneration architectures, including 170 MW and up to 370 MW process-heat modes. However, the current project has no validated off-design turbine/power-cycle output for its SMR duty and no demonstrated Singapore electricity offtake allocation.

Therefore E2B applies:
- electricity output credit = 0;
- electricity revenue = S$0/y;
- no reactor cost is shifted to an inferred electricity product.

Cogeneration remains a future design option, not a credited E2B benefit.

## 3. Track B — FOAK to mature deployment evidence

### 3.1 INL advanced-reactor cost meta-analysis

INL/RPT-23-72972 is the principal maturation source because it explicitly provides a worked **non-electric HTGR plant** example.

The report's medium generic BOAK inputs are:
- OCC = USD6,000/kWe (2019 USD);
- OPEX = USD25/MWh;
- FOAK premium = 1.6;
- learning rate = 10% per cumulative-production doubling;
- multi-unit adjustments are available but are **not** used here because E2B has one reactor module.

For a heat-only HTGR, INL's worked method:
1. removes the 9% energy-conversion-system share;
2. converts electric-normalized cost to thermal using 40% HTGR efficiency;
3. applies the FOAK premium where appropriate;
4. applies the learning equation
   [
   C_N=C_1(1-LR)^{log_2 N};
   ]
5. holds OPEX constant through the learning example.

The report's full heat-only ranges are:
- FOAK OCC about USD2,000–5,500/kWt;
- OPEX USD6–14/MWth-h;
- 10-OAK OCC about USD1,500–4,000/kWt.

E2B uses the **medium** source method rather than choosing a low value:
- BOAK heat-only OCC = 6000(1-0.09)(0.40) = **USD2,184/kWt**;
- heat-only OPEX = 25(0.40) = **USD10/MWth-h**;
- FOAK medium OCC = 2184(1.6) = **USD3,494.4/kWt**;
- 10-OAK medium OCC = 2184(0.9)^(log2 10) = **USD1,539.04/kWt**;
- OPEX remains USD10/MWth-h.

These values are MODELLED/PROJECTED, not observed HTGR construction costs.

### 3.2 DOE advanced-nuclear commercialization cross-check

DOE's Advanced Nuclear Liftoff work projects approximately 40% overnight-capital reduction from FOAK to NOAK through project planning, standardization, shorter construction, modularization and supply-chain development. It indicates that roughly 10–20 reactors may be needed to reach targeted NOAK costs at 12–15% reactor-to-reactor learning rates.

E2B does not substitute those rates into the preferred case because the INL heat-only worked example already provides a directly applicable 10% method. DOE is used as an independent plausibility cross-check on the scale and deployment sequence of maturation.

### 3.3 OECD-NEA qualitative support

OECD-NEA evidence supports the mechanisms—standardized series construction, fixed-program-cost spreading, productivity gains and multi-unit/site effects—but does not provide a project-specific HTGR Singapore cost point. It therefore supports the roadmap mechanism, not a numerical discount.

## 4. Cost normalization

INL's preferred cost inputs are 2019 USD.

The 2019 U.S. GDP implicit-price-deflator quarterly values are averaged:
103.328, 103.862, 104.192 and 104.516 → **103.9745**.

2025 annual GDP deflator = **128.893**.

Screening escalation factor:
128.893 / 103.9745 = 1.23966.

The existing project FX conversion of 1.2776 SGD/USD is then applied.

This is a transparent broad price-level normalization, not a nuclear-construction-specific index.

## 5. Forward-calculated scenario hierarchy

All cases below use:
- one 600 MWth reactor;
- no electricity revenue;
- 85% availability;
- S$15/GJ project NG assumption;
- retained aggregate CCS treatment;
- S$15/t captured T&S;
- TECDOC-1682 source integration anchor per H2 train;
- verified lifecycle abatement scaled only by real H2 train count.

### 5.1 FOAK two-train deployment

Selected architecture: two Case-6 trains because 353.6 MWth fits the 370 MW source heat branch.

Cost stage: INL medium heat-only BOAK cost × 1.6 FOAK premium.

Results:
- annual H2 = 195,892 t/y;
- lifecycle abatement = 1.834 MtCO2e/y;
- nuclear annual burden = S$381.83m/y;
- represented integration = S$22.85m/y;
- baseline represented annual cost = S$538.23m/y;
- candidate represented annual cost = S$790.88m/y;
- net incremental = S$252.65m/y;
- **S$137.74/tCO2e — FAIL cost threshold**;
- remaining margin to S$100/t = **-S$69.22m/y**.

This is the appropriate adverse FOAK result.

### 5.2 Early-commercial / BOAK two-train deployment

Same physical architecture. No learning discount is applied beyond the INL medium BOAK source basis.

Results:
- nuclear annual burden = S$265.18m/y;
- net incremental = S$136.00m/y;
- **S$74.14/tCO2e — PASS**;
- margin to S$100/t = **+S$47.43m/y**.

Evidence maturity: MODELLED source-method case, not an observed commercial HTGR cost.

### 5.3 Preferred mature 10-OAK deployment

The preferred mature configuration is defined **without reference to the S$100/t threshold**:

- reactor: one 600 MWth GTHTR300C-class high-temperature source;
- H2 trains: two identical 130 MMSCFD Case-6 process trains;
- process heat: 353.6 MWth total, below JAEA's 370 MW maximum source branch;
- reactor thermal utilization by reformers: 58.9% of installed thermal rating;
- process-heat branch utilization: 95.6%;
- no heat-sharing customer;
- no electricity/co-product credit;
- deployment maturity: 10th-of-a-kind mature/replicated case;
- capital: INL medium non-electric HTGR method with its 10% learning equation at N=10;
- O&M: INL medium heat-only value, held constant as in the source example;
- integration: two TECDOC-1682 preliminary IHX/secondary-loop anchors;
- NG/CCS/T&S/availability: unchanged project basis;
- unresolved costs: visible and not back-filled.

The architecture is chosen because it closes the source heat balance; the 10-OAK cost point is chosen because INL's worked heat-only example explicitly evaluates learning after 10 deployed units using the medium learning rate. Neither choice depends on the resulting S$/t.

Forward result:
- annual H2 = **195,892 t/y**;
- lifecycle abatement = **1,834,278 tCO2e/y**;
- annual represented baseline = **S$538.230m/y**;
- annual represented candidate = **S$616.814m/y**;
- net incremental annual cost = **S$78.583m/y**;
- nuclear annual burden = **S$207.763m/y**;
- represented integration = **S$22.846m/y**;
- **forward-calculated abatement cost = S$42.84/tCO2e**;
- abatement threshold: PASS;
- cost threshold: PASS;
- remaining represented-cost margin to S$100/t = **S$104.845m/y**.

This S$42.84/t value is the preferred mature/future E2B result for eventual E8 consideration. It is not current economics and is not a Singapore project quotation.

## 6. Why the passing result is architectural, not threshold tuning

A decisive falsification check is the one-train 10-OAK case.

Using exactly the same mature INL cost assumptions but retaining only one 176.8 MWth H2 train gives:
- nuclear annual burden = S$207.763m/y;
- net incremental = S$143.173m/y;
- **S$156.11/tCO2e — FAIL**.

Therefore learning alone does not create the E2B pass.

The pass emerges when a 600 MWth reactor is used in the source-supported high-process-heat architecture with two actual H2 trains. The second train doubles H2 service and abatement while the reactor annual burden is shared across real process output, not an invented electricity credit.

## 7. Combined cases and boundaries

Only one combined improvement is promoted:
**two-train source-supported heat utilization + evidence-backed maturation**.

E2B does not stack:
- optimistic NG price;
- low T&S;
- electricity revenue;
- heat-sharing revenue;
- low-end INL capital cost;
- O&M learning;
- carbon-tax transfers;
- arbitrary Singapore subsidies.

The preferred case uses medium INL cost inputs and unchanged project NG/CCS/T&S assumptions.

## 8. Unresolved costs and pass robustness

Unresolved categories remain:
- project-specific helium-heated reformer modification;
- nuclear/process IHX qualification beyond the preliminary source anchor;
- secondary-He circulator/piping details;
- Singapore site/EPC/owner costs;
- licensing/security/emergency-planning infrastructure;
- waste/spent-fuel/decommissioning/liability;
- detailed capture O&M;
- cross-border CCS contracted tariff;
- financing/schedule/localization effects.

The early-commercial case has only **S$47.43m/y** of represented margin to the S$100/t threshold and is therefore sensitive to unresolved additions.

The preferred 10-OAK case has **S$104.84m/y** of represented margin. This is a useful roadmap tolerance, not permission to assume unresolved costs are zero.

## 9. Comparator discipline: what nuclear adds relative to SMR+CCS

E2 already established that conventional SMR+CCS alone can exceed 0.25 MtCO2e/y and, under one harmonisation, is approximately S$97/tCO2e. Nuclear is therefore not necessary for the minimum abatement threshold.

The E2B two-train architecture adds:
- a second 130 MMSCFD H2 train and corresponding low-carbon output;
- displacement of fired high-temperature reformer heat for both trains;
- a large reduction in nuclear cost per unit of useful H2/abatement through real utilization of the reactor heat branch.

E2B does **not** establish that nuclear is universally preferable to one conventional SMR+CCS train. The mature two-train case is a larger industrial decarbonization architecture, not a proof that nuclear is required for the assignment.

No strategic/energy-security monetary benefit is included.

## 10. Roadmap gates implied by E2B

A future passing case should not be treated as deployable until the following measurable gates are met:

1. **FOAK demonstration gate:** demonstrate high-temperature HTGR process-heat service and project-relevant IHX/secondary-loop performance; FOAK economics may remain >S$100/t.
2. **Architecture gate:** demonstrate that a 600 MWth-class source can reliably provide at least **353.6 MWth** to two SMR-H2 trains while respecting the ~370 MW source branch and required temperatures.
3. **Replication gate:** establish a standardized repeated design and supply chain; verify actual cost learning against the INL/DOE projected range rather than assuming it.
4. **Early-commercial cost gate:** demonstrate heat-only reactor annual burden and integration costs consistent with a total represented abatement cost below S$100/t; E2B's medium BOAK case leaves ~S$47.4m/y unresolved-cost margin.
5. **Mature cost gate:** by approximately 10 replicated units, demonstrate cost performance near or better than the medium INL heat-only 10-OAK basis (~USD1,539/kWt in 2019 dollars, with OPEX near USD10/MWth-h on that source basis).
6. **Availability gate:** demonstrate availability consistent with the 85% annual project basis.
7. **CCS gate:** secure capture and cross-border T&S capacity for two-train scale with a contracted cost/lifecycle burden compatible with the remaining margin.
8. **Singapore deployment gate:** close siting, safety, regulatory, security, cooling, waste, liability and financing costs without consuming the threshold margin.

Failure of any gate invalidates the forward passing case for deployment.

## 11. Evidence maturity

- JAEA 600/370 MWth architecture: **SOURCE DESIGN / MODELLED DESIGN**.
- INL advanced-reactor cost distributions: **MODELLED META-ANALYSIS**.
- INL FOAK/10-OAK heat-only method: **MODELLED / PROJECTED**.
- DOE ~40% FOAK→NOAK and 10–20 reactor pathway: **PROJECTED**.
- OECD-NEA standardization/series mechanisms: **MODELLED/GENERAL EXPERIENCE**, not project cost.
- E2B Singapore forward result: **PROJECT-DERIVED SCREENING RESULT**.

There is no observed commercial NOAK HTGR cost dataset supporting S$42.84/t as an empirical outcome.

## 12. Answers to E2B acceptance questions

1. **Can architecture redesign reach <S$100/t?**  
   Yes. The two-train 600 MWth architecture reaches S$74.14/t on the medium INL BOAK heat-only basis.

2. **Can credible FOAK→NOAK maturation reach it?**  
   Yes for the two-train architecture; the preferred 10-OAK case is S$42.84/t. No for the under-utilized one-train architecture: even the same 10-OAK cost basis gives S$156.11/t.

3. **What exact conditions are required?**  
   Two 176.8 MWth trains (353.6 MWth useful process heat), one 600 MWth source with a 370 MW process-heat branch, no unsupported co-product credit, medium INL heat-only cost basis or better, 85% availability, current project NG/CCS/T&S assumptions, and unresolved additions below the scenario's remaining annual margin.

4. **Evidence-supported or aspirational?**  
   Architecture heat capacity and the costing method are source-supported/modelled. Actual FOAK→BOAK/10-OAK cost realization is projected and must be demonstrated.

5. **At what deployment stage are both thresholds met?**  
   In E2B, the medium BOAK/early-commercial two-train case first passes both. FOAK fails.

6. **Meaningful relative to SMR+CCS alone?**  
   It is meaningful as a larger two-train industrial decarbonization architecture and as a test of nuclear heat utilization, but nuclear remains unnecessary for merely exceeding 0.25 Mt/y because E2 showed CCS alone already does that.

7. **What enters the implementation roadmap?**  
   FOAK demonstration, two-train heat-integration proof, repeated standardized builds, measured cost learning, 85% availability, two-train CCS/T&S capacity, and Singapore-specific unresolved-cost closure.

## 13. Reproducibility

Rust:
- `model/src/e2b_maturation.rs`;
- `model/src/bin/e2b_architecture_maturation.rs`.

Machine-readable:
- `results/e2b_maturation/e2b_scenarios.csv`;
- `results/e2b_maturation/e2b_thresholds.csv`;
- `results/e2b_maturation/e2b_evidence_provenance.csv`.

Tests verify:
- preserved E1 threshold gap;
- two-train closure under the 370 MW branch;
- rejection of three trains;
- exact INL 10-OAK learning equation;
- FOAK failure;
- BOAK/10-OAK two-train passes;
- one-train 10-OAK failure;
- no co-product credit;
- emissions scaling only with real H2 trains.

## 14. E2B closure

E2B identifies a defensible **future pathway** to both CN4252 numerical thresholds, but not a current deployment pass.

The preferred mature result is:

**one 600 MWth high-temperature reactor + two 130 MMSCFD SMR-H2+CCS trains, 353.6 MWth useful reformer heat, 10-OAK medium INL heat-only cost basis → S$42.84/tCO2e and 1.834 MtCO2e/y avoided.**

This is a projected mature deployment target. FOAK remains adverse. The result is falsifiable through the roadmap gates above and must be revisited if actual reactor/integration/site/CCS costs exceed the represented margin.
