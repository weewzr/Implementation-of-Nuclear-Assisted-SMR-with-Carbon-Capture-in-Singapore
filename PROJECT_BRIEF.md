# Project Brief (student working brief)

The project originates from CN4252 — Hydrogen and Low Carbon Technologies. The official CN4252 Problem Statement is authoritative for assignment requirements; this brief records the student's candidate research direction.

## Working title
Nuclear-Assisted Steam Methane Reforming with Amine-Based Carbon Capture for Low-Carbon Hydrogen Production in Singapore

## Candidate concept
Investigate whether a high-temperature nuclear reactor, particularly an HTGR-class SMR scenario, can provide process heat and/or electricity to an SMR hydrogen plant with amine-based CO2 capture, reducing fossil energy use and lifecycle emissions.

## Initial hypothesis
Nuclear-assisted SMR + amine CCS could substantially reduce the carbon intensity of conventional hydrogen production by replacing fossil process heat and supplying low-carbon energy for CCS duties.

## Explicit falsification requirements
The project must test, rather than assume, whether:
- SMR + CCS alone captures most of the benefit;
- reactor temperature is sufficient for useful direct process heat;
- nuclear-electric eSMR is preferable;
- upstream methane emissions materially constrain lifecycle performance;
- electrolysis or another low-carbon route performs better;
- Singapore nuclear/CCS/siting/infrastructure constraints dominate feasibility.

## Initial system boundary
Natural gas -> steam generation -> reforming -> WGS -> H2 purification -> CO2 capture -> solvent regeneration -> CO2 compression -> transport/storage boundary.

Lifecycle extensions should include upstream natural gas, nuclear fuel cycle, and CO2 transport/storage where data are reliable.

## Required outputs envisioned by the student
Scientific/engineering paper, literature matrix, complete PFD, governing equations, mass/energy balances, reactor/steam/SMR/CCS models, emissions calculation, sensitivity and uncertainty analysis, competing configurations, Singapore feasibility, preliminary economics, reproducible Rust model, tests, figures, data provenance and LaTeX manuscript.

## Programming preference
Rust for computation; LaTeX as canonical manuscript. Avoid Python unless explicitly justified.

## Important scope rule
Do not confuse Small Modular Reactor (nuclear SMR) with Steam Methane Reforming (SMR).
