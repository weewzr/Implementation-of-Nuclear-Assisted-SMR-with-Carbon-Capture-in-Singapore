# E5 Quantitative Safety Depth — PRA, Source Term, QRA and Consequence Requirements

## 1. Decision

E5 advances E4 from causal safety structure to a **quantitative-analysis boundary and executable safety programme**. It does not manufacture licensing risk numbers.

**E5 conclusion: quantitative screening is PARTIAL; safety remains CONDITIONAL and licensing-level integrated safety is NOT DEMONSTRATED. Existing evidence does not falsify the concept on safety grounds, but several unresolved analyses could still do so.**

## 2. What is quantitatively closed now

The project can calculate or reproduce only quantities whose inputs are actually present:
- one-train process-heat demand step: **176.8 MWth**;
- simultaneous two-train heat-demand step: **353.6 MWth**;
- E3 full-power residual-disposition identity: **246.4 MWth**, explicitly not decay heat;
- E3 illustrative seawater flow at 246.4 MWth and 8 K: **7.531 m3/s**, explicitly not safety UHS duty;
- full-600-MWth illustrative 8 K seawater envelope: **18.339 m3/s**, not decay heat;
- NG operating flow: **68 MMSCFD**;
- H2 product operating flow: **260 MMSCFD**;
- captured CO2 throughput: approximately **1.085 Mt/y**, not stored vessel inventory;
- source HTTR secondary-helium tritium measurements: peak **4.7e-2 Bq/cm3(STP)** during power rise and about **2.2e-2 Bq/cm3(STP)** at the end of normal full-power operation. These are source measurements, not project predictions.

No new Rust module is warranted: these project arithmetic quantities are already deterministic outputs of E2B/E3. E5's new work is model structure/data sufficiency, not a new numerical physical model.

## 3. PRA screening structure

`e5_event_tree_register.csv` defines ten screening trees covering simultaneous process-load loss, loss of forced circulation, depressurisation, air ingress, water/steam ingress, UHS loss, SBO, IHX failure, secondary-loop circulation loss and external/common-cause events.

Canonical branch structure:
**INITIATOR -> PROTECTION/TRIP -> SHUTDOWN -> DECAY-HEAT REMOVAL -> ISOLATION -> BARRIER STATE -> ENDPOINT.**

No branch probability is assigned. A valid PRA requires the selected system architecture, component reliability, test/maintenance, common-cause and human-action data. Generic numbers are not inserted merely to obtain a frequency.

## 4. Reliability data status

The repository has operating/design evidence showing that protection, helium circulation, shutdown, isolation and heat removal are real safety functions. It does not have a selected 600 MWth project's component list or reliability database.

Required fault trees are specified for reactor trip, shutdown, decay-heat removal, emergency power, UHS, IHX isolation, secondary isolation and chemical ESD. Each needs architecture-specific basic-event and dependency data.

**PRA frequency status: NOT CALCULABLE NOW.**

## 5. Mechanistic source term and core inventory

The required chain is:
**core inventory -> TRISO release(T,t,failure mode) -> graphite/core retention -> primary-circuit deposition/dust -> pressure-boundary/confinement transport -> environmental release -> atmospheric dispersion -> dose.**

### Core inventory
600 MWth alone does not determine nuclide inventory. The project lacks batch-specific fuel loading/isotopics, burnup distribution, irradiation/cycle history and power history. A defensible inventory therefore requires an ORIGEN-class depletion calculation using selected GTHTR300C fuel-management data.

**No project radionuclide inventory is calculated.**

### TRISO release
AGR/HTTR evidence supports strong but nuclide-specific retention and identifies coating-failure/high-temperature behaviour. The project has no 600 MWth accident fuel-temperature/time history. AGR furnace-test release fractions therefore are not applied as a project release fraction.

**No project accident TRISO release is calculated.**

Primary-circuit plate-out/dust, confinement transport and release likewise require selected-design geometry/material/state data.

## 6. IHX leak and tritium

A project IHX leak calculation needs break/leak geometry, primary and secondary pressure/temperature/inventory, isolation response and radionuclide inventory. Those are absent.

Tritium can be represented by governing inventories:
[
\frac{dN_p}{dt}=G-R_{pur,p}-R_{perm}-R_{loss,p},
]
[
\frac{dN_s}{dt}=R_{perm}-R_{pur,s}-R_{proc}-R_{loss,s}.
]
A wall-permeation model requires a material permeability law, temperature field, effective area/thickness, oxide condition and isotope partial-pressure driving force. The repository has HTTR/Hastelloy-XR evidence that permeation is real and temperature/material dependent, but not the project geometry/purification/speciation needed to evaluate these equations.

