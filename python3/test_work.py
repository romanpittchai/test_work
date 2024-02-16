def main():
    print("Test task")

    the_highest_value = 0
    the_highest_sum = 0

    while True:
        try:
            the_entered_number = input("Enter an integer: ")
            the_entered_number_num = int(the_entered_number)

        except ValueError:
            print("Failed to read line! Enter only whole numbers!")
            continue

        if the_entered_number_num == 0:
            print(f"The highest value {the_highest_value}, sum {the_highest_sum}")
            break
        else:
            the_entered_number_iter = map(int, filter(
                str.isdigit, the_entered_number
                ))
            the_entered_number_sum = sum(the_entered_number_iter)

            if the_entered_number_sum > the_highest_sum:
                the_highest_sum = the_entered_number_sum
                the_highest_value = the_entered_number_num

if __name__ == "__main__":
    main()
