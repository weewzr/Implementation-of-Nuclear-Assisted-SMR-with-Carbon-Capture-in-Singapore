# E1 Engineering-Economic Model Replacement

## Status and scope

Phase E1 replaces the **controlling economic method for future final-report integration**, while preserving the historical S$3.725/tCO2e screening bridge in `results/COST_DERIVATION_DEEP_FEASIBILITY.md` and `model/src/final_design.rs`.

E1 does not alter the verified emissions model. The denominator remains **917,138.90 tCO2e/y lifecycle avoided**. It does not begin the E2 matched-comparator attribution.

## 1. Frozen comparison boundary

- **Common H2 service:** 130 MMSCFD source service; same physical/emissions basis as the verified final-design model.
- **Availability:** 85% for annual process quantities.
- **Baseline represented cost:** conventional SMR natural-gas expenditure at 52.5 MMSCFD and the existing S$15/GJ screening price. Common conventional process equipment is treated as cancelling because no defensible equipment-level differential recost is available.
- **Candidate represented cost:** 34.0 MMSCFD natural gas + nuclear-source annual burden + aggregate IEAGHG capture-capital annualisation + CO2 T&S.
- **Project electricity revenue:** S$0/y. Remaining reactor thermal capacity is not converted to MWe or revenue.
- **Target monetary basis:** 2025 price basis, SGD reporting currency. Dated project FX is 1.2776 SGD/USD (29 Sep 2026 conversion basis already used by the repository).
- **Real/nominal convention:** real screening comparison; no nominal financing/tax model.
- **Annualisation:** 8%, 25-y CRF for source CAPEX comparison, consistent with the project screening convention. No NPV/IRR/payback is claimed.
- **CCS:** aggregate IEAGHG capture CAPEX retained; individual absorber/stripper/reboiler/compressor costs are not added.
- **T&S:** retained separately at S$15/tCO2 as an explicit screening assumption, not a contract.

## 2. Cost-year and currency normalization

### IAEA TECDOC 2075 nuclear cases

IAEA TECDOC 2075 (published 2024) states that the **year of assessment is 2021**. Table 73 reports:
- HTGR-200 + SMR: 4 x 200 MWth, NPP CAPEX USD2.065b, NPP O&M USD192m/y;
- MHR-T + SMR: 4 x 600 MWth, NPP CAPEX USD2.748b, NPP O&M USD324m/y.

The source MHR-T steam-reforming case states that reactor power for hydrogen production is only **part** of total reactor power. E1 therefore evaluates both full-module and conditional thermal-share accounting.

2021 USD is escalated to 2025 using the U.S. BEA annual GDP implicit price deflator:
- 2021 = 110.159;
- 2025 = 128.893;
- factor = 1.1700633.

The normalized USD value is then converted using 1.2776 SGD/USD.

This broad GDP deflator is a transparent screening normalization, not a nuclear-construction-specific index.

### IAEA TECDOC 1682 integration anchor

TECDOC 1682 Table 3.17 reports **USD69m** for the GTHTR300C IHX + secondary helium loop serving a 170 MW high-temperature process-heat architecture. The report says the estimate is preliminary and derived from HTTR construction cost.

The repository does not preserve the underlying estimate's exact price year. E1 therefore does **not** pretend that 2012 is the true cost year. For a bounded sensitivity only, the TECDOC publication year (2012) is used as an explicit normalization proxy:
- 2012 proxy GDP deflator = 93.1835;
- 2025 = 128.893;
- then 1.2776 SGD/USD.

This gives an annualised source anchor of approximately **S$11.423m/y** at 8%/25 y. Linear duty scaling from 170 to 176.8 MWth gives **S$11.880m/y**, but that scaled value is a sensitivity, not a preferred estimate.

Because modern IAEA NPP CAPEX scope may already contain the process-heat interface, E1 does **not** stack this USD69m block on top of the modern NPP cases. It is tested separately as a replacement for the historical generic 10% integration/site allowance.

## 3. Nuclear economic cases

### Central: one 600 MWth MHR-T module

The proposed design basis is one 600 MWth-class reactor. The IAEA MHR-T source is four 600 MWth modules. E1 divides the source NPP CAPEX/O&M equally by four to obtain a one-module accounting burden; this is an accounting approximation, not a vendor quote.

