//! # Doc
pub mod find_highest_sum {
    //! # Doc
    use std::cmp::Ordering;
    use std::error::Error;
    use std::io;

    pub fn find_highest_sum_func(numbers: &[i128]) -> (i128, i128) {
        // Doc
        let mut the_highest_value: i128 = 0;
        let mut the_highest_sum: i128 = 0;

        for &the_entered_number_num in numbers.iter() {
            if the_entered_number_num == 0 {
                break;
            } else {
                let num_str = the_entered_number_num.to_string();
                let the_entered_number_iter = num_str
                    .chars()
                    .filter_map(|c| c.to_digit(10).map(|d| d as i128));

                let the_entered_number_sum: i128 = the_entered_number_iter.sum();
                match the_entered_number_sum.cmp(&the_highest_sum) {
                    Ordering::Equal | Ordering::Less => continue,
                    Ordering::Greater => {
                        the_highest_sum = the_entered_number_sum;
                        the_highest_value = the_entered_number_num;
                    }
                }
            }
        }
        (the_highest_value, the_highest_sum)
    }

    pub fn input_data() -> String {
        let mut the_entered_number: String = String::new();
        io::stdin()
            .read_line(&mut the_entered_number)
            .expect("Failed to read line!");
        the_entered_number
    }

    pub fn check_input_data(the_entered_number: &str) -> Result<i128, Box<dyn Error>> {
        let the_entered_number_num: Result<i128, Box<dyn Error>> =
            match the_entered_number.to_string().trim().parse::<i128>() {
                Ok(num) => Ok(num),
                Err(_) => Err("Failed to parse input as i128".into()),
            };
        the_entered_number_num
    }
}

#[cfg(test)]
mod tests {
    //The test module
    use crate::find_highest_sum::check_input_data;
    use crate::find_highest_sum::find_highest_sum_func;
    use std::error::Error;

    #[test]
    fn test_find_highest_sum() {
        // Test case 1: Empty array
        let numbers: Vec<i128> = Vec::new();
        assert_eq!(find_highest_sum_func(&numbers), (0, 0));

        // Test case 2: All positive numbers
        let numbers: Vec<i128> = vec![4, 120, 789, 45435435, 35485345445843, 1000000000000001];
        assert_eq!(find_highest_sum_func(&numbers), (35485345445843, 65));

        // Test case 3: All zeros
        let numbers: Vec<i128> = vec![0, 0, 0, 0];
        assert_eq!(find_highest_sum_func(&numbers), (0, 0));

        // Test case 4: Mixed positive and negative numbers
        let numbers: Vec<i128> = vec![-5, 10, -3, 8, 2];
        assert_eq!(find_highest_sum_func(&numbers), (8, 8));

        // Test case 5: Input with leading zeros
        let numbers: Vec<i128> = vec![001, 0003, 0005];
        assert_eq!(find_highest_sum_func(&numbers), (5, 5));
    }

    #[test]
    fn check_input_data_test() {
        let test_contents: [&str; 5] = ["not_a_number", "33.0", "0.33", "a33d.", "3.30"];

        for item in &test_contents {
            let expected_result: Result<i128, Box<dyn Error>> = Err("".into());
            assert_eq!(check_input_data(&item).is_err(), expected_result.is_err());
        }
    }
}
