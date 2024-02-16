use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Test task");
    
    let mut the_highest_value: i128 = 0;
    let mut the_highest_sum: i128 = 0;

    loop {
        println!("Enter an integer:");

        let mut the_entered_number: String = String::new();
        io::stdin()
            .read_line(&mut the_entered_number)
            .expect("Failed to read line!");

        let the_entered_number_num: i128 = match the_entered_number
            .trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Failed to read line! Enter only whole numbers!");
                    continue;
                }    
            };
        if the_entered_number_num == 0 {
            println!(
                "The highest value {}, sum {}", 
                the_highest_value, the_highest_sum
            );
            break;
        } else { 
            let the_entered_number_iter = the_entered_number
                .chars()
                .filter_map(|c| {c.to_digit(10).map(|d| d as i128)});

            let the_entered_number_sum: i128 = the_entered_number_iter.sum();
            match the_entered_number_sum.cmp(&the_highest_sum) {
                Ordering::Equal => continue,
                Ordering::Less => continue,
                Ordering::Greater => {
                    the_highest_sum = the_entered_number_sum;
                    the_highest_value = the_entered_number_num;
                }
            }
        }
    }
}
