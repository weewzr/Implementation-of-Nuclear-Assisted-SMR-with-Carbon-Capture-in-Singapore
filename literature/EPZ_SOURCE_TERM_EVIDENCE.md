# EPZ and Mechanistic Source-Term Evidence — Deep Feasibility Phase

## Research question

**What emergency-planning boundary could be justified for the selected HTGR given its accident spectrum, mechanistic source term, site meteorology, population, dose criteria and Singapore regulatory policy?**

The current project cannot calculate that boundary defensibly. The correct present classification is:

**UNRESOLVED — REQUIRES SITE-SPECIFIC SAFETY ANALYSIS.**

## Why the professor's ~400 m example is not a project EPZ

The recovered professor presentation uses a several-hundred-metre example to explain how a developer might justify a smaller emergency-planning boundary by combining accident likelihood and radiological consequence. The presentation itself warns that a safety zone needs a technical basis rather than a generic distance.

A directly relevant peer-reviewed precedent exists for **HTR-PM**, not this project: Ding, Tong, Wang and Zhang, *Development of emergency planning zone for high temperature gas-cooled reactor*, Annals of Nuclear Energy 111 (2018) 347–353, DOI 10.1016/j.anucene.2017.08.039. The paper reports that HTR-PM's EPZ can be on the order of several hundred metres / exclusion-area boundary after design-specific source-term, meteorological-dispersion and dose analysis.

**Transferability:** methodological only. HTR-PM is a different reactor architecture, inventory, source term, site and regulatory case. Its distance is not imported into the Singapore GTHTR300C-class screening project.

## Regulatory / authoritative reference ladder

### Existing large U.S. LWR reference
NRC operating-reactor emergency planning uses an approximately 10-mile (16 km) plume exposure pathway EPZ and approximately 50-mile ingestion pathway as generic operating-fleet reference. These are not automatic requirements for advanced reactors.

### U.S. advanced-reactor framework
The NRC's 2023 final rule for SMRs and other new technologies adds a performance-based framework and a **scalable** plume-exposure EPZ. The applicant must provide a technical analysis justifying the proposed EPZ size. NRC guidance is consequence-oriented, risk-informed, performance-based and technology-inclusive. Nearby-facility hazards must also be considered.

Implication for this project: a co-located hydrogen/SMR-H2 plant is not merely an industrial neighbour; its fire/explosion/toxic hazards can matter to emergency-plan implementation and nuclear safety.

### IAEA graded approach
IAEA GSR Part 7 establishes emergency-preparedness categories and a graded approach. Earlier IAEA guidance gives generic planning-zone ranges by reactor thermal power but explicitly permits different distances when substantiated by detailed safety analysis. These generic ranges are **references**, not a Singapore-project result.

## Mechanistic source term requirement

NRC describes a mechanistic source term as a design/accident-specific analysis of fission-product release and transport from fuel through reactor-system barriers/holdup volumes and mitigation features to the environment. For non-LWRs, source-term development requires sufficiently understood fuel/reactor behaviour, modelling of transport through all barriers/pathways, and a bounding set of accident categories.

For modular HTGRs, Petti et al. (Nuclear Technology 184, 2013, 181–197, DOI 10.13182/NT184-181) show the relevant barrier logic: TRISO fuel, graphite retention, primary-circuit transport/plate-out and functional containment all influence environmental source term. They also show source term depends on initial fuel quality, fuel failure/performance, reactor outlet temperature and retention outside the core.

Counter-evidence/qualification: Moormann (2008) reports AVR experience showing that primary-circuit fission-product deposition and carbonaceous dust materially affect source-term estimation and maintenance contamination. This reinforces that “TRISO retains fission products” is not a complete source-term calculation.

HTR-PM-specific source-term work also uses dedicated codes and V&V (e.g. HTR-STAC) and radionuclide-specific chemistry/transport models. This is evidence of the analysis burden required; it is not a substitute for a GTHTR300C-class project model.

## Required project chain

A defensible Singapore EPZ requires:

1. **candidate site and reactor configuration**;
2. **accident spectrum** including design-basis and relevant beyond-design-basis sequences;
3. **sequence frequency / risk information** where required by the regulatory method;
4. **radionuclide inventory**;
5. **fuel release** by radionuclide and temperature/history;
6. **graphite/core retention**;
7. **primary-circuit transport, deposition, dust and re-entrainment**;
8. **pressure-boundary / confinement release**;
9. **site meteorology and atmospheric dispersion**;
10. **off-site dose calculation**;
11. **population, shelter/evacuation and protective-action assumptions**;
12. **dose/protective-action criteria**;
13. **Singapore regulator policy and emergency-planning framework**.

The present CN4252 screening model contains none of items 2–12 at licensing fidelity. Therefore a numerical EPZ would be false precision.

## Figure specification

