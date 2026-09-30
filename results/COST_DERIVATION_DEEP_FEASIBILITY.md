# Canonical Cost Derivation — Engineering Reproduction Record

## Scope

This record expands the canonical zero-electricity-credit screening economics into equation → units → numerical substitution → result → provenance.

**Interpretation:** this is an abatement-cost screening model, not a turnkey Singapore nuclear project estimate.

## Common quantities

Availability:
A = 0.85.

Operating hours:
h = 8760 × 0.85 = **7446 h/y**.

Natural-gas price:
P_NG = **S$15/GJ**.

T&S screening tariff:
P_TS = **S$15/tCO2**.

Captured CO2:
M_capture = **542,361.98 t/y**.

Lifecycle avoided:
A_LC = **917,138.90 tCO2e/y**.

Project electricity revenue:
**S$0/y** by controlling boundary.

## 1. Baseline natural-gas expenditure

Baseline gas:
52.5 MMSCFD.

Annual gas cost:
C_base,NG = Q_NG,base × 365 × A × S$15/GJ.

Result:
**S$269,115,246/y**.

## 2. Candidate natural-gas expenditure

Candidate gas:
34.0 MMSCFD.

Result:
**S$174,284,159/y**.

## 3. Natural-gas expenditure saved

S_NG =
269,115,246
- 174,284,159

= **S$94,831,087/y SAVED**.

## 4. Selected reactor/source economic burden

The controlling model uses the coherent Nishihara GTHTR300C source-product economics and charges the **full published heat + electricity product burden**, while crediting no project electricity revenue.

### Price conversion

2007→2025 Japan GDP-deflator escalation:
112.27 / 99.59
= **1.12732**.

JPY→SGD:
0.008117 SGD/JPY (declared project conversion date/basis).

Source heat price:
0.52 JPY/MJ.

Converted:
0.52 × 1.12732 × 0.008117 × 1000 MJ/GJ
= **S$4.75825/GJ**.

Source electricity price:
4.9 JPY/kWh.

Converted:
4.9 × 1.12732 × 0.008117 × 1000 kWh/MWh
= **S$44.8373/MWh**.

### Heat-product burden

Source heat branch:
370 MWth.

Annual thermal energy:
370 MW × 7446 h × 3.6 GJ/MWh
= **9,918,? GJ/y** (Rust uses full precision).

Cost:
370 × 7446 × 3.6 × 4.758245876

= **S$47,192,625/y**.

### Source electricity-product burden

Source gross electricity product:
88 MWe.

Cost:
88 × 7446 × 44.837316909

= **S$29,379,562/y**.

### Total selected source burden

C_reactor =
47,192,625
+ 29,379,562

= **S$76,572,187/y ADDED**.

Important:
88 MWe is a **source economic product used to recover the full published source burden**, not a claimed project gross/net/export output.

Project electricity revenue remains:
**S$0/y**.

## 5. CCS capital

Reference IEAGHG CCS total-capital requirement:
€41.02 million (2014 basis).

Reference captured amount used by canonical scaling:
M_ref =
0.4660 × 100,000 × 8322 / 1000
= **387,805.2 t/y**.

Germany deflator proxy:
123.84 / 90.46
= **1.36834**.

EUR→SGD:
1.452 SGD/EUR.

Linear captured-tonne scaling:

CAPEX_CCS =
€41.02m
× 1.36834
× 1.452
× (542,361.98 / 387,805.2)

= **S$114,036,071**.

Boundary:
linear scale is a screening assumption, not a vendor quote.

## 6. Annualised CCS capital

Capital recovery factor:

CRF(i,n) =
i(1+i)^n / [(1+i)^n - 1].

For i=8%, n=25 y:
CRF(0.08,25) ≈ 0.09368.

Annual CCS:
114,036,071 × CRF(0.08,25)

= **S$10,682,760/y ADDED**.

## 7. Integration / site allowance represented in model

JAEA plant capital after escalation/conversion:
59.7 billion JPY
× 1.12732
× 0.008117

= **S$546,283,228**.

Screening integration capital basis:
10% × (plant CAPEX + CCS CAPEX).

Annualisation:
3% over 40 y.

C_integration =
0.10 × (546,283,228 + 114,036,071)
× CRF(0.03,40)

= **S$2,856,698/y ADDED**.

This is a screening allowance, not a detailed Singapore site/EPC estimate.

## 8. CO2 transport and storage

C_TS =
542,361.98 t/y
× S$15/t

