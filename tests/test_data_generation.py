import unittest
import os
import shutil
from pyspark.sql import SparkSession
from util.data_generation import generate_parquet  # Adjust the import as necessary

class TestGenerateParquet(unittest.TestCase):

    @classmethod
    def setUpClass(cls):
        cls.default_dir = os.path.join(os.path.dirname(__file__), '..', 'util', 'data')
        cls.custom_dir = os.path.join(os.path.dirname(__file__), 'test_data')

    def test_default_directory(self):
        # Test with default directory and default count
        generate_parquet()
        self.assertTrue(os.path.exists(os.path.join(self.default_dir, "mock_dataset.parquet")))

    def test_custom_directory(self):
        # Test with a custom directory
        generate_parquet(target_dir=self.custom_dir)
        self.assertTrue(os.path.exists(os.path.join(self.custom_dir, "mock_dataset.parquet")))

    def test_row_count(self):
        # Test with a specific row count
        row_count = 500
        generate_parquet(count=row_count, target_dir=self.custom_dir)
        spark = SparkSession.builder.appName("TestSession").getOrCreate()
        df = spark.read.parquet(os.path.join(self.custom_dir, "mock_dataset.parquet"))
        self.assertEqual(df.count(), row_count)
        spark.stop()

    @classmethod
    def tearDownClass(cls):
        # Clean up (remove the created test directories)
        if os.path.exists(cls.custom_dir):
            shutil.rmtree(cls.custom_dir)
        if os.path.exists(cls.default_dir):
            shutil.rmtree(cls.default_dir)

if __name__ == '__main__':
    unittest.main()