The manuscript should use an original methodology schematic:

ACCIDENT SEQUENCE
→ FREQUENCY
→ FUEL RELEASE
→ MECHANISTIC SOURCE TERM
→ CONFINEMENT RELEASE
→ ATMOSPHERIC DISPERSION
→ DOSE
→ PROTECTIVE ACTION
→ EPZ.

Evidence tags:
- **SUPPORTED BY EXTERNAL EVIDENCE:** methodology, generic barriers, HTR-PM example;
- **SUPPORTED BY THIS PROJECT:** selected reactor/process architecture only;
- **SITE-SPECIFIC INPUT REQUIRED:** meteorology, population, protective actions;
- **NOT MODELLED:** project accident frequencies, mechanistic source term, dispersion, dose, EPZ radius.

## Current conclusion

- “Does an SMR need a 16 km EPZ?” — wrong framing.
- “Is 400 m adequate for this project?” — **NOT SUPPORTED**.
- “Could an advanced HTGR justify a smaller EPZ than a conventional large LWR?” — **SUPPORTED AS A DESIGN/REGULATORY POSSIBILITY**, with HTR-PM and NRC advanced-reactor precedent.
- “What is the Singapore project EPZ?” — **UNRESOLVED — REQUIRES SITE-SPECIFIC SAFETY ANALYSIS**.

## Primary / authoritative sources

- U.S. NRC, *Emergency Preparedness Requirements for Small Modular Reactors and Other New Technologies* final rule and RG 1.242 (2023).
- U.S. NRC, *Nuclear Power Reactor Source Term* advanced-reactor guidance collection.
- IAEA GSR Part 7, *Preparedness and Response for a Nuclear or Radiological Emergency*.
- Ding, H.; Tong, J.; Wang, Y.; Zhang, L. (2018), *Development of emergency planning zone for high temperature gas-cooled reactor*, Annals of Nuclear Energy 111, 347–353.
- Petti, D.A.; Hobbins, R.R.; Lowry, P.; Gougar, H. (2013), *Representative source terms and the influence of reactor attributes on functional containment in modular high-temperature gas-cooled reactors*, Nuclear Technology 184, 181–197.
- Moormann, R. (2008), *Fission Product Transport and Source Terms in HTRs: Experience from AVR Pebble Bed Reactor*, Science and Technology of Nuclear Installations.


## Radionuclide-specific evidence chain

### AGR-1 post-irradiation safety tests

Demkowicz et al. tested irradiated AGR-1 UCO TRISO compacts at 1600 and 1800 C for approximately 300 h and measured Ag, Cs, Eu, Sr and Kr release.

Key observations relevant to source-term reasoning:
- **Cs:** release from particles with intact coatings was <1e-6 after 300 h at 1600 C or 100 h at 1800 C; rare SiC failures can produce significant Cs release.
- **Kr:** <2e-6 after 300 h at 1600 C; release rises after full coating failure.
- **Ag:** appreciable early measured release (3-34% of compact inventory) was associated largely with inventory already outside SiC in matrix/OPyC; additional release from intact particles becomes visible at 1800 C.
- **Eu/Sr:** low but measurable release; rates increase during long 1800 C exposure.

Interpretation: **TRISO is not one universal retention factor.** SiC integrity is especially important for Cs; noble-gas retention depends on dense coating integrity; metallic fission products have different transport behaviour. A mechanistic source term must therefore be radionuclide- and barrier-specific.

Transferability limit: AGR-1 furnace tests bound/represent fuel-level accident behaviour under specified irradiation and thermal histories. They do not directly provide a reactor environmental release fraction because graphite, primary-circuit deposition, dust, confinement and accident transport still intervene.

### Generic modular HTGR functional containment

Petti et al. (Nuclear Technology 184, 2013) construct representative modular-HTGR source terms and explicitly examine:
- initial fuel quality;
- in-reactor fuel performance/failure;
- reactor outlet temperature;
- retention outside the core.

Their barrier logic supports a functional-containment chain rather than a single containment factor:

fuel kernel/TRISO
→ fuel element/graphite
→ primary circuit deposition/holdup
→ reactor building/confinement
→ environment.

Project implication: the selected 925 C source operating point cannot be linked directly to off-site dose using only AGR particle-release data.

### Counter-evidence from AVR

Moormann (2008) reports AVR operating experience in which fission products deposited outside the active core are important for source-term estimation, particularly for depressurisation accidents. Carbonaceous dust can sorb activity, deposit on surfaces and be remobilised by flow disturbances. AVR experience also showed discrepancies between laboratory plate-out tests and in-reactor behaviour.

This is a valuable contradiction/qualification:
- **supports:** coated fuel is a strong barrier and many heat-up releases remain low when temperatures remain within design limits;
- **challenges simplistic inference:** primary-circuit contamination/dust and long-term operating history can create releasable inventories outside intact TRISO.

