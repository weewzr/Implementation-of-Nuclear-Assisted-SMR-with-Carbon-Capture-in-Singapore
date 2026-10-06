# E4 Integrated Nuclear + Chemical Safety Case

## 1. Decision and scope

E4 evaluates the E2B/E3 future architecture: one 600 MWth GTHTR300C-class high-temperature reactor, primary helium -> IHX -> secondary helium -> two 130 MMSCFD SMR-H2+CCS trains, 353.6 MWth total process heat, about 195,892 tH2/y and about 1.085 MtCO2/y captured. Jurong Island remains only a conditional candidate industrial context.

**E4 conclusion: ENGINEERING-SUPPORTED SAFETY ARCHITECTURE / INTEGRATED SAFETY CONDITIONAL / LICENSING-LEVEL SAFETY NOT DEMONSTRATED.**

This is a screening safety case, not a PRA, chemical QRA, licensing safety analysis, mechanistic source term, dose assessment or EPZ calculation.

## 2. Safety objectives and boundaries

### Nuclear/core
Required functions: terminate fission; maintain fuel temperatures within applicable limits; remove decay/residual heat; retain radionuclides through multiple barriers; control reactivity and core geometry.

### Primary helium pressure boundary
Maintain coolant inventory/pressure as designed; limit break consequences; prevent uncontrolled air/water ingress; provide isolation and a defined confinement/source-term path.

### IHX
Transfer high-temperature heat while maintaining primary/secondary pressure and radiological separation; detect/isolate leakage; withstand high-temperature creep/fatigue/transients.

### Secondary helium/process-heat loop
Deliver heat without becoming a propagation path; survive/isolate blowdown; control return temperature; provide a safe response to loss of circulation or process heat demand.

### Chemical trains
Contain and isolate NG, H2, CO/syngas, steam, amine and CO2 inventories; terminate feed/heat; prevent ignition/escalation; reach controlled depressurisation or validated safe disposal where appropriate.

### Cooling/power/control/support
Nuclear safety functions must not depend on continued chemical production. Common systems require independence/diversity analysis so one failure cannot defeat both safe states.

### External Jurong hazards
Industrial fire/explosion/toxic release, shipping, flooding, extreme rainfall, sea-level change, intake blockage, transport and security hazards must be treated as external initiating events, not accepted merely because Jurong is an industrial estate.

## 3. Defence in depth

The project barrier chain is:
TRISO fuel -> graphite/fuel element/core retention -> reactor pressure boundary -> reactor-building/confinement functions applicable to the selected design -> primary helium boundary -> IHX -> secondary helium boundary -> physical nuclear/chemical separation -> chemical containment/isolation -> detection/trip/mitigation.

The IHX is both heat-transfer equipment and a safety boundary. The built HTTR 10 MW He-He IHX demonstrates the physical principle at high temperature; the project-scale 176.8 MWth-per-train interface and long-life qualification are not demonstrated.

TRISO is a strong first barrier, not a claim of zero release. AGR-1 safety tests at 1600-1800 C show radionuclide-specific retention and failure behaviour; those particle/compact results cannot be converted directly into a project environmental release fraction.

## 4. Nuclear initiating events

### N1 — reactor/process-load mismatch
A simultaneous trip of both reformers removes **353.6 MWth of demanded process heat**; one train corresponds to 176.8 MWth. This is a project-derived heat-sink step, not a decay-heat value or dump-cooler design duty.

Required sequence: detect process/load loss -> control/trip reactor as required -> isolate/manage secondary loop -> transfer residual/decay heat to an independent validated heat sink -> maintain core and pressure-boundary limits. The chemical side simultaneously isolates feed and hot pressurised inventories.

JAEA buffering/load-change experiments support the concept that process disturbances can be attenuated. They do not validate the 600 MWth/two-train transient.

### N2 — loss of forced primary-helium circulation
HTTR provides experimental evidence at 9 MW: circulator stoppage without control-rod insertion showed strong negative-temperature feedback and slow thermal response from graphite inertia. This is important DEMONSTRATED/EXPERIMENTAL evidence, but geometry/power/heat-removal differences prevent direct transfer to the 600 MWth project.

