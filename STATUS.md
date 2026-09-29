# STATUS

## Current research gate
Gate 3 — Mathematical/model foundation; Independent Review 1 gate OPEN

## Current scientific question/task
Verify and propagate the newly implemented iterative fixed-H2 tail-recycle closure into fresh-NG energy, capture mass, lifecycle emissions and the S$100/t boundary. Do not treat numerical convergence as rigorous flowsheet validation: conversion and PSA-recovery coefficients remain explicit reduced-model assumptions.

## Latest implementation
- Added `iterative_tail_recycle_fixed_h2` to `model/src/lib.rs`.
- The solver repeatedly removes tail CO2, recycles H2/CO/CH4, converts recycled CO/CH4, applies the source-reconstructed PSA H2 recovery, adjusts fresh NG to hold H2 product fixed, regenerates the tail, and iterates to a fixed point.
- Added regression tests for convergence, recovery of the once-through limit when recycle/conversion are zero, and monotonic fresh-NG reduction with stronger conversion/recovery.
- Commit: `b50a451f22ee1a52456cfa7cfcafa5642fc9a944`.
- GitHub workflow status could not yet be verified through the connector because its commit-run endpoint exposes pull-request-triggered runs only and returned no run for this direct main-branch commit.
- Acceptance remains pending until the Rust tests are independently executed and the converged state is propagated through lifecycle/economic calculations.

## Analytical recycle benchmark added
An independent algebraic fixed-point benchmark has now been derived and encoded for the reduced recycle equations. For positive recycle/conversion coefficients, the reduced model gives

s = P / (P + H_tail + CO_tail + 4 CH4_tail),

where s is fresh-NG fraction of the IEAGHG baseline and P is fixed H2 product.

Using the rounded IEAGHG source streams gives s ~= 0.7371, fresh NG ~= 1073.1 kmol/h and fresh-feed energy ~= 249.7 MW_LHV. Thus the reduced fixed point displaces ~26.3% of fresh feed. At 80% CO conversion, CH4 conversion and recycle-H2 recovery, the corresponding analytical circulating tail is approximately 561.3 kmol/h H2, 282.2 kmol/h CO and 183.4 kmol/h CH4; the current algebraic capture expression is ~1163.5 kmol/h CO2 (~51.2 t/h).

This is a correction to interpreting a one-pass 80% sensitivity as an 80% fresh-feed displacement. In the reduced steady-state equations, positive conversion/recovery coefficients primarily change circulating inventory; the net fresh-feed fixed point is set by the source tail's H2-equivalent inventory. This is a model property, not yet a validated physical PSA/reformer result.

Commit `622856765a07f7f58d0133a33df84e2478c57925` adds the analytical benchmark and numerical-vs-analytical regression tests.

Execution verification remains pending: the available container has no Rust compiler and no outbound GitHub access, while the GitHub connector does not expose direct-main workflow runs. No test-pass claim is made.

## Converged recycle lifecycle and full-cost result
The fixed-point recycle implementation and its analytical benchmark are now CI-verified. Rust CI run 256 passed after two scientifically incorrect legacy test expectations were corrected; subsequent lifecycle and full-cost propagation runs 257-260 also passed.

Using the same explicit reference assumptions as the prior 80% sensitivity (90% capture, 11.5 gCO2e/MJ upstream NG, 162 MWth nuclear service, 5.5 gCO2e/kWh-e nuclear LCA proxy at 50.4% efficiency, 2.5% CCS-transport sensitivity), but replacing the one-pass recycle approximation with the verified fixed point:
- fresh-NG fraction = ~0.73711;
- fresh NG = ~1073.09 kmol/h;
- fresh-feed energy = ~249.74 MW_LHV;
- total NG displacement including eliminated supplementary furnace fuel = ~145.01 MW_LHV;
- unabated lifecycle screen = ~10.813 kgCO2e/kgH2;
- converged candidate lifecycle screen = ~1.897 kgCO2e/kgH2;
- specific lifecycle abatement = ~8.916 kgCO2e/kgH2;
- annual abatement at 8994 kgH2/h and 8322 h/y = ~0.667 MtCO2e/y;
- corresponding S$100/t annual incremental-cost allowance = ~S$66.74m/y.

For the existing representative full-cost assumptions (162 MWth, S$5.69/GJ nuclear heat, S$150/MWh separation electricity, S$50m/y allocated reactor cost, S$8.2m/y IHX/loop, S$5m/y other integration, and S$31.9m/y low Group-A-like CCS T&S), the corrected minimum NG value is ~S$14.70/GJ.

At NG=S$15/GJ, maximum compatible CCS T&S is only ~S$33.21m/y; at S$20/GJ it rises to ~S$54.93m/y. Therefore the representative shared-reactor case remains inside the S$100/t screen only in a constrained favourable region. The low-T&S/S$15 case is marginal; the high-T&S case requires materially higher gas value or lower other costs.

These are screening results, not a bankable cost estimate. The largest remaining scientific weakness is no longer recycle arithmetic; it is validation of the reduced recycle/reformer/PSA surrogate and the assumed 162 MWth integrated heat requirement under the changed recycle composition.

## Recycle-adjusted HTGR heat-service bound
The prior 162 MWth economic midpoint is no longer treated as a validated point duty. A literature cross-check confirms that JAEA's HTTR steam-reforming system distributes secondary-helium heat across the steam reformer, superheater and steam generator, and current JAEA work treats the coupled plant as a dynamic thermal-hydraulic system rather than a simple fresh-feed-scaled heater.

A new CI-verified bound therefore makes only the correction supported by the present reduced model:
- fresh-feed methane reaction heat removed according to the converged 26.3% fresh-NG displacement;
- recycled CO/CH4 reaction heat added from the fixed-point converted flow;
- the existing source-bounded MDEA incremental-heat interval retained;
- source-anchored furnace services are NOT multiplied wholesale by the 0.737 fresh-feed fraction.

At the 625 C reformer-inlet sensitivity, the resulting controlled envelope remains in approximately the mid-140s to high-170s MWth range. CI confirms the reaction-heat correction is a net saving (broadly -20 to -5 MW) and the final bounded service remains >140 MW and <200 MW.

This result is intentionally conservative in interpretation: recycle sensible heating, changed steam generation, pressure drop/compression, and altered syngas heat recovery are not yet closed. Therefore 162 MWth remains usable only as a representative midpoint inside the current bounded range, not as a solved integrated duty.

Commits `21185e05294de8c869ebcdb6ebfb5b54c8230279` and `40bcb9db43fb7e86890c1e4ddb5e47b4f83c0cec` implement and regression-lock this bound.

## Recycle pressure topology fixed for screening
Primary/source literature now constrains the recycle pressure architecture more tightly.

IEAGHG Case 2A states that low-pressure PSA tail gas is compressed to around 10 bar (~1 MPa) to enable MDEA CO2 capture. Independent IEAGHG supporting literature notes ~0.3 barg as typical for an SMR PSA tail-gas side. JAEA's HTTR hydrogen-production design specifies process gas at ~4.5 MPa. Therefore a direct tail-gas-capture-and-recycle architecture cannot treat the Case-2A compressor as the whole recycle pressure penalty: after MDEA, CO2-depleted recycle still needs a pressure lift from ~1 MPa toward the multi-MPa reformer feed unless the PSA/capture topology is redesigned.

The Rust model now encodes a transparent source-anchored screening topology:
1. PSA tail ~0.13 MPa -> MDEA ~1.0 MPa;
2. CO2-depleted recycle ~1.0 MPa -> JAEA-like reformer process pressure ~4.5 MPa.

Both compressor stages use an ideal-gas/intercooled sensitivity with explicit efficiency; they are not claimed as detailed compressor designs. This makes the post-capture pressure ratio (~4.5) visible instead of hiding it in the prior Case-2A 6.309 MWe electricity anchor.

The implementation and topology tests pass CI at commit `152d9df5804444cd479e89236360907ebdb313a4`.

