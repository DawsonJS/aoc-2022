pub mod day_one;

use std::env;
use std::fs;
use day_one::*;


fn main() {
    let args: Vec<String> = env::args().collect();
    let day_arg = &args[1].parse::<i32>();
    let day = match day_arg {
        Ok(day) => day,
        Err(_) => {
            println!("Valid integer should be passed in for day: {} <day_num> <puzzle_input>", &args[0]);
            return
        }
    };
    match day {
        1 => {

            let file_path = &args[2];

            let contents = fs::read_to_string(file_path)
                .expect("Should have been able to read the file");

            let calories = num_calories(&contents);
            let greatest_cal = greatest_num_calories(&calories);
            let top_three_elves = top_three_total(&calories);
            
            println!("Elf number {} has the greatest number of calories: {} calories", greatest_cal as i32 + 1, calories[greatest_cal]);
            println!("The top three calorie carrying elves are holding a total of {} calories", top_three_elves);
        },
        _ => {
            println!("This day has not been completed yet");
        }
    }
}

