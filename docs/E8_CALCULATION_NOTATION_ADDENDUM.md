# E8 Addendum — Calculation Legibility, Notation and Two-Train Derivation

## Authority

This is a bounded addendum to:

`docs/E8_MANUSCRIPT_REINTEGRATION_PROMPT.md`

Main Research should incorporate this requirement during E8.

Do not restart E8.
Do not redo E1–E7.
Do not change canonical scientific results merely for presentation.
Do not begin independent review.

## Purpose

The final CN4252 report must not merely contain correct headline numbers.

Every important project calculation must be **legible, auditable and understandable to an engineering reader**.

Use a consistent presentation pattern:

**physical question
→ governing equation
→ symbol and unit definitions
→ cited/justified input values
→ numerical substitution
→ numerical result
→ concise engineering interpretation.**

Avoid both extremes:
- unexplained headline numbers;
- pages of algebra without physical explanation.

The reader should understand why each equation is present and what its result means.

## 1. Notation standard

For every material equation:

1. introduce the engineering question in one or two sentences;
2. write the general equation before numerical substitution where useful;
3. define every non-obvious symbol;
4. state units;
5. identify externally sourced inputs with local citations;
6. identify project assumptions explicitly;
7. identify project-derived quantities as derived;
8. show numerical substitution for important headline results;
9. state the result with sensible significant figures;
10. follow with a short paragraph/sentence interpreting the result physically.

Use one consistent symbol for the same physical quantity throughout the manuscript where practical.

Avoid unnecessarily complicated subscripts.

Do not introduce symbols used only once if ordinary prose is clearer.

## 2. Per-train versus final two-train architecture

The manuscript currently contains valuable one-train/source derivations.

Do NOT delete them blindly.

Instead make the hierarchy explicit:

**SOURCE / PER-TRAIN BASIS
→ PROJECT TWO-TRAIN SCALING
→ FINAL PREFERRED ARCHITECTURE RESULT.**

The preferred future architecture uses:

[
N_{\rm train}=2.
]

For quantities that legitimately scale linearly with identical trains, show the derivation explicitly.

### Process heat

Per train:

[
\dot Q_{\rm process,train}=176.8~\mathrm{MW_{th}}.
]

Preferred architecture:

[
\dot Q_{\rm process,total}
=N_{\rm train}\dot Q_{\rm process,train}
=2(176.8)
=353.6~\mathrm{MW_{th}}.
]

Then explain that 353.6 MWth is below the approximately 370 MWth source process-heat branch and that this is why two trains, rather than three, were retained in E2B.

### Hydrogen

Show:

[
M_{H_2,\rm total}
=N_{\rm train}M_{H_2,\rm train}
=2(97{,}946)
\approx195{,}892~\mathrm{t/y}.
]

Preserve the underlying annualisation from 29,000 lb/h and 85% availability.

### Natural gas

Where the identical-train scaling is valid:

[
F_{\rm NG,total}
=2(34.0)
=68.0~\mathrm{MMSCFD}.
]

Explain that this is the operating candidate feed/fuel scale for the preferred two-train architecture, not a literature-reported two-train plant.

### Captured CO2

Show the two-train scaling from the per-train source annualisation:

[
M_{\rm CO_2,captured,total}
=2M_{\rm CO_2,captured,train}
\approx1.085~\mathrm{Mt/y}.
]

### Lifecycle abatement

Do not merely double a headline number without explaining the boundary.

Show why E2B scales the identical project train result and distinguish this from E2's cross-source comparator boundary.

State clearly why the preferred mature two-train result is approximately:

[
A_{\rm lifecycle,total}\approx1.834~\mathrm{MtCO_2e/y}.
]

Do not silently mix E2 and E2B denominators.

## 3. Reactor/process-heat notation

Preserve and improve the good existing structure for:
- 925 → 900 → 871 °C temperature cascade;
- 25 K reactor-to-secondary-He difference;
- 29 K reported secondary-He-to-reformer difference;
- helium sensible-heat consistency check;
- 600 MWth source rating;
- 370/371 MWth process-heat branch;
- 170 MWth reference physical IHX;
- 176.8 MWth per-train process duty;
- 353.6 MWth preferred two-train duty.

