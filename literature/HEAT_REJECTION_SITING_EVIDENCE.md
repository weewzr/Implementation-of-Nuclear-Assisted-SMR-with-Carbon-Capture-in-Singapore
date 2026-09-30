# Heat Rejection, Cooling and Singapore Siting Evidence

## Research questions

1. What heat must the selected 600 MWth HTGR ultimately reject after supplying 176.8 MWth to the hydrogen process?
2. What cooling system would be required?
3. Does Jurong Island, offshore/floating or underground siting improve feasibility?
4. What can be concluded without a site-specific nuclear design?

## Critical accounting correction

The project's **423.2 MWth remaining reactor thermal capacity is NOT a cooling duty**.

It is only:

600 MWth reactor rating - 176.8 MWth process heat = 423.2 MWth remaining thermal capacity.

In the source GTHTR300C architecture, remaining thermal energy can enter a direct Brayton power cycle. JAEA's design includes a gas turbine, recuperator and precooler; low-temperature heat removed in the precooler can itself be used for desalination/district heating. Therefore:

reactor thermal input
= high-temperature process heat
+ electrical work
+ recoverable lower-temperature heat
+ ultimate environmental heat rejection
+ losses.

Because the project intentionally makes **no exact off-design project gross/net/export MWe claim**, it also lacks the thermodynamic state points needed to calculate an exact project condenser/precooler heat-rejection duty.

**Current cooling-duty status: UNRESOLVED — REQUIRES A COHERENT OFF-DESIGN POWER/HEAT CYCLE OR A HEAT-ONLY REJECTION DESIGN.**

## Source GTHTR300C cooling architecture

JAEA's GTHTR300C source design is a direct helium Brayton cogeneration system. Published load-follow work describes:
- 600 MWth reactor;
- 950 C reactor outlet;
- topping IHX delivering 900 C process heat;
- direct-cycle gas turbine;
- recuperator;
- precooler removing low-temperature cycle heat;
- intermediate helium loop isolating the distant process plant.

The source literature notes that ~160 C waste heat from the precooler can be used for desalination/district heating without reducing gas-turbine output.

A JAEA cascade-energy concept explicitly shows seawater/freshwater desalination integrated with the low-temperature heat sink. This demonstrates a **design option**, not a Singapore cooling-system selection.

## Why a seawater-flow number is premature

A once-through seawater flow would require at least:

m_dot_water = Q_reject / (cp_water * DeltaT_water).

But neither Q_reject nor the allowable intake/outfall DeltaT is established for the project. Q_reject depends on:
- selected power-cycle operating point;
- gas-turbine/compressor/recuperator performance;
- whether remaining reactor capacity generates power;
- whether low-temperature heat is recovered;
- process heat returned/recovered from the chemical plant;
- auxiliary loads;
- environmental discharge limits.

Publishing a seawater flow now would therefore be false precision.

## Nuclear siting criteria relevant to Singapore

IAEA SSG-35 / SSR-1 require site survey/evaluation to address natural and human-induced hazards, population/emergency-response feasibility, cooling-water availability, security and environmental impacts.

For a Singapore coastal/industrial candidate, relevant criteria include:

### Natural / coastal
- coastal flooding, storm surge, tides, waves and combinations;
- extreme precipitation and inland/coastal compound flooding;
- sea-level change over plant lifetime;
- shoreline erosion/sedimentation;
- geotechnical stability, settlement/subsidence and liquefaction;
- extreme temperature/wind/lightning;
- intake blockage by debris or marine organisms;
- loss of ultimate heat sink.

### Human-induced / industrial
- refineries and oil/gas operations;
- hazardous-substance processing/storage;
- external fire/explosion/toxic releases;
- shipping collision and port hazards;
- aircraft/security threats;
- pipelines and neighbouring industrial infrastructure;
- electromagnetic/interference or grid/common-utility dependencies where relevant.

### Emergency / population
- population density and distribution;
- feasibility of sheltering/evacuation/protective actions;
- emergency access;
- ability to establish the regulator-approved emergency planning arrangements.

### Non-safety / deployment
- land footprint;
- access to cooling water;
- grid and industrial load proximity;
- construction/logistics access;
- ecological impacts and thermal discharge;
- public acceptance;
- security zoning.

## Jurong Island as a screening case