Scientific implication: the current economic model likely understates recycle electricity if it charges only the Case-2A separation/compression anchor. The next calculation must avoid double counting the Case-2A first-stage compression while adding the incremental post-capture recycle compressor and the recycle sensible-heat term.

## Case-2A electricity decomposition and recycle penalty
The IEAGHG primary Case-2A tables resolve the previous electricity-accounting ambiguity. CO2 Capture Plant Consumption is 4.575 MWe and explicitly includes the tail-gas compressor; the equipment list gives that compressor as 4.280 MW brake power for 0.126 -> 1.0 MPa. CO2 compression/dehydration is a separate 2.874 MWe and the sweet-tail-gas expander generates 1.140 MWe. Thus 4.575 + 2.874 - 1.140 = 6.309 MWe exactly reproduces the prior source anchor.

The nuclear recycle topology must therefore NOT add the 0.126->1 MPa compressor again. The model now adds only the incremental post-capture ~1->4.5 MPa recycle compressor.

A second correction is equally important: in Case 2A the sweet tail gas is expanded before being sent to furnace burners. In the proposed furnace-free recycle architecture that gas is instead returned to the reformer, so the 1.140 MWe expander credit is not physically available unless a different pressure-recovery scheme is designed. The conservative corrected screen removes that credit.

Both the source-ledger decomposition and the corrected recycle-electricity accounting pass CI (commit `04748db16ff72d0042557d70f8ed6f0c9349a84c`). The corrected full-cost propagation, including recycle sensible heating and post-capture recompression, also passes CI (commit `68923619edf34d871f58d99ece01d5eb0a19709e`).

This tightens the S$100/t boundary relative to the previous ~S$14.7/GJ minimum-gas screen. Exact design values remain sensitivity-dependent because recycle compressor efficiency and the final injection pressure are not yet equipment-selected.

## Recycle pressure/efficiency sensitivity implemented
The post-capture recycle compressor is now parameterised by injection pressure rather than fixed at 4.5 MPa. A reproducible 3x3 screening surface spans injection pressures 2.0/3.0/4.5 MPa and compressor isentropic efficiencies 0.65/0.75/0.85.

Each point propagates the compressor load through the corrected full-cost S$100/t boundary while retaining the same converged lifecycle denominator, recycle sensible-heat screen, Case-2A capture/CO2-compression loads, no source-expander credit, and representative reactor/IHX/integration/T&S assumptions.

Acceptance tests require:
- higher injection pressure -> higher minimum NG value for feasibility;
- higher compressor efficiency -> lower minimum NG value;
- monotonicity across the complete 3x3 surface.

Commit `5554a366b10131376343e7c956d88a274b7d9903` contains the implementation. GitHub Actions was still queued at the time of this STATUS update, so this sensitivity is not yet marked verified.

## Independent Review 1
- Review-response pass started from the substantive review available in the
  CN4252 Project conversation.
- The requested repository file `reviews/review_01_research_foundation.md`
  was absent when checked; this provenance discrepancy is recorded rather than
  hidden.
- Finding dispositions and acceptance criteria are recorded in
  `reviews/review_01_resolution.md`.
- Review gate remains OPEN because valid scientific blockers remain.

## Newly resolved scientific issue
The previous stream-4 -> stream-5 elemental non-closure has been diagnosed from
the full IEAGHG 2017-02 process description.

Stream 4 is feed to the pre-reformer. Between stream 4 and the published HTS
inlet (stream 5), IEAGHG explicitly describes a second HP-superheated-steam
addition and BFW desuperheating before the primary reformer.

Using the rounded published stream compositions:
- carbon residual is only about -0.020 kmol-C/h;
- hydrogen residual implies about 155.2 kmol/h H2O;
- oxygen residual implies about 153.7 kmol/h H2O;
- least-squares reconciliation gives about 154.4 kmol/h aggregate H2O addition.

Including that source-described but unnumbered water/steam addition closes C/H/O
to <0.1% of the stream-4 elemental inventories without tuning published carbon
species. The material-boundary component of Review blocker B1 is therefore
resolved.

## New energy result
IEAGHG's fixed-output comparison is now encoded and CI-verified:
- Base: 394.77 MW NG input, 9.918 MWe export, 0.8091 kgCO2/Nm3 H2.
- Shifted-syngas MDEA (1A): 407.68 MW NG, 1.492 MWe export, 0.3704 kgCO2/Nm3.
- Flue-gas MEA (3): 433.72 MW NG, 0.426 MWe export, 0.0888 kgCO2/Nm3.

Thus Case 1A adds 12.91 MW NG and loses 8.426 MWe export while avoiding 43.87 tCO2/h plant-gate; Case 3 adds 38.95 MW NG and loses 9.492 MWe while avoiding 72.03 tCO2/h. Capture topology therefore materially changes the energy burden and cannot be selected independently of nuclear integration.

## New high-temperature duty result
IEAGHG's base-case equipment list gives a steam-reformer **radiant duty of 82.63 MMkcal/h = 96.04 MW** at 100,000 Nm3/h H2. This is now encoded and regression-tested.

This resolves the first high-temperature process-duty anchor, but it is not the final HTGR requirement. Removing the fired furnace also removes convective flue-gas heat used for reformer/pre-reformer feed preheat, steam superheat, feed preheat and steam generation.

IEAGHG states that reformer syngas leaves at ~900-950 C and that ~75% of HP saturated steam is generated by cooling that syngas in the reformer waste-heat boiler. That syngas heat-recovery source can in principle remain in a nuclear-heated reformer.

JAEA provides a close physical precedent: HTTR supplies 950 C primary helium through an IHX, with secondary helium serving a methane steam reformer, superheater and steam generator. JAEA documents a 10 MW nuclear-heat steam-reforming system design and an 880 C helium-inlet mock-up. The IEAGHG 96 MW radiant duty is therefore roughly an order of magnitude above that demonstration/design heat-transfer scale, not a direct commercial-reactor sizing result.

## New helium-interface result
JAEA provides a direct physical anchor for the proposed architecture: 950 C primary HTTR helium transfers heat through an IHX to a secondary loop; the HTTR steam-reforming system design supplies 10 MW nuclear heat, and the mock-up used 4 MPa helium at 880 C at the steam-reformer inlet.

A generic screening model is now implemented:
Q = m_dot_He cp_He (T_hot - T_cold),
with explicit finite temperature-approach constraints across both IHX and reformer.

Using the verified 96.04 MW IEAGHG radiant duty and a temporary screening assumption cp_He=5.2 kJ/kg-K, 880->650 C secondary helium requires about 80 kg/s. This is an assumption-based screening value, not a final design flow.

The first CI run for these tests failed at compile time only because a Rust test function name began with a numeral; the identifier has been corrected and CI rerun is pending. No scientific assertion failed in that run.

## New large-scale helium benchmark
A published JAEA GTHTR300C IHX design provides a larger-scale comparison without selecting that reactor:
- 170 MWth IHX duty;
- secondary He 500 -> 900 C;
- 81 kg/s secondary-He flow;
- 5.15 MPa inlet pressure;
- 58 kPa secondary-side IHX pressure loss.

The project's independent 96.04 MW screening case at 880 -> 650 C and cp=5.2 kJ/kg-K gives ~80 kg/s. The similar mass-flow magnitude is an order-of-magnitude plausibility check, not a design match.

Using the GTHTR300C IHX-only pressure loss and an explicit 80% circulator-efficiency screening assumption gives ~1.8 MW of IHX-only pumping. Total loop pumping must be larger because reformer, steam generator, piping, valves and return-path losses are not yet included.

Rust regression tests for the large-scale helium-flow and IHX-only pumping calculation passed GitHub Actions.

## New loop-parasitic bound
No defensible complete component pressure-drop dataset has yet been found for the exact proposed secondary-He loop, so no reformer/pipe/SG pressure losses were fabricated.

