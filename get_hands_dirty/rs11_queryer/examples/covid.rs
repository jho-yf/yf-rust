use anyhow::Result;
use polars::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
    let df = CsvReadOptions::default()
        .with_infer_schema_length(Some(16))
        .try_into_reader_with_file_path(Some("owid-covid-latest.csv".into()))?
        .finish()?;

    println!(
        "{}", 
        df.filter(&df["new_deaths"].gt(50)?)?
        .select(["location", "total_cases", "new_cases", "total_deaths", "new_deaths"])?
    );

    Ok(())
}