use test_work::find_highest_sum::{check_input_data, find_highest_sum_func, input_data};

fn main() {
    println!("Test task");

    let mut numbers: Vec<i128> = Vec::new();

    loop {
        println!("Enter an integer:");

        let the_entered_number_num: i128 = match check_input_data(&input_data()) {
            Ok(num) => num,
            Err(err) => {
                eprintln!("Error: {}", err);
                continue;
            }
        };
        numbers.push(the_entered_number_num);

        if the_entered_number_num == 0 {
            let (highest_value, highest_sum) = find_highest_sum_func(&numbers);
            println!("The highest value {}, sum {}", highest_value, highest_sum);
            break;
        }
    }
}
