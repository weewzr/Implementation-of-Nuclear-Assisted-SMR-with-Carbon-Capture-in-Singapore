use plotters::prelude::*;
use crate::final_design::final_design;

pub fn threshold_svg(path:&str)->Result<(),Box<dyn std::error::Error>>{
 let x=final_design(0.0,false);
 let root=SVGBackend::new(path,(900,520)).into_drawing_area(); root.fill(&WHITE)?;
 let mut chart=ChartBuilder::on(&root).margin(35).caption("CN4252 verified model thresholds",("sans-serif",28)).x_label_area_size(55).y_label_area_size(70).build_cartesian_2d(0f64..4.2f64,0f64..110f64)?;
 chart.configure_mesh().x_desc("Annual abatement / required 0.25 MtCO2e/y (ratio)").y_desc("Abatement cost (S$/tCO2e)").draw()?;
 chart.draw_series(LineSeries::new(vec![(0.0,100.0),(4.2,100.0)],&BLACK.mix(0.5)))?;
 chart.draw_series(std::iter::once(Circle::new((x.lifecycle_avoided_t/250000.0,x.abatement_cost_sgd_t),8,BLUE.filled())))?.label("Verified zero-credit model").legend(|(x,y)|Circle::new((x,y),5,BLUE.filled()));
 chart.configure_series_labels().border_style(BLACK).draw()?; root.present()?; Ok(())
}