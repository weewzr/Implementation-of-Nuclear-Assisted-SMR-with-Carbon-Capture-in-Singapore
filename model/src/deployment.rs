use crate::*;

pub const DEPLOYMENT_ANNUAL_SCALE:f64=1.30;
pub const GTHTR_MODULE_MWTH:f64=600.0;
pub const JPY_TO_SGD:f64=0.0087;
pub const USD_TO_SGD:f64=1.30;
pub const EUR_TO_SGD:f64=1.50;
pub const JAEA_HEAT_JPY_MJ:f64=0.70;
pub const IEAGHG_CCS_INCREMENTAL_TCR_EUR:f64=41.02e6;
pub const IEAGHG_CCS_REF_CAPTURE_T_Y:f64=0.4660*100_000.0*8322.0/1000.0;

#[derive(Debug,Clone,Copy)]
pub enum DeploymentCostClass { JaeaMature, ModernCentral, FoakAdverse }
#[derive(Debug,Clone,Copy)]
pub struct DeploymentResult {
 pub class:DeploymentCostClass,pub cogeneration:bool,
 pub annual_h2_t:f64,pub annual_avoided_t:f64,pub throughput_scale:f64,
 pub process_heat_mw:f64,pub reactor_modules:u32,pub reactor_capacity_mwth:f64,
 pub reactor_utilisation:f64,pub co2_stored_t_y:f64,
 pub nuclear_capex_allocated_sgd:f64,pub ccs_capex_sgd:f64,pub integration_capex_sgd:f64,
 pub annual_nuclear_sgd:f64,pub annual_ccs_capital_sgd:f64,pub annual_integration_sgd:f64,
 pub annual_ts_sgd:f64,pub annual_other_incremental_sgd:f64,
 pub annual_incremental_sgd:f64,pub abatement_cost_sgd_t:f64,
 pub pass_abatement:bool,pub pass_cost:bool,pub joint_pass:bool,
}
pub fn crf(i:f64,n:u32)->f64 { i*(1.0+i).powi(n as i32)/((1.0+i).powi(n as i32)-1.0) }

