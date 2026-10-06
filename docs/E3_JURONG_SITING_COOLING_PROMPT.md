# E3 Main Research Prompt — Jurong Island Candidate-Context, Siting, Cooling and Infrastructure Feasibility

## Authority

Continue as Main Research for the existing CN4252 project.

Read first:
- docs/POST_SCREENING_ENGINEERING_CLOSURE_WORKFLOW.md
- results/E2B_ARCHITECTURE_MATURATION.md
- results/E2_INCREMENTAL_NUCLEAR_BENEFIT.md
- results/E1_ENGINEERING_ECONOMIC_MODEL.md
- docs/CN4252_BROAD_ASSIGNMENT_REQUIREMENTS.md
- docs/CN4252_PROBLEM_STATEMENT.md
- current Singapore feasibility/safety literature records and manuscript sections.

E2B is CLOSED at commit `3e567888ec47`.

E2B CI:
- Research CI `37437022912` — PASS
- Paper/reproducibility `37437022951` — PASS

This instruction authorises **E3 only — Jurong Island Candidate-Context, Siting, Cooling and Infrastructure Feasibility**.

Do not begin E4, W5, E8 or independent review automatically.

## Governing architecture

E3 must evaluate the preferred future E2B configuration, not silently revert to the old one-train architecture:

- one 600 MWth GTHTR300C-class high-temperature reactor;
- two 130 MMSCFD SMR-H2+CCS trains;
- 353.6 MWth total reformer/process-heat demand;
- approximately 195,892 tH2/y;
- approximately 1.834 MtCO2e/y lifecycle abatement on the E2B scaling basis;
- no unsupported electricity revenue;
- preferred mature/10-OAK economics approximately S$42.84/tCO2e under the declared E2B projected basis;
- FOAK remains adverse.

E3 does NOT need to prove that Jurong Island is an approved nuclear site. It must determine whether Jurong Island is a credible **candidate industrial integration context**, what engineering/site constraints arise, and what would have to be demonstrated before siting could be considered feasible.

## 1. Current authoritative research

Conduct current web/primary-source research where necessary.

Prioritise:
- Singapore Government;
- NEA;
- EMA;
- MTI;
- MSE;
- JTC;
- A*STAR / ISCE2;
- IAEA;
- OECD-NEA;
- relevant Singapore environmental/maritime/industrial authorities;
- authoritative Jurong Island infrastructure information;
- primary nuclear siting/cooling guidance.

Verify current facts rather than relying on old repository statements.

At minimum investigate:
- Jurong Island industrial role and land context;
- LCT3 / Low Carbon Technology Translational Testbed development;
- low-carbon/hydrogen-ready infrastructure;
- existing energy/chemical industrial context;
- Singapore nuclear readiness and INIR status;
- land/siting constraints;
- coastline/marine context;
- utilities;
- natural-gas infrastructure;
- hydrogen handling/offtake context where evidence exists;
- CO2 export/transport context;
- port/shipping constraints;
- environmental constraints.

Do NOT call LCT3 a nuclear project or nuclear site.

## 2. Candidate-context logic

Explain why Jurong Island is being examined:
- major energy/chemical industrial cluster;
- relevant NG/process infrastructure;
- low-carbon technology activity/testbed context;
- potential proximity to industrial H2 users;
- coastal access potentially relevant to cooling and CO2 logistics.

Then explicitly state why those advantages do NOT establish nuclear-site suitability.

The analysis must test the candidate context rather than advocate for it.

## 3. Physical deployment inventory

For the preferred E2B architecture identify the physical systems that would require land/infrastructure:

NUCLEAR:
- reactor building/island;
- IHX/process-heat interface;
- primary support systems;
- secondary-helium systems;
- safety/security areas;
- cooling/heat rejection;
- waste/fuel handling interfaces where relevant.

CHEMICAL:
- two SMR-H2 trains;
- WGS;
- amine CCS;
- PSA;
- CO2 conditioning/compression;
- H2 handling;
- utilities;
- flare/safe-release systems where applicable.

LOGISTICS:
- NG;
- water/steam;
- electricity;
- cooling;
- H2;
- captured CO2;
- waste;
- access/security.

Do not invent exact equipment footprints if data are unavailable.

## 4. Footprint / land screen

Attempt the strongest defensible land/footprint screen.

Use source-backed footprint/site-area evidence where available for:
- HTGR/nuclear island;
- SMR-H2 trains;
- CCS;
- utilities;
- cooling system;
- security/separation/buffer areas.

Distinguish:
- equipment footprint;
- plant plot;
- protected/security area;
- emergency-planning considerations.

Do not equate reactor-building footprint with total nuclear-site area.

If a numerical total cannot be defended, calculate a bounded range or identify the exact missing data.

Compare with Jurong Island land constraints only at the appropriate evidence level.

