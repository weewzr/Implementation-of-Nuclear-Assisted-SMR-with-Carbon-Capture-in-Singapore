# Replace Figures 1–3 with User Attachments — Main Research Instruction

## Authority and scope

The user has now uploaded the intended Figures 1–3 to the repository:

- `attachments/figure1_conventional_smr_ccs.png`
- `attachments/figure2_htgr_reference_power.png`
- `attachments/figure3_proposed_htgr_smr_ccs.png`

The user explicitly wants these attachment figures used in the report in place of the currently generated/recreated Figures 1–3 because the attachments better represent the intended concept.

This is a **bounded figure-replacement and verification task**. It temporarily takes priority over advancing to the next progressive manuscript phase.

Do not begin W5 until this task is complete and verified.

## 1. Inspect the actual images

Visually inspect all three PNGs at full resolution before editing the manuscript.

Do not rely on filenames alone.

For each figure determine:
- what equipment is actually depicted;
- process/nuclear flow direction;
- labels and numerical annotations;
- whether pipes/arrows connect correctly;
- whether primary helium, secondary helium, process gas, H2 and CO2 are distinguishable where applicable;
- whether any annotation conflicts with the canonical final design.

Use `docs/FIGURE_1_2_3_SPECIFICATION.md` as the scientific acceptance specification.

## 2. Intended mapping

Unless visual inspection proves the uploaded file is mismatched:

### Figure 1
Use:
`attachments/figure1_conventional_smr_ccs.png`

Role:
**Conventional SMR-H2 + CCS reference**

It should communicate:
natural gas + steam → fired reformer → WGS → CO2 capture → PSA → H2,
with conventional fired high-temperature heat and captured CO2 proceeding toward conditioning/T&S.

### Figure 2
Use:
`attachments/figure2_htgr_reference_power.png`

Role:
**Reference/conventional HTGR energy/power pathway**

It should orient the reader to:
fission/core heat → primary helium → heat-transfer/steam-generation equipment → separate steam/water power cycle → turbine/generator.

It must NOT imply a numerical electricity output for the proposed CN4252 project.

### Figure 3
Use:
`attachments/figure3_proposed_htgr_smr_ccs.png`

Role:
**Proposed HTGR-assisted SMR-H2 + CCS system**

It should communicate:
fission → reactor/core → primary helium → IHX → secondary helium → reformer heat → SMR-H2 → WGS → CO2 capture → PSA → H2,
with captured CO2 proceeding toward the conditional conditioning/transport/storage boundary.

## 3. Prefer the user attachments

The user's intention is to replace the generated/recreated Figures 1–3 with these uploaded images.

Therefore, if an attachment is scientifically acceptable, **use it directly** rather than recreating another schematic.

Do not replace it with the previous Rust/TikZ artwork merely because the old artwork is easier to reproduce.

The uploaded images are user-provided project artwork and may be included as such.

## 4. Scientific validation before inclusion

Before inserting each image, check it against the canonical model and `docs/FIGURE_1_2_3_SPECIFICATION.md`.

For Figure 3 in particular preserve these distinctions:

- primary reactor helium must not visually enter the chemical process;
- secondary helium must be distinct from primary helium and chemical process gas;
- 600 MWth = GTHTR300C-class design basis;
- 925 °C = source reactor-outlet case;
- 900 °C = secondary-He supply;
- ~466 °C = secondary-He return;
- 78.49 kg/s = source-basis secondary-He flow;
- 176.8 MWth = reformer/process heat requirement;
- 871 °C = reformer/process outlet;
- S/C = 3.0;
- 88% = PSA H2 recovery, NOT purity;
- 130 MMSCFD = H2 product basis;
- ~97,946 t H2/y = project annualised output;
- 423.2 MWth = remaining reactor thermal capacity, NOT electricity, turbine input, cooling duty or waste heat;
- ~170 MWth physical-IHX reference is distinct from the ~370/371 MWth source process-heat branch;
- ~370/371 MWth must NOT be presented as one physical IHX;
- CO2 T&S must remain conditional rather than depicted as guaranteed operating Singapore infrastructure.