Normalized and annualised one-module burden:
**S$217.291m/y**.

No co-product is credited with the remaining reactor cost because the project lacks a validated project power/output model or demonstrated external cost bearer.

### Alternative: three HTGR-200 modules

Three 200 MWth modules are required to match a 600 MWth reactor-scale basis. The corresponding normalized annual NPP burden is:
**S$432.145m/y**.

This is strongly adverse and is not the central architecture.

### Conditional shared/cogeneration allocations

For falsification, E1 also evaluates simple process-heat shares:
- MHR-T: 176.8/2400 of the four-module source annual NPP cost = **S$64.028m/y**;
- HTGR-200: 176.8/800 = **S$127.339m/y**.

These are **conditional accounting cases only**. They require the remaining plant cost to be borne by real, valuable co-products/users. E1 does not infer such revenue or output from 423.2 MWth remaining thermal capacity.

## 4. Integration/IHX treatment

The old 10% integration/site allowance is no longer used in the E1 central case.

Two tests are preserved:
1. the TECDOC-1682 USD69m IHX + secondary-loop source anchor and its linear-duty sensitivity;
2. a historical-Nishihara case in which the old 10% integration/site allowance is explicitly **replaced** by the source-specific TECDOC-1682 annualised anchor.

The historical replacement case rises from the old S$3.725/tCO2e to approximately **S$13.065/tCO2e**, showing that the source-specific integration evidence is materially larger than the old generic allowance while still leaving that historical nuclear-cost case below S$100/t.

The modern central case does not add the USD69m block because overlap with modern NPP CAPEX cannot be excluded. This is a double-counting control, not an assumption that integration is free. Project-specific reformer modification, helium piping/circulator and site/EPC integration remain unresolved.

## 5. CCS treatment

E1 retains the existing aggregate IEAGHG capture-capital method:
- captured tonnes are scaled against the IEAGHG reference;
- 2014 EUR is normalized by the existing German/euro-area deflator proxy and dated EUR/SGD conversion;
- capital is annualised at 8%/25 y.

Central annual capture-capital burden:
**S$10.683m/y**.

No CAPCOST/DFP absorber, stripper, reboiler, condenser or compressor is added because that would partially double count the aggregate capture block.

CO2 T&S remains separate:
**S$8.135m/y** at S$15/tCO2.

## 6. Chemical-equipment and OPEX completeness

### Adopted
- baseline/candidate natural gas expenditure;
- nuclear-source CAPEX annualisation and source O&M;
- aggregate capture CAPEX annualisation;
- T&S screening tariff.

### Not adopted because required design inputs are missing
- helium-heated reformer modification;
- nuclear/process IHX bottom-up equipment cost;
- secondary-He circulator/piping;
- WGS vessel;
- PSA package;
- amine tower/reboiler/condenser bottom-up replacement;
- CO2 compressor bottom-up replacement;
- pumps/storage vessels;
- detailed steam/cooling utilities;
- operating labour;
- maintenance/repairs/operating supplies;
- laboratory/supervision;
- insurance/taxes/overhead;
- solvent/catalyst replacement;
- working capital.

The CAPCOST/DFP/CCEP correlations are therefore not forced into E1. The result is not called a complete COM/COMd or TCI.

## 7. Deterministic results

At S$15/GJ natural gas and S$15/tCO2 T&S:

| Case | Annual baseline (S$/y) | Annual candidate (S$/y) | Net incremental (S$/y) | S$/tCO2e | CN4252 cost test |
|---|---:|---:|---:|---:|---|
| **MHR-T one 600 MWth module — central** | 269.115m | 410.393m | **+141.278m** | **154.042** | **FAIL** |
| HTGR-200 three modules | 269.115m | 625.248m | +356.132m | 388.308 | FAIL |
| MHR-T thermal-share — conditional | 269.115m | 257.131m | -11.985m | -13.067 | PASS before unresolved omissions |
| HTGR-200 thermal-share — conditional | 269.115m | 320.441m | +51.326m | 55.963 | PASS before unresolved omissions |
| Legacy Nishihara + source-specific integration replacement | 269.115m | 281.097m | +11.982m | 13.065 | historical sensitivity only |