The model now parameterises total loop loss as a multiple of the published GTHTR300C IHX loss (58 kPa). At the published 170 MW / 81 kg/s / 5.15 MPa benchmark and an explicit 80% circulator-efficiency assumption:
- IHX-only lower layer is ~1.1% parasitic;
- 3x the IHX pressure loss is ~3.2% parasitic.

This is a sensitivity bound, not a final loop result. Rust regression tests passed GitHub Actions.

## New steam-service result
IEAGHG's base heat/material balance directly reports 95.301 t/h HP steam to process at 400 C / 4.29 MPa and 46.053 t/h HP steam export at 395 C / 4.23 MPa: ~141.354 t/h combined.

IEAGHG states around 75% of saturated HP steam is generated in the reformer syngas waste-heat boiler, implying an approximate ~106 t/h retained syngas-WHB generation scale if comparable reformer outlet conditions are preserved. The remaining ~35.3 t/h is an upper-group bound shared between shift heat recovery and the furnace convection steam-generator coil; it must not be labelled as furnace-only steam generation.

Crucially, IEAGHG routes saturated HP steam through the furnace steam-superheater coil before process/export use. Thus furnace removal creates a superheating service for roughly the full 141 t/h steam flow, not merely the non-WHB steam fraction. The next calculation will use NIST/IAPWS steam enthalpies rather than an assumed constant cp.

## New quantified furnace-service floor
The HP steam superheater service is now bounded from IAPWS-consistent steam properties. The IEAGHG base plant routes ~141.354 t/h HP steam through the furnace superheater. Bracketing the 4.23-4.29 MPa source pressure with 4.0 and 4.5 MPa steam-table states gives approximately 16.0-16.25 MW of superheat duty.

Adding this to the verified 96.04 MW radiant reformer duty yields a currently quantified furnace-dependent thermal-service floor of approximately 112.0-112.3 MW at the 100,000 Nm3/h H2 reference scale.

This is not final HTGR duty: feed/pre-reformer/reformer preheat, furnace-convection steam generation and nuclear-loop losses remain unresolved. Conversely, syngas/shift heat recovery must not be double-counted as nuclear duty.

## New convection-duty lower bound
IEAGHG states that feedstock NG is heated from 135 C to 370 C in the fired-furnace Feed Pre-Heater Coil. Using the published 1455.8 kmol/h NG feed, 89 mol% CH4, and NIST SRD 69 methane Shomate enthalpy, the methane contribution alone is 4.05 MW.

This is a rigorous partial lower bound because CO2/C2+/N2 and recycled H2 sensible duties are omitted. Adding it to the verified 96.04 MW radiant duty and ~16.0-16.25 MW HP-steam superheat raises the currently demonstrated furnace-dependent service floor above ~116 MW.

The next model layer will add remaining species rather than applying an arbitrary mixture cp.

## Completed NG feed-preheater calculation
The complete natural-gas portion of the fired-furnace 135->370 C Feed Pre-Heater Coil has now been calculated from NIST SRD 69 properties.

Using Shomate enthalpies for CH4/CO2/N2 and integrated NIST ideal-gas Cp tables for C2H6/C3H8/n-C4H10/n-C5H12 gives Delta h_NG = ~11.922 kJ/mol mixture and Q_NG = ~4.821 MW at the published 1455.8 kmol/h NG flow.

This supersedes the prior 4.05 MW methane-only lower bound. Recycled H2 remains excluded until its source flow is recovered.

Combining 96.04 MW radiant + ~16.0-16.25 MW HP steam superheat + 4.82 MW NG feed preheat gives a currently quantified furnace-dependent service floor of ~116.9-117.1 MW, still excluding several convection duties and nuclear-loop losses.

## Recycle and reformer-preheater status
The previously missing H2 recycle is source-resolved: IEAGHG stream 13 is 29.1 kmol/h (59 kg/h), 40 C, 2.51 MPa, >99.99 mol% H2. Its feed-preheat contribution is only ~0.08 MW using NIST H2 properties.

The corrected Pre-Reformer Feed Pre-Heater lower-bound implementation now passes CI. It deliberately omits C2+ sensible terms above their encoded NIST Cp-table range rather than extrapolating, so it remains conservative.

The standalone IEAGHG summary does not expose the exact primary-reformer inlet temperature. A sensitivity function is therefore implemented rather than a fabricated point value. A related IEAGHG study provides 600-650 C only as contextual range. Rust tests confirm the calculated duty increases monotonically across 600, 625 and 650 C.

## First combined HTGR service envelope
The current Rust model now produces and CI-locks the first combined furnace-dependent thermal-service envelope at the 100,000 Nm3/h H2 reference scale:
- 600 C reformer-inlet sensitivity: 130.74-130.97 MWth;
- 625 C: 132.48-132.71 MWth;
- 650 C: 134.23-134.46 MWth.

Component values are: 96.034 MW radiant, 16.008-16.240 MW HP-steam superheat, 4.899 MW NG+recycle feed preheat, 7.050 MW conservative pre-reformer feed-preheat lower bound, and 6.750/8.486/10.241 MW reformer-preheat sensitivity at 600/625/650 C.

These remain incomplete lower/service envelopes because furnace-only saturated-steam generation and nuclear-loop thermal losses are unresolved.

For the current ~80.30 kg/s helium screening flow at 5.15 MPa and 80% circulator efficiency, loop pressure loss equal to 1x/2x/3x the published 58 kPa IHX anchor gives ~2.17/4.33/6.50 MWe circulator power. Electrical parasitics remain separate from MWth service.

## Furnace steam-generation ambiguity is now bounded
IEAGHG states that ~75% of saturated HP steam comes from the syngas WHB and that the remainder is shared by shift heat recovery and the furnace convection steam-generator coil. The accessible source does not publish that split, so it has not been invented.

The entire ~25% non-WHB remainder is ~35.34 t/h. Assigning all of it to the furnace coil gives an intentionally conservative upper bound of ~16.45-16.82 MW of saturated-steam generation near 4.23 MPa. The true furnace contribution is lower because shift heat recovery demonstrably shares the remainder.

Combining this with the verified service calculations yields a current conventional furnace-service bound of approximately **131-151 MWth** at 100,000 Nm3/h H2, before nuclear-loop thermal losses. This converts the missing steam split from an unbounded blocker into a <=~17 MW uncertainty.

## First direct-heat vs nuclear-electric screen
The unresolved furnace steam-generation contribution is now rigorously bounded without inventing the shift/furnace split. Assigning the entire ~25% non-syngas-WHB steam remainder to the furnace gives a deliberately conservative ~16.45-16.82 MW upper bound. The current conventional furnace-service range is therefore approximately 131-151 MWth.

At the 625 C reformer-preheat sensitivity the service range is ~132.5-149.5 MWth. Using a conservative 90% eSMR electricity-to-heat efficiency and JAEA's optimistic 50.4% GTHTR300 net generation efficiency gives ~147-166 MWe and ~292-330 MWth reactor heat for the nuclear-electric route. A 45.8% power-cycle benchmark raises the reactor-thermal requirement to roughly 321-363 MWth.

This establishes a reactor-thermal-utilisation hypothesis in favour of direct heat, not an overall ranking. A 2026 peer-reviewed direct comparison reports lower LCOH for helium-heated SMR ($2.37/kg vs $2.99/kg) but slightly lower GWP for eSMR (3.18 vs 3.5 kgCO2/kgH2), showing the tradeoff is multidimensional.

## Matched emissions comparison established
A controlled direct-heat/eSMR comparison now holds H2 output, NG feed, carbon conversion/capture, PSA recovery/tail-gas disposition, CCS boundary and upstream NG supply chain identical. Under those conditions, plant-gate feedstock-carbon emissions are identical by construction; only the energy-delivery architecture differs.

