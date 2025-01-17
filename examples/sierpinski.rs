use plotters::coord::Shift;
use plotters::prelude::*;

pub fn sierpinski_carpet(
    depth: u32,
    drawing_area: &DrawingArea<BitMapBackend, Shift>,
) -> Result<(), Box<dyn std::error::Error>> {
    if depth > 0 {
        let sub_areas = drawing_area.split_evenly((3, 3));
        for (idx, sub_area) in (0..).zip(sub_areas.iter()) {
            if idx != 4 {
                sub_area.fill(&BLUE)?;
                sierpinski_carpet(depth - 1, sub_area)?;
            } else {
                sub_area.fill(&WHITE)?;
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root =
        BitMapBackend::new("plotters-doc-data/sierpinski.png", (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    let root = root
        .titled("Sierpinski Carpet Demo", ("Arial", 60))?
        .shrink(((1024 - 700) / 2, 0), (700, 700));

    sierpinski_carpet(5, &root)
}
