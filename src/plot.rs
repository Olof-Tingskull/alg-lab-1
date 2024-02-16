use plotters::prelude::*;
use std::error::Error;
use num_traits::cast::AsPrimitive;


pub fn plot<X, Y>(name: &str, data: Vec<(X, Y)>) -> Result<(), Box<dyn Error>> 
    where
        X: AsPrimitive<f64> +,
        Y: AsPrimitive<f64>,
{
    let path = format!("plots/{}.png", name);
    let root = BitMapBackend::new(&path, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let data_f64: Vec<(f64, f64)> = data.iter()
        .map(|&(x, y)| (x.as_(), y.as_()))
        .collect();

    let min_y = data_f64.iter().map(|&(_, y)| y).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.);
    let max_y = data_f64.iter().map(|&(_, y)| y).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.);
    let min_x = data_f64.iter().map(|&(x, _)| x).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.);
    let max_x = data_f64.iter().map(|&(x, _)| x).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.);

    let mut chart = ChartBuilder::on(&root)
        .caption("Time per Iteration", ("sans-serif", 40))
        .margin(5)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        data_f64.into_iter(),
        &RED,
    ))?;

    Ok(())
}