The IEAGHG feedstock contains ~1578.56 kmol-C/h, equivalent to ~7.72 kgCO2/kgH2 if all feed carbon ultimately becomes CO2. A 90% permanent feedstock-carbon capture screening case therefore leaves ~0.77 kgCO2/kgH2 plant-gate feedstock-carbon emissions before lifecycle additions.

This makes an important interpretation explicit: Ahn & Lee's 2026 h-SMR/eSMR GWP difference (3.5 vs 3.18 kgCO2/kgH2) cannot be attributed intrinsically to helium versus electricity under matched chemistry. It must reflect process/resource/lifecycle differences in their configurations. Their result remains an external benchmark, not a value to import.

DOE 45VH2-GREET confirms a well-to-gate methodology including feedstock extraction/delivery, electricity, facility emissions, CO2 capture/delivery and potential storage leakage. IEA reports 10-12 kgCO2e/kgH2 for unabated NG hydrogen and warns upstream/midstream emissions remain material after CCS. UNECE provides a 5.1-6.4 gCO2e/kWh nuclear-electric lifecycle range, but a defensible allocation to direct nuclear process heat remains unresolved.

## Singapore-oriented lifecycle layer established
EMA reports 11 Mtoe of natural-gas imports in 2024, including 6 Mtoe LNG (~55% on that energy basis), so Singapore cannot be represented honestly by a single LNG-only or pipeline-only upstream factor.

IEA's latest global anchors are ~11.5 gCO2e/MJ for extraction/processing/transport of gas overall and 18.6 gCO2e/MJ for delivered LNG production-to-regasification in 2025, with large geographic uncertainty. On the IEAGHG feedstock basis (1219.7 GJ/h NG for 8994 kgH2/h), these correspond to ~1.56 and ~2.52 kgCO2e/kgH2 upstream, respectively. These are sensitivity anchors, not Singapore route-specific measurements.

UNECE's nuclear-electricity LCA range is 5.1-6.4 gCO2e/kWh. A direct-heat allocation proxy has been defined from the electric factor times net electric efficiency; at 5.5 g/kWh_e and 50.4% this is ~2.77 g/kWh_th. Under a 140 MW service example, nuclear lifecycle contributions are only order ~0.04 kgCO2e/kgH2 direct and ~0.10 kgCO2e/kgH2 electric, much smaller than upstream gas uncertainty.

Singapore lacks domestic geological storage and is pursuing cross-border CCS with Indonesia/Malaysia. No storage site is assumed. IEAGHG ship-transport emissions are therefore used only as an explicit transport-fraction sensitivity until a route is selected.

The emerging falsification hypothesis is that lifecycle performance may be limited more by imported-gas emissions than by direct-vs-electric nuclear heat architecture.

## First assignment-level emissions-scale screen
A matched 90% feedstock-carbon-capture screening model now combines plant carbon, upstream NG, nuclear lifecycle and CO2 transport terms.

On the IEAGHG feedstock basis, IEA's 11.5 gCO2e/MJ global gas-supply anchor gives ~1.56 kgCO2e/kgH2 upstream; the 18.6 g/MJ global LNG anchor gives ~2.52. At 90% feedstock-carbon capture, residual plant carbon is ~0.77 kgCO2/kgH2. A 2.5%-of-captured-CO2 historical shipping sensitivity adds ~0.17 kgCO2e/kgH2. Nuclear energy contributes only order ~0.04 direct / ~0.10 electric kgCO2e/kgH2 under the current proxy assumptions.

This produces illustrative matched lifecycle intensities of roughly ~2.5-2.6 kgCO2e/kgH2 on the lower/global-gas anchor, rising by ~1 kg/kg under the LNG anchor. These are screening values, not route-specific Singapore results.

Against IEA's 10-12 kgCO2e/kgH2 unabated-NG hydrogen benchmark, the implied specific abatement is roughly 6.4-9.5 kgCO2e/kgH2, requiring order 26-39 ktH2/y to exceed 0.25 MtCO2e/y. The IEAGHG reference plant produces ~72 ktH2/y at 8000 h/y, so the assignment's scale threshold appears achievable in principle across much of the current screening envelope.

The dominant scientific uncertainty after high capture is now upstream imported-gas emissions, not direct-vs-electric nuclear lifecycle emissions.

## Economic framework opened
The CN4252 economic metric is now formalised as incremental annual cost divided by annual lifecycle CO2e avoided. At the minimum qualifying 0.25 MtCO2e/y abatement, S$100/t implies a maximum incremental annual cost of S$25 million/y; this is a budget identity, not a cost prediction.

IEAGHG's historical standalone SMR+CCS benchmark reports EUR47-70/tCO2 avoided, EUR40-176m additional capital and 18-33% higher H2 operating cost (Q4 2014 basis). These values are useful comparators but cannot be directly compared to S$100/t without price-year/currency/boundary harmonisation.

Singapore MTI stated in April 2025 that clearer estimates for the complete cross-border CCS capture/transport/storage value chain were still being developed, so no official Singapore CCS S$/t value has been fabricated.

Singapore's carbon tax is S$45/tCO2e in 2026-2027, with a previously stated view of S$50-80 by 2030. It is retained as policy/economic context, not substituted for abatement cost.

Rust now implements CRF annualisation, generic annualised cost, incremental abatement cost and the maximum annual incremental-cost budget implied by a target S$/t.

## Economic parameter matrix and break-even budget
A first economic parameter matrix now separates source values, legacy technology anchors and unresolved Singapore inputs. JAEA's GTHTR300 design study reports ~JPY200,000/kWe capital and ~JPY4.2/kWh at 80% utilisation; a JAEA HTGR hydrogen study used 0.7 JPY/MJ nuclear heat and 5.8 JPY/kWh electricity. These are legacy Japanese design-study anchors, not current Singapore prices.

IAEA's recent hydrogen TECDOC provides larger HTGR+SMR cases: e.g. 4x200 MWth HTGR-200 with USD2.065b NPP CAPEX plus USD1.013b hydrogen-plant CAPEX and reported H2 cost USD1.28/kg; an MHR-T+SMR case reports USD2.748b NPP plus USD1.496b H2 plant and USD0.96/kg. These are multi-module external benchmarks and are not linearly scaled to Singapore.

For the 71.952 ktH2/y reference plant, an illustrative 7.5 kgCO2e/kgH2 specific abatement yields ~0.540 Mt/y avoided and therefore ~S$54m/y maximum incremental cost at S$100/t. At 8%/25y, allocating that entire budget to CAPEX alone would imply an optimistic ~S$576m ceiling. At the minimum 0.25 Mt/y assignment scale, the corresponding CAPEX-only ceiling is ~S$267m. Both are upper bounds because real incremental OPEX/CCS/T&S costs consume part of the budget.

This makes reactor cost allocation a first-order question: dedicated HTGR and cogeneration/shared-reactor architectures must be tested separately.

## Dedicated/cogeneration allocation and Singapore CCS pressure
The authoritative Project Source was rechecked: CN4252 requires >0.25 MtCO2e/y within Singapore at <S$100/tCO2e. A current public SPEED page has a different <S$250/t desirable threshold, but it is not the assignment source and does not replace the CN4252 target.

JAEA GTHTR300C provides a transparent cogeneration allocation anchor: 170 MWth of a 600 MWth reactor goes to hydrogen and 430 MWth to power, so a thermal-energy-share allocation assigns 28.33% of common reactor cost to hydrogen. JAEA's own IS-process economics show cogeneration materially reducing attributed H2 cost, but those numerical reductions are not transferred to SMR.

More importantly, an IEAGHG 2023 ExxonMobil analysis specifically for Singapore CO2 sources reports T&S cost groups of USD50-75/t (Group A), USD75-150/t (B), and USD150-450/t (C). These are study estimates, not tariffs.

At ~0.50 MtCO2/y captured and an explicit 1.276 SGD/USD FX sensitivity, Group-A T&S alone is ~S$31.9-47.9m/y. Against the illustrative ~S$54m/y incremental-cost budget for 0.54 MtCO2e/y avoided, that consumes ~59-89% of the budget before capture, nuclear integration and O&M. This is now the strongest economic falsification pressure identified.

