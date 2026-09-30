use crate::*;
use crate::deployment::{crf,ccs_capex_sgd,jaea_plant_capex_sgd,jaea_heat_sgd_gj,jaea_electric_sgd_mwh,JAEA_AVAIL};

pub const REFORMER_C:f64=871.0;
pub const PROCESS_HEAT_C:f64=900.0;
pub const REACTOR_OUT_C:f64=950.0;
pub const INL_ROT_C:f64=925.0;
pub const PRESSURE_BAR:f64=31.7;
pub const STEAM_CARBON:f64=3.0;
pub const METHANE_CONVERSION:f64=0.781;
pub const PSA_RECOVERY:f64=0.88;
pub const H2_LB_H:f64=29_000.0;
pub const H2_MMSCFD:f64=130.0;
pub const NG_BASE_MMSCFD:f64=52.5;
pub const NG_FINAL_MMSCFD:f64=34.0;
pub const HEAT_MWTH:f64=176.8;
pub const PROCESS_ELECTRIC_MWE:f64=17.3;
pub const CAPTURED_SHORT_T_D:f64=1927.0;
pub const EMITTED_SHORT_T_D:f64=142.0;
pub const BASE_EMITTED_SHORT_T_D:f64=3205.0;
pub const REACTOR_MWTH:f64=600.0;
pub const REFERENCE_IHX_MWTH:f64=170.0;
pub const SECONDARY_RETURN_C:f64=585.0; // JAEA helium-heated SMR precedent; integration assumption for flow sizing.
pub const CP_HE_KJ_KG_K:f64=5.2;
pub const NG_HHV_BTU_SCF:f64=1044.0;
pub const GAS_PRICE_SGD_GJ:f64=15.0;
pub const ELEC_VALUE_SGD_MWH:f64=150.0;
pub const T_AND_S_SGD_T:f64=15.0;

