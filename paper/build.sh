#!/bin/sh
set -eu
ROOT=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
cd "$ROOT"
(
  cd model
  cargo test --all-targets
)
sh model/scripts/generate_gate5_results.sh
cd paper
latexmk -C
latexmk -pdf -interaction=nonstopmode -halt-on-error main.tex
if grep -E "undefined references|undefined citations|Citation .* undefined|Reference .* undefined" main.log; then
  echo "Unresolved LaTeX references/citations detected" >&2
  exit 1
fi