A negative abatement cost in the conditional MHR-T shared case means represented annual savings exceed represented added costs; it is **not** a claim of bankable profit.

## 8. Threshold falsification

### Central unresolved-cost margin

For the central MHR-T one-module case:
- CN4252 maximum net annual cost at S$100/t = **S$91.714m/y**;
- represented net incremental cost = **S$141.278m/y**;
- signed additional-cost margin = **-S$49.564m/y**.

Therefore the central case has **no positive additional unresolved-cost allowance**. It already exceeds the S$100/t ceiling before reformer modification, helium-loop project BOP, labour, site/EPC and other unresolved costs.

### Natural-gas break-even

Holding central nuclear/CCS/T&S treatment fixed, the natural-gas price required to reach exactly S$100/t is approximately:
**S$22.840/GJ**.

This is a threshold calculation, not a forecast.

### T&S break-even

At S$15/GJ natural gas, the algebraic T&S tariff required for the central case to reach S$100/t is approximately:
**-S$76.385/tCO2**.

Because a negative T&S tariff is not a normal project cost, this demonstrates that reducing T&S to zero alone cannot rescue the central case.

## 9. Double-counting controls

- Modern IAEA NPP burden replaces the historical Nishihara source-product bridge in the central E1 result.
- TECDOC-1682 integration is not stacked on modern NPP CAPEX where interface inclusion is unresolved.
- In the legacy integration test, TECDOC-1682 replaces the old 10% allowance.
- IEAGHG aggregate capture CAPEX is retained without adding individual capture equipment.
- Project electricity revenue remains zero.
- No common conventional equipment is re-added without a matched differential design basis.

## 10. Interpretation and candidate controlling result

**Candidate controlling E1 economic result for later E8 manuscript integration: S$154.0/tCO2e (central one-module MHR-T source case), which FAILS the CN4252 S$100/tCO2e economic threshold.**

This is a substantive change from the historical S$3.725/tCO2e bridge. The difference is driven primarily by replacing the favourable historical source-product economic allocation with a modern IAEA source CAPEX/O&M burden assigned to the physically matched 600 MWth module without unsupported co-product cost sharing.

The economic conclusion is architecture/accounting-sensitive:
- dedicated one-module central treatment: FAIL;
- three-module HTGR-200 alternative: FAIL;
- shared/cogeneration allocations: can remain below S$100/t **only conditionally**, before unresolved project-specific costs, and only if the remaining common plant burden is demonstrably carried by other products/users.

E1 therefore does not conclude that every possible shared nuclear architecture fails. It concludes that the current project cannot claim <S$100/t on a standalone 600 MWth-module economic boundary using the stronger source case.

## 11. Historical result preservation

The S$3.725/tCO2e result remains untouched in the historical final-design Rust path and derivation record. It is preserved for audit/reproducibility but is **superseded as the candidate future final-report controlling economic result** by E1.

The active manuscript is intentionally not rewritten in E1; E8 will perform controlled reintegration after E2--E7.

## 12. Remaining unresolved cost categories

Material unresolved categories include:
- project-specific reformer conversion/retrofit;
- project IHX qualification/design cost;
- secondary-He circulator/piping;
- Singapore site/EPC/owner costs;
- licensing/security/emergency-planning infrastructure;
- waste/spent-fuel/decommissioning/liability;
- detailed capture O&M/solvent/catalyst;
- labour/maintenance/overhead;
- cross-border CCS contracted tariff;
- financing/schedule/FOAK premium.

These omissions do not rescue the central case because the central represented cost already fails the threshold. They can materially worsen it. For conditional shared cases, they can consume the positive margin and therefore remain decision-critical.

## 13. Reproducibility

Canonical E1 code:
- `model/src/e1_economics.rs`.

Deterministic generator:
- `model/src/bin/e1_engineering_economics.rs`.

Machine-readable outputs:
- `results/e1_economics/e1_input_provenance.csv`;
- `results/e1_economics/e1_replacement_ledger.csv`;
- `results/e1_economics/e1_cases.csv`;
- `results/e1_economics/e1_thresholds.csv`.

Regression tests verify normalization, module allocation, central threshold failure, conditional shared case behavior, source-specific integration replacement, threshold sensitivities and preservation of the verified emissions denominator.
