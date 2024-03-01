import os
from pyspark.sql import SparkSession
from pyspark.sql.functions import monotonically_increasing_id, col, udf
from pyspark.sql.types import StringType, FloatType, BooleanType, StructType, StructField, TimestampType, DoubleType
from datetime import datetime, timedelta
import numpy as np

# Initialize Spark session
spark = SparkSession.builder.appName("UniqueCoffeeShopDataSimulation").getOrCreate()

def save_spark_dataframe_to_csv(spark_df, output_path, header=True, delimiter=","):
    """
    Save a Spark DataFrame to a CSV file.
    """
    spark_df.write.mode("overwrite").option("header", header).option("delimiter", delimiter).csv(output_path)

def generate_data(existing_df, num_records=100, start_datetime=datetime.now()):
    """
    Generate simulated data for a coffee shop.
    """
    # Generate timestamps
    time_increments = np.random.normal(loc=5, scale=1, size=num_records).clip(min=1)
    timestamps = [start_datetime + timedelta(minutes=np.sum(time_increments[:i])) for i in range(1, num_records + 1)]

    # Order processing times (normally distributed around 5 minutes with a standard deviation of 2)
    order_processing_times = np.random.normal(loc=5, scale=2, size=num_records).clip(min=0)  # Ensure no negative times
    order_processing_times = order_processing_times.tolist()
    
    # Generate customer satisfaction levels
    customer_satisfaction_choices = ['Very Satisfied', 'Satisfied', 'Neutral', 'Unsatisfied', 'Very Unsatisfied']
    customer_satisfaction = np.random.choice(customer_satisfaction_choices, size=num_records).tolist()
    
    # Generate sales amounts and round them
    sales_amount = np.round(np.random.uniform(3, 20, size=num_records), 2)
    sales_amount = sales_amount.tolist()  # Convert to Python float list
    
    # Generate correctness of orders
    order_correct = np.random.choice([True, False], p=[0.9, 0.1], size=num_records)
    order_correct = list(map(bool, order_correct))
    
    # Prepare data for DataFrame creation
    data = list(zip(timestamps, customer_satisfaction, sales_amount, order_correct, order_processing_times))
    
    # Define schema
    schema = StructType([
        StructField("DateTime", TimestampType(), True),
        StructField("Customer Satisfaction", StringType(), True),
        StructField("Sales Amount ($)", FloatType(), True),
        StructField("Order Correct", BooleanType(), True),
        StructField("Order Processing Time (mins)", DoubleType(), True)
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
    # Generate data and save to CSV
    spark_df = generate_data(existing_df=None, num_records=100, start_datetime=datetime.now())
    output_path = "./data/test.csv"  # Ensure the './data' directory exists or change to a valid path
    save_spark_dataframe_to_csv(spark_df, output_path)
