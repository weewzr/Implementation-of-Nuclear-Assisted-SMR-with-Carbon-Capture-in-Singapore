use nuclear_assisted_smr::final_design::ccs_capture_sensitivity;
fn main(){
 println!("# CCS Capture / Storage Availability Sensitivity\n");
 println!("Screening sensitivity on the fraction of the canonical source-model captured stream actually captured and stored. This is not a capture-unit turndown model.\n");
 println!("| Fraction of canonical captured stream | Captured (t/y) | Candidate direct CO2 (t/y) | Lifecycle avoided (tCO2e/y) | Incremental S$/y | S$/tCO2e | >0.25 Mt/y | <S$100/t |");
 println!("|---:|---:|---:|---:|---:|---:|:---:|:---:|");
 for f in [0.0,0.25,0.50,0.557,0.75,0.90,1.0]{
  let x=ccs_capture_sensitivity(f);
  println!("| {:.1}% | {:.0} | {:.0} | {:.0} | {:.0} | {:.2} | {} | {} |",100.0*f,x.captured_t_y,x.candidate_direct_t_y,x.lifecycle_avoided_t,x.annual_incremental_sgd,x.abatement_cost_sgd_t,if x.pass_abatement{"PASS"}else{"FAIL"},if x.pass_cost{"PASS"}else{"FAIL"});
 }
 let (mut lo,mut hi)=(0.0,1.0);
 for _ in 0..80{let mid=(lo+hi)/2.0;if ccs_capture_sensitivity(mid).lifecycle_avoided_t>250_000.0{hi=mid}else{lo=mid}}
 let crit=hi;
 println!("\nMinimum fraction of the canonical captured stream required to exceed 0.25 MtCO2e/y in this screening model: **{:.2}%**.",100.0*crit);
 println!("\nSingapore cross-border CCS remains an infrastructure/regulatory dependency; passing this arithmetic sensitivity does not establish storage-chain availability.");
}