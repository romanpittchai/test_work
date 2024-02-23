import unittest
from unittest.mock import patch
import sys
from lib import * 

# Tests
class TestFindHighestSum(unittest.TestCase):

    def test_find_highest_sum_empty_array(self):
        numbers = []
        self.assertEqual(find_highest_sum_func(numbers), (0, 0))

    def test_find_highest_sum_all_positive_numbers(self):
        numbers = [4, 120, 789, 45435435, 35485345445843, 1000000000000001]
        self.assertEqual(find_highest_sum_func(numbers), (35485345445843, 65))

    def test_find_highest_sum_all_zeros(self):
        numbers = [0, 0, 0, 0]
        self.assertEqual(find_highest_sum_func(numbers), (0, 0))

    def test_find_highest_sum_mixed_positive_and_negative_numbers(self):
        numbers = [-5, 10, -3, 8, 2]
        self.assertEqual(find_highest_sum_func(numbers), (8, 8))

    def test_find_highest_sum_input_with_leading_zeros(self):
        numbers = [1, 3, 5]
        self.assertEqual(find_highest_sum_func(numbers), (5, 5))

def check_input_data(item):
    try:
        return int(item)
    except ValueError as e:
        raise ValueError("Invalid input data") from e

class CheckInputDataTest(unittest.TestCase):
    def test_check_input_data(self):
        test_contents = ["not_a_number", "33.0", "0.33", "a33d.", "3.30"]
        for item in test_contents:
            with self.assertRaises(ValueError):
                check_input_data(item)

if __name__ == '__main__':
    unittest.main()

