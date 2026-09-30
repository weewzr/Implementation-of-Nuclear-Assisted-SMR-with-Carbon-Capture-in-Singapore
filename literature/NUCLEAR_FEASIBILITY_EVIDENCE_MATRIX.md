# Nuclear / Feasibility Evidence Matrix — Targeted Extension

Purpose: claim-level evidence map for the reopened CN4252 extension. Categories distinguish DEMONSTRATED, DESIGNED, MODELLED, PROPOSED and PROJECT ASSUMPTION.

| Question / claim | Evidence | Evidence class | What it supports | Limitation / manuscript implication |
|---|---|---|---|---|
| HTGR high-temperature capability | JAEA HTTR operating record | DEMONSTRATED | 30 MWth HTTR achieved 950 C outlet at full power; 50-day high-temperature/full-power operation | Demonstrates reactor-class temperature, not 600 MWth commercial deployment |
| Low power density / graphite thermal inertia | JAEA HTTR specifications; HTTR LOFC analysis | DEMONSTRATED + ANALYSED | HTTR average power density 2.5 MW/m3; graphite heat capacity gives slow transients | Do not generalise exact transient response to GTHTR300C without design analysis |
| Negative temperature feedback under LOFC | HTTR 9 MW LOFC test / Takamatsu analysis | DEMONSTRATED + ANALYSED | Circulator trip without control-rod insertion led power toward decay-heat level; negative feedback is physically relevant | Test was 9 MW, not final 600 MWth project |
| TRISO retention | INL AGR programme irradiation + post-irradiation safety tests | EXPERIMENTAL | UCO TRISO qualification data include irradiation and 1600-1800 C safety tests; fission-product retention/failure statistics are measured | TRISO is not failure-proof; higher-temperature tests show SiC/coating failures can occur |
| TRISO manufacturing maturity | INL AGR-5/6/7 fabrication | ENGINEERING-SCALE EXPERIMENTAL | Engineering-scale fabrication and qualification programme exists | Not proof of Singapore supply chain or chosen vendor qualification |
| IHX isolation role | JAEA Technology 2018-004; IAEA nuclear-hydrogen coupling reports | CONSTRUCTED + TECHNICAL BASIS | IHX separates primary and secondary helium, limits contamination transfer and disturbance propagation | Final project IHX size/detailed design is not completed |
| HTTR IHX temperature/material | HTTR IHX development / structural design papers | CONSTRUCTED + TESTED | 10 MW He-He IHX, Hastelloy XR tubes >900 C; creep, creep-fatigue, seismic, thermal-hydraulic and inspection R&D | Scale-up to 176.8 MWth requires engineering qualification |
| High-temperature material constraints | JAEA Hastelloy XR creep/helium chemistry work | EXPERIMENTAL | Creep and corrosion/helium impurity control are genuine design constraints at 800-1000 C | Materials feasibility is conditional, not automatic from temperature match |
| Nuclear/process dynamic coupling | IAEA TECDOC-1614 / JAEA programme | DESIGNED + TEST PROGRAMME | Return-helium fluctuations can perturb reactor; control/isolation technology required | Integrated commercial nuclear-SMR operation not demonstrated |
| Chemical-to-nuclear hazards | NRC NGNP PIRT; IAEA coupling literature | SAFETY ANALYSIS | Separation distance, inventories, blast/fire/toxic releases and different nuclear/chemical safety philosophies must be addressed | Screening paper cannot replace site-specific PHA/QRA/nuclear safety case |
| Air ingress / graphite oxidation | IAEA/JAEA HTGR safety literature | SAFETY ANALYSIS + R&D | Depressurisation/air ingress can oxidise graphite; oxidation is a retained accident mechanism | Do not describe HTGR as risk-free or 'meltdown impossible' |
| Singapore nuclear status | MTI/EMA 2026 | AUTHORITATIVE CURRENT POLICY | No deployment decision; INIR Phase 1 from 2027; safety, waste, emergency planning among readiness areas | Regulatory/siting readiness remains unresolved |
| Singapore safety criterion | MTI 5 May 2026; EMA 1 Jun 2026 | AUTHORITATIVE CURRENT POLICY | Any decision considers safety, reliability, affordability, sustainability; dense city-state makes safety overriding | Numerical CN4252 pass cannot establish nuclear feasibility |
| Singapore CCS storage | MTI Carbon page 2026 | AUTHORITATIVE CURRENT POLICY | Singapore lacks suitable geological storage and is pursuing cross-border CCS | Storage contract/infrastructure is a binding external dependency |
| Cross-border CCS progress | Singapore-Indonesia MOU 2025; Malaysia MOU 2025 | AUTHORITATIVE AGREEMENT | Government-to-government cooperation is advancing | MOU is not an operating transport/storage chain or tariff |
| CCS cost status | MTI PQ 8 Apr 2025 | AUTHORITATIVE CURRENT POLICY | Government still studying full capture/transport/storage value chain and costs | Project S$15/t T&S remains screening assumption |
| OUTRAM PARK method | public Rust workspace README/V&V records | SOFTWARE METHODOLOGY | Explicit V&V labels, evidence tiers, deterministic Rust components and testable outputs are useful patterns | Not scientific validation of this project's model |

## Key synthesis

1. **Supported:** temperature compatibility and screening heat-capacity compatibility.
2. **Supported with demonstrated precedent:** helium cooling, high-temperature IHX operation, negative-feedback/thermal-inertia safety features and TRISO experimental performance.
3. **Conditional:** scale-up of IHX/materials, integrated nuclear/chemical transients, site separation, process-to-nuclear hazard control.
4. **Unresolved for Singapore:** licensing/siting/emergency-planning acceptability, project-specific nuclear safety case, bankable FOAK economics, and operating cross-border CCS service.
5. **Not demonstrated:** a commercial 600 MWth GTHTR300C nuclear-SMR hydrogen plant in Singapore.

This matrix supplements, rather than replaces, `literature/RESEARCH_MATRIX.md`.
