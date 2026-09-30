# Selected-Design Accident / Transient Sequence Register

Purpose: structure the safety questions for the selected HTGR + IHX + secondary-helium + SMR-H2/CCS architecture. This is **not a PRA**: frequencies and off-site consequences are not calculated.

| ID | Initiating event | Immediate physical response | Key barriers / safety functions | Potential release / process pathway | Evidence maturity | Missing model/data | Disposition |
|---|---|---|---|---|---|---|---|
| AS-01 | Loss of forced primary-helium circulation | core heat removal decreases; fuel/graphite heat up; negative temperature feedback reduces fission power | reactor shutdown, graphite thermal inertia, passive/residual heat removal | normally no immediate release; consequence depends heat removal/barrier integrity | HTTR 9 MW LOFC experimentally demonstrated; selected 600 MWth design not tested | selected-design transient temperatures/heat-removal capacity | HIGHER-FIDELITY THERMAL-HYDRAULICS |
| AS-02 | Primary depressurisation / break | helium inventory/pressure loss; potential later air ingress | vessel/confinement, shutdown, passive heat removal, geometry limiting ingress | fission products already in circuit + fuel/graphite release if heat/oxidation severe | NRC/JAEA mechanism evidence; project source term absent | break spectrum, ingress rate, dust/plate-out, source term | PRA + SOURCE TERM REQUIRED |
| AS-03 | Air ingress after depressurisation | graphite oxidation; CO/CO2; possible heat generation/material degradation | geometry/natural circulation resistance, confinement, heat removal | altered fuel/graphite retention + mobilised circuit inventory | strong external evidence; design-specific severity | selected geometry, oxidation kinetics, sequence duration | HIGHER-FIDELITY ACCIDENT MODEL |
| AS-04 | Water/steam ingress to primary | graphite-steam reaction; H2/CO; chemistry/reactivity perturbation | pressure boundary, leak detection/isolation, purification/shutdown | chemical reaction products + possible source-term changes | NRC HTGR PIRT mechanism | credible project pathway and ingress quantity | DESIGN-SPECIFIC ANALYSIS |
| AS-05 | Loss of ultimate heat sink / residual heat removal | decay heat accumulates | passive heat-removal paths, shutdown, thermal inertia | barrier challenge only if temperatures exceed design envelopes | generic HTGR safety evidence | selected design heat-removal configuration and site heat sink | DESIGN-SPECIFIC ANALYSIS |
| AS-06 | IHX primary-to-secondary leak | primary helium/radionuclides enter secondary loop depending pressure gradient | tube integrity, leak detection, pressure hierarchy, isolation | secondary-helium/process-side contamination | HTTR IHX + IAEA coupling evidence | leak size/frequency, tritium/fission-product transport | COMPONENT + SOURCE-TERM ANALYSIS |
| AS-07 | Secondary-helium rupture/blowdown | secondary pressure loss; abrupt process heat-sink change | isolation/relief, reactor control/trip, bypass/dump heat sink | no direct fuel release expected unless propagated | NRC process-heat PIRT | transient pressure/temperature/IHX loads | COUPLED TRANSIENT MODEL |
| AS-08 | Reformer/process trip: loss of 176.8 MWth heat load | hot secondary helium return changes rapidly; nuclear heat sink reduced | bypass/dump cooler, reactor control/trip, thermal inventory | nuclear transient; chemical plant isolated | IAEA/JAEA coupling methodology | control-system response and dump-heat design | COUPLED TRANSIENT MODEL |
| AS-09 | Reactor trip / loss of nuclear process heat | reformer heat collapses while methane/steam inventory remains | feed isolation, steam management, depressurisation, backup heat if designed | coking/flammable inventory/process upset | chemical engineering mechanism; project dynamic model absent | reformer/catalyst dynamic safe-state model | PROCESS DYNAMICS REQUIRED |
| AS-10 | Hydrogen/methane release + ignition | fire/jet/vapour-cloud explosion | detection, isolation, ventilation, layout, blast/fire barriers | external fire/overpressure/projectiles challenge nuclear SSCs | NRC co-location PIRT supports hazard class | inventory, CFD/fire contours, SSC fragility, separation | SITE-SPECIFIC QRA |
| AS-11 | CO/syngas toxic release | toxic cloud / personnel impairment | detection, ventilation, isolation, access control | emergency-response/access challenge | industrial hazard established | inventory/dispersion/occupancy | SITE-SPECIFIC QRA |
| AS-12 | CO2 pipeline/compression rupture | cold/dense CO2 cloud, asphyxiation, pressure effects | isolation, routing, ventilation/exclusion | personnel/access impact; CCS outage | CCS hazard class established | project pressure/inventory/topography | SITE-SPECIFIC CONSEQUENCE ANALYSIS |
| AS-13 | External refinery/petrochemical fire/explosion | thermal radiation, blast, smoke, projectiles | site separation, barriers, nuclear SSC qualification | common-cause challenge to reactor/process/utilities | IAEA/NRC siting requirement | candidate neighbour inventories/event frequencies | JURONG-SPECIFIC EXTERNAL HAZARD STUDY |
| AS-14 | Coastal flood/storm surge | inundation/loss of access/utilities/heat sink | elevation/barriers, waterproofing, diverse heat sinks | common-cause challenge | PUB coastal risk + IAEA siting requirement | parcel elevation, hazard curves, climate allowance | SITE-SPECIFIC HAZARD STUDY |
| AS-15 | Loss of off-site/shared electrical power | process and active auxiliaries trip | passive reactor heat removal, qualified emergency power/control | process shutdown; reactor decay-heat sequence | generic HTGR safety basis | selected electrical architecture | DESIGN-SPECIFIC SBO ANALYSIS |
| AS-16 | CCS export/storage unavailable | captured CO2 cannot be exported | buffer storage, capture turndown, venting or production reduction | atmospheric CO2 release; not radiological | Singapore CCS chain unresolved | buffer size/contracts/outage duration | OPERABILITY / EMISSIONS ROBUSTNESS |
| AS-17 | Spent-fuel handling/drop/storage event | package/canister mechanical challenge | TRISO coatings, canister, handling controls, shielding | local contamination/source term depending failure | HTR-PM dry-canister/drop evidence; project fuel geometry differs | selected package/fuel/drop/fire analysis | WASTE-DESIGN SPECIFIC |
| AS-18 | Security/aircraft/shipping external event | impact/fire/sabotage | physical protection, structural barriers, security response | potentially multi-system | regulatory hazard category | design-basis threat/site geometry classified/design-specific | REGULATORY / SITE-SPECIFIC |

## Frequency–consequence status

No row has a project frequency or off-site dose because the repository does not contain:
- a selected vendor PRA;
- component failure-rate database mapped to the project;
- project event trees/fault trees;
- mechanistic radionuclide source term;
- site meteorology/dispersion;
- SSC fragility;
- candidate-site population/emergency-response model.

Therefore the register supports **hazard completeness and model scoping**, not numerical risk ranking.

## Priority sequences for future high-fidelity work

1. AS-02/03 — depressurisation + air ingress, because they connect reactor thermal response to source term.
2. AS-06 — IHX leakage, because it tests the claimed nuclear/chemical isolation barrier.
3. AS-08/09 — bidirectional reactor/process trip coupling, because industrial continuity and safe shutdown depend on it.
4. AS-10/13 — internal/external chemical fire/explosion propagation, because Singapore industrial co-location may control site feasibility.
5. AS-14 — coastal flooding, because candidate coastal/industrial sites are exposed to long-life climate hazards.
6. AS-16 — CCS interruption, because it directly changes CN4252 emissions performance.

## Manuscript rule

The final manuscript may state that these hazards are **identified and bounded conceptually**. It may not claim that their frequencies, doses, EPZ or risk acceptance have been demonstrated.
