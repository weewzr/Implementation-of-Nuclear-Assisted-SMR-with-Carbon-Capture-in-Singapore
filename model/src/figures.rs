use plotters::prelude::*;
use crate::final_design::{final_design,availability_sensitivity,gas_backup_sensitivity,ccs_capture_sensitivity};

pub fn threshold_svg(path:&str)->Result<(),Box<dyn std::error::Error>>{
 let x=final_design(0.0,false);
 let root=SVGBackend::new(path,(900,520)).into_drawing_area(); root.fill(&WHITE)?;
 let mut chart=ChartBuilder::on(&root).margin(35).caption("CN4252 verified model thresholds",("sans-serif",28)).x_label_area_size(55).y_label_area_size(70).build_cartesian_2d(0f64..4.2f64,0f64..110f64)?;
 chart.configure_mesh().x_desc("Annual abatement / required 0.25 MtCO2e/y (ratio)").y_desc("Abatement cost (S$/tCO2e)").draw()?;
 chart.draw_series(LineSeries::new(vec![(0.0,100.0),(4.2,100.0)],&BLACK.mix(0.5)))?;
 chart.draw_series(std::iter::once(Circle::new((x.lifecycle_avoided_t/250000.0,x.abatement_cost_sgd_t),8,BLUE.filled())))?.label("Verified zero-credit model").legend(|(x,y)|Circle::new((x,y),5,BLUE.filled()));
 chart.configure_series_labels().border_style(BLACK).draw()?; root.present()?; Ok(())
}

pub fn availability_abatement_svg(path:&str)->Result<(),Box<dyn std::error::Error>>{
 let root=SVGBackend::new(path,(900,520)).into_drawing_area();root.fill(&WHITE)?;
 let mut chart=ChartBuilder::on(&root).margin(35).caption("Availability sensitivity: lifecycle abatement",("sans-serif",26)).x_label_area_size(55).y_label_area_size(80).build_cartesian_2d(25f64..100f64,0f64..1_100_000f64)?;
 chart.configure_mesh().x_desc("Effective annual availability (%)").y_desc("Lifecycle CO2e avoided (t/y)").draw()?;
 chart.draw_series(LineSeries::new(vec![(25.0,250_000.0),(100.0,250_000.0)],&BLACK.mix(0.5)))?.label("CN4252 threshold");
 let pts=(25..=100).map(|i|{let a=i as f64/100.0;let x=availability_sensitivity(a);(i as f64,x.lifecycle_avoided_t)});
 chart.draw_series(LineSeries::new(pts,&BLUE))?.label("No-backup screen");
 let bpts=(50..=85).map(|i|{let a=i as f64/100.0;let x=gas_backup_sensitivity(a);(i as f64,x.lifecycle_avoided_t)});
 chart.draw_series(LineSeries::new(bpts,&RED))?.label("Gas-backup lower-bound screen");
 chart.configure_series_labels().border_style(BLACK).draw()?;root.present()?;Ok(())
}

pub fn availability_cost_svg(path:&str)->Result<(),Box<dyn std::error::Error>>{
 let root=SVGBackend::new(path,(900,520)).into_drawing_area();root.fill(&WHITE)?;
 let mut chart=ChartBuilder::on(&root).margin(35).caption("Availability sensitivity: abatement cost",("sans-serif",26)).x_label_area_size(55).y_label_area_size(80).build_cartesian_2d(25f64..100f64,-20f64..120f64)?;
 chart.configure_mesh().x_desc("Effective annual availability (%)").y_desc("Screening abatement cost (S$/tCO2e)").draw()?;
 chart.draw_series(LineSeries::new(vec![(25.0,100.0),(100.0,100.0)],&BLACK.mix(0.5)))?.label("CN4252 ceiling");
 let pts=(25..=100).map(|i|{let a=i as f64/100.0;let x=availability_sensitivity(a);(i as f64,x.abatement_cost_sgd_t)});
 chart.draw_series(LineSeries::new(pts,&BLUE))?.label("No-backup screen");
 let bpts=(50..=85).map(|i|{let a=i as f64/100.0;let x=gas_backup_sensitivity(a);(i as f64,x.abatement_cost_sgd_t)});
 chart.draw_series(LineSeries::new(bpts,&RED))?.label("Gas-backup lower-bound screen");
 chart.configure_series_labels().border_style(BLACK).draw()?;root.present()?;Ok(())
}

pub fn ccs_robustness_svg(path:&str)->Result<(),Box<dyn std::error::Error>>{
 let root=SVGBackend::new(path,(900,520)).into_drawing_area();root.fill(&WHITE)?;
 let mut chart=ChartBuilder::on(&root).margin(35).caption("CCS delivery sensitivity",("sans-serif",26)).x_label_area_size(60).y_label_area_size(80).build_cartesian_2d(0f64..100f64,0f64..1_000_000f64)?;
 chart.configure_mesh().x_desc("Fraction of canonical captured stream stored (%)").y_desc("Lifecycle CO2e avoided (t/y)").draw()?;
 chart.draw_series(LineSeries::new(vec![(0.0,250_000.0),(100.0,250_000.0)],&BLACK.mix(0.5)))?.label("CN4252 threshold");
 let pts=(0..=100).map(|i|{let x=ccs_capture_sensitivity(i as f64/100.0);(i as f64,x.lifecycle_avoided_t)});
 chart.draw_series(LineSeries::new(pts,&BLUE))?.label("Steady capture/storage screen");
 chart.configure_series_labels().border_style(BLACK).draw()?;root.present()?;Ok(())
}
