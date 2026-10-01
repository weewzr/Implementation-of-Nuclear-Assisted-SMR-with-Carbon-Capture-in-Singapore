# HTGR Process-Heat Scale-Up and Reliability Evidence

## Research question

Can the selected HTGR architecture reliably supply the project's 176.8 MWth, 900 C-class reformer heat requirement?

## Answer

**Temperature compatibility: SUPPORTED at experimental + source-model level.**

**176.8 MWth integrated duty: SUPPORTED as a process/design calculation, NOT demonstrated at that scale.**

**Commercial lifetime / availability of the complete 600 MWth reactor + large IHX + industrial reformer coupling: CONDITIONAL — REQUIRES COMPONENT QUALIFICATION AND COUPLED DEMONSTRATION.**

## Evidence ladder

| Layer | Evidence | What is actually established | Transfer limit |
|---|---|---|---|
| HTTR reactor | 30 MWth, helium cooled, 4 MPa, 950 C outlet | JAEA achieved 950 C in 2004 and a 50-day continuous full-power 950 C run in 2010 | Test reactor is 20x smaller in thermal power than 600 MWth project reactor |
| HTTR IHX | 10 MW helium-helium shell/tube IHX; primary 950 C → secondary max 905 C; Hastelloy XR structures ~930 C | Hardware precedent for high-temperature primary/secondary helium isolation; creep-based structural design for 10^5 h design life | 10 MW component is ~17.7x smaller than 176.8 MWth project duty |
| German process-heat tests | 10 MW component loop >18,400 h, 38% >900 C; industrial-scale reformer tubes heated with 900 C helium at 4 MPa | Long-duration component/process-heat precedent and helium-heated methane-reforming experiments | Not the project's reactor/IHX/reformer geometry or scale |
| GTHTR300C reactor | 600 MWth, 950 C outlet design | Published JAEA commercial cogeneration design basis; max 370 MW to secondary system | DESIGNED, not an operating 600 MWth plant |
| GTHTR300C IHX | 170 MWth shell-and-tube conceptual design based on HTTR | JAEA primary-stress/heat-transfer conceptual study reported technical feasibility | 170 MWth is below project 176.8 MWth duty and remains conceptual; project cannot call one reference IHX sufficient |
| INL TEV-961 Case 6 | 925 C ROT → 900 C helium supply → 871 C reformer; 176.8 MWth heat; 78.49 kg/s helium; 17.3 MWe process electricity | Source process model directly supports selected operating point and complete nuclear reforming heat at 130 MMSCFD H2 | Process analysis/model, not component qualification or plant demonstration |
| Project | 600 MWth GTHTR300C-class source; 370 MWth source/secondary branch; 176.8 MWth process duty | Thermal capacity screening and integrated CN4252 model | Detailed IHX count/geometry, pressure drop, lifetime, transients and availability remain unresolved |

## Dimensional scale-up

Project duty relative to demonstrated HTTR IHX:

176.8 MWth / 10 MWth = **17.68**.

Project duty relative to the published GTHTR300C reference IHX:

176.8 / 170 = **1.040**, i.e. **4.0% above** the single reference-IHX rating.

This does not imply that an exchanger can simply be scaled linearly. It establishes why the prior manuscript's conservative doubled-IHX / secondary-loop treatment is a screening sensitivity rather than a completed mechanical design.

The selected 600 MWth reactor thermal rating itself matches the published GTHTR300C design basis, but the reactor is a design study rather than an operating demonstration. Therefore the correct evidence statement is:

**30 MWth / 950 C reactor operation: DEMONSTRATED.**

**10 MWth / ~905 C secondary helium IHX: DEMONSTRATED hardware precedent.**

**170 MWth GTHTR300C IHX: CONCEPTUALLY DESIGNED.**

**176.8 MWth project process duty: MODELLED / SOURCE-SUPPORTED.**

**600 MWth GTHTR300C-class integrated project: DESIGNED + MODELLED, NOT DEMONSTRATED.**

## Temperature cascade

INL TEV-961 explicitly assumes a 25 C approach between primary and secondary helium. Its Case 6 is:

925 C reactor outlet
→ 900 C secondary/process-heat helium
→ 871 C target reformer outlet.

The nominal margins are therefore:

- primary-to-secondary approach: **25 C**;
- supplied helium above reformer outlet: **29 C**.

TEV-961 reports that Case 6 alone among the temperature cases eliminates the fired second reforming stage: nuclear heat alone can supply the desired reforming temperature. The same table gives 176.8 MWth heat demand, 78.49 kg/s helium flow, 466 C helium return and 17.3 MWe electricity demand.

