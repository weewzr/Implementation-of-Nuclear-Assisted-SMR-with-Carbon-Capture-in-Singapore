# Nuclear-Assisted Steam Methane Reforming + Carbon Capture in Singapore

**Can high-temperature nuclear process heat make established hydrogen production cleaner at industrial scale—and, eventually, economically credible in Singapore?**

Steam methane reforming (SMR) is an established route for manufacturing hydrogen, but methane is used both as chemical feedstock and as fuel for the high-temperature reformer. Carbon capture and storage (CCS) addresses much of the process CO2; it does not eliminate upstream methane emissions or guarantee permanent storage. This project investigates a more demanding intervention: replace fired reformer heat with high-temperature nuclear heat, retain the familiar SMR-H2+amine-CCS chemical process, and test the result against Singapore's CN4252 abatement and cost thresholds.

**The result is deliberately conditional.** The preferred future architecture has attractive *modelled mature* economics, but the first-of-a-kind (FOAK) configuration fails the cost target, and Singapore siting, licensing, nuclear/chemical safety, cooling and contracted CO2 storage remain unresolved. Nuclear-assisted SMR+CCS is the **proposed solution**; conventional SMR+CCS is the **non-nuclear comparator**, not a replacement project.

## Start here: the project in two minutes

**What does the plant do?** Natural gas and steam react to make hydrogen. Carbon capture removes much of the CO2 generated in that process, while a high-temperature nuclear reactor would supply heat that is normally produced by burning additional gas.

**Why two trains?** The literature-based chemical train needs 176.8 MWth of heat. Two identical trains need 353.6 MWth, which fits inside the roughly 370 MWth process-heat branch of the reference 600 MWth reactor concept. This is the best-supported configuration found, **not** a proven global optimum.

**How is success measured?** First calculate annual avoided lifecycle emissions: baseline minus candidate. Next calculate annual incremental cost: candidate minus baseline. Divide incremental cost by avoided tonnes. The model predicts ~1.834 MtCO2e/y avoided and S$42.84/t in a mature 10-OAK scenario, but S$137.74/t at FOAK, which **fails** the specified cost limit.

**What is still unknown?** The site, licensing, integrated safety, emergency-planning zone (EPZ), detailed heat rejection, hydrogen offtake and permanent CO2 storage must be demonstrated. The model is not a permit or an operating plant.

To read the mathematics, begin with the manuscript's **How to read this paper** guide, then follow the process and carbon sections before the equation-first economics section.

## Controlling architecture and results

One **600 MWth GTHTR300C-class high-temperature gas-cooled reactor (HTGR)** supplies heat through a primary-helium/intermediate-heat-exchanger (IHX)/secondary-helium interface to **two 130 MMSCFD SMR-H2+CCS trains**, each requiring 176.8 MWth. This is a **project-proposed, literature-anchored architecture**, not a built Singapore facility or an exact published JAEA design.

| Quantity | Preferred two-train screening result |
|---|---:|
| Reformer process heat | 353.6 MWth |
| Hydrogen production (85% availability basis) | ~195,892 tH2/y |
| Captured CO2 throughput | ~1.085 MtCO2/y |
| Lifecycle emissions avoided (E2B boundary) | ~1.834 MtCO2e/y |
| Two-train FOAK cost | **S$137.74/tCO2e — FAIL** |
| Early-commercial/BOAK cost | **S$74.14/tCO2e — projected/modelled PASS** |
| Preferred mature 10-OAK cost | **S$42.84/tCO2e — projected/modelled PASS** |

The originating screening criteria are **>0.25 MtCO2e/y** and **<S$100/tCO2e**. Future projected numerical passes are **not** present-day commercial or licensing feasibility. E2's separately bounded conventional SMR+CCS comparator avoids ~0.461 MtCO2e/y on its matched-service screen, so nuclear is not necessary *merely* to exceed the minimum abatement target. The added nuclear intervention must justify its capital, interfaces and risks.

