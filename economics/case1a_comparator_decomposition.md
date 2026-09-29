# IEAGHG Case 1A comparator decomposition

## Source basis

IEAGHG 2017-02 uses Q4-2014 euros and an AACE Class 4 estimate (+35%/-15%). The study assumes:
- 25-year economic life;
- 8% discount rate;
- 8322 h/y after first year;
- NG price EUR6/GJ LHV;
- electricity EUR80/MWh;
- CO2 T&S EUR10/tCO2 stored, sensitivity -20 to 40 EUR/t.

For Case 1A:
- Base emissions = 0.8091 kgCO2/Nm3 H2;
- Case 1A emissions = 0.3704;
- captured CO2 = 0.4660;
- reported CAC = EUR47.1/tCO2 avoided;
- LCOH = 13.5 euro-cent/Nm3 vs 11.4 base.

Primary source: IEAGHG 2017-02, Tables 1, 2 and 5.

## Separate T&S from capture/integration economics

Avoided CO2 per Nm3 H2:

m_avoided = 0.8091 - 0.3704
          = 0.4387 kg/Nm3.

Captured/avoided ratio:

R_CA = 0.4660/0.4387
     = 1.0622.

Therefore a T&S tariff p per tonne captured contributes:

CAC_TS = 1.0622 p

per tonne avoided.

At IEAGHG's p = EUR10/t captured:

CAC_TS = EUR10.62/t avoided.

Hence the implied Case-1A avoidance cost excluding T&S is:

CAC_nonTS,2014
= 47.1 - 10.62
= approximately EUR36.48/tCO2 avoided.

This decomposition reconstructs the published EUR47.1/t result and is regression-tested in Rust.

## Why this is valuable

A Singapore T&S tariff must be charged per tonne captured/transported, whereas the CN4252 metric is per tonne lifecycle CO2e avoided.

The ratio R_CA prevents these denominators being confused.

For every +1 currency-unit/t captured added to T&S, Case-1A avoidance cost increases by approximately +1.062 currency-units/t avoided, before lifecycle-boundary corrections.

## Price-year harmonisation status

The EUR36.48/t non-T&S term remains in **Q4-2014 euros**.

It has NOT yet been escalated to 2026 SGD.

Consumer HICP is available from Eurostat/ECB, but it is a consumer-price index rather than a chemical-process-plant cost index. Using it for the capture/integration component would be only a screening proxy.

Process-plant cost indices such as CEPCI or Marshall & Swift are methodologically more appropriate for capital escalation, but current full index values are often proprietary/licensed. The repository will not fabricate a 2026 CEPCI value.

Therefore the next economic comparison should carry at least two layers:
1. exact source-basis decomposition in EUR2014;
2. explicitly labelled escalation sensitivity for translation to SGD2026.

## Important comparator insight

Case 1A's non-T&S CCS/integration burden is already ~EUR36.5/t avoided on the source basis.

Singapore-source T&S study values are USD50-75/t captured even for the lowest-cost group.

Without harmonising price years/currencies, they cannot simply be added numerically. But the magnitude comparison shows that T&S is plausibly comparable to or larger than the entire original capture/integration avoidance-cost component.

This makes conventional SMR+CCS itself a stringent economic comparator for Singapore.