**Project tritium concentration and chemical-plant radiological classification remain unquantified.**

## 7. Chemical QRA

Nine scenario families are structured in `e5_qra_scenario_register.csv`: H2, NG, syngas/CO, high-pressure process rupture, hot reformer release, PSA/tail gas, CO2, CO2 conditioning/compression and amine/CCS.

For a continuous gas source, a release calculation generally requires upstream thermodynamic state and opening/pipe geometry in addition to available plant throughput. Throughput alone is not an instantaneous leak rate. Vessel inventory similarly requires volume/state data.

Therefore:
- 260 MMSCFD H2 and 68 MMSCFD NG are valid plant flow scales;
- ~1.085 Mt/y CO2 is a valid throughput scale;
- none is treated as a vessel inventory or rupture release rate.

QRA sequence:
**source -> isolation -> rate/duration -> dispersion -> ignition/no ignition -> fire/explosion/toxic/asphyxiant consequence -> escalation -> receptors/nuclear SSCs -> frequency/risk.**

No risk contour is generated.

## 8. Blast, fire and separation

Required calculation:
**release scenario -> dispersion/congestion -> ignition -> explosion/fire model -> overpressure/impulse/thermal-radiation contours -> nuclear SSC fragility/acceptance -> separation/barrier requirement.**

JAEA/NRC evidence supports this methodology and Singapore QRA supplies a chemical-risk framework. Missing project inventory, opening geometry, layout/congestion, ignition treatment and SSC fragility prevent a defensible distance.

**Project nuclear/chemical separation distance: NOT CALCULATED.**

## 9. CO and CO2 dispersion

CO requires source composition/rate/duration, meteorology/building effects and toxic dose criteria. CO2 can require dense-gas treatment depending release phase/conditions and needs inventory, pressure/temperature, terrain/buildings and occupancy/access data.

No candidate parcel/site meteorological dataset is selected. Generic Singapore weather is insufficient for a project risk contour.

**CO toxicity contours, CO2 asphyxiation contours and control-room/access habitability are NOT CALCULATED.**

## 10. Jurong external hazards

Each E4 hazard is converted into a quantitative chain:
**hazard -> authoritative/site data -> frequency/intensity model -> load at candidate parcel -> SSC response/acceptance -> site decision.**

Required data include neighbour inventories/processes, pipelines, dangerous-goods traffic, marine movements, flood/storm-surge/extreme-rainfall hazard curves, future sea-level allowance, intake blockage statistics, geotechnical/seismic characterisation and applicable transport/aircraft/security inputs.

E3's island-scale context does not supply parcel-level frequencies or loads.

**Jurong external-hazard quantitative acceptability: NOT DEMONSTRATED.**

## 11. UHS and station blackout

Known normal-operation screens from E3 are not decay heat. Neither 600 MWth nor 246.4 MWth is used as decay heat.

A safety UHS/SBO calculation needs:
- selected reactor decay-heat curve versus time;
- passive/active DHR configuration and heat-transfer capacity;
- required mission time;
- emergency electrical loads;
- battery/generator/autonomy data if used;
- seawater/intake or alternate-sink architecture;
- flood/blockage/common-cause assumptions.

Output must show that fuel/barrier limits and essential control functions are maintained for the required mission time.

**Safety UHS duty/autonomy: NOT CALCULATED.**

## 12. Site dispersion, dose and EPZ

Required chain:
**mechanistic time-dependent source term + release geometry/energy + site meteorology + terrain/buildings + receptors/population -> dispersion -> dose -> protective-action basis -> EPR/EPZ.**

IAEA GSR Part 7 provides international emergency-preparedness requirements. The repository establishes no current Singapore project-specific nuclear EPZ rule or regulator-approved radius for this reactor. Singapore remains in pre-deployment nuclear capability/framework development.

Chinese HTR-PM radii are methodology/implementation evidence only and are not transferred.

**Project dose: NOT CALCULATED. Project EPZ: UNKNOWN/SITE-SPECIFIC.**

## 13. Risk and acceptance criteria

Chemical and nuclear acceptance must remain separate but simultaneously satisfied.

### Chemical
Singapore's Major Hazard Installation regime requires safety-case/QRA treatment for major hazards and ALARP-style risk management. A future project must use the then-applicable Singapore individual/societal risk criteria and approved QRA methodology. E5 does not invent numerical thresholds not established in the repository evidence set.

