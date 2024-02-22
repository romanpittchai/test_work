import sys
from lib import *  

def main():
    print("Test task")
    numbers = []
    while True:
        print("Enter an integer:")
        try:
            the_entered_number_num = check_input_data(input_data())
        except Exception as err:
            print(f"Error: {err}", file=sys.stderr)
            continue
        numbers.append(the_entered_number_num)
        if the_entered_number_num == 0:
            highest_value, highest_sum = find_highest_sum_func(numbers)
            print(f"The highest value {highest_value}, sum {highest_sum}")
            break

if __name__ == "__main__":
    main()