## Analytical S$100/t feasibility boundary
For the illustrative reference case (71.952 ktH2/y, 7.5 kgCO2e/kgH2 avoided), the S$100/t threshold permits ~S$54m/y incremental cost. With ~0.50 MtCO2/y captured, the absolute T&S ceiling is S$108/t captured only if every other incremental cost were zero.

The IEAGHG 2023 Singapore-source Group-A T&S range (USD50-75/t) is ~S$63.8-95.7/t at an explicit 1.276 SGD/USD sensitivity, costing ~S$31.9-47.85m/y at 0.50 Mt/y. That leaves only ~S$22.1m/y to ~S$6.15m/y for capture, nuclear, IHX/loop, tail-gas integration and all other incremental costs before credits.

Under a GTHTR300C-like 28.33% thermal-energy-share allocation, cogeneration expands the total common-reactor annual-cost ceiling by 600/170 ~= 3.53 relative to dedicated allocation, but it cannot remove the T&S constraint.

IEAGHG Case 1A conventional SMR+CCS already reports EUR47.1/tCO2 avoided on Q4-2014 assumptions including only EUR10/t stored. This makes a Singapore-adjusted conventional SMR+CCS comparator the next critical economic baseline before nuclear cost is added.

## Case 1A conventional CCS comparator decomposed
Primary IEAGHG data now allow the published Case-1A EUR47.1/tCO2 avoided result to be separated exactly from its EUR10/t captured T&S assumption. Case 1A captures 0.4660 kg/Nm3 while avoiding 0.4387 kg/Nm3 versus the base, so captured/avoided = 1.0622. The EUR10/t T&S charge therefore contributes EUR10.62/t avoided, leaving an implied non-T&S capture/integration CAC of **EUR36.48/t avoided in Q4-2014 euros**.

The Rust model reconstructs the published EUR47.1/t result from these components and tests the tariff sensitivity.

This is the correct base for Singapore substitution. The EUR36.48/t term has deliberately NOT yet been inflated to 2026 SGD. Eurostat HICP is a consumer-price index; using it to escalate a chemical-process plant is only a screening proxy. CEPCI/Marshall & Swift are more appropriate for plant/equipment escalation, but a reliable current public index value has not yet been established, so no fabricated engineering escalation is recorded.

## Singapore-adjusted Case-1A screening comparator
The conventional comparator has now been harmonised to a 2025 screening basis without using consumer HICP for plant-cost escalation.

Source decomposition: IEAGHG Case 1A CAC EUR47.1/t avoided includes EUR10/t captured T&S. With captured/avoided = 1.0622, its non-T&S component is EUR36.48/t avoided in Q4-2014 euros.

Process-cost escalation screen: a peer-reviewed 2025 TEA reports CEPCI 2024=800; Chemical Engineering reports the 2025 annual average 1.6% above 2024, giving a derived 2025 screening index ~812.8. Using the standard 2014 CEPCI 576.1 gives ~EUR51.5/t avoided non-T&S. ECB 31-Dec-2025 EUR/SGD=1.5105 converts this to ~S$77.7/t avoided.

Singapore-source Group-A T&S USD50-75/t captured converts at ~1.2863 SGD/USD to ~S$64.3-96.5/t captured, or ~S$68.3-102.5/t avoided after the 1.0622 captured/avoided ratio.

Combined conventional Case-1A screening CAC is therefore **~S$146-180/t avoided**, above the authoritative S$100/t target. Even without CEPCI escalation, the same arithmetic is roughly S$123-158/t, so the threshold pressure is not solely an escalation artifact.

This remains a screening result: IEAGHG CAC is plant-gate CO2, not lifecycle CO2e; Singapore T&S is a study range, not tariff; and Singapore location/finance factors are not yet applied.

## Nuclear required-savings diagnostic
At IEAGHG's 8322 h/y, Case 1A directly avoids ~365.1 ktCO2/y versus the base plant. Holding that denominator fixed only as a diagnostic, reducing the current Singapore-adjusted screening comparator from S$146-180/t to S$100/t requires roughly **S$16.8-29.2m/y of net savings** before any positive nuclear premium can be tolerated.

This is not the final lifecycle CN4252 denominator: nuclear integration may increase capture/abatement and therefore enlarge the denominator.

Singapore's 2026-27 carbon tax is S$45/tCO2e. At ~365 kt/y, avoided tax could be order S$16m/y for a fully taxable facility, but tax is a transfer/policy cash-flow effect, not automatically a societal resource saving. The model will therefore keep resource abatement cost separate from private project cash flow.

The next test is whether real savings—avoided furnace NG, utilities, steam/power changes and potentially cogeneration value—are of the same order as S$17-29m/y. Only after that should detailed nuclear CAPEX be imposed.

## Avoided supplementary-furnace-NG savings sensitivity
IEAGHG's 55.94 MW_LHV purchased supplementary furnace NG corresponds to ~1.676 million GJ/y at 8322 h/y. The Singapore industrial NG commodity price remains an explicit sensitivity; town-gas retail tariffs are not substituted.

If nuclear integration displaced 100% of this purchased supplementary NG, avoided NG alone would close the current S$16.8-29.2m/y fixed-denominator economic gap only at gas prices of roughly **S$10.0-17.4/GJ**, before any nuclear/IHX/tail-gas/circulator costs are charged.

This shows avoided fuel can be material but is not automatically sufficient. PSA tail gas is not counted as purchased-fuel savings; its furnace sink disappears and must be recycled/treated consistently.

## PSA tail-gas and utility boundary quantified
The IEAGHG base PSA tail gas is now reconstructed energetically from its published 2106.3 kmol/h composition. H2+CO+CH4 combustible flow is ~1004 kmol/h and the standard-LHV chemical inventory is ~101.95 MW_LHV. This is internal fuel, not purchased energy, so it cannot be credited as avoided fuel cost when the furnace disappears.

IEAGHG explicitly sends base tail gas to the SMR burners. Ahn & Lee (2026) independently eliminate the fired furnace in HTGR h-SMR/eSMR and recycle PSA tail gas as process feed, reporting 18.8% and 23.3% NG reductions versus their gray baseline. Those percentages are external validation of the architecture, not values imported into our IEAGHG model. A 2025 hybrid cryogenic/two-stage-PSA study provides a competing tail-gas recovery route with additional H2 recovery and >90% CO2 removal.

Power/steam economics are now correctly bounded: IEAGHG base exports 9.918 MWe while Case 1A exports 1.492 MWe, but a nuclear architecture cannot claim the 8.426 MWe difference as a credit until its steam/power network is solved. Base steam export is ~46.053 t/h; preserving it is neutral versus baseline, losing it is a penalty, and increasing it is a credit. Rust now contains explicit annual electricity/steam value functions with prices left as inputs.

## Tail-gas configuration screen
Three competing furnace-free tail-gas pathways are now formalised: (A) untreated recycle, (B) CO2 removal + combustible recycle, and (C) enhanced H2/CO2 recovery.

The source tail gas contains ~499 kmol/h H2, ~1073 kmol/h CO2 (~47.2 t/h), ~306 kmol/h CO and ~199 kmol/h CH4. A stoichiometric upper-bound recovery calculation gives ~1600 kmol/h H2-equivalent potential if existing H2 is recovered, CO fully shifted and CH4 fully reformed+shifted. This is not a process yield; it demonstrates the stream is a first-order chemical resource.

Configuration B is selected for the next model **for information value, not as a final design**. IEAGHG Case 2A provides same-scale source evidence: PSA tail gas compression from ~0.2 to 1 MPa, 4.575 MWe capture-plant consumption, 2.874 MWe CO2 compression/dehydration, 1.140 MWe tail-gas expander recovery, ~1.07 MWe net grid import, and ~66.9 t/h LP steam regeneration demand. Case 2A burns the sweet tail gas; the nuclear variant will instead test recycle to process.

