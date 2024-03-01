import unittest 
import pandas as pd
import os

from app.i_chart import generate_control_chart  # Replace with your actual module name

class TestControlChart(unittest.TestCase):

    def setUp(self):
        # Create a simple test DataFrame
        self.data = pd.DataFrame({'test_column': [1, 2, 3, 4, 5]})
        self.test_column = 'test_column'
        self.test_file_name = 'test_control_chart'
        self.test_dir = 'test_plots'

    def test_control_chart_file_creation(self):
        # Generate the control chart and save it
        generate_control_chart(self.data, self.test_column, self.test_file_name, self.test_dir)

        # Check if the file is created
        file_path = os.path.join(self.test_dir, f"{self.test_file_name}.svg")
        self.assertTrue(os.path.isfile(file_path))

        # Clean up: Remove the created file and directory
        os.remove(file_path)
        os.rmdir(self.test_dir)

    # def test_control_chart_default_directory(self):
    #     # Generate the control chart with default directory
    #     generate_control_chart(self.data, self.test_column, self.test_file_name)
    #
    #     # Default directory path
    #     script_dir = os.path.dirname(os.path.abspath(__file__))
    #     default_dir = os.path.join(script_dir, 'saved_plots')
    #     file_path = os.path.join(default_dir, f"{self.test_file_name}.svg")
    #
    #     # Check if the file is created in the default directory
    #     self.assertTrue(os.path.isfile(file_path))
    #
    #     # Clean up: Remove the created file and directory
    #     os.remove(file_path)
    #     os.rmdir(default_dir)


    def test_control_chart_calculations(self):
        # Generate the control chart and save it
        mean, ucl, lcl = generate_control_chart(self.data, self.test_column, self.test_file_name, self.test_dir)

        # Check if the mean is calculated correctly
        self.assertEqual(mean, 3)

        # Assuming standard deviation calculation is correct, check UCL and LCL
        std_dev = self.data['test_column'].std()
        self.assertEqual(ucl, mean + 3 * std_dev)
        self.assertEqual(lcl, mean - 3 * std_dev)

        # Clean up: Remove the created file and directory
        file_path = os.path.join(self.test_dir, f"{self.test_file_name}.svg")
        os.remove(file_path)
        os.rmdir(self.test_dir)

    def test_invalid_column(self):
        # Test with a non-existent column
        with self.assertRaises(Exception):  # Catching the general exception; specify if your function raises a specific one
            generate_control_chart(self.data, 'non_existent_column', self.test_file_name, self.test_dir)

        # Clean up: Remove the created file and directory
        file_path = os.path.join(self.test_dir, f"{self.test_file_name}.svg")
        if os.path.exists(file_path):
            os.remove(file_path)
        if os.path.exists(self.test_dir):
            os.rmdir(self.test_dir)

    # Additional tests can be written for other edge cases

if __name__ == '__main__':
    unittest.main()
