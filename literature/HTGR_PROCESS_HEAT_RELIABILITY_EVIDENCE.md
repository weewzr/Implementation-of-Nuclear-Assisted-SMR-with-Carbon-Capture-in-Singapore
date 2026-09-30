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

Peer-reviewed energy/economic work on nuclear/solar-heated SMR (Schroders et al., NED 329, 2018) explicitly concludes that the heat-supply system should include a fossil backup heater. This is useful counter-evidence to any assumption that a single nuclear heat source automatically meets industrial continuity requirements.

## What may be quantified next

A simple **availability sensitivity** is scientifically justified if it is clearly labelled a screening sensitivity:

A = annual effective availability fraction.

For fixed operating-point rates and no backup heat, annual quantities that accrue only while operating scale as:

H2(A) = H2_ref * A/A_ref

and likewise for operating emissions/abatement terms whose rate basis is unchanged.

However, **abatement cost does not necessarily scale identically**, because annualised reactor/CCS capital burdens can remain largely fixed while fuel savings and production-dependent T&S vary with operation. The Rust sensitivity must therefore recompute fixed and variable ledger terms separately rather than divide the base result by availability.

Backup-heat cases require their own fuel/emissions/cost terms and will not be invented until a defensible backup configuration and efficiency are sourced.

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
