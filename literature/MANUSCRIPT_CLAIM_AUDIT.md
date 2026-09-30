# Active Manuscript Claim Audit — Targeted Extension

Audit basis: active files included by `paper/main.tex`. Historical/non-input sections are excluded from current-claim status but remain in the repository.

| Active section | Paragraph purpose | Source-dependent claims | Audit result / action |
|---|---|---|---|
| Abstract | problem, final configuration, headline result, qualification | INL temperatures/duty; Nishihara architecture | PASS; quantitative result remains model output and feasibility is qualified |
| Introduction | establish SMR/nuclear-heat question and Singapore screening objective | HTGR relevance; Singapore deployment status | PASS; avoids claiming deployment |
| Background | explain SMR/CCS, nuclear heat, IHX isolation | IEAGHG process; INL/JAEA architecture | PASS; primary/authoritative citations attached to engineering claims |
| System boundary | define baseline/candidate/lifecycle/economic boundary | INL same-output baseline | PASS; zero electricity revenue explicit |
| Final design | establish source process, architecture, heat/lifecycle/economic equations | INL TEV-953/961; Nishihara 2007; IEA/UNECE proxies | PASS with limitations explicit |
| Nuclear feasibility/safety | demonstrated vs designed; TRISO; LOFC; IHX; accidents; co-location | JAEA/HTTR, AGR, NRC, IAEA | STRENGTHENED: primary sources added; generic 'safe' language rejected |
| Integrated configuration | show nuclear/process isolation and final flow | INL + Nishihara | PASS; no project electricity claim |
| Final results | report verified model outputs and threshold margins | model output, not external observation | PASS; interpreted as conditional model result |
| CO2/cost ledgers | explain arithmetic and economic boundary | canonical Rust ledger outputs | ADDED; generated data now reconcile directly to final model |
| Comparators | contextual comparison, not ranking | IEAGHG and historical project evidence | PASS; does not overwrite final baseline |
| Singapore feasibility | scale, nuclear readiness, CCS, feedstock, economics, roadmap | MTI/EMA current policy; model outputs | STRENGTHENED; complete CN4252 feasibility dimensions and decision gates added |
| Limitations | state what model does not establish | architecture/source limitations | CORRECTED stale 170 MW statement; project-scale IHX and bankability remain unresolved |
| Discussion | interpret architecture/economics | source/model distinctions | PASS |
| Conclusions | answer numerical and real-world questions separately | model + current Singapore evidence | STRENGTHENED; supported/conditional/unresolved/not-demonstrated categories explicit |
| Reproducibility appendix | reproduce outputs | repository workflow | PASS subject to final CI |
| Traceability appendix | map assignment requirements | official CN4252 statement | PASS; expanded requirements matrix also maintained |

## Paragraph-level rules applied

- Attach citations to the sentence carrying the external claim.
- Do not cite model outputs as though they are literature measurements.
- State facility/configuration and evidence maturity when transferring nuclear evidence.
- Do not transfer HTTR 9 MW transient performance quantitatively to a 600 MWth design.
- Do not equate TRISO retention with zero release.
- Do not equate a designed GTHTR300C architecture with demonstrated commercial operation.
- Do not equate CN4252 numerical threshold passage with licensing, safety or bankability.
- Keep project assumptions/proxies visibly labelled.
- Remove/rewrite paragraphs whose only function was obsolete historical architecture justification.

## Remaining audit risks before Independent Review

1. Final Paper CI must prove all newly added citations resolve.
2. Generated ledger tables and Rust heat figure must be visually inspected at manuscript scale.
3. New feasibility table/roadmap must be checked for page overflow.
4. Bibliography entries for web/technical sources must render cleanly.
5. Exact final PDF must be reread for accidental overstatement introduced by layout/section transitions.
