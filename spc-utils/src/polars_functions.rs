// Ensure you have the necessary dependencies in Cargo.toml
// [dependencies]
// polars = { version = "0.15", features = ["parquet"] }

use polars::prelude::*;
use std::error::Error;

fn read_parquet_and_describe(file_path: &str) -> Result<DataFrame, Box<dyn Error>> {
    // Read the Parquet file into a DataFrame
    let df = ParquetReader::from_path(file_path)?.finish()?;

    // Print the number of rows and columns
    println!("Number of rows: {}", df.height());
    println!("Number of columns: {}", df.width());

    // Print column names
    println!("Column names:");
    for name in df.get_column_names() {
        println!("{}", name);
    }

    // Return the DataFrame
    Ok(df)
}
