#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
mkdir -p ../results/generated
cargo run --quiet --bin gate5_threshold_scatter_csv > ../results/generated/gate5_threshold_scatter.csv
cargo run --quiet --bin gate5_threshold_scatter_finite_csv > ../results/generated/gate5_threshold_scatter_finite.csv
cargo run --quiet --bin gate5_binding_counts_csv > ../results/generated/gate5_binding_counts.csv
cargo run --quiet --bin gate5_driver_csv > ../results/generated/gate5_driver_effects.csv
cargo run --quiet --bin gate5_driver_cost_finite_csv > ../results/generated/gate5_driver_cost_finite.csv
cargo run --quiet --bin gate5_case1a_comparator_csv > ../results/generated/gate5_case1a_comparator.csv
cargo run --quiet --bin gate5_case1a_comparator_table_csv > ../results/generated/gate5_case1a_comparator_table.csv
cargo run --quiet --bin gate6_singapore_scale_csv > ../results/generated/gate6_singapore_scale.csv
cargo run --quiet --bin gate6_lifecycle_decomposition_csv > ../results/generated/gate6_lifecycle_decomposition.csv
cargo run --quiet --bin gate6_domain_table_csv > ../results/generated/gate6_domain_table.csv
cargo run --quiet --bin gate6_threshold_magnitude_csv > ../results/generated/gate6_threshold_magnitude.csv
cargo run --quiet --bin gate6_threshold_plot_bounds_csv > ../results/generated/gate6_threshold_plot_bounds.csv
cargo run --quiet --bin gate5_results_synthesis > ../results/generated/gate5_results_synthesis.md
cargo run --quiet --bin gate5_figure_manifest > ../results/generated/gate5_figure_manifest.md

cargo run --quiet --bin deployment_cases_csv > ../results/generated/deployment_cases.csv
cargo run --quiet --bin deployment_cost_breakdown_csv > ../results/generated/deployment_cost_breakdown.csv
cargo run --quiet --bin deployment_scale_curve_csv > ../results/generated/deployment_scale_curve.csv
cargo run --quiet --bin deployment_cost_curve_csv > ../results/generated/deployment_cost_curve.csv
cargo run --quiet --bin deployment_evidence_csv > ../results/generated/deployment_evidence.csv
