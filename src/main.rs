use polars::prelude::*;

fn main() -> PolarsResult<()> {
    let df = CsvReader::from_path("data/example.csv")?
        .infer_schema(Some(10))
        .has_header(true)
        .finish()?;

    let df = df.lazy()
        .with_column((col("population") / lit(1000)).alias("population_thousands"))
        .collect()?;

    let idx = df.column("population")?.idx_max().unwrap();
    let city = df.column("city")?.utf8()?.get(idx).unwrap();
    let pop = df.column("population")?.get(idx).unwrap();

    println!("Largest city: {} with {}", city, pop);
    println!("\nTransformed data:\n{}", df);

    Ok(())
}