Important limitation: these are system-model conditions. A 29 C helium-to-reformer-outlet difference is **not** itself a detailed tube-wall/local-pinch proof. Detailed reformer heat-transfer geometry, wall temperatures, pressure drop and lifetime still require qualification.

## Direct HTGR-SMR evidence

The concept is not unique to the INL flowsheet. Primary/peer-reviewed work includes:

- Yin, Jiang & Zhang (2006/2007): equilibrium and reformer models for HTGR-heated steam methane reforming, including comparison to experimental data.
- Hoseinzade & Adams (2017): dynamic two-dimensional integrated nuclear-heat/SMR model validated against reported design data.
- Zhou et al. (2021): HTR-PM/SMR thermodynamic analysis over temperature, pressure and S/C ranges.
- Wu et al. (2025): one-dimensional reformer-tube model with reported agreement against experimental results.

Counter-evidence / qualification: recent HTGR-SMR work continues to identify the finite helium temperature and convective heat-transfer limitation as constraints on conversion. The project must therefore retain the exact source-supported 925/900/871 C point rather than generalising that any HTGR temperature is sufficient.

## Reliability and availability

The HTTR 50-day run establishes **stable high-temperature heat supply for a finite demonstration period**. It does not establish the annual availability of a commercial 600 MWth nuclear/chemical cogeneration plant.

The existing project's 85% annual availability originates from a design/economic screening assumption, not from an operating GTHTR300C fleet. It therefore must not be labelled a validated reliability value.

The coupled system has at least these availability contributors:

- reactor planned/forced outages;
- primary circulator and heat-removal systems;
- IHX / secondary-helium loop;
- reformer and chemical train;
- CO2 capture/compression;
- CO2 transport/storage service;
- shared utilities and grid support.

A single-reactor architecture creates a common process-heat interruption unless backup heat, storage, turndown or a second module is provided.

Peer-reviewed reliability work is stronger than a generic availability caveat. Herd, Lommers and Southworth (Nuclear Engineering and Design 251, 2012, 282-291) analyse HTR process-heat systems against a **99.9% process-heat availability requirement** and an n+2 redundancy check. They compare 600 MWth and 350 MWth HTRs with either additional HTRs or gas-fired boiler backup and conclude that substantial excess capacity is required; gas-fired backup is economically favoured across the studied demand range. Their loads are 200-1500 MWth, so the project's 176.8 MWth demand is just below their range and the exact redundancy result is not directly transferable. The methodological conclusion is transferable: industrial heat reliability can be materially stricter than single-reactor capacity factor.

Schroders, Verfondern and Allelein (Nuclear Engineering and Design 329, 2018, 234-246) independently evaluate nuclear-heated SMR and conclude that the nuclear heat-supply system should include a fossil backup heater; even a small amount of natural-gas backup can reduce hydrogen-production cost. This is useful counter-evidence to any assumption that a single nuclear heat source automatically meets industrial continuity requirements.

NEA/industrial-cogeneration literature likewise notes that existing fossil boilers/cogeneration units can serve as backup when HTGR heat is integrated into an industrial steam network, while industrial-scale flexibility/reliability remains to be demonstrated.

## What may be quantified next

A simple **availability sensitivity** is scientifically justified if it is clearly labelled a screening sensitivity:

A = annual effective availability fraction.

For fixed operating-point rates and no backup heat, annual quantities that accrue only while operating scale as:

H2(A) = H2_ref * A/A_ref

and likewise for operating emissions/abatement terms whose rate basis is unchanged.

However, **abatement cost does not necessarily scale identically**, because annualised reactor/CCS capital burdens can remain largely fixed while fuel savings and production-dependent T&S vary with operation. Review 07 therefore restricts the no-backup availability sensitivity to throughput/emissions: the repository does not invent a fixed-versus-variable cost decomposition, and no no-backup availability economics are exposed.

Backup-heat cases require their own fuel/emissions/cost terms. Literature now supports **gas-fired backup as an architecture option**, but a project sensitivity still requires a defensible heater efficiency and emissions factor before numerical results are added. No backup performance will be invented.

## Current disposition

- DF-01 HTGR ~900 C heat: **ANSWERED — SUPPORTED BY EXPERIMENT + SOURCE MODEL**.
- DF-02 sustained 950 C operation: **ANSWERED for 50-day HTTR demonstration; commercial-year extrapolation CONDITIONAL**.
- DF-03 176.8 MWth IHX lifetime: **REQUIRES EXPERIMENTAL / COMPONENT QUALIFICATION**.
- DF-04 scale-up gap: **ANSWERED / QUANTIFIED as evidence ladder**.
- DF-05 helium flow: **78.49 kg/s source-model value supported; piping pressure-drop/circulator sizing UNRESOLVED**.
- DF-06 temperature cascade: **ANSWERED at source-model level; local heat-transfer qualification unresolved**.
- DF-07 availability threshold: **READY FOR VERIFIED SCREENING SENSITIVITY**.
- DF-08 single-reactor continuity: **UNRESOLVED / likely requires backup or accepted outage strategy**.
- DF-09 backup strategy: **UNRESOLVED; fossil backup has literature precedent but project case not yet defined**.