## Configuration-B fixed-output upper bound
For CO2 removal + combustible recycle, source tail gas contains ~505 kmol-C/h as CO+CH4 versus ~1579 kmol-C/h in fresh feedstock NG. Therefore the absolute carbon-equivalent fresh-feed displacement ceiling is about **32%**, corresponding to roughly ~470 kmol/h NG mixture or order ~110 MW_LHV on the source feed-energy basis.

This is deliberately an upper bound, not a predicted recycle saving. Recycled CO is already partially oxidised; H2/CO/CH4 have different stoichiometry; steam, heat, equilibrium and PSA losses must be solved.

At 8322 h/y the ~110 MW upper-bound feed displacement is ~3.3 million GJ/y, so its gross value would be order S$33/50/66m/y at S$10/15/20 per GJ. This demonstrates why tail-gas recycle can be economically first-order if a substantial fraction of the bound survives the process model.

IEAGHG Case 2A provides a same-scale utility anchor for tail-gas CO2 removal: 4.575 MWe capture plant + 2.874 MWe CO2 compression/dehydration - 1.140 MWe expander recovery = **6.309 MWe** before other recycle-specific costs and solvent-steam demand.

## Reduced fixed-H2 tail-recycle result
A reduced stoichiometric model now distinguishes recycled H2, CO and CH4 and uses the source-reconstructed PSA H2 recovery (~90%). It is not yet an equilibrium/recycle-convergence model.

At equal 50/80/100% H2 recovery + CO/CH4 conversion sensitivities, fresh-NG displacement is ~16.6/26.6/33.2%, corresponding to ~56.3/90.0/112.5 MW of fresh-feed energy. The 80% case requires ~563 kmol/h additional stoichiometric water.

The ideal 33.2% fresh-NG displacement slightly exceeds the earlier ~32% carbon-equivalent ceiling because existing recycled H2 can displace fresh NG without carrying carbon; the earlier statement is corrected to apply specifically to carbon-equivalent displacement.

For the 80% case, at illustrative S$150/MWh electricity, the Case-2A 6.309 MWe separation anchor costs ~S$7.88m/y. Gross fresh-feed savings at S$10/15/20 per GJ are ~S$27.0/40.4/53.9m/y, leaving optimistic **partial** net values of ~S$19.1/32.6/46.1m/y before solvent steam, altered reformer duty, recycle compression and CAPEX.

This keeps tail-gas integration economically material but does not establish feasibility.

## Tail-recycle thermal penalty screen
NETL reaction data give DeltaH°298 ~+205.8 kJ/mol for SMR and -41.2 kJ/mol for WGS. In the 80% reduced recycle case, recycled CO/CH4 contributes only ~+4.5 MW of standard reaction heat because WGS offsets part of CH4 reforming. The displaced fresh NG contains enough CH4 that its methane-only standard reform+shift duty is ~15-16 MW, so the recycle case conservatively **reduces** standard reaction heat by order ~11 MW relative to the displaced fresh feed. C2+ omission makes this savings magnitude conservative.

The ~563 kmol/h gross stoichiometric water consumption is not added directly as extra plant steam: ~420 kmol-C/h fresh feed is removed while ~404 kmol-C/h recycled CO+CH4 is converted, so steam must be recomputed from the combined feed/S:C constraint.

The major new thermal penalty is IEAGHG Case-2A MDEA regeneration: ~66.9 t/h LP steam. Saturated-steam latent heat at ~4-7 barg implies **~38.1-39.2 MWth** of low-temperature regeneration service. This is not high-grade reformer heat and may be partly supplied by retained syngas/WGS recovery or the helium temperature cascade.

A crude +38-39 MW MDEA minus ~11 MW standard reaction-heat saving gives order +27-28 MW, but this is explicitly NOT a final HTGR increment because heat-grade integration, sensible duties, feed-preheat reductions and recycle compression remain unresolved.

## First temperature-grade heat-cascade result
External integrated-SMR evidence materially changes the MDEA penalty interpretation. A recent electrified/convective SMR+CCS study supplies **63% of solvent-regeneration heat directly from post-LTS/condensing syngas** and only 37% from LP steam. A 2025 advanced blue-H2 study independently reports pinch-designed low-quality waste-heat recovery capable of eliminating external heating demand in its integrated configurations. These are external benchmarks, not IEAGHG Case-2A reconstructed values.

The model now carries an explicit MDEA waste-heat fraction f_WH. For the ~38.1-39.2 MW Case-2A regeneration service, f_WH=0/0.5/0.63/0.75 gives incremental heat of ~38-39/~19/~14/~9.5-9.8 MW respectively.

At the external 63% anchor, incremental MDEA heat is ~14.1-14.5 MW. Combining this only with the conservative ~-11 MW standard reaction-heat change from 80% recycle gives an incomplete net thermal increment of order **~3 MW**, rather than the naive +27-28 MW obtained when all MDEA heat was assumed incremental.

This is not yet a design result: the 63% value is external and temperature-approach feasibility for the IEAGHG streams has not been proven. It does establish that f_MDEA,nuclear=1 is unnecessarily conservative.

## IEAGHG-source heat-cascade bound
The first source-based heat-integration constraint is now derived without importing the external 63% benchmark.

IEAGHG states that reformer syngas leaves at ~900-950 C and is cooled to ~320 C in the reformer WHB, which supplies ~75% of saturated HP steam. That large heat source is therefore already committed to the baseline steam network and cannot be double-counted as free MDEA heat. citeturn0search12

The remaining ~25% saturated-steam-generation group (~35.34 t/h) is shared by shift heat recovery and the fired-furnace steam-generator coil. Its latent-heat scale is ~16.45-16.82 MWth. Assigning the **entire** group to retained shift heat gives an intentionally optimistic source-ledger upper bound of only **~42-44%** of the ~38.1-39.2 MW MDEA regeneration duty. The true shift-steam contribution is smaller because the furnace coil shares that 25%.

At this ~42-44% upper-group bound, incremental MDEA heat is order ~21-23 MWth; combined only with the conservative ~-11 MW recycle reaction-heat change, the incomplete net thermal increment is order ~10-12 MW. This is between the no-integration (+27-28 MW) and external-63%-anchor (~+3 MW) screens.

This does not rule out >44% recovery: additional low-grade sensible/condensing syngas heat may exist outside the steam-generation ledger. It means that extra heat must be explicitly reconstructed rather than assumed.

## Direct IEAGHG shifted-syngas heat reconstruction
The full primary IEAGHG heat/material balance resolves the missing states: HTS inlet stream 5 is 320 C/2.80 MPa/8370.3 kmol/h; HTS outlet stream 6 is 412 C/2.77 MPa at the same flow; PSA inlet stream 7 is 35 C/2.58 MPa after downstream heat recovery and condensate separation. Stream 6 contains 21.37 mol% H2O. citeturn5view0

NIST SRD 69 CO Shomate data were added. To avoid extrapolating NIST water-vapour Shomate below its verified 500 K lower limit, the first direct calculation cools stream 6 only from 412 C to 226.85 C. This yields **~14.49 MWth of sensible heat**, enough in pure availability terms for ~37-38% of the ~38.1-39.2 MW MDEA latent duty. No condensation heat is credited.

This is not freely allocatable heat: IEAGHG explicitly routes shifted syngas through the shift WHB, BFW preheater, feed preheater, condensate preheater, air cooler and demi-water preheater before condensate separation. citeturn4view0

The large water reduction from stream 6 (8370.3 kmol/h, 21.37% H2O) to stream 7 (6596.9 kmol/h, 0.24% H2O) proves substantial condensation occurs downstream. The next calculation must therefore use pressure-dependent water phase equilibrium/IAPWS enthalpies; ideal-gas extrapolation is no longer acceptable.

