from pyspark.sql import SparkSession
import os
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

def read_parquet_data(file_path):
    spark = SparkSession.builder.appName("DataAnalysis").getOrCreate()
    df = spark.read.parquet(file_path)
    return df

def generate_control_chart(data, column, file_name, save_dir=None):
    # Set the default save directory to a sibling of the script's directory
    if save_dir is None:
        script_dir = os.path.dirname(__file__)  # Directory of the script
        save_dir = os.path.join(script_dir, 'saved_plots')  # Default save directory

    # Ensure the save directory exists
    os.makedirs(save_dir, exist_ok=True)

    # Calculate the mean and standard deviation
    mean = data[column].mean()
    std_dev = data[column].std()

    # Calculate control limits
    ucl = mean + 3*std_dev
    lcl = mean - 3*std_dev

    # Plotting the control chart
    plt.figure(figsize=(15, 6))
    plt.plot(data[column], marker='o', color='b', label='Data')
    plt.axhline(mean, color='green', linestyle='--', label='Mean')
    plt.axhline(ucl, color='red', linestyle='--', label='UCL')
    plt.axhline(lcl, color='red', linestyle='--', label='LCL')
    plt.title(f'Control Chart for {column}')
    plt.xlabel('Sample')
    plt.ylabel(column)
    plt.legend()

    # Save the plot to a file in the specified directory
    file_path = os.path.join(save_dir, f"{file_name}.svg")
    plt.savefig(file_path)
    plt.close()

    # Return values for testing
    return mean, ucl, lcl


if __name__ == "__main__":
    # Replace with your file path
    file_path = '/path/to/mock_dataset.parquet'

    # Read the data
    spark_df = read_parquet_data(file_path)

    # Convert to Pandas DataFrame for easier processing
    pandas_df = spark_df.toPandas()

    # Generate a control chart for a specific column
    # Replace 'transaction_count' with the column you want to analyze
    generate_control_chart(pandas_df, 'transaction_count')
