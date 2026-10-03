# Figures 1–3: Exact Content and Flow Specification

This file defines what Figures 1–3 are intended to communicate. Externally generated artwork may replace the current schematic artwork, but it must preserve these scientific roles and flows.

Geometry/artwork is schematic, not a mechanical design drawing. Do not infer equipment dimensions from the figures.

## Figure 1 — Conventional SMR + CCS reference

**Purpose:** show the conventional hydrogen-production reference before nuclear heat is introduced.

**Reactor:** none.

**Core process:** conventional Steam Methane Reforming (SMR) with Carbon Capture and Storage (CCS).

**Required flow:**

Natural gas + steam
→ fired Steam Methane Reformer (SMR)
→ hydrogen-rich synthesis gas
→ Water-Gas Shift (WGS)
→ carbon-dioxide capture
→ Pressure Swing Adsorption (PSA)
→ hydrogen product.

A separate conventional heat input must be clear:

fired furnace / fossil combustion
→ high-temperature reformer duty.

Captured carbon dioxide follows:

CO2 capture
→ conditioning/compression
→ Transport and Storage (T&S) boundary.

**Key scientific message:** methane is both chemical feedstock and, conventionally, part of the fired heat supply. CCS captures the represented concentrated carbon stream, while the fired heat source contributes combustion emissions.

**Numbers:** Figure 1 is primarily conceptual. Do not add unsupported temperatures, duties, capture purities, pressures or equipment sizes merely to make the artwork look complete.

## Figure 2 — Reference HTGR electricity-generation pathway

**Purpose:** teach the reader how an HTGR moves nuclear heat through helium and show a conventional/reference power pathway. It is not the proposed CN4252 project's electricity output.

**Reactor:** generic/reference **High-Temperature Gas-Cooled Reactor (HTGR)**. It is an orientation reactor, not a claim that the proposed project produces a specified electrical output.

**Required nuclear flow:**

nuclear fission/core heat
→ primary helium hot outlet
→ heat-transfer / steam-generation equipment
→ primary-helium return
→ HTGR.

The primary helium must form a visibly closed reactor loop.

**Required power-cycle flow:**

heat crosses from the helium side
→ separate steam/water circuit
→ steam
→ turbine
→ mechanical shaft
→ generator
→ electricity.

Return side:

turbine exhaust
→ condenser
→ condensate
→ pump
→ feedwater
→ steam generator / heat-transfer equipment.

**Key scientific message:** primary helium transports reactor heat; a separate power cycle can convert transferred heat to electricity. This figure is a reference architecture only.

**Numbers:** do **not** show or imply a numerical project electricity output. Do not label the project's 423.2 MWth remaining thermal capacity as electricity, turbine input, or electrical generation.

## Figure 3 — Proposed HTGR-assisted SMR + CCS system

**Purpose:** show the proposed implementation and the physical separation of nuclear coolant, secondary process-heat helium, and chemical process gas.

### Reactor/design basis

**Selected reactor basis:** 600 MWth **GTHTR300C-class** high-temperature gas-cooled reactor design basis.

This is a literature design basis for screening; it is not a claim that a 600 MWth project is operating in Singapore.

**Reactor outlet orientation:** 925 °C.

### Nuclear primary-helium loop

Required closed flow:

HTGR hot primary-helium nozzle
→ primary-He pipe
→ Intermediate Heat Exchanger (IHX) primary inlet
→ heat transfer across IHX
→ IHX primary outlet
→ primary-He return pipe
→ HTGR return nozzle.

Primary reactor helium must **not** visually enter the chemical plant.

### Secondary-helium process-heat loop

Required closed flow:

IHX secondary outlet
→ secondary helium at 900 °C
→ source-basis helium flow 78.49 kg/s
→ SMR heat-transfer side
→ supplies 176.8 MWth reformer process heat
→ secondary-helium return at approximately 466 °C
→ IHX secondary-return inlet.

The secondary helium transfers heat to the reformer but does **not** mix with natural gas/steam or synthesis gas.

### Chemical process train

Feed:

natural gas + steam, with S/C = 3.0
→ SMR chemical/process side.

Then:

SMR
→ reformer/process outlet at 871 °C
→ WGS
→ CO2 capture
→ H2-rich gas
→ PSA
→ H2 product.

**PSA:** 88% H2 recovery. This is recovery, **not hydrogen purity**.

**H2 product basis:** 130 MMSCFD.

**Annualised project result:** approximately 97,946 t H2/y at the canonical availability basis. This is derived from the product basis/model and should not be presented as an independent literature reactor parameter.

### Carbon-dioxide branch

CO2 capture
→ captured CO2
→ conditioning/compression
→ conditional Transport and Storage (T&S) boundary.

The figure must not imply that a specific operating Singapore cross-border storage chain has already been secured.

### Reactor thermal allocation

Reformer process heat:
**176.8 MWth**.

600 MWth design basis minus 176.8 MWth reformer allocation:
**423.2 MWth remaining reactor thermal capacity**.

The 423.2 MWth quantity is:
- thermal capacity remaining after the modelled reformer allocation;
- **not** electricity output;
- **not** turbine input;
- **not** cooling duty;
- **not** waste heat.

### IHX/source-branch qualifications

The literature approximately **170 MWth physical-IHX reference** and approximately **370/371 MWth source process-heat branch** are different quantities.

Do not depict 370/371 MWth as the rating of one physical IHX.

Neither quantity by itself establishes a qualified commercial 176.8 MWth project IHX. Project-scale IHX qualification remains a deployment question.

## Artwork replacement requirements

Replacement artwork may be more realistic/visually detailed than the current deterministic TikZ figures. It may show recognisable reactor vessels, heat exchangers, reformers, columns, PSA beds, turbines, generators and connected piping.

However, the replacement must preserve:

- the figure's role above;
- correct equipment-to-equipment topology;
- pipes visibly touching the intended equipment/nozzles;
- separation of primary He, secondary He and process gas;
- flow direction;
- canonical Figure-3 values and their meanings;
- no unsupported numbers;
- no project electricity claim;
- captions that expand acronyms and preserve qualifications.

For Figure 3, a simple fixed-colour legend is preferred for Primary He, Secondary He, Process gas, CO2 and H2. Figures 1–2 may use only the categories they need.