## Core sources

- Iyoku et al. (2011), *High-Temperature Continuous Operation of the HTTR*, Transactions of the Atomic Energy Society of Japan 10(4), 290-300, DOI 10.3327/taesj.J11.020.
- JAEA, HTTR operating/test records.
- HTTR IHX structural-design paper, Atomic Energy Society of Japan 37(4), 316-326, DOI 10.3327/jaesj.37.316.
- Kunitomi et al. (2007), *JAEA's VHTR for Hydrogen and Electricity Cogeneration; GTHTR300C*, Nuclear Engineering and Technology 39(1), 9-20.
- Kato, Nishihara & Kunitomi (2007), GTHTR300C IHX design, DOI 10.3327/taesj.J06.023.
- INL TEV-961 (2010), *Sensitivity of Hydrogen Production via Steam Methane Reforming to High Temperature Gas-Cooled Reactor Outlet Temperature Process Analysis*.
- IAEA TECDOC-1645, *High Temperature Gas Cooled Reactor Fuels and Materials* / process-heat operating experience sections.
- Hoseinzade & Adams (2017), International Journal of Hydrogen Energy 42, 25048-25062, DOI 10.1016/j.ijhydene.2017.08.031.
- Schroders, Verfondern & Allelein (2018), Nuclear Engineering and Design 329, 234-246, DOI 10.1016/j.nucengdes.2017.08.007.


## Reliability evidence added after initial scale-up pass

### Herd et al. (2012)
**Question:** how much redundancy is needed when HTRs serve continuous industrial process heat?

**Method:** Monte Carlo availability analysis plus redundancy/failure-mode checks and economic comparison for 200-1500 MWth heat loads.

**Key assumptions:** 99.9% required process-heat availability; n+2 reliability criterion; 600 MWth or 350 MWth HTR modules; HTR or gas-fired backup.

**Finding:** substantial excess capacity is needed; gas-fired boiler backup is more economical than reactor-only redundancy over the studied range.

**Limitation for this project:** 176.8 MWth is below the paper's minimum 200 MWth demand and the selected plant is direct high-temperature reforming heat rather than generic process steam. The exact number of backup units/boilers is not imported.

**Project implication:** DF-08 changes from simply UNRESOLVED to **LITERATURE SUPPORTS NEED FOR REDUNDANCY/BACKUP; PROJECT CONFIGURATION UNRESOLVED**.

### Schroders et al. (2018)
**Question:** can nuclear heat economically replace fossil heat for SMR?

**Method:** energy-economic optimisation of fossil-, solar- and HTGR-heated steam methane reforming.

**Finding:** nuclear/solar heat systems benefit from fossil backup; nuclear heat is technically capable but continuity/economics improve with backup.

**Project implication:** a gas-fired trim/backup case is scientifically defensible as a future sensitivity, but it must carry its own CO2 and cost penalty.

### IAEA / JAEA demonstration status
IAEA's operating-nuclear hydrogen review reports that JAEA and MHI are pursuing a stepwise HTTR-to-SMR hydrogen demonstration programme with connection technologies targeted for confirmation around 2030. This is important maturity evidence: **the coupled nuclear-SMR system is still being demonstrated**, even though high-temperature reactor and component precedents already exist.


## Chinese commercial-operation counter-evidence

NNSA reported an HTR-PM operating event dated 15 January 2024: the maximum steam-generator heat-transfer-tube outlet steam-temperature deviation on Unit 1 exceeded the Final Safety Analysis Report requirement. The operator reduced reactor power and performed performance testing/maintenance; Unit 1 later shut down on 29 February and Unit 2 on 6 June.

NNSA classified the event preliminarily as **INES level 0**. Throughout the event:
- the plant remained in a safe state;
- all three safety barriers remained intact;
- there was no external radioactive release.

Why this matters:
- commercial HTGR operation is real evidence, not only design analysis;
- real operation still encounters component/temperature-distribution deviations and maintenance outages;
- this supports retaining explicit availability, inspection and component-performance uncertainty;
- it does **not** imply that HTR-PM is unsafe.

Transferability:
HTR-PM's steam generator differs from the project's IHX/reformer interface. The event is used as an operating-reliability/human-factors comparator, not as a project failure rate.