Required safe state: subcritical core, fuel temperatures within limits and reliable decay-heat removal.

### N3 — primary depressurisation
A break loses helium pressure/inventory and can establish later ingress pathways. The safety case requires shutdown, residual heat removal, break isolation where possible and retention of radionuclides. Break sizes, temperatures and release are not calculated in E4.

### N4 — air ingress
Japanese and Chinese experiments/models and NRC PIRT evidence establish graphite oxidation as a real HTGR accident mechanism. Required analysis is selected-geometry ingress -> oxidation -> temperature/material response -> radionuclide transport. E4 assigns no release fraction.

### N5 — water/steam ingress
External HTGR evidence supports reactivity/chemistry/pressure effects and graphite-steam production of H2/CO. The project's He-He IHX removes some steam-generator-specific coupling, but the actual water/steam interfaces must still be mapped. No transferable ingress mass is adopted.

### N6 — reactivity/control failure
Negative temperature feedback is a favourable characteristic, not a substitute for independent shutdown/protection and selected-design transient analysis.

### N7 — loss of ultimate heat sink / station blackout
E3 did not select or safety-qualify a Jurong cooling architecture. E4 therefore requires nuclear decay-heat removal and essential I&C to survive loss of grid/process cooling/common seawater infrastructure through qualified independent/diverse functions.

## 5. IHX and process-interface events

I1/I2: IHX leak or rupture can couple pressure transients and radiological inventory across loops. Direction depends on pressure state. Detection, pressure management and rapid isolation are required; core cooling must remain available after isolation.

I3/I4: loss of secondary circulation or depressurisation removes process heat transfer and changes the IHX/primary boundary condition. Both reformers require safe shutdown while the reactor retains an independent heat sink.

I5: excessive secondary temperature/control failure can overheat reformer/process equipment; temperature trips, isolation and bypass/heat-management logic are required.

I6: tritium proves radiological isolation is finite. HTTR measured non-zero tritium in secondary helium and Hastelloy XR experiments demonstrate hydrogen-isotope permeation. The project needs a generation -> primary transport -> IHX permeation -> secondary purification -> reformer/product model before chemical-plant radiological classification can be claimed.

## 6. Chemical hazards

The two-train plant contains major flammable/toxic/high-pressure hazards:
- C1 H2 release: jet/flash fire, deflagration/explosion and confined accumulation;
- C2 NG/methane release: fire/explosion;
- C3 CO/syngas release: toxic exposure and emergency-response impairment;
- C4 reformer/hot-process leak: fire/escalation even without a fired reformer furnace;
- C5 high-pressure steam/process rupture: jets, burns and projectiles;
- C6 PSA/tail-gas event: flammable/toxic release using only the established source topology;
- C7 amine/CCS event: solvent/reboiler/leak/degradation hazards to be resolved for the selected solvent;
- C8 CO2 release: high-pressure/cold dense cloud, asphyxiation and access impairment;
- C9 CO2 compressor/conditioning failure: pressure/mechanical/release hazard without inventing compressor states.

Singapore's MHI Safety Case regime is directly relevant to the chemical side: major-hazard installations must demonstrate prevention/mitigation and ALARP risk management; QRA can be required for significant flammable/hazardous inventories. This does not replace nuclear licensing.

## 7. Bidirectional propagation

The canonical matrix is `results/e4_safety/e4_propagation_matrix.csv`.

Nuclear -> chemical propagation is dominated by loss/excess process heat, secondary-loop pressure failures, IHX radiological transfer and shared utility/cooling disturbances.

Chemical -> nuclear propagation is dominated by H2/NG blast/fire/projectiles, high-pressure rupture, CO/CO2 effects on personnel/access, and common utility/cooling loss.

NRC RIL 2026-04 confirms that external-hazard risk from a co-located hydrogen-production facility is an active advanced-reactor regulatory research problem. E4 therefore does not invent a generic separation distance.