## 5. What to do if an attachment contains an error

Do NOT reject the whole attachment for a minor fixable issue.

If the artwork is conceptually correct but contains a small scientific/label problem, prefer one of these bounded solutions:

1. correct the image if an editable source exists and the correction is straightforward;
2. crop/remove a misleading annotation if that preserves the intended artwork;
3. clarify the issue explicitly in the manuscript caption if the artwork itself remains scientifically safe;
4. if the image contains a serious topology error that cannot be safely corrected, STOP and report exactly why before replacing it.

Do not silently retain a scientifically wrong label.

Do not invent new numbers.

## 6. Attribution/provenance

Update `results/FIGURE_PROVENANCE.md`.

Figures 1–3 should be identified honestly as **user-provided project artwork/attachments**, not as Rust-generated figures if they are now directly included PNGs.

Their scientific basis/citations should still be the appropriate primary/technical sources in the manuscript captions/text:
- IEAGHG / INL for SMR-H2/CCS architecture as appropriate;
- INL TEV-953/961 for process states;
- JAEA/JAERI/Nishihara for HTGR/GTHTR300C architecture;
- other established primary sources only where actually relevant.

User artwork is design/visual provenance, not scientific validation.

## 7. Manuscript integration

Update the active Proposed System section so Figures 1–3 use the attachment PNGs.

Use normal LaTeX image inclusion with sensible sizing.

Preserve:
- figure numbering;
- `fig:smr-explainer`;
- `fig:htgr-power-orientation`;
- `fig:proposed-orientation`;

unless there is a compelling technical reason to change labels.

Rewrite captions as needed so they accurately describe the attachment actually shown.

Do not leave captions describing old artwork that is no longer present.

Do not add unnecessary duplicate schematics.

The separate IHX/isolation schematic may remain if it still adds distinct explanatory value; assess rather than automatically remove it.

## 8. Reproducibility interpretation

The replacement Figures 1–3 are user-provided static project artwork, so do not pretend they are generated from Rust.

This is acceptable because these are conceptual architecture figures rather than quantitative plots.

Keep Rust generation for quantitative figures/tables where model/data reproducibility matters.

The correct provenance distinction is:

**conceptual project artwork → user attachment + technical-source validation**

versus

**quantitative scientific figure → deterministic model/data → Rust generator → manuscript**.

## 9. Do not alter science

This task does not authorise changes to:
- canonical model;
- canonical numerical results;
- reactor selection;
- lifecycle calculation;
- economics;
- CN4252 classification;
- W4 scientific basis.

If visual inspection reveals a genuine conflict between the uploaded intended architecture and the canonical scientific model, STOP and report it as a substantive discrepancy.

## 10. Build and visual QA

After integration:

1. build the canonical manuscript;
2. run Research CI;
3. run Paper/reproducibility CI;
4. require both to pass;
5. verify zero undefined citations/references;
6. download/render the exact generated PDF;
7. inspect Figures 1–3 at normal page scale and zoomed scale;
8. verify legibility;
9. verify no clipping, stretching or pixelation severe enough to impair use;
10. verify captions match the actual images;
11. verify W2/W3 narrative still flows correctly;
12. verify no canonical scientific numbers changed.

## 11. Report and stop

Report:

FIGURE REPLACEMENT TASK:

FIGURE 1:
- attachment used?
- visual/scientific assessment
- corrections made
- caption/source treatment

FIGURE 2:
- attachment used?
- visual/scientific assessment
- corrections made
- caption/source treatment

FIGURE 3:
- attachment used?
- visual/scientific assessment
- corrections made
- caption/source treatment

FIGURE PROVENANCE UPDATED?:

OLD GENERATED FIGURES STILL ACTIVE?: YES/NO and why

SCIENTIFIC DISCREPANCIES FOUND:

SCIENTIFIC MODEL CHANGED?: YES/NO

CANONICAL NUMBERS CHANGED?: YES/NO

RESEARCH CI:

PAPER/REPRODUCIBILITY CI:

EXACT PDF INSPECTION:

COMMIT SHA:

Then STOP.

Do not begin W5 automatically.