The final report must make these distinctions impossible to confuse.

For the preferred architecture show process-heat branch utilisation explicitly, for example:

[
f_{\rm branch,2train}
=\frac{353.6}{370}
\approx0.956.
]

Interpretation:
approximately 95.6% of the rounded source process-heat branch is used by the two reformer trains.

Do NOT imply this is 95.6% of total 600 MWth reactor rating.

Also show:

[
f_{\rm reactor,2train}
=\frac{353.6}{600}
\approx0.589.
]

Interpretation:
approximately 58.9% of total reactor thermal rating is delivered as reformer process heat in the preferred architecture.

## 4. Carbon and lifecycle notation

Retain the existing strong sequence:

source daily rate
→ unit conversion
→ annualisation
→ direct emissions
→ upstream emissions
→ nuclear lifecycle burden
→ auxiliary burden
→ T&S burden
→ lifecycle total/avoided emissions.

For every lifecycle term state whether it is:
- SAVING;
- ADDED BURDEN;
- SOURCE VALUE;
- PROJECT ASSUMPTION;
- PROJECT DERIVATION.

Explain in prose why captured CO2 is not added twice.

For the preferred two-train architecture, provide a compact final lifecycle ledger rather than forcing the reader to infer it from one-train equations.

## 5. Economics — REQUIRED FULL DERIVATION

The final economic section must not simply state:

“S$42.84/tCO2e.”

Show the complete forward chain used by E2B.

At minimum establish notation for:

- source overnight/capital cost basis;
- FOAK/BOAK/10-OAK maturation factor;
- cost-year escalation;
- currency conversion;
- annualisation/CRF;
- annual nuclear O&M;
- annual NG cost;
- CCS cost;
- T&S cost;
- integration/IHX cost;
- annual baseline represented cost;
- annual candidate represented cost;
- net incremental annual cost;
- lifecycle avoided emissions;
- final abatement cost.

Use clear notation such as, where consistent with existing code:

[
C_{\rm abat}
=
\frac{C_{\rm cand,annual}-C_{\rm base,annual}}
{A_{\rm lifecycle}}.
]

Define:
- (C_{\rm abat}): abatement cost (S$/tCO2e);
- (C_{\rm cand,annual}): represented candidate annual cost (S$/y);
- (C_{\rm base,annual}): represented baseline annual cost (S$/y);
- (A_{\rm lifecycle}): annual lifecycle emissions avoided (tCO2e/y).

Then show the actual preferred mature substitution using canonical E2B values:

[
C_{\rm abat,mature}
=
\frac{616.814-538.230}{1.834278}
\approx42.84~\mathrm{S\$/tCO_2e},
]

with monetary values expressed consistently in S$ million/y and abatement in MtCO2e/y so the unit cancellation is clear.

Explain immediately afterward that:
- this is the preferred **projected mature/10-OAK** case;
- it is not an observed Singapore project quotation;
- it contains no electricity revenue;
- it contains no fictional heat-sharing customer;
- unresolved project costs remain subject to the E7 gates.

Also show the equivalent FOAK and BOAK calculations or a compact table with sufficient derivation/provenance.

## 6. FOAK → BOAK → 10-OAK notation

Do not merely quote the three values.

Explain the maturation method succinctly.

Where E2B uses the INL learning equation, show:

[
C_N=C_1(1-LR)^{\log_2 N},
]

define:
- (C_N);
- (C_1);
- (LR);
- (N).

State the source and evidence maturity.

Show why the selected 10-OAK cost point is an independently sourced/projected maturation case rather than a value chosen to make the CN4252 threshold pass.

Keep the back-calculated S$100 threshold as a diagnostic, not the derivation of the mature result.

## 7. Cooling/site calculation notation

Preserve E3's distinction:

[
\dot m
=
\frac{\dot Q}{c_p\Delta T}.
]

Define all quantities and explain that the 246.4 MWth and 600 MWth cases are illustrative heat-sink envelopes, not the canonical operating/decay-heat duty.

