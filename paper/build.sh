#!/bin/sh
set -eu
ROOT=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
cd "$ROOT"
sh model/scripts/generate_gate5_results.sh
cd paper
latexmk -pdf -interaction=nonstopmode -halt-on-error main.tex