= **S$8,135,430/y ADDED**.

The S$15/t value is a screening assumption, not a contracted Singapore cross-border CCS tariff.

## 9. Project electricity revenue

No validated off-design project power-cycle model exists for the 176.8 MWth process draw.

Therefore:

R_electricity =
0 MWh/y claimed export × any tariff
= **S$0/y**.

This is deliberately conservative and prevents unsupported electricity-credit economics.

## 10. Net annual incremental cost

Candidate added/retained costs relative to baseline:

C_net =
C_candidate,NG
+ C_reactor
+ C_CCS,annual
+ C_integration
+ C_TS
- R_electricity
- C_baseline,NG.

Substitution:

174,284,159
+ 76,572,187
+ 10,682,760
+ 2,856,698
+ 8,135,430
- 0
- 269,115,246

= **S$3,415,988/y** (minor display differences from rounded component values).

Canonical manuscript shorthand:
**~S$3.416 million/y**.

## 11. Abatement cost

C_abatement =
C_net / A_LC

= S$3,415,988/y
/ 917,138.90 tCO2e/y

= **S$3.7246/tCO2e**.

Rounded:
**S$3.725/tCO2e**.

CN4252 cost ceiling:
S$100/tCO2e.

Screening margin:
~S$96.28/tCO2e below the ceiling.

## Simple cost table

| Item | Baseline | Candidate | SAVED ↓ / ADDED ↑ | Annual change (S$/y) | Why |
|---|---:|---:|---|---:|---|
| Natural gas | 269,115,246 | 174,284,159 | SAVED ↓ | +94,831,087 | nuclear heat reduces NG from 52.5 to 34 MMSCFD |
| Selected reactor/source burden | 0 | 76,572,187 | ADDED ↑ | -76,572,187 | full source heat + electricity economic burden |
| CCS annualised capital | 0 | 10,682,760 | ADDED ↑ | -10,682,760 | scaled IEAGHG capture CAPEX × CRF |
| Integration/site allowance | 0 | 2,856,698 | ADDED ↑ | -2,856,698 | 10% screening allowance annualised |
| CO2 transport/storage | 0 | 8,135,430 | ADDED ↑ | -8,135,430 | captured tonnes × S$15/t |
| Project electricity revenue | 0 | 0 | NO CREDIT → | 0 | no validated project export model |
| **TOTAL** | — | — | **NET COST ADDED** | **~3,415,988** | new costs minus gas savings |

### Totals

**TOTAL MONEY SAVED**
= S$94.831 million/y.

**TOTAL NEW COSTS ADDED**
= 76.572 + 10.683 + 2.857 + 8.135
= **~S$98.247 million/y**.

**NET ANNUAL COST CHANGE**
= ~98.247 - 94.831
= **~S$3.416 million/y ADDED**.

**LIFECYCLE CO2e SAVED**
= **917,138.90 t/y**.

**ABATEMENT COST**
= **~S$3.725/tCO2e**.

## Represented versus omitted costs

### Represented
- baseline/candidate natural gas;
- full selected source reactor heat/electric product burden;
- CCS scaled capital annualisation;
- screening integration/site allowance;
- CO2 T&S screening tariff;
- zero project electricity revenue.

### Not separately represented / incomplete
No values are invented for:
- Singapore FOAK premium;
- project financing structure beyond screening CRFs;
- Singapore-specific nuclear construction/site premium;
- detailed licensing/regulatory programme;
- detailed EPC and owner's costs;
- nuclear security;
- emergency-planning infrastructure;
- detailed 176.8 MWth IHX qualification;
- secondary-helium piping/circulator detailed CAPEX/OPEX;
- detailed process-trip/dump-heat system;
- backup-heater CAPEX/fixed O&M/start-up/integration;
- cooling-water/intake/outfall system;
- detailed CCS compression/turndown beyond source/model boundary;
- negotiated cross-border CCS infrastructure/contracts;
- contingency;
- schedule delay;
- insurance/nuclear liability;
- spent-fuel/interim-storage/final-disposal system;
- decommissioning;
- public/stakeholder programme.

Therefore **S$3.725/tCO2e is not the turnkey cost of deploying nuclear-assisted hydrogen in Singapore**.

## Provenance chain

Nishihara/JAEA + IEAGHG + declared screening assumptions
→ `model/src/deployment.rs`
→ `final_cost_ledger()`
→ reconciliation test against `final_design()`
→ generated cost ledger
→ manuscript table.

The Rust implementation remains canonical.
