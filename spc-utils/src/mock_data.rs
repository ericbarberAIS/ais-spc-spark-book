use rand::{distributions::Uniform, Rng};
use rand::prelude::SliceRandom;
use csv::Writer;
use std::error::Error;

use arrow::{array::{StringArray, Int32Array, Float64Array}, datatypes::{DataType, Field, Schema}, record_batch::RecordBatch};
use parquet::{file::properties::WriterProperties, arrow::ArrowWriter};
use std::sync::Arc;
use std::fs::File;

pub fn generate_csv() -> Result<(), Box<dyn Error>> {

    // Get the current directory
    let mut path = env::current_dir()?;

    // Move up one level and enter the 'data' directory
    path.pop();
    path.push("data");

    // Create the 'data' directory if it doesn't exist
    fs::create_dir_all(&path)?;

    // Set the file path for the CSV file
    path.push("mock_dataset.csv");

    // Create the CSV writer with the current path
    let mut wtr = Writer::from_path(path)?;

    // Define the data ranges and options
    let office_locations = ["New York", "Los Angeles", "Chicago", "Houston", "Phoenix"];
    let product_classes = ["Commercial", "Financial", "Consumer", "Support"];
    let customer_classes = ["10-19", "20-29", "30-39", "40-49", "50-59", "60-69", "70-79", "80+"];
    let year_range = Uniform::new(2000, 2023);
    let month_range = Uniform::new(1, 13);
    let transaction_count_range = Uniform::new(1, 101);
    let transaction_amount_range = 100.0..10001.0;


    let mut rng = rand::thread_rng();

    // Generate 10,000 records
    for _ in 0..10_000 {
        let year = rng.sample(year_range);
        let month = rng.sample(month_range);
        let office_location_str = office_locations.choose(&mut rng).unwrap().to_string();
        let product_class_str = product_classes.choose(&mut rng).unwrap().to_string();
        let customer_class_str = customer_classes.choose(&mut rng).unwrap().to_string();
        let transaction_count = rng.sample(transaction_count_range);
        let transaction_amount: f64 = rng.gen_range(transaction_amount_range.clone());

        // Format the year-month string
        let year_month = format!("{}-{:02}", year, month);

        // Write the record
        wtr.write_record(&[
            &year_month,
            &office_location_str,
            &product_class_str,
            &customer_class_str,
            &transaction_count.to_string(),
            &format!("{:.2}", transaction_amount),
        ])?;
    }

    wtr.flush()?;
    println!("Dataset generated: data/mock_dataset.csv");
    Ok(())
}

pub fn generate_parquet() -> Result<(), Box<dyn Error>> {
    let office_locations = ["New York", "Los Angeles", "Chicago", "Houston", "Phoenix"];
    let product_classes = ["Commercial", "Financial", "Consumer", "Support"];
    let customer_classes = ["10-19", "20-29", "30-39", "40-49", "50-59", "60-69", "70-79", "80+"];
    let year_range = Uniform::new(2000, 2023);
    let month_range = Uniform::new(1, 13);
    let transaction_count_range = Uniform::new(1, 101);
    let transaction_amount_range = 100.0..10001.0;

    let mut rng = rand::thread_rng();

    let mut year_month_data = Vec::new();
    let mut office_location_data = Vec::new();
    let mut product_class_data = Vec::new();
    let mut customer_class_data = Vec::new();
    let mut transaction_count_data = Vec::new();
    let mut transaction_amount_data = Vec::new();

    for _ in 0..10_000 {
        let year = rng.sample(year_range);
        let month = rng.sample(month_range);
        let year_month = format!("{}-{:02}", year, month);
        let office_location = office_locations.choose(&mut rng).unwrap();
        let product_class = product_classes.choose(&mut rng).unwrap();
        let customer_class = customer_classes.choose(&mut rng).unwrap();
        let transaction_count = rng.sample(transaction_count_range);
        let transaction_amount: f64 = rng.gen_range(transaction_amount_range.clone());

        year_month_data.push(year_month);
        office_location_data.push(office_location.to_string());
        product_class_data.push(product_class.to_string());
        customer_class_data.push(customer_class.to_string());
        transaction_count_data.push(transaction_count);
        transaction_amount_data.push(transaction_amount);
    }

    let schema = Arc::new(Schema::new(vec![
        Field::new("year_month", DataType::Utf8, false),
        Field::new("office_location", DataType::Utf8, false),
        Field::new("product_class", DataType::Utf8, false),
        Field::new("customer_class", DataType::Utf8, false),
        Field::new("transaction_count", DataType::Int32, false),
        Field::new("transaction_amount", DataType::Float64, false),
    ]));

    let year_month_array = StringArray::from(year_month_data);
    let office_location_array = StringArray::from(office_location_data);
    let product_class_array = StringArray::from(product_class_data);
    let customer_class_array = StringArray::from(customer_class_data);
    let transaction_count_array = Int32Array::from(transaction_count_data);
    let transaction_amount_array = Float64Array::from(transaction_amount_data);

    let record_batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(year_month_array),
            Arc::new(office_location_array),
            Arc::new(product_class_array),
            Arc::new(customer_class_array),
            Arc::new(transaction_count_array),
            Arc::new(transaction_amount_array),
        ],
    )?;

    // Get the current directory
    let mut path = env::current_dir()?;

    // Move up one level and enter the 'data' directory
    path.pop();
    path.push("data");

    // Create the 'data' directory if it doesn't exist
    fs::create_dir_all(&path)?;

    // Set the file path for the Parquet file
    path.push("mock_dataset.parquet");

    // Create the Parquet file at the specified path
    let file = File::create(path)?;
    let props = WriterProperties::builder().build();
    let mut writer = ArrowWriter::try_new(file, schema, Some(props))?;

    writer.write(&record_batch)?;
    writer.close()?;

    Ok(())
}
