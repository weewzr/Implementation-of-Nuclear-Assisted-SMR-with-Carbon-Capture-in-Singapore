# CN4252 final-design manuscript traceability

| Requirement | Final-design evidence |
|---|---|
| Proposed concept | INL/NGNP high-temperature HTGR-assisted SMR with CCS; JAEA GTHTR300C cogeneration hardware |
| Why Singapore | gas-dependent energy system, low-carbon hydrogen/decarbonisation context, conditional nuclear/CCS infrastructure |
| How emissions are reduced | fired reformer heat replaced by nuclear process heat + process CO2 capture + lower NG use |
| >0.25 MtCO2e/y | final lifecycle model approximately 0.917 MtCO2e/y avoided: PASS |
| <S$100/tCO2e | controlling zero-electricity-revenue case approximately S$3.725/tCO2e: PASS; no project electricity export credit is claimed |
| Assumptions controlling result | results/final_design/SOURCE_REGISTER.md and ASSUMPTIONS.md |
| Implementation constraints | Singapore implementation section: nuclear readiness, IHX/reformer qualification, CCS chain, electricity offtake |
| Evidence conclusion | independently verified conditional model result; not observed commercial feasibility |
| Reproducibility | model/src/final_design.rs + deterministic CSVs + sh paper/build.sh |

Historical Gate-5 0/64 and Review-5 600 C results remain repository provenance and are not the final submission design.


## Targeted-extension coverage

| CN4252 dimension | Current evidence |
|---|---|
| Technical feasibility | Sections 4--6: source-backed temperature/duty, Rust-generated thermal-capacity figure, helium/IHX screen |
| Nuclear safety | Section 5: HTTR demonstrated tests, TRISO evidence, retained accident mechanisms, IHX isolation, integration hazards |
| Chemical/process safety | Section 5: H2/CH4/CO/hot-pressure hazards, propagation, PHA/QRA limitations |
| Singapore nuclear feasibility | Section 10: no deployment decision, INIR/capability building, safety/regulatory/siting gates |
| CCS feasibility | Section 10: no suitable domestic geological storage, cross-border dependency and cost uncertainty |
| Economic feasibility | Sections 7 and 10: verified zero-credit threshold result separated from bankable project economics |
| Implementation roadmap | Section 10: staged evidence, regulatory, CCS, vendor, demonstration, licensing and FOAK decision gates |
| Overall feasibility | Section 10 evidence-status matrix: supported / conditional / unresolved / not demonstrated |
| Reproducible visualisation | Rust -> deterministic CSV/TikZ -> manuscript via model scripts and paper/build.sh |


## Final numerical-provenance audit coverage

| Audit requirement | Canonical evidence |
|---|---|
| No consequential orphan numbers | `results/final_design/MASTER_NUMBER_PROVENANCE_REGISTER.md` |
| Stable assumption identities | `results/final_design/ASSUMPTIONS.md` A-01 onward |
| Source vs assumption boundary | `results/final_design/SOURCE_REGISTER.md` numerical-audit clarifications |
| Stream/parameter table | Appendix B, Table `tab:stream-parameters` |
| Reactor temperature/power selection | Section 4 numbered temperature/helium equations + reactor evidence table |
| Annual H2 derivation | Section 7 ledger, Eq. `eq:annual-h2` |
| Direct/lifecycle CO2 derivation | Section 7 numbered carbon equations + lifecycle ledger |
| Cost derivation | Section 7 numbered cost equations + cost ledger |
| Original project-output traceability | `results/FINAL_ORIGINAL_PLAN_TRACEABILITY.md` |
| CN4252 final requirement traceability | Appendix B + `results/FINAL_ORIGINAL_PLAN_TRACEABILITY.md` |

The audit does not promote unavailable WGS/PSA/compression state points to project values and does not change the Review-07 scientific classification.
