from pyspark.sql import SparkSession
from pyspark.sql.functions import rand, randn
import os

def generate_parquet():
    spark = SparkSession.builder.appName("DataGeneration").getOrCreate()

    # Number of rows for the DataFrame
    num_rows = 10000

    # Generate the DataFrame
    df = spark.range(num_rows).selectExpr(
        "cast(year(current_date()) - rand() * 23 as string) as year",
        "cast(month(current_date()) - rand() * 12 as int) as month",
        "case when id % 5 = 0 then 'New York' when id % 5 = 1 then 'Los Angeles' when id % 5 = 2 then 'Chicago' when id % 5 = 3 then 'Houston' else 'Phoenix' end as office_location",
        "case when id % 4 = 0 then 'Commercial' when id % 4 = 1 then 'Financial' when id % 4 = 2 then 'Consumer' else 'Support' end as product_class",
        "case when id % 8 = 0 then '10-19' when id % 8 = 1 then '20-29' when id % 8 = 2 then '30-39' when id % 8 = 3 then '40-49' when id % 8 = 4 then '50-59' when id % 8 = 5 then '60-69' when id % 8 = 6 then '70-79' else '80+' end as customer_class",
        "cast(rand() * 100 as int) as transaction_count",
        "round(rand() * 9901 + 100, 2) as transaction_amount"
    )
    
    # Combine year month into one column
    df = df.withColumn("year_month", df["year"] + "-" + df["month"])

    # Drop the extra columns
    df = df.drop("year", "month")

    # Create the 'data' directory if it doesn't exist
    data_dir = os.path.join(os.path.dirname(__file__), 'data')
    os.makedirs(data_dir, exist_ok=True)

    # Set the file path for the Parquet file
    file_path = os.path.join(data_dir, "mock_dataset.parquet")

    # Write the DataFrame to a Parquet file
    df.write.parquet(file_path)

    spark.stop()
    return True

if __name__ == "__main__":
    generate_parquet()