Transferability limit: AVR was a pebble-bed experimental reactor with operating/fuel-quality history different from a modern prismatic GTHTR300C-class design. Its measured dust/source-term behaviour should not be numerically imposed on this project. It demonstrates a mechanism and uncertainty that must be addressed.

### HTR-PM source-term V&V precedent

Chen et al. (2018) describe the HTR-STAC source-term package for HTR-PM, comprising dedicated units for:
- primary-circuit source term;
- normal airborne release;
- accident release categories;
- C-14;
- tritium.

The work reports formal verification/validation activities including independent code/algorithm review and regulatory assessment by China's NNSA.

Project implication: licensing-grade HTR source-term analysis is a **multi-code / multi-phenomenon task**, not a spreadsheet release fraction. The present CN4252 Rust model does not claim this capability.

## Source-term V&V status for this project

| Submodel | Verification | Validation | Uncertainty | Extrapolation |
|---|---|---|---|---|
| TRISO qualitative barrier model | literature cross-check | AGR furnace experiments external to project | strong nuclide/temperature dependence | fuel-test → reactor source term is large extrapolation |
| Core/graphite retention | not implemented | none in project | not quantified | unresolved |
| Primary-circuit plate-out/dust | not implemented | AVR gives external mechanism evidence | high/design-dependent | AVR pebble bed → GTHTR300C not quantitatively transferable |
| Confinement release | not implemented | none | unresolved | unresolved |
| Atmospheric dispersion | not implemented | none | site/weather dependent | site-specific |
| Dose / EPZ | not implemented | none | regulator/site dependent | cannot be calculated defensibly |

**Conclusion:** the project can explain the mechanistic chain and evidence maturity, but a numerical project source term or EPZ remains outside current model validity.


## Japanese/Chinese native accident-mechanism saturation check

### Air ingress / graphite oxidation — Japan
JAERI/JAEA performed dedicated HTGR air-ingress experiments using graphite-tube test sections over roughly 400-1050 C to study natural convection, multicomponent diffusion and graphite oxidation after primary-pipe rupture. Japanese VHTR safety analyses explicitly model depressurisation followed by air ingress/graphite oxidation at 950 C-class outlet conditions.

Later JAEA design work treats standpipe rupture/air ingress as a concrete design driver and proposes reflector/flow-path changes to suppress core air ingress. This is evidence that air ingress is not merely a generic PIRT item; it has shaped Japanese prismatic-HTGR design.

### Air ingress — China
Tsinghua native publication records show an extensive HTR-PM programme covering:
- diffusion/natural-circulation onset;
- massive air ingress;
- IG-110 graphite oxidation kinetics;
- non-uniform fuel-element oxidation;
- bottom-reflector oxidation;
- mitigation measures;
- graphite dust generation/transport.

These models and experiments were applied to HTR-PM safety review. This independently cross-checks the Japanese mechanism set while highlighting pebble-bed-specific geometry/dust differences.

### Water/steam ingress — Japan
JAEA small-HTGR preliminary safety work explicitly models steam-generator tube rupture / water ingress. In the cited analysis, water entering the primary system can:
- insert reactivity;
- oxidise graphite through C + H2O -> CO + H2;
- increase primary pressure;
- generate combustible H2 + CO;
- potentially lead to safety-valve release to confinement.

One analysed small-HTGR case conservatively estimated 875 kg primary water ingress, 110 kg reaching the core and ~1.8 vol% H2+CO in the stated confinement assumptions, below that study's flammability criterion.

**The numbers are not transferable to this project.** The mechanism chain is.

### Water ingress — China
HTR-PM research likewise analyses steam-generator blowdown/water ingress transients. This is especially relevant because HTR-PM's operating plant couples helium directly to steam generators, whereas the project's nuclear/process interface is helium-helium IHX. The project may therefore remove some steam-generator-specific pathways at the reactor/process interface, but water/steam ingress can still arise from whatever water/steam equipment is actually connected to the selected design.

## Saturation conclusion for DF-13 / DF-14

The mechanism question is now **research-saturated at screening level**:
- air ingress -> graphite oxidation / structural and source-term consequences: strongly supported by Japanese and Chinese native evidence plus NRC;
- water/steam ingress -> reactivity/pressure/graphite reaction/H2+CO consequences: strongly supported by Japanese/Chinese evidence plus NRC.

What remains is not more generic literature. It is **selected-design sequence quantification**:
break/leak location and size
→ ingress mass/rate
→ circulation/diffusion
→ graphite/fuel temperatures
→ reaction/oxidation
→ pressure/combustible gas
→ fission-product transport
→ confinement release.

Decision consequence:
if the selected GTHTR300C-class configuration cannot bound these sequences within fuel/structural/release criteria, its pressure-boundary, confinement, isolation or component architecture must change.
