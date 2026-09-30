use nuclear_assisted_smr::final_design::{availability_sensitivity,gas_backup_sensitivity};
fn main(){
 println!("# Final-Design Reliability Sensitivity\n");
 println!("Screening sensitivities only; neither is a PRA/reliability prediction.\n");
 println!("## No-backup availability sensitivity\n");
 println!("| Availability | H2 (t/y) | Lifecycle avoided (tCO2e/y) | Incremental S$/y | S$/tCO2e | >0.25 Mt/y | <S$100/t |");
 println!("|---:|---:|---:|---:|---:|:---:|:---:|");
 for a in [0.30,0.40,0.50,0.60,0.70,0.80,0.85,0.90,0.95]{
  let x=availability_sensitivity(a);
  println!("| {:.0}% | {:.0} | {:.0} | {:.0} | {:.2} | {} | {} |",100.0*a,x.annual_h2_t,x.lifecycle_avoided_t,x.annual_incremental_sgd,x.abatement_cost_sgd_t,if x.pass_abatement{"PASS"}else{"FAIL"},if x.pass_cost{"PASS"}else{"FAIL"});
 }
 let acrit=0.85*250_000.0/availability_sensitivity(0.85).lifecycle_avoided_t;
 println!("\nBecause lifecycle avoided emissions are linear in operating hours in this no-backup screen, the abatement threshold occurs at approximately **{:.1}% availability**.\n",100.0*acrit);
 println!("## Gas-backup operating sensitivity\n");
 println!("Gas backup maintains the base 85% process-service hours when nuclear availability is lower. Backup CAPEX/staffing/maintenance/start-up/integration are excluded; this is a lower-bound operating sensitivity.\n");
 println!("| Nuclear availability | Backup h/y | Backup fuel (GJ/y) | Backup direct CO2 (t/y) | Lifecycle avoided (tCO2e/y) | Backup fuel S$/y | S$/tCO2e | Joint pass |");
 println!("|---:|---:|---:|---:|---:|---:|---:|:---:|");
 for a in [0.50,0.60,0.70,0.80,0.85]{
  let x=gas_backup_sensitivity(a);
  println!("| {:.0}% | {:.0} | {:.0} | {:.0} | {:.0} | {:.0} | {:.2} | {} |",100.0*a,x.backup_hours,x.backup_fuel_gj,x.backup_direct_co2_t,x.lifecycle_avoided_t,x.backup_variable_cost_sgd,x.abatement_cost_sgd_t,if x.joint_pass{"PASS"}else{"FAIL"});
 }
 println!("\nInterpretation: the backup table does **not** establish bankable backup economics because backup capital and fixed O&M are deliberately not invented.");
}