## 5. Cooling and heat rejection — REQUIRED ENGINEERING CALCULATION

This is not to remain a generic limitation paragraph.

Construct an explicit energy/heat-rejection screen for the preferred two-train architecture.

Account, as far as current data permit, for:
- 600 MWth reactor input;
- 353.6 MWth useful reformer/process heat;
- any source-backed/project electrical loads;
- remaining thermal energy;
- chemical/process heat rejection where identifiable;
- CCS/compression cooling where identifiable.

Do NOT automatically call:
600 - 353.6 = 246.4 MWth
the cooling duty.

That is remaining reactor thermal capacity, not automatically rejected heat.

Define the actual energy-conversion/use boundary first.

For each credible architecture determine what heat ultimately requires rejection.

## 6. Cooling options

Evaluate candidate cooling strategies appropriate to coastal Singapore/Jurong Island, for example where evidence supports them:
- once-through seawater cooling;
- closed-loop seawater/heat-exchanger arrangement;
- cooling towers;
- hybrid cooling;
- other industrial heat-sink/use options.

For each discuss:
- heat-removal capacity;
- water demand/flow;
- pumping;
- land;
- thermal discharge;
- marine/environmental implications;
- plume/intake/outfall considerations;
- reliability;
- nuclear safety implications;
- chemical-plant integration.

Do not select a cooling option merely because it uses less land.

## 7. Cooling-water calculation

Where sufficient thermophysical assumptions can be sourced, calculate illustrative required cooling-water flow using:

Q = m_dot Cp DeltaT.

Every assumed:
- Q;
- Cp;
- inlet temperature;
- allowed temperature rise

must be cited or explicitly classified.

Use sensitivity/range rather than false precision where Singapore seawater/environmental discharge limits are uncertain.

Do not claim an environmental permit.

## 8. Water / steam feasibility

Assess:
- process steam/water needs;
- makeup water;
- desalinated/industrial-water implications where relevant;
- cooling-water interaction;
- whether the two-train scale creates a material water constraint.

Use available source data; do not invent a complete water balance.

## 9. Natural-gas infrastructure

The preferred architecture still uses methane.

Quantify the two-train NG requirement from the canonical process basis.

Assess whether Jurong Island's industrial gas context makes supply physically plausible.

Do not equate national pipeline availability with guaranteed project capacity.

Identify:
- supply dependency;
- upstream emissions;
- resilience/security implications;
- required capacity confirmation.

## 10. Hydrogen scale and handling

Quantify:
- 260 MMSCFD H2;
- approximately 195,892 t/y.

Assess what this scale means physically for:
- local industrial use;
- distribution;
- storage/buffering;
- export only if relevant evidence exists.

Do not invent an offtaker.

If demand evidence is insufficient, calculate the required offtake scale and mark it as a deployment gate.

## 11. CO2 logistics and cross-border storage

Scale captured CO2 to the two-train architecture.

Assess:
- conditioning/compression;
- transport mode;
- port/pipeline/shipping interface;
- temporary storage/buffering;
- cross-border dependency;
- storage capacity/contract requirement;
- regulatory/accounting dependency.

Use current Singapore CCS policy/infrastructure evidence.

Do not assume an operating storage chain exists.

Quantify the annual captured-CO2 handling requirement.

## 12. Nuclear/chemical separation

E4 will perform the deep safety case, but E3 must identify the siting implications.

Assess conceptually:
- nuclear island vs chemical plant separation;
- hydrogen/methane/fire/explosion hazard proximity;
- CO2/asphyxiation hazards;
- hazardous-industry neighbours;
- shared utilities;
- common-cause hazards;
- access/security;
- emergency response.

Do not invent a definitive separation distance unless an applicable source provides one.

Where no universal distance exists, state what QRA/PRA/consequence analysis must determine it.

## 13. External hazards / Jurong context

Identify site-relevant external hazards requiring later analysis, using authoritative evidence where possible:
- marine/shipping;
- industrial fire/explosion;
- hazardous-material releases;
- flooding/coastal hazards;
- extreme rainfall;
- sea-level rise/storm surge where relevant;
- aircraft/transport where applicable;
- grid loss;
- external utility loss;
- seismic basis;
- malicious/security considerations at a high level.

Do not perform E4 accident modelling yet.

## 14. Nuclear siting/regulatory status

Research current Singapore nuclear readiness accurately.

State:
- whether Singapore has decided to deploy nuclear;
- current capability-building/regulatory assessment;
- IAEA INIR status/timeline;
- current safety/regulatory studies;
- what institutions/capabilities would be required before site licensing.

Do not imply Jurong Island is selected, reserved or approved for nuclear.

## 15. Jurong Island candidate-site decision matrix

Create a matrix:

Criterion | Evidence | Engineering implication | Status | Required next evidence