#[derive(Clone,Copy,Debug)]
pub struct FinalDesign {
 pub annual_h2_t:f64,pub captured_t_y:f64,pub emitted_t_y:f64,
 pub annual_avoided_t:f64,pub lifecycle_avoided_t:f64,pub lifecycle_ci_kgkg:f64,
 pub helium_flow_kg_s:f64,pub reactor_utilisation:f64,
 pub gross_electric_mwe:f64,pub net_export_mwe:f64,pub annual_export_mwh:f64,
 pub reactor_capex_sgd:f64,pub ccs_capex_sgd:f64,pub annual_incremental_sgd:f64,
 pub abatement_cost_sgd_t:f64,pub pass_abatement:bool,pub pass_cost:bool,pub joint_pass:bool
}
fn short_t_to_t(x:f64)->f64{x*0.90718474}
fn ng_gj_day(mmscfd:f64)->f64{mmscfd*1e6*NG_HHV_BTU_SCF*1.05505585262e-6}
pub fn final_design(electricity_value:f64,double_ihx:bool)->FinalDesign{
 let hours=JAEA_AVAIL*8760.0;
 let annual_h2_t=H2_LB_H*0.45359237*hours/1000.0;
 let captured_t_y=short_t_to_t(CAPTURED_SHORT_T_D)*365.0*JAEA_AVAIL;
 let emitted_t_y=short_t_to_t(EMITTED_SHORT_T_D)*365.0*JAEA_AVAIL;
 let base_emitted=short_t_to_t(BASE_EMITTED_SHORT_T_D)*365.0*JAEA_AVAIL;
 let direct_avoided=base_emitted-emitted_t_y;
 let upstream_base=ng_gj_day(NG_BASE_MMSCFD)*365.0*JAEA_AVAIL*11.5/1000.0;
 let upstream_final=ng_gj_day(NG_FINAL_MMSCFD)*365.0*JAEA_AVAIL*11.5/1000.0;
 let nuclear_lca=HEAT_MWTH*1000.0*hours*(5.5*0.504)/1e6;
 let aux_lca=(PROCESS_ELECTRIC_MWE-6.3).max(0.0)*1000.0*hours*5.5/1e6;
 let transport=0.025*captured_t_y;
 let lifecycle_avoided=direct_avoided+(upstream_base-upstream_final)-nuclear_lca-aux_lca-transport;
 let lifecycle_ci=(base_emitted+upstream_base-lifecycle_avoided)*1000.0/annual_h2_t;
 let helium_flow=HEAT_MWTH*1000.0/(CP_HE_KJ_KG_K*(PROCESS_HEAT_C-SECONDARY_RETURN_C));
 let util=HEAT_MWTH/REACTOR_MWTH;
 // JAEA cogeneration envelope: 202 MWe at 170 MWth heat; use conservative 202 MWe at 176.8 MWth.
 let gross=202.0;
 let net_export=(gross-PROCESS_ELECTRIC_MWE).max(0.0);
 let annual_export=net_export*hours;
 // 176.8 MWth exceeds the 170 MWth reference IHX; baseline therefore uses doubled-IHX source sensitivity.
 let heat_burden=HEAT_MWTH*hours*3.6*jaea_heat_sgd_gj(double_ihx);
 let power_burden=gross*hours*jaea_electric_sgd_mwh(double_ihx);
 let reactor_burden=heat_burden+power_burden;
 let power_value=annual_export*electricity_value;
 let ccs_cap=ccs_capex_sgd(captured_t_y);
 let annual_ccs=ccs_cap*crf(0.08,25);
 let integration=0.10*(jaea_plant_capex_sgd(double_ihx)+ccs_cap)*crf(0.03,40);
 let ts=captured_t_y*T_AND_S_SGD_T;
 let base_ng=ng_gj_day(NG_BASE_MMSCFD)*365.0*JAEA_AVAIL*GAS_PRICE_SGD_GJ;
 let final_ng=ng_gj_day(NG_FINAL_MMSCFD)*365.0*JAEA_AVAIL*GAS_PRICE_SGD_GJ;
 let incremental=final_ng+reactor_burden+annual_ccs+integration+ts-power_value-base_ng;
 let cost=incremental/lifecycle_avoided;
 FinalDesign{annual_h2_t,captured_t_y,emitted_t_y,annual_avoided_t:direct_avoided,lifecycle_avoided_t:lifecycle_avoided,
 lifecycle_ci_kgkg:lifecycle_ci,helium_flow_kg_s:helium_flow,reactor_utilisation:util,gross_electric_mwe:gross,
 net_export_mwe:net_export,annual_export_mwh:annual_export,reactor_capex_sgd:jaea_plant_capex_sgd(double_ihx),
 ccs_capex_sgd:ccs_cap,annual_incremental_sgd:incremental,abatement_cost_sgd_t:cost,
 pass_abatement:lifecycle_avoided>250_000.0,pass_cost:cost<100.0,joint_pass:lifecycle_avoided>250_000.0&&cost<100.0}
}
pub fn break_even_electricity()->f64{
 let z=final_design(0.0,true);let target=100.0*z.lifecycle_avoided_t;
 (z.annual_incremental_sgd-target)/z.annual_export_mwh
}
pub fn results_csv()->String{
 let mut s=String::from("scenario,h2ty,heatmw,heliumkgs,capturedty,avoidedty,incrementalsgd,costsgdt,apass,cpass,joint\n");
 for (n,e) in [("Final baseline",final_design(150.0,true)),("Doubled-IHX zero surplus",final_design(0.0,true)),("Singapore low",final_design(100.0,true)),("Singapore high",final_design(200.0,true)),("Reference-IHX sensitivity",final_design(150.0,false))]{
 s.push_str(&format!("{},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{},{},{}\n",n,e.annual_h2_t,HEAT_MWTH,e.helium_flow_kg_s,e.captured_t_y,e.lifecycle_avoided_t,e.annual_incremental_sgd,e.abatement_cost_sgd_t,e.pass_abatement,e.pass_cost,e.joint_pass));}s
}
pub fn source_balance_csv()->String{format!("parameter,value,unit,class\nReformer outlet,{REFORMER_C},C,SOURCE-BACKED\nProcess heat delivery,{PROCESS_HEAT_C},C,SOURCE-BACKED\nReactor outlet INL,{INL_ROT_C},C,SOURCE-BACKED\nReactor primary JAEA,{REACTOR_OUT_C},C,SOURCE-BACKED\nPressure,{PRESSURE_BAR},bar,SOURCE-BACKED\nSteam-carbon,{STEAM_CARBON},mol/mol,SOURCE-BACKED\nMethane conversion,{METHANE_CONVERSION},fraction,SOURCE-BACKED\nPSA recovery,{PSA_RECOVERY},fraction,SOURCE-BACKED\nHydrogen,{H2_MMSCFD},MMSCFD,SOURCE-BACKED\nNatural gas final,{NG_FINAL_MMSCFD},MMSCFD,SOURCE-BACKED\nProcess heat,{HEAT_MWTH},MWth,SOURCE-BACKED\nProcess electricity,{PROCESS_ELECTRIC_MWE},MWe,SOURCE-BACKED\nCaptured CO2,{CAPTURED_SHORT_T_D},short ton/day,SOURCE-BACKED\nEmitted CO2,{EMITTED_SHORT_T_D},short ton/day,SOURCE-BACKED\n")
}
pub fn temperature_sensitivity_csv()->String{
 String::from("case,reactoroutc,heatdeliveryc,reformerc,architecture\nINL lower,875,850,871,nuclear plus fired trim\nFINAL,925,900,871,all nuclear reforming heat\nJAEA primary envelope,950,900,871,GTHTR300C hardware basis\n")
}
#[cfg(test)]mod tests{use super::*;
 #[test]fn source_temperature_ladder(){assert!(REACTOR_OUT_C>PROCESS_HEAT_C&&PROCESS_HEAT_C>REFORMER_C);assert_eq!(PROCESS_HEAT_C-REFORMER_C,29.0);}
 #[test]fn source_balance_values(){assert_eq!(METHANE_CONVERSION,0.781);assert_eq!(STEAM_CARBON,3.0);assert_eq!(PSA_RECOVERY,0.88);}
 #[test]fn ihx_requires_conservative_sensitivity(){assert!(HEAT_MWTH>REFERENCE_IHX_MWTH&&HEAT_MWTH<REACTOR_MWTH);}
 #[test]fn final_mass_scale_positive(){let x=final_design(150.0,true);assert!(x.annual_h2_t>90_000.0&&x.captured_t_y>500_000.0);}
 #[test]fn final_lifecycle_positive_and_threshold(){let x=final_design(150.0,true);assert!(x.lifecycle_avoided_t>250_000.0);}
 #[test]fn no_free_reactor_capacity(){let x=final_design(0.0,true);assert!(x.annual_incremental_sgd>final_design(150.0,true).annual_incremental_sgd);}
 #[test]fn break_even_reproduces_cost_threshold(){let v=break_even_electricity();assert!((final_design(v,true).abatement_cost_sgd_t-100.0).abs()<1e-8);}
 #[test]fn historical_gate5_still_reproduces(){let x=r3_uncertainty_summary();assert_eq!(x.n,64);assert_eq!(x.both_pass,0);}
}