## 8. Common-cause and Jurong external hazards

Potential shared dependencies:
- grid and emergency electrical distribution;
- seawater intake/cooling;
- firewater and utilities;
- I&C/communications;
- emergency access;
- flood protection;
- external industrial/port response.

Independence/diversity is required wherever failure of one shared service could disable both nuclear decay-heat removal and chemical inventory isolation.

Jurong register:
1. neighbouring refinery/chemical fire/explosion;
2. hazardous/toxic releases;
3. pipelines/storage;
4. shipping/port collision/fire/explosion;
5. dangerous-goods transport;
6. coastal flooding/storm surge;
7. extreme rainfall/compound flooding;
8. sea-level rise over plant life;
9. intake blockage/fouling;
10. seismic/geotechnical basis;
11. aircraft/transport where applicable;
12. malicious/security events at high level.

No item is declared acceptable without parcel-specific analysis.

## 9. Safe-state logic

Nuclear safe state requires: fission terminated; fuel temperature controlled; decay heat removed; radionuclide barriers retained; failed boundaries isolated; required cooling/control restored or maintained.

Chemical safe state requires: NG/H2/CO feed/inventories isolated; reaction and external heat input stopped as required; pressure reduced through a validated route where appropriate; ignition/escalation prevented; safe vent/flare/disposal used only if actually designed.

Integrated safe state requires: one plant's trip/emergency action cannot disable the other's safe-state functions; safety power/cooling/control and emergency access must have adequate independence.

Detailed control sequences are intentionally not invented.

## 10. Evidence maturity

### Demonstrated/experimental
- HTTR 950 C-class operation;
- HTTR 9 MW LOFC response;
- built 10 MW HTTR He-He IHX;
- non-zero secondary-loop tritium measured in operating HTTR;
- Hastelloy XR isotope-permeation experiments;
- AGR TRISO high-temperature safety tests.

### Source design/modelled
- 600 MWth GTHTR300C-class architecture;
- ~170 MWth reference commercial-scale IHX design;
- process-heat separation/buffering concepts;
- INL 176.8 MWth-per-train chemical/process state.

### Project screening
- two-train 353.6 MWth process-heat step;
- bidirectional propagation map;
- Jurong common-cause/external-hazard register;
- barrier and safe-state mapping.

### Not demonstrated
- project PRA;
- event frequencies;
- mechanistic project source term;
- 600 MWth accident transient temperatures;
- project IHX rupture/leak consequences;
- project tritium/product concentration;
- chemical QRA/risk contours;
- blast/fire separation distance;
- site dispersion/off-site dose;
- project EPZ;
- licensing/regulatory acceptance.

## 11. FMEA/bow-tie structure

The machine-readable hazard register acts as the FMEA backbone.

Generic bow-tie:
INITIATOR -> LOSS OF CONTROL/CONTAINMENT -> PROPAGATION PATH
with preventive barriers before loss of control and mitigating barriers after it
-> SAFE-STATE FUNCTIONS -> residual consequence requiring E5 quantification.

No probability is attached to any branch.

## 12. Safety-critical unresolved questions: E5 handoff

| Required input | Required method | Required output | Decision enabled |
|---|---|---|---|
| selected component failure data + system architecture | PRA event/fault trees | sequence frequencies/contributors | dominant nuclear sequences and reliability needs |
| core inventory + fuel/core/circuit/confinement models | mechanistic source-term analysis | nuclide-specific release to environment | consequence/dose basis |
| project thermal-hydraulic geometry/control | coupled transient simulation | peak T/P/flow and barrier margins | N1-N7/I1-I5 acceptability |
| IHX geometry/material/pressure + leak spectrum | structural/thermal-hydraulic/source-term analysis | leak/rupture loads and cross-loop transfer | interface qualification/isolation |
| tritium generation/speciation/purification/material data | transport/permeation model | secondary/process/product concentration | radiological classification |
| H2/NG inventories/layout/ignition/congestion | QRA + CFD/blast/fire models | overpressure/thermal contours + frequencies | separation/barrier design |
| CO/CO2 inventories/pressure/topography/weather | dispersion/QRA | toxic/asphyxiant contours and risk | access/layout/emergency controls |
| parcel/neighbour inventories + transport activity | external-hazard QRA | external event loads/frequencies | Jurong site acceptability |
| UHS/electrical architecture + coastal hazards | SBO/UHS/common-cause analysis | safety-function availability/loads | independence/diversity design |
| site meteorology/population/source term | dispersion/dose/emergency analysis | dose/protective-action basis | EPZ/EPR |
| security/site geometry | regulator-led security assessment | protected-area/access requirements | site feasibility |

