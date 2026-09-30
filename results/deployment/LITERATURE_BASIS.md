# Review-5 deployment remediation — verified evidence basis

## Heat interface
Primary JAEA HTTR steam-reforming evidence is controlling for the deployment process state:
- JAEA-Technology 2018-004: HTTR can supply 950 C helium; the chemical reactor receives about 880 C helium; methane steam reforming is an approximately 800 C-class heat application in the programme-level description.
- JAEA-Technology 2007-022 / HTTR mock-up specifications: process gas 450 C inlet and 580 C outlet; helium 880 C inlet and 585 C outlet at the steam-reformer mock-up.
- JAERI-Tech 99-080: steam-reformer shell-side helium maximum 880 C and tube-side process gas maximum 600 C.

The deployment reference therefore uses 880 C helium at the reformer inlet and 600 C reacting-gas outlet. The GTHTR300C 950 C reactor outlet / 900 C secondary-helium IHX outlet is retained as the upstream heat-delivery benchmark, not as the reacting-gas temperature.

## GTHTR300C cogeneration economics
Primary Nishihara, Mouri & Kunitomi (ICONE15, 2007) source values:
- nuclear plant construction cost: 59.7 billion JPY;
- reactor components 17.1; power conversion 7.8; auxiliary 6.7; electrical/C&I 5.8; IHX + secondary loop 11.2; buildings 11.1 billion JPY;
- nuclear heat: 0.52 JPY/MJ;
- electricity: 4.9 JPY/kWh;
- availability: 85%;
- doubled IHX+secondary-loop sensitivity: plant 70.9 billion JPY; heat 0.57 JPY/MJ; electricity 5.5 JPY/kWh.

GTHTR300C design literature gives 600 MWth reactor, 170 MWth IHX heat and about 202 MWe at the reference cogeneration point. JAEA load-follow descriptions show electricity increasing toward about 276 MWe as heat extraction is reduced while reactor power remains at 100%.

## Singapore electricity context
EMA Singapore Energy Statistics Chapter 5 reports 2025 USEP largely in the S$100--200/MWh range. Review-5 remediation evaluates zero, 100, 150 and 200 S$/MWh explicitly. Electricity value is a separate credit against the full reactor burden; it is not used as a hidden allocation.

## Cost-year normalization
Common basis: 2025-price source currency, then 29 Sep 2026 spot FX to SGD.
- Japan GDP deflator: 2007 99.59; 2025 112.27 -> factor 1.1273.
- Germany/euro-area screening proxy: 2014 90.46; 2025 123.84 -> factor 1.3690.
- JPY/SGD on 29 Sep 2026: 0.008117 SGD/JPY.
- EUR/SGD on 29 Sep 2026: 1.452 SGD/EUR.
- USD/SGD on 29 Sep 2026: 1.2776 SGD/USD.
The GDP deflator is a broad screening index, not a nuclear construction index; this limitation is explicit.

## Superseded result
The pre-Review-5 S$47.637/t conditional pass is superseded. After the source-temperature re-solve and actual-H2 lifecycle normalization, specific lifecycle abatement is non-positive, so no finite deployment scale or electricity value can produce a CN4252 joint pass.