Where water volumetric flow is reported, show density conversion or state the density basis.

Do not let a marker interpret 7.53 m3/s as a designed nuclear cooling-water requirement.

## 8. Safety calculations

Do not manufacture equations merely to make the safety section look quantitative.

For E4/E5:
- show equations only where they clarify an actual calculable/parameterised quantity;
- explain why missing inputs prevent numerical PRA/QRA/source-term/dose results;
- preserve parameterised equations for release, inventory, risk and mechanistic source term where useful;
- follow each with plain-language explanation of what data are missing.

Good notation includes knowing when NOT to insert a number.

## 9. Calculation-to-prose rule

Every important displayed calculation should be followed by a concise interpretation.

Examples:

“Thus the two reformers use 353.6 MWth, or 95.6% of the rounded 370 MWth process-heat branch, leaving little process-heat-branch margin.”

“Thus the preferred mature configuration produces approximately 196 ktH2/y at the declared 85% availability.”

“Thus the forward mature economic model gives S$42.84/tCO2e, below the CN4252 S$100/t threshold, but only for the projected mature deployment case.”

Avoid paragraphs that simply repeat the equation numerically.

Explain the engineering consequence.

## 10. Significant figures and units

Use sensible engineering precision.

Do not show excessive decimals in submission-facing prose.

Machine-readable outputs may retain full precision.

Preferred manuscript style:
- 353.6 MWth;
- 195,892 tH2/y or ~196 kt/y depending context;
- ~1.085 MtCO2/y;
- ~1.834 MtCO2e/y;
- S$137.74/tCO2e where needed for threshold comparison;
- S$74.14/tCO2e;
- S$42.84/tCO2e.

Be consistent between tables, equations and prose.

Use:
MWth / MWe
consistently and avoid ambiguous “MW” where heat/electricity matters.

## 11. Citation/provenance beside inputs

Every external numerical input must have its citation close to where it is introduced.

Examples:
- 176.8 MWth → INL;
- 600 MWth → JAEA/Nishihara;
- 370/371 MWth → JAEA/Nishihara;
- INL cost method → INL cost report;
- availability → correct source/assumption;
- lifecycle factors → corresponding source.

Project-derived arithmetic does not need a fake external citation; cite/identify its inputs.

Project assumptions must be labelled as assumptions.

## 12. Calculation summary table

Consider a compact final table:

Calculation | Equation/input | Result | Meaning | Evidence class

for the principal project outputs.

Do not make it an enormous provenance register.

Its purpose is to let a marker trace:
HEAT
→ H2
→ NG
→ CO2
→ LIFECYCLE
→ COST
→ CN4252 THRESHOLD.

## 13. Acceptance test

The calculation presentation passes only if a technically trained reader can answer for every headline number:

WHAT IS BEING CALCULATED?

WHAT EQUATION IS USED?

WHAT DOES EACH SYMBOL MEAN?

WHAT ARE THE UNITS?

WHERE DO THE INPUTS COME FROM?

WHAT IS ASSUMED?

WHAT IS PROJECT-DERIVED?

HOW ARE THE NUMBERS SUBSTITUTED?

WHAT IS THE RESULT?

WHAT DOES THE RESULT MEAN PHYSICALLY?

WHAT DOES IT NOT MEAN?

If the reader has to inspect Rust or repository Markdown merely to understand a headline report calculation, the manuscript presentation is incomplete.

## 14. E8 report requirement

When E8 reports completion, add:

CALCULATION/NOTATION QUALITY PASS: PASS/FAIL

PER-TRAIN → TWO-TRAIN DERIVATION EXPLICIT: YES/NO

MATURE S$42.84/t DERIVATION SHOWN: YES/NO

FOAK/BOAK/10-OAK METHOD EXPLAINED: YES/NO

HEADLINE NUMBERS HAVE LOCAL PROVENANCE: YES/NO

DISPLAYED EQUATIONS HAVE SUCCINCT INTERPRETATION: YES/NO

UNIT/SYMBOL CONSISTENCY CHECK: PASS/FAIL

Then continue with the existing E8 STOP condition.

Do not begin external visual generation or independent review automatically.