fn params(k:DeploymentCostClass)->(f64,f64,f64,u32,f64,f64,f64,f64) {
 // cf,wacc,life,occ SGD/kWth, O&M SGD/MWhth,T&S SGD/t,integration fraction, gas price
 match k {
  DeploymentCostClass::JaeaMature => (0.80,0.03,40,50.0e9*JPY_TO_SGD/600_000.0,0.0,15.0,0.10,15.0),
  DeploymentCostClass::ModernCentral => (0.93,0.075,60,2500.0*USD_TO_SGD,12.0*USD_TO_SGD,30.0,0.20,17.5),
  DeploymentCostClass::FoakAdverse => (0.80,0.10,60,3250.0*USD_TO_SGD,16.0*USD_TO_SGD,45.0,0.30,20.0),
 }
}
pub fn deployment_minimum_annual_h2_t()->f64 {
 let p=r3_uncertainty_point(950.0,20.0,0.90,0.95,11.5,5.5,15.0,5.69,150.0,80e6);
 let specific=p.annual_avoided_t/(IEAGHG_BASE.h2_kg_per_h*8322.0/1000.0);
 250_000.0/specific
}
pub fn deployment_case(k:DeploymentCostClass,cogeneration:bool)->DeploymentResult {
 let (cf,wacc,life,occ,om,ts,integ_frac,gas_price)=params(k);
 let hours=cf*8760.0;
 let annual_ratio=DEPLOYMENT_ANNUAL_SCALE;
 let throughput_scale=annual_ratio*8322.0/hours;
 let s=r3_solve_case(950.0+273.15,20.0,0.95,0.90);
 let h=r3_heat_cascade(650.0,20.0,30.0,500.0);
 let c=r3_ccs_ledger(s);
 let circ=r3_helium_circulator_hi_mwe(h);
 let process_heat=h.nuclear_heat_hi_mw*throughput_scale;
 let modules=(process_heat/GTHTR_MODULE_MWTH).ceil() as u32;
 let reactor_capacity=modules as f64*GTHTR_MODULE_MWTH;
 let util=process_heat/reactor_capacity;
 let annual_h2=IEAGHG_BASE.h2_kg_per_h*hours*throughput_scale/1000.0;
 let p=r3_uncertainty_point(950.0,20.0,0.90,0.95,11.5,5.5,gas_price,5.69,150.0,0.0);
 let annual_avoided=p.annual_avoided_t*annual_ratio;
 let purge_c=c.purge_oxidation_co2_kmol_h;
 let captured_kmol_h=s.captured_co2_kmol_h+0.95*purge_c;
 let stored=captured_kmol_h*44.0095*hours*throughput_scale/1000.0;
 let ccs_cap=IEAGHG_CCS_INCREMENTAL_TCR_EUR*EUR_TO_SGD*(stored/IEAGHG_CCS_REF_CAPTURE_T_Y);
 let full_reactor_cap=occ*1000.0*reactor_capacity;
 let alloc_cap=if cogeneration{full_reactor_cap*util}else{full_reactor_cap};
 let integration_cap=integ_frac*(alloc_cap+ccs_cap);
 let annual_ccs_cap=ccs_cap*crf(0.08,25);
 let annual_integration=integration_cap*crf(wacc,life);
 let heat_energy_gj=process_heat*hours*3.6;
 let annual_nuclear=match k {
   DeploymentCostClass::JaeaMature => {
     let priced_mw=if cogeneration{process_heat}else{reactor_capacity};
     priced_mw*hours*3.6*(JAEA_HEAT_JPY_MJ*JPY_TO_SGD*1000.0)
   },
   _ => alloc_cap*crf(wacc,life)+om*(if cogeneration{process_heat}else{reactor_capacity})*hours,
 };
 let base=annual_thermal_energy_cost_sgd(ieaghg_total_ng_lhv_mw()*throughput_scale,hours,gas_price);
 let fresh=annual_thermal_energy_cost_sgd(ieaghg_feed_lhv_mw()*s.fresh_fraction*throughput_scale,hours,gas_price);
 let aux=(c.co2_compression_mwe+c.tail_compression_mwe+circ)*throughput_scale*hours*175.0;
 let other=fresh+aux-base;
 let annual_ts=stored*ts;
 let incremental=other+annual_nuclear+annual_ccs_cap+annual_integration+annual_ts;
 let ac=incremental/annual_avoided;
 DeploymentResult{class:k,cogeneration,annual_h2_t:annual_h2,annual_avoided_t:annual_avoided,
  throughput_scale,process_heat_mw:process_heat,reactor_modules:modules,reactor_capacity_mwth:reactor_capacity,
  reactor_utilisation:util,co2_stored_t_y:stored,nuclear_capex_allocated_sgd:alloc_cap,
  ccs_capex_sgd:ccs_cap,integration_capex_sgd:integration_cap,
  annual_nuclear_sgd:annual_nuclear,annual_ccs_capital_sgd:annual_ccs_cap,
  annual_integration_sgd:annual_integration,annual_ts_sgd:annual_ts,
  annual_other_incremental_sgd:other,annual_incremental_sgd:incremental,
  abatement_cost_sgd_t:ac,pass_abatement:annual_avoided>250_000.0,
  pass_cost:ac<100.0,joint_pass:annual_avoided>250_000.0&&ac<100.0}
}
pub fn deployment_break_even_heat_sgd_gj()->f64 {
 let x=deployment_case(DeploymentCostClass::JaeaMature,true);
 let heat_gj=x.process_heat_mw*(0.80*8760.0)*3.6;
 let nonheat=x.annual_incremental_sgd-x.annual_nuclear_sgd;
 (100.0*x.annual_avoided_t-nonheat)/heat_gj
}
pub fn deployment_cases_csv()->String {
 let mut s=String::from("case,cogeneration,h2_t_y,throughput_scale,reactor_mwth,process_heat_mw,utilisation,co2_stored_t_y,annual_avoided_t,annual_incremental_sgd,cost_sgd_t,abatement_pass,cost_pass,joint_pass\n");
 for k in [DeploymentCostClass::JaeaMature,DeploymentCostClass::ModernCentral,DeploymentCostClass::FoakAdverse] {
  for cog in [true,false] { let x=deployment_case(k,cog); let n=match k{DeploymentCostClass::JaeaMature=>"JAEA mature",DeploymentCostClass::ModernCentral=>"Modern central",DeploymentCostClass::FoakAdverse=>"FOAK adverse"};
   s.push_str(&format!("{},{},{:.3},{:.6},{:.1},{:.3},{:.4},{:.3},{:.3},{:.3},{:.3},{},{},{}\n",n,if cog{"cogeneration"}else{"hydrogen-only"},x.annual_h2_t,x.throughput_scale,x.reactor_capacity_mwth,x.process_heat_mw,x.reactor_utilisation,x.co2_stored_t_y,x.annual_avoided_t,x.annual_incremental_sgd,x.abatement_cost_sgd_t,x.pass_abatement,x.pass_cost,x.joint_pass));
  }
 } s
}
pub fn deployment_cost_breakdown_csv()->String {
 let x=deployment_case(DeploymentCostClass::JaeaMature,true);
 format!("component,annual_sgd\nNG and auxiliary net,{:.3}\nNuclear heat service,{:.3}\nCCS capital annualisation,{:.3}\nIntegration/site annualisation,{:.3}\nCO2 transport-storage,{:.3}\n",x.annual_other_incremental_sgd,x.annual_nuclear_sgd,x.annual_ccs_capital_sgd,x.annual_integration_sgd,x.annual_ts_sgd)
}
pub fn deployment_scale_curve_csv()->String {
 let p=r3_uncertainty_point(950.0,20.0,0.90,0.95,11.5,5.5,15.0,5.69,150.0,0.0);
 let mut s=String::from("annual_scale,h2_t_y,annual_avoided_t\n");
 for i in 10..=30 {let a=i as f64/10.0;s.push_str(&format!("{:.1},{:.3},{:.3}\n",a,IEAGHG_BASE.h2_kg_per_h*8322.0/1000.0*a,p.annual_avoided_t*a));} s
}
#[cfg(test)] mod tests {
 use super::*;
 #[test] fn scale_one_reproduces_canonical_annual_h2(){assert!((IEAGHG_BASE.h2_kg_per_h*8322.0/1000.0-r3_singapore_scale().annual_h2_t).abs()<1e-9);}
 #[test] fn minimum_scale_exceeds_original(){assert!(deployment_minimum_annual_h2_t()>r3_singapore_scale().annual_h2_t);}
 #[test] fn crf_identity(){assert!((crf(0.03,40)-0.04326237789046286).abs()<1e-12);}
 #[test] fn module_sizing_and_utilisation_are_physical(){for k in [DeploymentCostClass::JaeaMature,DeploymentCostClass::ModernCentral,DeploymentCostClass::FoakAdverse]{let x=deployment_case(k,true);assert!(x.reactor_modules>=1&&x.reactor_utilisation>0.0&&x.reactor_utilisation<=1.0);}}
 #[test] fn ccs_scales_and_cost_identity_holds(){let x=deployment_case(DeploymentCostClass::JaeaMature,true);assert!(x.co2_stored_t_y>r3_singapore_scale().total_co2_to_storage_t_y);assert!((x.abatement_cost_sgd_t-x.annual_incremental_sgd/x.annual_avoided_t).abs()<1e-12);}
 #[test] fn original_gate5_result_is_untouched(){let s=r3_uncertainty_summary();assert_eq!(s.n,64);assert_eq!(s.both_pass,0);}
 #[test] fn mature_cogeneration_is_conditional_joint_pass(){let x=deployment_case(DeploymentCostClass::JaeaMature,true);assert!(x.joint_pass);assert!(!deployment_case(DeploymentCostClass::JaeaMature,false).joint_pass);}
 #[test] fn modern_and_foak_cases_do_not_pass_cost(){assert!(!deployment_case(DeploymentCostClass::ModernCentral,true).pass_cost);assert!(!deployment_case(DeploymentCostClass::FoakAdverse,true).pass_cost);}
}
