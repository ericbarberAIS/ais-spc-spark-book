use std::error::Error;
use std::io;

mod mock_data;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Enter the desired file format (csv or parquet):");
    let mut file_format = String::new();
    io::stdin().read_line(&mut file_format)?;
   
    match file_format.trim().to_lowercase().as_str() {
            "csv" => {
                println!("Generating CSV file...");
                mock_data::generate_csv()?;
            },
            "parquet" => {
                println!("Generating Parquet file...");
                mock_data::generate_parquet()?;
            },
            _ => {
                println!("Invalid format. Please enter 'csv' or 'parquet'.");
                return Ok(());
            }
        }


    println!("File generation completed.");
    Ok(())
}
