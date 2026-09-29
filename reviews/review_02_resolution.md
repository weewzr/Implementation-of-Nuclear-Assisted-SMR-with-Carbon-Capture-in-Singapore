# Independent Review 2 resolution record

## Decision

Independent Review 2 gate: **OPEN**.

This file records Main Research corrections to the existing independent Review 2. It is not a new review.

## R2-B03 — Thermodynamic property-range handling

**Disposition: RESOLVED.**

### Correction
The thermodynamic layer now uses property objects carrying explicit validity metadata and rejects evaluation outside documented NIST Shomate intervals. H2 dispatches between 298-1000 K and 1000-2500 K sets; CO2 between 298-1200 K and 1200-6000 K. Public SMR/WGS equilibrium functions now route through interval-safe dispatch.

### Evidence
NIST SRD 69 independently confirms those intervals and coefficients. Commits:
- `897353f09fedaba27b950c51671c04e21fa70339`
- `4e0e65943bbd75e1efb9de3cef13e67fc752190c`
- `0ee42368b7b13b43edf0c035cd50932144ee6609`

### Verification
Rust CI verifies no low-H2 interval use at 1173-1223 K; explicit failure outside supported intervals; H2 H/S/G/Cp continuity at 1000 K; CO2 H/S/G continuity and the small separately-fitted Cp jump at 1200 K within 0.5%; SMR K continuity across 1000 K; and finite monotonic K through the reformer range.

An independent Daubert-based thermochemical table gives dimensionless SMR Kp = 12.735, 171.07, 1485.1 and 9199.6 at 973, 1073, 1173 and 1273 K. The NIST-derived implementation agrees within 3% at all four points.

### Acceptance criterion status
- no Shomate set outside documented interval: **PASS**
- H/S/G/Cp boundary continuity within expected fit/rounding: **PASS**
- independent external SMR K(T) checks: **PASS**
- reformer tests include 1173-1223 K: **PASS**
- invalid temperatures fail explicitly: **PASS**

R2-B03 is closed. This does not validate recycle predictions; R2-B01 remains the next blocker.

## Remaining Review-2 critical path

1. R2-B01 — full-species physical recycle/purge/reformer/WGS/PSA steady state.
2. R2-B02 — integrated temperature-resolved candidate energy balance.
3. R2-M01 + R2-M02.
4. R2-M04.
5. R2-M03 + R2-M05.