The post-E8 architecture-sizing assessment **retains two identical trains as the best-supported choice, not a proven global optimum**. The 353.6 MWth process duty occupies ~95.6% of the rounded 370 MWth source process-heat branch but ~58.9% of the full reactor thermal rating. The conditional 246.4 MWth full-power residual is **not** a measured cooling duty or decay heat. No electricity/co-product revenue is credited.

## Feasibility and evidence maturity

- **Process:** INL Case-6 operating conditions and source-model balances anchor each train; doubling is an explicit project assumption.
- **Economics:** INL/DOE-inspired FOAK-to-mature cost learning is a projected method, not vendor pricing or observed construction performance.
- **Jurong Island:** credible industrial-integration *context*, **not** a selected or approved nuclear site.
- **Safety:** HTTR/TRISO/IHX evidence supports source-technology mechanisms; project integrated safety is conditional. PRA, mechanistic source term, tritium, chemical QRA, nuclear/chemical separation, dose and emergency-planning zone (EPZ) remain unquantified or site-specific.
- **Infrastructure:** hydrogen offtake, NG, normal/safety cooling and permanent transport/storage for ~1.085 MtCO2/y must close before deployment.

**Overall verdict:** current/FOAK **not deployment-feasible as demonstrated**; preferred mature architecture **conditional future feasibility**.

## Read the research

| Start here | Purpose |
|---|---|
| [Current research state](STATUS.md) | Current controlling gates, unresolved questions and provenance |
| [LaTeX manuscript](paper/main.tex) | Canonical paper; PDF is generated, not separately maintained |
| [CN4252 synthesis](results/E6_CN4252_SYNTHESIS.md) | Assignment thresholds and stage-specific verdict |
| [Architecture maturation](results/E2B_ARCHITECTURE_MATURATION.md) | Two-train design and FOAK/BOAK/10-OAK economics |
| [Sizing reoptimization](results/POST_E8_ARCHITECTURE_SIZING_REOPTIMIZATION.md) | Why two trains remain preferred; alternatives and limits |
| [Jurong feasibility](results/E3_JURONG_SITING_COOLING.md) | Site context, cooling and infrastructure |
| [Integrated safety](results/E4_INTEGRATED_SAFETY_CASE.md) | Hazards, barriers and safe states |
| [Quantitative safety gaps](results/E5_QUANTITATIVE_SAFETY_DEPTH.md) | PRA/QRA/source-term requirements, no fabricated risk |
| [Implementation roadmap](results/E7_IMPLEMENTATION_ROADMAP.md) | Decision gates, stop/redesign conditions |
| [Model code](model/src/) | Reproducible Rust calculations |
| [Visual source plan](results/FINAL_VISUAL_SOURCE_PLAN.md) | Traceable base images/templates; not final approved artwork |

## Reproduce

From a fresh repository checkout with Rust/Cargo and a suitable LaTeX toolchain:

```bash
sh paper/build.sh
```

This runs model tests, regenerates manuscript data and compiles `paper/main.tex` to `paper/main.pdf`. See [paper build instructions](paper/README.md) and GitHub Actions **Research CI** / **Paper and reproducibility** for verification. The canonical manuscript is LaTeX; do not edit a generated PDF independently.

## Historical results, course origin and provenance

The earlier **one-train** screen produced ~97,946 tH2/y, ~917,139 tCO2e/y and an optimistic **S$3.725/tCO2e differential**. Those numbers are preserved for historical traceability, **not** the controlling final economics. The stronger E1 one-module accounting gave ~S$154/tCO2e (FAIL), and E2B subsequently developed the source-utilising two-train architecture above.

Historical review, experiments, assumptions, source registers and model outputs remain in the repository. Their presence documents the research path; it does not supersede the current E2B–E8 conclusions.

## Scope and contribution

This is a reproducible academic engineering feasibility study, not a reactor design approval, investment memorandum or operating safety case. Contributions should preserve citations, distinguish literature from project-derived quantities, run the Rust and paper workflows, and never replace an adverse result merely to improve the narrative.
