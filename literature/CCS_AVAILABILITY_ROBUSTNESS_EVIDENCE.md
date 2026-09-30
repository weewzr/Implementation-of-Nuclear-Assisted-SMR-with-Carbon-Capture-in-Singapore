# CCS Availability and Partial-Capture Robustness Evidence

## Research question

How dependent is the CN4252 result on the assumed CO2 capture/storage service, and what does Singapore actually have today?

## Singapore infrastructure status

Singapore has no known suitable domestic geological formations for permanent CO2 storage. MTI therefore treats CCS as a **cross-border** decarbonisation pathway and is working with countries including Australia, Indonesia and Malaysia.

Evidence of progress:
- Singapore–Indonesia CCS Letter of Intent (2024);
- Singapore–Malaysia CCS MOU (January 2025);
- Singapore–Indonesia CCS MOU (June 2025);
- continued 2026 government work on bilateral agreements and private-sector investment certainty.

The June 2025 Singapore–Indonesia statement describes a proposed Singapore project of **2 MtCO2/y** as a pathfinder concept. The project's canonical captured flow (~0.542 Mt/y) is below that headline scale.

Critical qualification: MTI stated in April 2025 that Government was still studying the full capture/transport/storage value chain and obtaining clearer cost estimates. Current MTI material still describes Singapore as **exploring** cross-border CCS.

Therefore:
- regional geological storage potential: SUPPORTED;
- government-to-government cooperation: SUPPORTED;
- operating Singapore cross-border CCS service: NOT DEMONSTRATED;
- contracted project storage capacity/tariff: NOT DEMONSTRATED;
- project's S$15/t transport/storage value: SCREENING ASSUMPTION, not a market quote.

## Capture-rate evidence

IEAGHG's merchant-SMR CCS study evaluates several capture locations/configurations. Mature/reference cases include:
- shifted-syngas MDEA Case 1A: ~55.7% overall CO2 capture;
- flue-gas MEA Case 3: ~90% capture.

These values demonstrate that “capture rate” depends on capture location and flowsheet. They are not direct alternative operating points for the INL nuclear-heated process.

The project's source process already reports a captured stream of 1,927 short ton/day and emitted stream of 142 short ton/day at the selected operating point. The new Rust sensitivity therefore varies **the fraction of that canonical captured stream actually captured/stored**, not a generic percentage of total carbon entering the plant.

## Screening sensitivity definition

Let f be the delivered fraction of the canonical captured stream, 0 <= f <= 1.

At f = 1:
- canonical final design is recovered exactly.

For f < 1:
- H2 production, natural-gas use and nuclear heat remain at the source operating point;
- captured/stored tonnes scale with f;
- the uncaptured portion of the canonical captured stream is returned to direct atmospheric emissions;
- T&S lifecycle burden and T&S variable cost scale with captured tonnes;
- screening CCS capital is scaled with captured tonnes using the existing project cost model;
- process electricity is held fixed.

This is deliberately **not** an absorber/compressor turndown model. Real partial capture can change energy, solvent circulation, compression, equipment utilisation and costs.

## Why this sensitivity is useful

It answers a narrow robustness question:

**If the Singapore CCS chain cannot accept the full canonical captured stream, how much of that stream must still be captured/stored for the CN4252 numerical threshold to remain satisfied under the existing screening model?**

It does not establish:
- capture-plant operability at that fraction;
- storage-chain availability;
- contract structure;
- dynamic outage response;
- bankable CCS cost.

## Infrastructure-availability interpretation

A cross-border storage outage is not equivalent to lowering steady capture fraction. Depending on storage/buffer capacity and plant design, a prolonged outage could require:
- venting captured CO2;
- reducing capture;
- reducing hydrogen production;
- temporary CO2 storage;
- plant shutdown.

The project currently lacks dynamic CO2 buffer/storage and contract availability data. Therefore the steady-f sensitivity is a **screening proxy**, not an operational outage model.

## Current dispositions

- DF-27 cross-border CCS availability: **PARTIALLY ANSWERED — regional cooperation/capacity concept exists; operating contracted service remains unresolved**.
- DF-28 partial/unavailable CCS: **RUST SCREENING SENSITIVITY IMPLEMENTED; dynamic outage response unresolved**.
- CN4252 robustness: exact threshold fraction is generated deterministically by Rust and printed in Paper/reproducibility CI logs.

## Sources

- Singapore MTI, *Carbon* (current cross-border CCS policy/status).
- Singapore MTI, Written Reply on projected CCS cost, 8 Apr 2025.
- Singapore–Indonesia CCS LOI, 15 Feb 2024.
- Singapore–Malaysia CCS MOU, 7 Jan 2025.
- Singapore–Indonesia CCS MOU / ministerial speech, 13 Jun 2025.
- IEAGHG 2017-02 / 2017 technical reviews on merchant SMR CCS capture configurations.
