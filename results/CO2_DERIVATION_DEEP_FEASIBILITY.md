# Canonical CO2 Derivation — Engineering Reproduction Record

## Scope

This record expands the canonical Rust lifecycle calculation into equation → units → numerical substitution → result → provenance.

**Important boundary:** the INL source provides baseline/candidate natural-gas rates and direct/captured CO2 stream rates from its process simulation. The repository does **not** contain enough source natural-gas composition detail to reconstruct the source simulator's direct CO2 stream exactly from a pure-methane feed-carbon assumption. Therefore direct stack/process CO2 is converted from the source-reported short-ton/day values rather than reverse-engineered from MMSCFD. This avoids false carbon-balance precision.

## Constants

Availability:
A = 0.85.

Short ton to metric tonne:
1 short ton = 0.90718474 t.

Annual operating days:
365 A = 310.25 d/y.

Annual operating hours:
8760 A = 7446 h/y.

Natural-gas HHV:
1044 Btu/scf.

Upstream NG lifecycle factor:
11.5 kgCO2e/GJ.

Nuclear electricity lifecycle factor:
5.5 kgCO2e/MWh.

Process-heat allocation factor:
0.504 MWh_e-equivalent/MWh_th (screening proxy already used by canonical model).

T&S lifecycle factor:
0.025 tCO2e/tCO2 transported/stored.

## 1. Baseline natural-gas / process-carbon basis

INL baseline natural gas:
52.5 MMSCFD.

Candidate natural gas:
34.0 MMSCFD.

Natural-gas displacement:
18.5 MMSCFD.

The direct CO2 stream is not calculated from 18.5 MMSCFD alone because the source model distinguishes feed/fuel/process streams and supplies its own direct CO2 outputs.

## 2. Baseline direct CO2

Source:
baseline emitted CO2 = 3205 short ton/day.

Equation:
E_base,direct = 3205 short ton/d × 0.90718474 t/short ton × 365 d/y × 0.85.

Substitution:
3205 × 0.90718474 × 365 × 0.85

Result:
**902,060.28 tCO2/y**.

Provenance:
INL source process result → `BASE_EMITTED_SHORT_T_D` → Rust conversion.

## 3. Candidate captured process CO2

Source:
captured CO2 = 1927 short ton/day.

Equation:
M_capture = 1927 × 0.90718474 × 365 × 0.85.

Result:
**542,361.98 tCO2/y captured**.

This captured amount is not itself “avoided lifecycle CO2”; transport/storage burdens are added later.

## 4. Candidate residual direct CO2

Source:
emitted CO2 = 142 short ton/day.

Equation:
E_cand,direct = 142 × 0.90718474 × 365 × 0.85.

Result:
**39,966.48 tCO2/y**.

## 5. Direct CO2 saved

Equation:
A_direct = E_base,direct - E_cand,direct.

Substitution:
902,060.28 - 39,966.48.

Result:
**862,093.80 tCO2/y saved**.

Rounded manuscript value:
**862,094 t/y**.

## 6. Natural-gas upstream lifecycle change

Natural-gas energy per day:
Q_NG = F_NG × 10^6 scf/MMSCF × 1044 Btu/scf × 1.05505585262e-6 GJ/Btu.

Baseline:
Q_base = 52.5 × 10^6 × 1044 × 1.05505585262e-6
= **57,838.56 GJ/day** approximately.

Candidate:
Q_cand = 34.0 × 10^6 × 1044 × 1.05505585262e-6
= **37,459.64 GJ/day** approximately.

Annual upstream emissions:
E_up = Q_NG × 365 × A × 11.5 kgCO2e/GJ / 1000 kg/t.

Canonical Rust results:
- baseline upstream = **206,321.69 tCO2e/y**;
- candidate upstream = **133,617.86 tCO2e/y**.

Saved:
206,321.69 - 133,617.86
= **72,703.83 tCO2e/y saved**.

## 7. Nuclear process-heat lifecycle burden

Canonical screening proxy:

