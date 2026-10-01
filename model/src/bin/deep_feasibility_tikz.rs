use nuclear_assisted_smr::final_design::{availability_sensitivity,gas_backup_sensitivity,ccs_capture_sensitivity,final_design,final_cost_ledger,final_lifecycle_ledger};
use std::{fs,fmt::Write};
fn main(){
 let mut a=String::new();
 writeln!(a,r"\begin{{tikzpicture}}\begin{{axis}}[width=0.94\linewidth,height=6.2cm,xlabel={{Effective annual availability (\%)}},ylabel={{Lifecycle CO$_2$e avoided (t/y)}},xmin=20,xmax=100,ymin=0,ymax=1100000,grid=major,legend style={{font=\small,at={{(0.02,0.98)}},anchor=north west}}]").unwrap();
 writeln!(a,r"\addplot[dashed] coordinates {{(20,250000)(100,250000)}};\addlegendentry{{CN4252 threshold}}").unwrap();
 write!(a,r"\addplot[thick] coordinates {{").unwrap(); for i in 20..=100{let x=availability_sensitivity(i as f64/100.0);write!(a,"({},{:.1})",i,x.lifecycle_avoided_t).unwrap();} writeln!(a,r"}};\addlegendentry{{No-backup screen}}").unwrap();
 write!(a,r"\addplot[thick,densely dotted] coordinates {{").unwrap(); for i in 50..=85{let x=gas_backup_sensitivity(i as f64/100.0);write!(a,"({},{:.1})",i,x.lifecycle_avoided_t).unwrap();} writeln!(a,r"}};\addlegendentry{{Gas-backup lower-bound screen}}\end{{axis}}\end{{tikzpicture}}").unwrap();
 fs::write("../results/final_design/generated/availability_abatement_figure.tex",a).unwrap();

 let mut b=String::new();
 writeln!(b,r"\begin{{tikzpicture}}\begin{{axis}}[width=0.94\linewidth,height=6.2cm,xlabel={{Fraction of canonical captured stream stored (\%)}},ylabel={{Lifecycle CO$_2$e avoided (t/y)}},xmin=0,xmax=100,ymin=0,ymax=1000000,grid=major,legend style={{font=\small,at={{(0.02,0.98)}},anchor=north west}}]").unwrap();
 writeln!(b,r"\addplot[dashed] coordinates {{(0,250000)(100,250000)}};\addlegendentry{{CN4252 threshold}}").unwrap();
 write!(b,r"\addplot[thick] coordinates {{").unwrap(); for i in 0..=100{let x=ccs_capture_sensitivity(i as f64/100.0);write!(b,"({},{:.1})",i,x.lifecycle_avoided_t).unwrap();} writeln!(b,r"}};\addlegendentry{{Steady capture/storage screen}}\end{{axis}}\end{{tikzpicture}}").unwrap();
 fs::write("../results/final_design/generated/ccs_robustness_figure.tex",b).unwrap();

 let d=final_design(0.0,false);
 let l=final_lifecycle_ledger();
 let direct=l.direct_saved;
 let up=l.upstream_saved;
 let nuc=l.nuclear_added; let aux=l.auxiliary_added; let ts=l.transport_storage_added;
 let mut w=String::new();
 writeln!(w,r"\begin{{tikzpicture}}\begin{{axis}}[ybar,width=0.94\linewidth,height=6.3cm,ylabel={{Contribution (ktCO$_2$e/y)}},symbolic x coords={{Direct saved,Upstream saved,Nuclear added,Aux added,T\&S added,Net avoided}},xtick=data,x tick label style={{rotate=25,anchor=east}},grid=major]").unwrap();
 writeln!(w,r"\addplot coordinates {{(Direct saved,{:.3})(Upstream saved,{:.3})(Nuclear added,{:.3})(Aux added,{:.3})(T\&S added,{:.3})(Net avoided,{:.3})}};\end{{axis}}\end{{tikzpicture}}",direct/1000.0,up/1000.0,-nuc/1000.0,-aux/1000.0,-ts/1000.0,d.lifecycle_avoided_t/1000.0).unwrap();
 fs::write("../results/final_design/generated/co2_bridge_figure.tex",w).unwrap();

 let x=final_cost_ledger();
 let mut k=String::new();
 writeln!(k,r"\begin{{tikzpicture}}\begin{{axis}}[ybar,width=0.94\linewidth,height=6.3cm,ylabel={{Annual contribution (S\$ million/y)}},symbolic x coords={{NG saved,Reactor,CCS cap.,Integration,T\&S,Elec. revenue,Net change}},xtick=data,x tick label style={{rotate=25,anchor=east}},grid=major]").unwrap();
 writeln!(k,r"\addplot coordinates {{(NG saved,{:.3})(Reactor,{:.3})(CCS cap.,{:.3})(Integration,{:.3})(T\&S,{:.3})(Elec. revenue,{:.3})(Net change,{:.3})}};\end{{axis}}\end{{tikzpicture}}",-x.ng_saved/1e6,x.reactor_added/1e6,x.ccs_annual_added/1e6,x.integration_added/1e6,x.transport_storage_added/1e6,-x.electricity_revenue/1e6,x.net_incremental/1e6).unwrap();
 fs::write("../results/final_design/generated/cost_bridge_figure.tex",k).unwrap();
}