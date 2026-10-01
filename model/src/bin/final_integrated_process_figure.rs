use nuclear_assisted_smr::final_design::{HE_RETURN_C, HE_SUPPLY_C, HEAT_MWTH, H2_MMSCFD, INL_ROT_C, NISHIHARA_IHX_MWTH, PSA_RECOVERY, REACTOR_MWTH, REFORMER_C};
use std::{fmt::Write, fs};
fn main() {
 let remaining = REACTOR_MWTH - HEAT_MWTH;
 let mut s = String::new();
 writeln!(s, r"\begin{{tikzpicture}}[>=Latex,font=\\scriptsize,equip/.style={{draw,rounded corners,align=center,minimum width=24mm,minimum height=11mm}},boundary/.style={{draw,dashed,rounded corners,inner sep=4mm}},flow/.style={{->,thick}},heat/.style={{->,very thick,dashed}},note/.style={{align=left,font=\\scriptsize}}]").unwrap();
 writeln!(s, r"\node[equip] (rx) at (0,2.7) {{High-Temperature\\\\Gas-Cooled Reactor\\\\(HTGR)\\\\{:.0} MWth}};", REACTOR_MWTH).unwrap();
 writeln!(s, r"\node[equip] (ihx) at (3.6,2.7) {{Intermediate Heat\\\\Exchanger (IHX)}};").unwrap();
 writeln!(s, r"\node[equip] (ref) at (7.2,2.7) {{Steam Methane\\\\Reformer (SMR)\\\\{:.0}$^\\circ$C outlet\\\\{:.1} MWth}};", REFORMER_C, HEAT_MWTH).unwrap();
 writeln!(s, r"\node[equip] (wgs) at (10.5,2.7) {{Water-Gas Shift\\\\(WGS)}};").unwrap();
 writeln!(s, r"\node[equip] (cap) at (10.5,0.5) {{Amine CO$_2$\\\\capture}};").unwrap();
 writeln!(s, r"\node[equip] (psa) at (7.2,0.5) {{Pressure Swing\\\\Adsorption (PSA)\\\\{:.0}\\% H$_2$ recovery}};", PSA_RECOVERY*100.0).unwrap();
 writeln!(s, r"\node[equip] (h2) at (3.6,0.5) {{H$_2$ product\\\\{:.0} MMSCFD}};", H2_MMSCFD).unwrap();
 writeln!(s, r"\node[equip] (co2) at (10.5,-1.7) {{CO$_2$ compression /\\\\conditioning\\\\$\\rightarrow$ cross-border T\\&S\\\\screening boundary}};").unwrap();
 writeln!(s, r"\node[note] (feed) at (7.2,4.35) {{Natural gas + steam\\\\S/C = 3.0}};").unwrap();
 writeln!(s, r"\node[note] (remain) at (0,0.45) {{Remaining reactor\\\\thermal capacity:\\\\{:.1} MWth\\\\\\textbf{{no project electricity\\\\output inferred}}}};", remaining).unwrap();
 writeln!(s, r"\draw[flow] (feed)--(ref); \\draw[flow] (ref)--(wgs); \\draw[flow] (wgs)--(cap); \\draw[flow] (cap)--(psa); \\draw[flow] (psa)--(h2); \\draw[flow] (cap)--(co2);").unwrap();
 writeln!(s, r"\draw[heat] (rx.east) -- node[above,align=center]{{PRIMARY HELIUM LOOP\\\\reactor outlet {:.0}$^\\circ$C}} (ihx.west);", INL_ROT_C).unwrap();
 writeln!(s, r"\draw[heat] (ihx.east) -- node[above,align=center]{{SECONDARY HELIUM PROCESS-HEAT LOOP\\\\supply {:.0}$^\\circ$C; source flow 78.49 kg/s}} (ref.west);", HE_SUPPLY_C).unwrap();
 writeln!(s, r"\draw[heat] (ref.south west) to[out=230,in=310] node[below,align=center]{{secondary-He return $\\approx${:.0}$^\\circ$C}} (ihx.south east);", HE_RETURN_C).unwrap();
 writeln!(s, r"\draw[heat] (ihx.north west) to[out=130,in=50] node[above]{{primary He returns to reactor}} (rx.north east);").unwrap();
 writeln!(s, r"\node[note] at (3.6,4.35) {{Reference physical IHX $\\sim$170 MWth;\\\\project duty {:.1} MWth; qualification/scaling required.}};", HEAT_MWTH).unwrap();
 writeln!(s, r"\node[note] at (0,-1.55) {{Source process-heat branch: $\\sim${:.0} MWth\\\\(architecture/economic branch, \\textbf{{not}} one IHX).}};", NISHIHARA_IHX_MWTH).unwrap();
 writeln!(s, r"\node[boundary,fit=(rx)(ihx)(remain)] (nuc) {{}}; \\node[above=1mm of nuc.north] {{\\textbf{{NUCLEAR HEAT SYSTEM}}}};").unwrap();
 writeln!(s, r"\node[boundary,fit=(ref)(wgs)(cap)(psa)(h2)(co2)] (chem) {{}}; \\node[below=1mm of chem.south] {{\\textbf{{CHEMICAL PROCESS / CO$_2$ BOUNDARY}}}};").unwrap();
 writeln!(s, r"\end{{tikzpicture}}").unwrap();
 fs::write("../results/final_design/generated/final_integrated_process_figure.tex", s).unwrap();
}