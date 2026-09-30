# Independent Review 5 resolution

Status: **CLOSED**.

This resolves findings from `reviews/review_05_deployment_scale_technoeconomic.md`; it does not repeat Review 5.

## R5-B01 — deployment heat interface
**Correction:** removed the pre-review 950 -> 920 -> 900 C deployment assumption. Primary JAEA HTTR steam-reforming evidence is now represented explicitly as 950 C reactor primary outlet, 900 C secondary-helium IHX outlet for the GTHTR300C heat-delivery benchmark, 880 C helium at the steam-reformer inlet after transport loss, and 600 C reacting-gas outlet (JAEA HTTR/mock-up design state). Automated checks require positive IHX and reformer hot-end approaches.

**Code/model:** `model/src/deployment.rs` re-solves recycle/reforming/WGS/PSA/capture at the 600 C process-gas outlet rather than forcing the old fixed-H2 900 C state.

**Numerical effect:** at scale 1.0 the re-solved plant produces 23,620.918 tH2/y and -450,167.160 tCO2e/y avoided; at the diagnostic 1.50 scale it produces 35,431.378 tH2/y, requires 66.704 MWth process heat, and gives -675,250.740 tCO2e/y avoided. Therefore the minimum scale for >0.25 MtCO2e/y is infinite/non-existent; the former 1.30-scale passing construction is superseded.

**Evidence:** JAEA-Technology 2018-004; JAEA-Technology 2007-022; JAERI-Tech 99-080; GTHTR300C primary design literature.

**Verification:** Rust tests require 880 C helium inlet, 600 C process outlet, >=50 K primary/IHX hot-end approach, >=40 K helium/process approach, duty below 170 MW IHX capacity, and preservation of the original 64/0 Gate-5 result.

**Acceptance:** PASS.

## R5-B02 — one-module GTHTR300C mature economics
**Correction:** removed the later four-unit 0.7 JPY/MJ value as the primary one-module basis. The mature case now reconstructs the primary 2007 GTHTR300C cogeneration study: 59.7 bn JPY total plant, including 11.2 bn JPY IHX+secondary loop; 0.52 JPY/MJ heat; 4.9 JPY/kWh electricity; 85% availability. The doubled-IHX sensitivity uses the source 70.9 bn JPY / 0.57 JPY/MJ / 5.5 JPY/kWh case.

**Economic closure:** full reactor burden is source-priced heat plus source-priced electricity. Electricity value is then credited separately; no reactor capacity is free. Source cogeneration output is 202 MWe at 170 MWth process heat, with the lower-heat electric branch bounded by the published JAEA operating envelope.

**Numerical effect:** the source mature plant normalizes to S$546.283 million on the declared 2025-price/dated-FX screening basis. At the 1.50 diagnostic scale, process heat is 66.704 MWth, gross/net electric output is 246.964 MWe and annual electricity is 1,838,894.280 MWh/y. Zero-value, S$100/150/200-MWh, doubled-IHX, modern-central and FOAK/adverse cases all have infinite/undefined S$/t because lifecycle abatement is non-positive. The former S$47.637/t conditional pass is superseded.

**Acceptance:** PASS.

## R5-M01 — common cost year
**Correction:** created `results/generated/review5_cost_ledger.csv`. Historical anchors are normalized to 2025-price basis using explicit national GDP-deflator ratios, then converted at explicitly dated 29 Sep 2026 FX. JAEA uses Japan 2007->2025; IEAGHG uses Germany/euro-area proxy 2014->2025. INL 2024 inputs use a 2024->2025 US screening escalation.

**Caveat:** broad GDP deflators are transparent screening approximations, not nuclear-specific construction indices.

**Verification:** representative escalation/FX conversion is regression-tested.

**Acceptance:** PASS.

## R5-M02 — cogeneration closure
**Correction:** generated `review5_cogeneration.csv` reports reactor MWth, process heat, remaining thermal service, gross/net MWe, annual MWh, full reactor economic burden, zero-value case and Singapore electricity-value context. EMA 2025 USEP S$100--200/MWh is evaluated at 100/150/200 S$/MWh plus zero-value surplus.

**Break-even:** no finite CN4252 break-even electricity value exists when avoided emissions are non-positive; it is represented as infinity rather than reverse-engineered.

**Acceptance:** PASS.

## R5-M03 — abatement margin
**Correction:** scale sensitivity explicitly evaluates 1.0, 1.30, 1.35, 1.40 and 1.50. The model no longer selects 1.30 because it happened to sit just above the old threshold.

**Numerical effect:** all scales retain non-positive specific abatement, so no finite minimum deployment scale or positive threshold margin exists.

**Acceptance:** PASS.

## Review-5 scientific decision
The **previous deployment conditional pass does not survive remediation**. This is not a failure of the remediation: it is the required falsification result after replacing the unsupported heat interface and incomplete economic allocation with source-supported physics and closed cogeneration accounting.

Verification evidence: dedicated Research CI run `36655965073` PASS at repository state `f39fb2d8b2f843f757c1a823d7523cbc2cb99ba5`. Paper/reproducibility run `36655616510` PASS at corrected model/manuscript state `649398e14be3a5a78e255d3988ccbcb6b6248025`; artifact `11072086648`, SHA-256 `868f83c11cd84d523ee02789c242832c198c5769f427d5cbda546e8088590198`; 18-page PDF visually inspected, including historical-vs-remediated temperature labels, cogeneration table, cost ledger and scenario/scale tables. All previously stated Review-5 BLOCKER and MAJOR acceptance criteria remain satisfied.

**Independent Review 5 gate: CLOSED.**