E_nuclear = Q_heat × 1000 kW/MW × h × (5.5 × 0.504) kgCO2e/MWh-equivalent / 10^6.

Substitution:
176.8 MWth × 1000 × 7446 h × (5.5 × 0.504) / 10^6.

Result:
**3,649.21 tCO2e/y ADDED**.

Boundary:
this is a screening allocation proxy, not a project-specific nuclear process-heat LCA.

## 8. Incremental auxiliary-electricity lifecycle burden

Candidate process electricity:
17.3 MWe.

Common/baseline process electricity retained in model:
6.3 MWe.

Increment:
11.0 MWe.

Equation:
E_aux = 11.0 MW × 1000 × 7446 h × 5.5 kgCO2e/MWh / 10^6.

Result:
**450.48 tCO2e/y ADDED**.

## 9. CO2 transport/storage lifecycle burden

Equation:
E_TS = 0.025 tCO2e/tCO2 × 542,361.98 tCO2/y.

Result:
**13,559.05 tCO2e/y ADDED**.

Boundary:
generic screening factor; future Singapore cross-border route remains unresolved.

## 10. Baseline and candidate lifecycle totals

Baseline lifecycle:
E_base,LC = E_base,direct + E_base,upstream

= 902,060.28 + 206,321.69
= **1,108,381.97 tCO2e/y**.

Candidate lifecycle:
E_cand,LC =
E_cand,direct
+ E_cand,upstream
+ E_nuclear
+ E_aux
+ E_TS

= 39,966.48
+ 133,617.86
+ 3,649.21
+ 450.48
+ 13,559.05

= **191,243.07 tCO2e/y**.

## 11. Net lifecycle avoided emissions

Equivalent ledger equation:

A_LC =
A_direct
+ A_upstream
- E_nuclear
- E_aux
- E_TS.

Substitution:

862,093.80
+ 72,703.83
- 3,649.21
- 450.48
- 13,559.05

= **917,138.90 tCO2e/y avoided**.

Rounded:
**917,139 tCO2e/y = 0.917 MtCO2e/y**.

Threshold ratio:
917,138.90 / 250,000
= **3.67× the CN4252 annual-abatement requirement**.

## Reconciliation table

| Item | Baseline (tCO2e/y) | Candidate (tCO2e/y) | SAVED ↓ / ADDED ↑ | CO2e change (t/y) | Why |
|---|---:|---:|---|---:|---|
| Direct plant CO2 | 902,060.28 | 39,966.48 | SAVED ↓ | +862,093.80 | nuclear heat + CCS reduce direct source emissions |
| Upstream natural gas | 206,321.69 | 133,617.86 | SAVED ↓ | +72,703.83 | candidate uses 34 vs 52.5 MMSCFD NG |
| Nuclear process heat LCA | 0 | 3,649.21 | ADDED ↑ | -3,649.21 | screening nuclear lifecycle proxy |
| Incremental auxiliary electricity | 0 | 450.48 | ADDED ↑ | -450.48 | 11 MWe incremental auxiliary burden |
| CO2 transport/storage LCA | 0 | 13,559.05 | ADDED ↑ | -13,559.05 | 0.025 tCO2e/t captured |
| **TOTAL LIFECYCLE** | **1,108,381.97** | **191,243.07** | **NET SAVED** | **+917,138.90** | baseline minus candidate |

Integer display reconciliation:
862,094 + 72,704 - 3,649 - 450 - 13,559 = 917,140 t/y.
The ~1 t/y difference from 917,139 is display rounding only; canonical Rust uses unrounded terms.

## Provenance chain

INL process rates
→ canonical constants in `model/src/final_design.rs`
→ `final_lifecycle_ledger()`
→ reconciliation test against `final_design()`
→ generated ledger CSV
→ manuscript table.

## What remains outside this derivation

- Singapore-specific upstream gas LCA;
- project-specific nuclear process-heat LCA;
- dynamic CCS outage/capture energy;
- project cooling-system LCA;
- construction/decommissioning changes outside selected factors.

These are uncertainty/future-analysis terms, not silently assumed zero.
