import sys

def find_highest_sum_func(numbers):
    the_highest_value = 0
    the_highest_sum = 0
    for the_entered_number_num in numbers:
        if the_entered_number_num == 0:
            break
        else:
            num_str = str(the_entered_number_num)
            the_entered_number_iter = (int(c) for c in num_str if c.isdigit())
            the_entered_number_sum = sum(the_entered_number_iter)
            if the_entered_number_sum > the_highest_sum:
                the_highest_sum = the_entered_number_sum
                the_highest_value = the_entered_number_num
    return the_highest_value, the_highest_sum

def input_data():
    the_entered_number = input()
    return the_entered_number

def check_input_data(the_entered_number):
    try:
        the_entered_number_num = int(the_entered_number.strip())
        return the_entered_number_num
    except ValueError:
        raise Exception("Failed to parse input as i128")

