# Canonical paper

`paper/main.tex` is the single canonical manuscript source.

## Build

From repository root:

```bash
sh paper/build.sh
```

The script first regenerates Gate-5 result data from the verified Rust model, then builds `paper/main.pdf` with `latexmk`.

Requirements:
- stable Rust/Cargo;
- a LaTeX distribution providing `latexmk`, `pdflatex`, `natbib`, `siunitx`, `booktabs`, `microtype` and standard AMS packages.

The PDF is generated and must not be edited independently. The GitHub Actions paper workflow also rejects unresolved LaTeX citations/references and uploads the successfully generated PDF as the `cn4252-manuscript` workflow artifact.

## Scientific provenance

- computation: `model/src/lib.rs`;
- Gate-5 data generation: `model/scripts/generate_gate5_results.sh`;
- result contract: `results/GATE5_RESULTS.md`;
- bibliography: `paper/references.bib`;
- manuscript: `paper/main.tex` + `paper/sections/*.tex`.

The historical `manuscript/` skeleton is superseded and is not a competing manuscript.

Review-4 manuscript inputs are generated deterministically as `gate6_domain_table.csv`, `gate6_threshold_magnitude.csv`, and `gate6_threshold_plot_bounds.csv`; these expose the tested 64-case domain, threshold-failure magnitudes, and finite-data plot bounds without duplicating scientific values in LaTeX.
