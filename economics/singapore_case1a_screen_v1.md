# Singapore-adjusted conventional SMR+CCS comparator — screening v1

## Purpose

Test whether conventional SMR + shifted-syngas MDEA CCS (IEAGHG Case 1A) can plausibly satisfy the authoritative CN4252 <S$100/tCO2e target before any nuclear integration is charged.

This is a **screening harmonisation**, not a final cost estimate.

## Source-basis decomposition

IEAGHG 2017-02:
- Case 1A CAC = EUR47.1/t avoided;
- T&S assumption = EUR10/t captured;
- captured = 0.4660 kg/Nm3 H2;
- avoided = 0.8091 - 0.3704 = 0.4387 kg/Nm3 H2;
- captured/avoided = 1.0622.

Therefore:
- source T&S contribution = EUR10.62/t avoided;
- non-T&S Case-1A component = EUR36.48/t avoided, Q4-2014 basis.

## Process-plant escalation screen

Chemical Engineering identifies CEPCI as the standard CPI process-plant cost index used to adjust construction costs between periods.

A peer-reviewed 2025 CO2-transport TEA reports:
- CEPCI 2024 = 800;
- CEPCI 2020 = 596.2;
- CEPCI 2017 = 567.5;
- CEPCI 2015 = 556.8.

A standard chemical-process design reference gives:
- CEPCI 2014 = 576.1.

Chemical Engineering reports that the 2025 annual-average CEPCI was 1.6% above the 2024 annual average.

Using 800 as the 2024 anchor gives the derived 2025 screening index:

CEPCI_2025 ~= 800*1.016 = 812.8.

Thus:

36.48 * 812.8/576.1
~= EUR51.47/t avoided, 2025 process-cost screening basis.

This escalation is much more appropriate than consumer HICP for a process plant, but still carries uncertainty because the 2014 and 2024 values are compiled from different public-access sources rather than one licensed CEPCI database extraction.

## Currency conversion

ECB reference rate, 31 Dec 2025:

1 EUR = 1.5105 SGD.

Therefore:

non-T&S Case 1A
~= 51.47*1.5105
~= S$77.7/t avoided.

For T&S, 31 Dec 2025 USD/SGD is approximately 1.2863 SGD/USD.

IEAGHG 2023 Singapore-source Group A:
USD50-75/t captured

becomes approximately:

S$64.3-96.5/t captured.

Because captured/avoided = 1.0622, the contribution to avoidance cost is:

~S$68.3-102.5/t avoided.

## Combined screening result

Case 1A with Singapore Group-A-like T&S:

low:
~77.7 + 68.3
= ~S$146/t avoided.

high:
~77.7 + 102.5
= ~S$180/t avoided.

Therefore the first harmonised screening range is:

**~S$146-180/tCO2 avoided**

for conventional Case 1A + Group-A-like Singapore cross-border T&S.

## Important limitations

This is NOT yet the final CN4252 lifecycle abatement cost because:

1. IEAGHG CAC is based on direct plant CO2 avoidance, while CN4252 should use lifecycle CO2e;
2. the Singapore T&S study is a 2023 cost study, not a 2025 contracted tariff;
3. CEPCI escalation is a process-plant screening method, not a Singapore location factor;
4. Singapore labour/land/utilities/financing are not substituted;
5. carbon tax/incentive treatment is excluded;
6. actual Singapore storage route is unresolved;
7. Case 1A captures only ~56% of plant CO2, whereas the project's matched nuclear screening has explored higher feedstock-carbon capture.

## Robustness observation

Even without CEPCI escalation, simply converting the original EUR36.48/t non-T&S term at 2025 EUR/SGD gives ~S$55/t avoided. Adding Group-A-like T&S then gives roughly ~S$123-158/t avoided.

Thus the conclusion that this comparator is under severe pressure relative to S$100/t is not solely caused by the CEPCI escalation assumption.

This is still not proof that all Singapore SMR+CCS configurations fail the assignment target. Different capture configurations, storage tariffs, incentives, avoided fuel costs and lifecycle denominators can change the result.

## Scientific consequence for nuclear integration

Nuclear integration must now be evaluated against two separate questions:

1. Does it lower lifecycle emissions?
2. Can it lower **net incremental annual cost** enough relative to Singapore-adjusted conventional CCS to offset its additional capital/integration cost?

If nuclear only adds cost while leaving the same expensive T&S chain, it cannot rescue the S$100/t criterion.

If direct nuclear heat removes enough fuel/utility/carbon-tax cost, enables a different capture topology, or supports economically valuable cogeneration, a feasibility region may still exist.

The next analysis should therefore derive the **maximum nuclear premium (or required nuclear savings)** compatible with S$100/t once conventional CCS already consumes the majority/exceeds the budget.
