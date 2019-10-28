use plotters::prelude::*;
use wasm_bindgen::prelude::*;

fn start_plotting(
    element: &str,
    rate: f32,
    max: f32,
    mid: f32,
    y_min: f32,
    y_max: f32,
) -> Result<Box<dyn Fn((i32, i32)) -> Option<(f32, f32)>>, Box<dyn std::error::Error>> {
    let backend = CanvasBackend::new(element).unwrap();
    let root = backend.into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(y_min..y_max, 0f32..1.1f32 * max)?;

    chart.configure_mesh().x_labels(3).y_labels(3).draw()?;

    chart.draw_series(LineSeries::new(
        (y_min as i32 * 1000..= y_max as i32 * 1000)
            .map(|x| x as f32 / 50.0)
            .map(|x| (x, max / ( 1f32 + (-rate * (x - mid)).exp2()))),
        &RED,
    ))?;

    root.present()?;
    return Ok(Box::new(chart.into_coord_trans()));
}

#[wasm_bindgen]
pub fn draw_func(element: &str, rate: f32, max: f32, mid: f32, y_min: f32, y_max: f32) -> JsValue {
    crate::make_coord_mapping_closure(start_plotting(element, rate, max, mid, y_min, y_max).ok())
}