## Shifted-syngas dew point resolves the latent-heat question
The model now implements the IAPWS-IF97 Region-4 saturation-temperature equation and independently checks the normal boiling point. For IEAGHG stream 6, y_H2O=0.2137 and P=2.77 MPa give p_H2O ~=0.592 MPa and an ideal-mixture water dew point in the high-150 C range. citeturn0search25turn0search0

For an illustrative 160 C MDEA reboiler with DeltaTmin=10 K, the hot stream must remain >=170 C. Because stream-6 bulk water condensation begins below that temperature, the large ~32 t/h water-condensation latent heat is **not directly pinch-feasible** for a conventional 160 C MDEA reboiler under this screen.

This corrects the prior expectation that condensation might close the gap to the external 63% heat-recovery benchmark. For the IEAGHG stream, temperature-feasible MDEA heat is primarily shifted-syngas sensible heat above the pinch; condensation heat is better suited to lower-temperature duties or would require a different solvent/heat-pump architecture.

The remaining direct calculation gap is 226.85 C -> ~170 C sensible cooling. NIST's encoded water-vapour Shomate correlation stops at 500 K, so it will not be extrapolated.

## Full shifted-syngas sensible-heat ceiling closed
The 412->170 C temperature-feasible stream-6 sensible heat is now **18.756-18.842 MWth**. The already verified 412->226.85 C portion is 14.489 MW; the remaining 226.85->170 C interval is bounded without extrapolating NIST H2O/N2 Shomate fits. IAPWS-IF97 is the authoritative future replacement for the narrow Cp bracket. citeturn0search0turn0search7

Against the ~38.1-39.2 MW Case-2A MDEA latent duty, this is only ~47.8-49.5% availability **before** competing downstream duties. The property-bracket width is only ~0.085 MW, so the dominant uncertainty is now heat allocation, not thermophysical properties.

The IAPWS dew-point result remains decisive: bulk condensation begins below the 170 C hot-side pinch for a 160 C reboiler/10 K approach, so the large latent heat cannot fill the remaining MDEA duty in a simple conventional exchanger.

## MDEA residual heat ceiling tightened by source allocation
IEAGHG explicitly states that NG is heated to 135 C in the Feed Pre-heater using shifted syngas leaving the BFW preheater. The already verified feed-preheater duty is ~4.899 MW, so it must be preserved as a competing shifted-syngas heat-recovery service. citeturn1search0

Subtracting that duty from the closed 18.756-18.842 MW 412->170 C sensible-heat ceiling leaves only **~13.86-13.94 MW** before any Shift-WHB, BFW, condensate or demi-water duties are charged. Against the ~38.1-39.2 MW MDEA regeneration latent duty, this is an optimistic upper fraction of only **~35-37%**.

Therefore, under a 160 C reboiler and 10 K approach, the actual directly recoverable MDEA heat fraction must be below ~37% unless existing downstream duties are reassigned elsewhere. The external 63% benchmark is no longer a plausible central value for this IEAGHG configuration under the current pinch assumptions.

The public IEAGHG equipment-list text names the remaining exchangers but does not expose their numerical duties in the searchable table, so those duties have not been fabricated.

## Unavailable exchanger duties bounded and propagated
A focused primary-source check confirms the remaining Shift-WHB/BFW/condensate/demi-water exchanger duties are named but not numerically tabulated in the accessible IEAGHG report. They are therefore no longer treated as a blocker and have not been fabricated. citeturn0search0turn0search1

Direct MDEA heat recovery is carried as 0 to ~13.9 MW, giving incremental MDEA heat of roughly ~24-39 MW. Combining this with the conservative ~-11 MW reaction-heat change yields a source-bounded 80% recycle thermal increment of order **~13-28 MWth**.

This interval is now propagated into the HTGR service model. At the 625 C reformer-preheat sensitivity, the recycle configuration moves the reactor-side process-service requirement from the previous furnace-only range into approximately the **mid-140s to high-170s MWth** range, before nuclear-loop thermal losses. Separation/circulator electricity remains separate.

This replaces the earlier optimistic ~3 MW recycle-heat assumption with a defensible source-bounded uncertainty.

## 80% recycle operating-energy economics propagated
The source-bounded recycle architecture now has a full pre-CAPEX operating-energy screen. At 80% recycle, ~90.0 MW_LHV fresh-feed NG plus 55.94 MW_LHV purchased supplementary furnace NG are displaced, ~145.95 MW combined. The source-bounded reactor-side process-service range is approximately 145-179 MWth before nuclear-loop thermal losses; Case-2A separation remains 6.309 MWe.

JAEA's legacy HTGR hydrogen study used 0.7 JPY/MJ nuclear heat and 5.8 JPY/kWh electricity. At late-Sep-2026 FX (~0.00813 SGD/JPY), 0.7 JPY/MJ is numerically ~S$5.69/GJ. This is explicitly a legacy technology-cost anchor, not a current Singapore nuclear-heat tariff. citeturn0search0turn1search0

At S$150/MWh separation electricity and S$5.69/GJ nuclear heat, annual operating-energy net value before CAPEX/fixed O&M is:
- gas S$10/GJ: ~S$5.3-11.1m/y;
- gas S$15/GJ: ~S$27.2-33.0m/y;
- gas S$20/GJ: ~S$49.1-54.9m/y.

The corresponding operating-energy-only gas break-even is ~S$7.45-8.78/GJ across 145-179 MWth service. Therefore the concept is not yet economically falsified: at ~S$15/GJ gas the operating-energy benefit is comparable to the previously identified ~S$17-29m/y fixed-denominator savings gap, but there is little/no room for CAPEX at the low-gas case and substantial sensitivity to CCS T&S.

## Inverse S$100/t economic feasibility surface
The economic screen is now inverted so no speculative Singapore nuclear-heat tariff is required. For allowed annual budget B, NG saving S_NG, separation electricity C_e, CCS T&S C_TS and delivered nuclear heat Q_N:

p_N,max = (B + S_NG - C_e - C_TS - C_other)/Q_N.

At an assumed heat price, the remaining hydrogen-side annualised HTGR/IHX/integration headroom is B + S_NG - C_e - C_TS - p_N Q_N.

Using the illustrative B~S$54m/y, 80% recycle (~145.95 MW NG displacement), 6.309 MWe separation at S$150/MWh, 145/162/179 MWth process service, and Group-A-like T&S (~S$31.9m/y low; ~S$47.85m/y high), the maximum delivered nuclear-heat price spans roughly:
- low Group-A T&S: ~S$10-23/GJ across NG S$10-20/GJ and service 145-179 MW;
- high Group-A T&S: ~S$7-19/GJ.

At NG~S$15/GJ and 162 MWth, the boundary is roughly ~S$15.9/GJ under low Group-A T&S and ~S$12.6/GJ under high Group-A T&S before other fixed incremental costs. These are screening boundaries, not predicted nuclear prices.

Dedicated reactor allocation uses 100% of hydrogen-side headroom. A GTHTR300C-like 170/600 thermal-share allocation expands allowable total common-reactor annual cost by ~3.53x, but only when hydrogen-side headroom is positive and the other product genuinely bears the remaining cost.

## HTGR source costs mapped onto inverse boundary
IAEA TECDOC 2075 provides modern multi-module source cases: HTGR-200+SMR has 800 MWth NPP, USD2.065b NPP CAPEX and USD192m/y NPP O&M; MHR-T+SMR has 2400 MWth NPP, USD2.748b CAPEX and USD324m/y O&M. citeturn0search48

At 8%/25y, annual common NPP cost is ~USD385m/y and ~USD581m/y respectively. A simple 162 MWth thermal-share allocation gives ~USD78m/y for HTGR-200 and ~USD39m/y for MHR-T; at ~1.276 SGD/USD these are order ~S$100m/y and ~S$50m/y.

