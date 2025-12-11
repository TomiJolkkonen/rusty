use polars::prelude::*;
use plotters::prelude::*;
use std::fs;

fn main() -> PolarsResult<()> {
    let students = CsvReader::from_path("bronze/student.csv")?
        .infer_schema(Some(10)).has_header(true).finish()?;

    let grades = CsvReader::from_path("bronze/grades.csv")?
        .infer_schema(Some(10)).has_header(true).finish()?;

    // SILVER: clean and join
    let students_clean = students.lazy()
        .filter(col("age").gt(lit(0)))
        .collect()?;

    let grades_clean = grades.lazy()
        .filter(col("grade").gt(lit(0)))
        .collect()?;

    let unified = students_clean.lazy()
        .inner_join(grades_clean.lazy(), col("student_id"), col("student_id"))
        .collect()?;

    fs::create_dir_all("silver").unwrap();
    CsvWriter::new(fs::File::create("silver/unified.csv")?)
        .finish(&unified)?;

    // GOLD – star schema
    let star = unified.select(&[
        col("student_id"),
        col("name"),
        col("age"),
        col("course"),
        col("grade")
    ]);

    fs::create_dir_all("gold").unwrap();
    CsvWriter::new(fs::File::create("gold/star.csv")?)
        .finish(&star)?;

    // Scatter: age vs grade
    fs::create_dir_all("plots").unwrap();
    let root = BitMapBackend::new("plots/scatter.png", (400, 400)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let ages: Vec<f64> = star.column("age")?.f64()?.into_no_null_iter().collect();
    let grades: Vec<f64> = star.column("grade")?.f64()?.into_no_null_iter().collect();
    let points: Vec<(f64, f64)> = ages.into_iter().zip(grades).collect();

    let mut chart = ChartBuilder::on(&root)
        .caption("Age vs Grade", ("sans-serif", 15))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(18.0..30.0, 0.0..6.0)
        .unwrap();

    chart.configure_mesh().draw().unwrap();
    chart.draw_series(points.iter().map(|(x, y)| Circle::new((*x, *y), 4, RED.filled()))).unwrap();

    println!("Pipeline done. Check silver/, gold/, plots/scatter.png");
    Ok(())
}
