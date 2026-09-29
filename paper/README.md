# Paper and reproducibility

The canonical manuscript is `paper/main.tex`. The PDF is generated and must
not be edited independently.

## Full reproduction

```bash
sh scripts/reproduce.sh
```

This runs the Rust test suite, regenerates Gate-5 result data, and compiles the
LaTeX manuscript with `latexmk`.

## Scientific provenance

- computational source: `model/src/lib.rs`;
- generated result contract: `results/GATE5_RESULTS.md`;
- Gate-5 closure: `results/gate5_closure.md`;
- literature matrix: `literature/RESEARCH_MATRIX.md`;
- manuscript: `paper/`.

The manuscript must preserve the verified adverse result: zero joint passes in
the declared 64-case nuclear-assisted domain. A successful paper build is not
permission to change the model or expand the domain to seek a favourable result.