### Why it is attractive
JTC describes Jurong Island as a ~3,000 ha integrated energy-and-chemicals complex with >100 companies and >100 km of pipelines. This makes it an obvious **industrial-integration screening case** for natural gas, hydrogen, CO2 handling, utilities and industrial offtake.

### Why it is not selected
The same industrial concentration creates nuclear external-hazard questions: refineries, chemical plants, hazardous inventories, pipelines, shipping and common infrastructure are exactly the types of human-induced hazards IAEA siting guidance requires evaluating.

PUB has a dedicated Jurong Island coastal-protection study because coastal flood/sea-level-rise risk is material. Singapore is low-lying, and PUB states ~30% of the island is below 5 m above mean sea level. New critical infrastructure is being planned with higher coastal protection/platform levels.

No project-specific study currently establishes:
- nuclear-compatible parcel size;
- separation distance from hazardous neighbours;
- design-basis industrial explosions/fires;
- flood elevation/protection;
- intake/outfall design;
- emergency-planning feasibility;
- security perimeter;
- geotechnical suitability.

**Disposition: JURONG ISLAND = SCREENING CASE ONLY — REQUIRES SITE-SPECIFIC NUCLEAR/INDUSTRIAL HAZARD STUDY.**

## Offshore / floating option

Potential advantages:
- reduced competition for land;
- direct access to seawater heat sink;
- potentially greater separation from population/industrial hazards.

Potential disadvantages/questions:
- marine collision and anchoring/mooring hazards;
- storm/wave/sea-level/extreme-weather design;
- corrosion;
- security and territorial/control issues;
- emergency access/evacuation;
- spent-fuel/radioactive-material transport;
- subsea/ship-to-shore hydrogen/heat/electric/CO2 interfaces;
- maintenance/drydock strategy;
- regulator and safeguards framework.

**Disposition: FUTURE WORK / ALTERNATIVE SITE CONCEPT.** No evidence currently supports selecting it over land-based siting.

## Underground option

Potential advantages:
- shielding/protection from some external events;
- possible security/consequence benefits depending design.

Potential disadvantages/questions:
- excavation/geotechnical suitability;
- flooding/water ingress;
- heat removal;
- access/egress and emergency response;
- construction cost/schedule;
- hydrogen/chemical process cannot simply be assumed underground with reactor;
- maintenance/replacement logistics.

**Disposition: FUTURE WORK / ALTERNATIVE SITE CONCEPT.**

## Cooling / siting V&V maturity

| Item | Verification | Validation | Uncertainty | Extrapolation |
|---|---|---|---|---|
| 423.2 MWth residual capacity identity | exact arithmetic/testable | N/A | none | none |
| source GTHTR300C Brayton/precooler architecture | literature cross-check | component models partly tied to HTTR/mock-ups | design-study uncertainty | source design → project off-design state |
| project heat rejection | not modelled | none | high | cannot infer from residual capacity |
| Singapore cooling-water system | not modelled | none | site/environment dependent | requires candidate site |
| Jurong flood/external hazards | screening evidence | PUB/JTC authoritative context | parcel-specific unknown | requires site survey |
| site acceptability | not modelled | none | regulatory/site dependent | cannot be claimed |

## Current conclusions

- **DF-21 heat rejection:** remaining 423.2 MWth is not waste heat; exact rejection duty is **UNRESOLVED**.
- **DF-22 floating/offshore:** **FUTURE WORK**, no preferred-site conclusion.
- **DF-23 underground:** **FUTURE WORK**, no preferred-site conclusion.
- **DF-19 Jurong Island:** **REQUIRES SITE-SPECIFIC ANALYSIS**; industrial integration is attractive but external-hazard/flood/security/emergency issues are potentially decisive.
- Cooling-system numerical sizing should wait until a coherent project power/heat cycle is selected.

## Primary / authoritative sources

- Yan et al. / JAEA, GTHTR300C load-follow and cogeneration design literature.
- JAEA-Technology 2011-013, GTHTR300 series specifications/cascade-energy system.
- IAEA SSR-1, *Site Evaluation for Nuclear Installations*.
- IAEA SSG-35, *Site Survey and Site Selection for Nuclear Installations*.
- IAEA meteorological/hydrological site-evaluation safety guidance.
- PUB Singapore, coastal protection and Jurong Island site-specific studies.
- JTC, Jurong Island official industrial/infrastructure context.
