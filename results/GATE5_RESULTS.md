# Gate 5 results — canonical experiment artifacts

## Status

Gate-5 Experiments 01-04 are implemented against the verified R3 computational
model. Their tabular outputs are generated directly from Rust and are not
independently maintained by hand.

## Reproduce

From `model/`:

```bash
cargo test --all-targets
cargo run --bin gate5_experiment01
cargo run --bin gate5_experiment02
cargo run --bin gate5_experiment03
cargo run --bin gate5_experiment04
cargo run --bin gate5_uncertainty_csv > ../results/gate5_uncertainty.csv
cargo run --bin gate5_driver_csv > ../results/gate5_driver_effects.csv
cargo run --bin gate5_results_summary > ../results/gate5_results_summary.md
```

The generated files are derivative artifacts. The Rust functions remain the
canonical computational source.

## Experiment map

### Experiment 01 — threshold failure topology
Classifies the 64 coupled nuclear cases and identifies best-abatement,
best-cost and closest-joint cases.

### Experiment 02 — driver attribution
Quantifies local changes in annual abatement and S$/t from reformer temperature,
pressure, PSA recovery, capture fraction, carbon/electricity corner and
heat/fixed-cost corner.

### Experiment 03 — Case-1A comparator
Confirms conventional IEAGHG Case 1A exceeds the 0.25 Mt/y direct-abatement
scale at the common H2 output. Source EUR2014 non-T&S cost and Singapore SGD T&S
scenario contributions remain separate.

### Experiment 04 — binding constraints
Partitions all nuclear cases into abatement-only, cost-only, both-fail and joint
pass classes and reports normalized threshold gaps.

## Results that must not be lost

- Nuclear coupled design: **64 cases**.
- Nuclear joint CN4252 passes: **0**.
- Conservative credible nuclear case: fails both thresholds.
- IEAGHG Case 1A: exceeds 0.25 Mt/y direct avoided CO2 on the common source
  production scale.
- Case-1A Singapore <S$100/t result: not established on a common currency/year
  basis.

These are experimental findings. Do not alter ranges or accounting solely to
create a passing nuclear point.


## Results synthesis layer

The repository now provides a figure-ready threshold dataset and a generated
scientific synthesis:

```bash
cargo run --bin gate5_threshold_scatter_csv > ../results/gate5_threshold_scatter.csv
cargo run --bin gate5_results_synthesis > ../results/gate5_results_synthesis.md
```

The threshold CSV contains all 64 cases with:
- annual avoided tCO2e/y;
- forward S$/tCO2e;
- both assignment threshold coordinates;
- binding-constraint class;
- reformer T/P;
- PSA recovery;
- capture fraction;
- carbon/electricity corner;
- cost corner.

The synthesis deliberately limits claim strength:
**0/64 joint passes is a verified experimental result for the declared domain,
not proof that every conceivable nuclear-assisted configuration fails.**

Case 1A remains a separate source-backed comparator: its annual direct-abatement
scale is comparable, while its Singapore economic result is not merged into the
nuclear S$/t plot without a common currency/year cost basis.


## Reproducible figure-data pipeline

Gate-5 figure inputs are now generated in one deterministic command:

```bash
sh model/scripts/generate_gate5_results.sh
```

This creates under `results/generated/`:
- `gate5_threshold_scatter.csv`
- `gate5_binding_counts.csv`
- `gate5_driver_effects.csv`
- `gate5_results_synthesis.md`
- `gate5_figure_manifest.md`

The figure manifest defines:
1. threshold scatter: annual avoided CO2e vs S$/t with the two CN4252 threshold
   lines and binding classes;
2. binding-class counts;
3. normalized local driver effects.

The repository intentionally keeps plotted graphics derivative from these
canonical generated data. No plotting dependency was added solely for
presentation aesthetics.
