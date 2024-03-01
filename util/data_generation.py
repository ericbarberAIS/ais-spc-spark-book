import os
from pyspark.sql import SparkSession
from pyspark.sql.functions import monotonically_increasing_id, col, udf
from pyspark.sql.types import StringType, FloatType, BooleanType, StructType, StructField, TimestampType
from datetime import datetime, timedelta
import numpy as np
spark = SparkSession.builder.appName("UniqueCoffeeShopDataSimulation").getOrCreate()

def save_spark_dataframe_to_csv(spark_df, output_path, header=True, delimiter=","):
    """
    Save a Spark DataFrame to a CSV file.

    Parameters:
    - spark_df: The Spark DataFrame to save.
    - output_path: The output path where the CSV file(s) will be saved. This can be a local path or a path in HDFS or S3, depending on your Spark setup.
    - header: Whether to save the first row as a header (default: True).
    - delimiter: The column delimiter for the CSV output (default: ',').
    """
    spark_df.write.option("header", header).option("delimiter", delimiter).csv(output_path)


def generate_data(existing_df, num_records=100, start_datetime=datetime.now()):
    # Generate random increments around 5 minutes for each new customer
    time_increments = np.random.normal(loc=5, scale=1, size=num_records).clip(min=1)
    
    timestamps = [start_datetime + timedelta(minutes=np.sum(time_increments[:i])) for i in range(num_records)]
    
    # Generate customer satisfaction, sales amount, and order correctness for each record
    customer_satisfaction_choices = ['Very Satisfied', 'Satisfied', 'Neutral', 'Unsatisfied', 'Very Unsatisfied']
    customer_satisfaction = np.random.choice(customer_satisfaction_choices, size=num_records)
    
    sales_amount = np.round(np.random.uniform(3, 20, size=num_records))

    # Convert numpy.float64 to Python native float for compatibility
    sales_amount = sales_amount.astype(float).tolist()
    
    order_correct = np.random.choice([True, False], p=[0.9, 0.1], size=num_records)

    # Convert numpy.bool_ to Python native bool for compatibility
    order_correct = list(map(bool, order_correct))
    
    # Prepare data for DataFrame creation
    data = list(zip(timestamps, customer_satisfaction, sales_amount, order_correct))
    
    # Define schema
    schema = StructType([
        StructField("DateTime", TimestampType(), True),
        StructField("Customer Satisfaction", StringType(), True),
        StructField("Sales Amount ($)", FloatType(), True),
        StructField("Order Correct", BooleanType(), True)
    ])
    
    # Create a DataFrame from the generated data
    new_data = spark.createDataFrame(data, schema=schema)
    
    # If there's existing data, append the new data
    if existing_df is not None and not existing_df.rdd.isEmpty():
        updated_df = existing_df.union(new_data)
    else:
        updated_df = new_data
    
    return updated_df

if __name__ == "__main__":
    # Example usage: generate_parquet('/path/to/target/dir')
    # If no argument is provided, it defaults to a sibling 'data' directory
    spark_df = generate_data(existing_df=None, num_records=100, start_datetime=datetime.now())
    output_path = "./data/test.csv"
    save_spark_dataframe_to_csv(spark_df, output_path, header=True, delimiter=",")
