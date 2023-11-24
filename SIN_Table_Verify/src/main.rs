/*
This file will calculate Sin anc Cos functions using an approximate fixed table
and will plot te error compared with builtin float implementation
*/

use fixed::types::I4F12;
use plotters::prelude::*;
mod sin_table;
mod table_trig;

fn main() -> std::io::Result<()>{

    let root_drawing_area = BitMapBackend::new("plot.png", (1024, 768))
    .into_drawing_area();

    let root_drawing_area_sinerror = BitMapBackend::new("sin_error.png", (1024, 768))
    .into_drawing_area();

    let root_drawing_area_coserror = BitMapBackend::new("cos_error.png", (1024, 768))
    .into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();
    root_drawing_area.margin(10,10,10,10);
    root_drawing_area_sinerror.fill(&WHITE).unwrap();
    root_drawing_area_sinerror.margin(10,10,10,10);
    root_drawing_area_coserror.fill(&WHITE).unwrap();
    root_drawing_area_coserror.margin(10,10,10,10);

    let mut chart = ChartBuilder::on(&root_drawing_area)
    .build_cartesian_2d(-6.28..6.28, -1.2..1.2)
    .unwrap();
    let mut chart_1 = ChartBuilder::on(&root_drawing_area_sinerror)
    .caption("Sin approximation error",("sans-serif", 40).into_font())
    .x_label_area_size(20)
    .y_label_area_size(40)
    .build_cartesian_2d(-6.28..6.28, -0.025..0.025)
    .unwrap();
    let mut chart_2 = ChartBuilder::on(&root_drawing_area_coserror)
    .caption("Cos approximation error",("sans-serif", 40).into_font())
    .x_label_area_size(20)
    .y_label_area_size(40)
    .build_cartesian_2d(-6.28..6.28, -0.025..0.025)
    .unwrap();

    chart_1.configure_mesh()
    .x_labels(5)
    .y_labels(5)
    .draw()
    .unwrap();
    chart_2.configure_mesh()
    .x_labels(5)
    .y_labels(5)
    .draw()
    .unwrap();

    chart.draw_series(LineSeries::new(
    (-628..628).map(|x| x as f64 / 100.0).map(|x| (x, x.sin())),
    &RED
    )).unwrap();
    chart.draw_series(LineSeries::new(
        (-628..628).map(|x| x as f64 / 100.0).map(|x| (x, f64::from(table_trig::sin_t(I4F12::from_num(x),&sin_table::SIN_TABLE,12-7)))),
        &BLUE
        )).unwrap();
    chart.draw_series(LineSeries::new(
        (-628..628).map(|x| x as f64 / 100.0).map(|x| (x, x.cos())),
        &RED
        )).unwrap();
    chart.draw_series(LineSeries::new(
        (-628..628).map(|x| x as f64 / 100.0).map(|x| (x, f64::from(table_trig::cos_t(I4F12::from_num(x),&sin_table::SIN_TABLE,12-7)))),
        &BLUE
        )).unwrap();
    chart_1.draw_series(LineSeries::new(
        (-628..628).map(|x| x as f64 / 100.0).map(|x| (x, x.sin()-f64::from(table_trig::sin_t(I4F12::from_num(x),&sin_table::SIN_TABLE,12-7)))),
        &BLUE
        )).unwrap();
    chart_2.draw_series(LineSeries::new(
        (-628..628).map(|x| x as f64 / 100.0).map(|x| (x, x.cos()-f64::from(table_trig::cos_t(I4F12::from_num(x),&sin_table::SIN_TABLE,12-7)))),
        &BLUE
        )).unwrap();
        Ok(())

}