### Nuclear
Singapore has not yet established a project-specific licensing/siting/risk framework for this reactor. Future nuclear safety goals, deterministic acceptance criteria, probabilistic criteria and dose/protective-action requirements must come from the competent Singapore framework informed by IAEA and chosen technology/licensing basis.

An integrated site cannot average or merge chemical and nuclear risk into one unsupported metric. It must satisfy both regimes and address cross-domain initiators explicitly.

## 14. Parameterised equations instead of guessed inputs

Where data are absent, E5 preserves the calculation form.

Continuous release mass over isolation interval:
[
m_{rel}=\int_0^{t_{iso}} \dot m_{leak}(P,T,A_{hole},D_{pipe},\text{fluid})\,dt.
]

Inventory for a known vessel state would require a real-fluid or appropriate equation of state:
[
m_{inv}=\rho(P,T,\text{composition})V.
]

Individual chemical risk conceptually:
[
IR(x)=\sum_i f_i P_i C_i(x),
]
where frequencies and conditional probabilities are not assigned until a valid QRA database/scenario model exists.

Mechanistic environmental source term for nuclide (j) is represented as sequential, state-dependent retention/release rather than one generic factor:
[
S_j(t)=I_j\,F_{fuel,j}(t)\,F_{core,j}(t)\,F_{circuit,j}(t)\,F_{conf,j}(t),
]
with the warning that the factors are generally time/state dependent and must come from validated models/data; E5 assigns none numerically.

## 15. Analyses that could still falsify the project

The concept remains vulnerable to:
1. 600 MWth coupled transients exceeding fuel/IHX/reformer limits or lacking a credible heat sink;
2. PRA showing unacceptable dominant sequences/common-cause vulnerabilities;
3. mechanistic source term/dose incompatible with a feasible Singapore site/EPR;
4. IHX leak/rupture or tritium requiring unacceptable nuclear classification/redesign;
5. H2/NG QRA requiring separation/barriers that cannot fit a candidate parcel;
6. CO/CO2 consequences preventing emergency access/control habitability;
7. Jurong external hazards exceeding practical protection/design capability;
8. UHS/SBO autonomy not closable under coastal/common-cause hazards;
9. security requirements conflicting with industrial co-location/access.

No current quantitative result proves any of these failure conditions, but none is closed.

## 16. CN4252 safety-feasibility interpretation

**Source technology safety evidence: SUPPORTED.**
Relevant HTTR/AGR experimental/operational evidence exists.

**Project safety architecture: ENGINEERING-SUPPORTED.**
E4/E5 define barriers, safe states, event trees and analysis paths.

**Quantitative integrated safety: CONDITIONAL / PARTIALLY QUANTIFIED.**
Only limited flow/thermal/source-measurement quantities are available.

**Licensing/site safety: NOT DEMONSTRATED.**

Existing quantitative evidence does not falsify the concept. This means only that no demonstrated contradiction has yet been calculated; it does **not** mean the plant is safe.

The mature E2B economic case cannot be treated as deployable until the E5 programme closes sufficiently to establish acceptable transient behaviour, risk/source term, interface safety, chemical separation, UHS/SBO, site hazards, dose/EPR and security.

## 17. Implementation safety gates

The canonical programme is `results/e5_safety/e5_analysis_program.csv`.

Minimum sequence:
1. freeze selected reactor/IHX/secondary-loop and chemical P&IDs/layout basis;
2. acquire vendor fuel/core/reliability/component data;
3. close 600 MWth coupled transient and DHR/UHS design;
4. develop PRA event/fault trees and reliability database;
5. calculate depletion/core inventory and mechanistic source term;
6. qualify IHX leak/rupture and tritium transport;
7. perform chemical HAZOP/QRA and blast/fire/toxic consequence models;
8. perform candidate-parcel Jurong external-hazard/site study;
9. integrate source term with site dispersion/dose;
10. establish regulator-approved EPR/EPZ and security arrangements;
11. iterate layout/design or reject the site/architecture if criteria fail.

## 18. E5 closure

E5 has converted "do PRA/QRA later" into explicit event-tree structures, reliability inputs, mechanistic source-term stages, QRA scenarios, parameterised equations, acceptance domains and design/site decision gates.

It intentionally stops before inventing the missing physical and probabilistic inputs.

**E5 final classification: QUANTITATIVE SAFETY SCREEN PARTIAL; INTEGRATED SAFETY CONDITIONAL; LICENSING-LEVEL SAFETY NOT DEMONSTRATED.**