At minimum:
- industrial integration;
- land/footprint;
- high-temperature heat integration;
- NG;
- H2 offtake;
- CCS logistics;
- cooling;
- water;
- grid/utilities;
- nuclear/chemical separation;
- external hazards;
- security;
- emergency planning;
- environmental permitting;
- nuclear licensing/regulation.

Use:
SUPPORTED
CONDITIONAL
UNRESOLVED
NOT DEMONSTRATED
INFEASIBLE

only where evidence warrants.

## 16. Candidate layout — DATA SPECIFICATION ONLY

Do not generate final polished artwork yet.

The user will create/edit final conceptual site artwork externally after the report content is complete.

However, create a **visual specification** for the later image-generation/editing pass.

Specify what a conceptual Jurong Island integration figure would need to show:
- candidate industrial context;
- nuclear island;
- two H2/CCS trains;
- nuclear/chemical separation;
- cooling intake/outfall or alternative;
- NG connection;
- H2 product;
- CO2 conditioning/export;
- security/site boundary;
- conditional/not-to-scale labels.

Do not create a fictional exact site plan.

## 17. Implementation implications

Translate E3 findings into measurable future roadmap gates.

Examples:
- candidate-site land envelope demonstrated;
- cooling architecture closes heat rejection;
- environmental discharge screen acceptable;
- H2 offtake capacity secured;
- NG supply capacity confirmed;
- two-train CO2 export/storage capacity secured;
- nuclear/chemical separation validated by QRA/PRA;
- regulatory siting framework established.

Do not assign arbitrary calendar dates.

## 18. CN4252 relevance

Explicitly answer how E3 affects:
- solution-at-scale within Singapore;
- feasibility;
- potential effectiveness;
- implementation roadmap;
- accuracy.

Also explain what "within Singapore" means for a plant producing H2 and capturing CO2 in Singapore while geological storage may occur cross-border.

## 19. Durable outputs

Create at minimum:
- results/E3_JURONG_SITING_COOLING.md
- a source/evidence matrix;
- cooling/heat-rejection calculation data where quantitative;
- infrastructure-demand table;
- Jurong feasibility decision matrix;
- later-visual specification.

Implement deterministic calculations in Rust where project-derived quantities warrant it.

## 20. Acceptance test

E3 passes only if the project can answer:

WHY JURONG ISLAND IS BEING CONSIDERED?

WHAT PHYSICAL INFRASTRUCTURE THE TWO-TRAIN SYSTEM REQUIRES?

CAN THE HEAT-REJECTION/COOLING PROBLEM BE QUANTIFIED?

WHAT COOLING OPTIONS ARE PLAUSIBLE?

WHAT NG, H2 AND CO2 FLOWS MUST THE SITE HANDLE?

WHAT LAND/SEPARATION QUESTIONS REMAIN?

WHAT CURRENT SINGAPORE NUCLEAR REGULATORY CONDITIONS APPLY?

WHAT WOULD HAVE TO BE TRUE BEFORE JURONG ISLAND COULD BE CONSIDERED A FEASIBLE NUCLEAR/SMR-H2+CCS SITE?

If the evidence indicates Jurong Island is unsuitable, retain that finding and identify whether another Singapore context would need consideration.

## 21. Verification

At completion:
- run relevant Rust tests;
- run Research CI;
- run Paper/reproducibility CI if triggered;
- verify E1/E2/E2B results remain unchanged;
- verify no fictional site approval;
- verify no invented cooling/environmental permit;
- verify every quantitative site/cooling value has provenance.

## 22. Report and STOP

Report:

PHASE: E3 — Jurong Island Siting/Cooling Feasibility

STARTING HEAD:

CURRENT JURONG ISLAND CONTEXT:

LCT3 / LOW-CARBON TESTBED RELEVANCE:

PREFERRED E2B ARCHITECTURE EVALUATED:

LAND/FOOTPRINT FINDING:

HEAT-REJECTION BOUNDARY:

COOLING OPTIONS:

COOLING-WATER CALCULATION:

WATER/STEAM FINDING:

TWO-TRAIN NG REQUIREMENT:

TWO-TRAIN H2 SCALE:

TWO-TRAIN CAPTURED CO2 SCALE:

CO2 EXPORT/STORAGE REQUIREMENT:

NUCLEAR/CHEMICAL SEPARATION FINDING:

EXTERNAL HAZARDS:

NUCLEAR REGULATORY/SITING STATUS:

JURONG FEASIBILITY CLASSIFICATION:

CRITICAL UNRESOLVED CONDITIONS:

IMPLEMENTATION-ROADMAP GATES:

VISUAL SPECIFICATION CREATED:

RUST FILES/TESTS:

DURABLE OUTPUTS:

RESEARCH CI:

PAPER CI:

COMMIT SHA:

RECOMMENDED NEXT ACTION:

Then STOP.

Do not begin E4 automatically.
Do not begin W5.
Do not begin E8.
Do not begin independent review.