## 13. CN4252 feasibility classification

**SUPPORTED:** the source technology has relevant experimental/operational safety evidence: high-temperature HTTR operation, LOFC behaviour, TRISO retention evidence and a real He-He IHX.

**ENGINEERING-SUPPORTED:** the project architecture has identifiable defence-in-depth barriers, bidirectional propagation paths and safe-state functions.

**CONDITIONAL:** integrated deployment is plausible only if coupled transients, IHX qualification, tritium, chemical QRA/separation, independent heat sink/power and Jurong external hazards close acceptably.

**NOT DEMONSTRATED:** licensing-level integrated safety for a Jurong deployment.

Therefore safety does not falsify the concept at E4, but it remains a major CN4252 feasibility gate. The implementation roadmap must require successful E5 quantitative closure before any site/deployment claim.

## 14. Later visual specification

Created: `results/e4_safety/E4_LATER_VISUAL_SPECIFICATION.md`.

Final safety artwork must remain conceptual/not-to-scale. Event trees, risk contours, barrier chains and quantitative safety diagrams should be deterministic technical graphics, not AI artwork.

## 15. Durable outputs

- `results/E4_INTEGRATED_SAFETY_CASE.md`
- `results/e4_safety/e4_hazard_register.csv`
- `results/e4_safety/e4_barrier_matrix.csv`
- `results/e4_safety/e4_propagation_matrix.csv`
- `results/e4_safety/e4_safe_state_matrix.csv`
- `results/e4_safety/e4_evidence_matrix.csv`
- `results/e4_safety/E4_LATER_VISUAL_SPECIFICATION.md`

No new Rust module is justified: E4 contains causal safety structure, while its only new arithmetic quantity (353.6 MWth simultaneous two-train heat-sink step) is already deterministic E2B architecture data. Fake numerical risk precision was deliberately avoided.

## 16. Source basis

Primary/authoritative basis retained from the repository:
- JAEA/JAERI HTTR and GTHTR300C evidence;
- NRC HTGR/TRISO and NGNP PIRTs;
- INL AGR TRISO evidence;
- IAEA nuclear design/siting/emergency standards;
- Singapore E3 Jurong evidence;
- MOM/NEA/SCDF Major Hazard Installation/QRA framework.

Current cross-checks:
- U.S. NRC, RIL 2026-04, *External Hazard Risk Assessment Framework for a Co-Located Hydrogen Production Facility*.
- Singapore MOM, MHI Safety Case Regime / QRA / Safety Case preparation and EC&I guidance.
- IAEA SSR-2/1 (Rev. 1), nuclear power plant design safety requirements.

## 17. E4 closure

A technically trained reader can now trace:
**initiator -> immediate hazard -> cross-plant propagation -> preventive/mitigating barrier -> required safe state -> evidence maturity -> missing E5 analysis.**

The project may claim a credible screening safety architecture and relevant demonstrated source-technology evidence. It may not claim that the integrated Jurong plant is "safe", licensed, quantitatively acceptable, or associated with any project EPZ/separation radius.

**E4 classification: ENGINEERING-SUPPORTED SAFETY ARCHITECTURE; INTEGRATED SAFETY CONDITIONAL; LICENSING-LEVEL SAFETY NOT DEMONSTRATED.**