CI exposed and corrected an interpretation error in the inverse boundary: at the illustrative S$15/GJ gas, 162 MWth, S$5.69/GJ legacy heat anchor, S$150/MWh separation power and low Group-A T&S, hydrogen-side annualised capital/fixed-O&M headroom is ~**S$52.2m/y**, not only a few million. The S$54m/y abatement allowance is available in addition to operating savings.

Against ~S$52m/y headroom:
- HTGR-200 thermal-share allocation does not fit (~S$100m/y before integration);
- MHR-T thermal-share allocation approximately reaches the boundary (~S$50m/y), leaving little room for IHX/secondary-loop/reformer modifications;
- charging the full large-reactor annual cost to one hydrogen train is far outside the boundary.

Thus a dedicated large HTGR is strongly disfavoured by the current economic screen, while a large shared/cogeneration reactor is **not yet falsified** but survives only in a narrow favourable region.

## Source-backed integration cost closes the midpoint shared-reactor overlap
IAEA TECDOC-1682 gives a directly relevant GTHTR300C component estimate: **USD69.0m for the IHX + secondary helium loop** in a 600 MWth cogeneration system delivering 170 MW high-temperature process heat and 202 MWe. This is preliminary/legacy and estimated from HTTR construction cost, but its process-heat scale closely matches the project's 145-179 MWth envelope. citeturn0search48

At 8%/25y, USD69m annualises to ~USD6.46m/y before integration-specific O&M, or order ~S$8.2m/y at 1.276 SGD/USD. General Atomics independently estimated an intermediate helium loop at USD43/kWth and USD0.1/MWth-h O&M, confirming nonzero integration cost but with materially different scope/basis. citeturn0search49 JAEA also identifies high-temperature creep/material requirements as an IHX manufacturing-cost issue. citeturn0search0

The previously surviving representative midpoint had only order ~S$2m/y residual after the MHR-T thermal-share reactor allocation. Therefore that midpoint **fails once the IAEA IHX/secondary-loop anchor is added**, even before reformer modification, tail-gas recycle equipment, additional O&M or contingency.

This does not yet falsify all shared-reactor cases: higher NG value, lower T&S, smaller allocation fraction or lower integration cost can reopen headroom. The next task is to quantify that surviving sensitivity region rather than treating the midpoint as feasible.

## Shared-reactor full-cost region resolved
A full-cost boundary now reserves: ~S$50m/y MHR-T thermal-share reactor allocation, ~S$8.2m/y annualised IAEA IHX+secondary-loop anchor, and an explicit **S$5m/y reformer/recycle integration sensitivity**. The S$5m/y term is deliberately nonzero but is not claimed as a source-derived helium-reformer retrofit cost. Modern eSMR literature supports reuse of existing reformer infrastructure, but no directly transferable helium-heated retrofit CAPEX was found. citeturn0search1turn0search2

For the 162 MWth midpoint, legacy S$5.69/GJ nuclear-heat operating anchor, S$150/MWh separation power and low Group-A T&S (~S$31.9m/y), the minimum NG value required by the current S$100/t screen is approximately **S$17.5/GJ**. Thus S$10 and S$15/GJ cases fail; S$20/GJ survives this screening combination.

At S$20/GJ gas, maximum compatible T&S is ~**S$42.8m/y**. The low Group-A sensitivity (~S$31.9m/y) fits, while the high Group-A sensitivity (~S$47.85m/y) does not.

Therefore the shared architecture retains a real but narrow favourable region; it is not globally falsified. Dedicated large HTGR remains outside the current region.

The next dependency is lifecycle consistency: the S$54m/y budget was derived from an illustrative 7.5 kgCO2e/kgH2 abatement. The 80% recycle case reduces fresh NG and changes upstream emissions/capture mass, so its actual annual abatement and allowable S$100/t budget must now be recomputed on the same physical case.

## Matched lifecycle denominator materially reopens shared-reactor region
The 80% recycle/shared-HTGR economic case is now matched to its own lifecycle inventory instead of retaining the illustrative 7.5 kgCO2e/kgH2 denominator.

Using the global-gas upstream anchor (11.5 gCO2e/MJ), 90% capture of reduced external feed carbon, 162 MWth direct nuclear service, 5.5 gCO2e/kWh_e nuclear LCA proxy at 50.4% efficiency, and 2.5% captured-CO2 transport-emission sensitivity:
- unabated IEAGHG lifecycle screen ~= **10.813 kgCO2e/kgH2**;
- recycle/shared-HTGR candidate ~= **1.890 kgCO2e/kgH2**;
- specific abatement ~= **8.923 kgCO2e/kgH2**;
- at 8994 kgH2/h and 8322 h/y, annual abatement ~= **0.668 MtCO2e/y**;
- matched S$100/t annual cost allowance ~= **S$66.8m/y**.

This replaces the inconsistent ~S$54m/y allowance previously used for the recycle case.

Rerunning the full-cost boundary with ~S$50m/y MHR-T allocation, ~S$8.2m/y IHX/loop, S$5m/y reformer/recycle allowance, 162 MWth, S$5.69/GJ nuclear heat and S$150/MWh separation power:
- low Group-A T&S (~S$31.9m/y) requires NG value only ~**S$14.6/GJ**;
- at S$20/GJ gas, maximum T&S rises to ~**S$55.5m/y**, above both prior Group-A sensitivities;
- at S$15/GJ gas, max T&S is only ~**S$33.7m/y**, so the low Group-A end is marginally feasible but the high end is not.

Thus lifecycle consistency reopens a meaningful part of the shared-reactor feasibility region. Dedicated large HTGR remains outside the current screen.

The central remaining weakness is that 80% recycle and 90% capture are still reduced-model assumptions rather than a converged recycle flowsheet.

## Preserved findings
- PSA tail gas remains a first-order nuclear-integration constraint.
- Replacing make-up furnace NG alone does not remove feedstock carbon.
- Existing HTS C/H/O closure and reaction-extent verification remain valid.
- The 298 K reaction-duty calculation is a lower thermochemical layer, NOT
  reformer furnace duty.
- Captured CO2 and avoided CO2 remain separate metrics.
- No final nuclear reactor or CCS topology has been selected.

## Valid blockers still open
1. **Conventional energy balance:** temperature-dependent enthalpy, steam
   generation/superheat, reformer duty, heat recovery and furnace losses are not
   yet closed against an authoritative benchmark.
2. **PSA-tail-gas disposition:** every species needs a defined destination in
   each furnace-free/nuclear candidate.
3. **Capture topology:** cannot be frozen until tail-gas/carbon architecture is
   selected; stream-specific pressure/composition must drive solvent choice and
   regeneration duty.
4. **Common comparison specification:** 1 kg H2 is the canonical mass basis,
   but final product pressure and full nested boundary matrix still require
   authoritative selection.

## Major items coupled to blockers
- Add stream-specific CCS literature/parameters.
- Build thermodynamic property layer.
- Preserve nuclear integration as a heat-temperature envelope before mapping
  reactor concepts.
- Freeze transparent incremental economic assumptions before assignment-level
  S$/tCO2e results.
- Add independent predictive validation beyond source reconstruction.

## Verification status
- Rust regression for the corrected stream-4/5 interpretation and inferred interstage water addition passed GitHub Actions.
- IEAGHG baseline energy-invariant tests also passed GitHub Actions.
- A later auxiliary reformer diagnostic is still running; it is not required for the already-passed reconciled closure test.
- No production simulation is accepted as a scientific result.
- No detailed integrated nuclear model is permitted while Review 1 blockers
  remain.

## Next highest-priority task
Implement an iterative fixed-H2 tail-recycle closure. Starting from the IEAGHG PSA tail composition, remove CO2, recycle H2/CO/CH4, reduce fresh NG to hold H2 product fixed, apply reaction conversion and PSA recovery, regenerate the new tail composition, and iterate to convergence. Then recompute fresh-NG energy, external carbon, capture mass, process heat, lifecycle emissions and the S$100/t boundary from the converged state. Do not tighten economic conclusions further until this physical closure replaces the 80% sensitivity.
