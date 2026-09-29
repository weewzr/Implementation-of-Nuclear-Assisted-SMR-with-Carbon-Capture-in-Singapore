#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
mkdir -p ../results/generated
cargo run --quiet --bin gate5_threshold_scatter_csv > ../results/generated/gate5_threshold_scatter.csv
cargo run --quiet --bin gate5_binding_counts_csv > ../results/generated/gate5_binding_counts.csv
cargo run --quiet --bin gate5_driver_csv > ../results/generated/gate5_driver_effects.csv
cargo run --quiet --bin gate5_results_synthesis > ../results/generated/gate5_results_synthesis.md
cargo run --quiet --bin gate5_figure_manifest > ../results/generated/gate5_figure_manifest.md
