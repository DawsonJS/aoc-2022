pub mod day_one;
pub mod day_two;
pub mod day_three;

use std::env;
use std::fs;
use day_one::*;
use day_two::*;
use day_three::*;


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
        2 => {
            let file_path = &args[2];

            let contents = fs::read_to_string(file_path)
                .expect("Should have been able to read the file");
            let mut puzzle_one_scores: Vec<i32> = Vec::new();

            for l in contents.lines() {
                puzzle_one_scores.push(calc_points(l, PuzzlePart::One));
            }

            let mut total: i32 = puzzle_one_scores.iter().sum();
            println!("Total score following assumed code: {}", total);

            let mut puzzle_two_scores: Vec<i32> = Vec::new();

            for l in contents.lines() {
                puzzle_two_scores.push(calc_points(l, PuzzlePart::Two));
            }

            total = puzzle_two_scores.iter().sum();

            println!("Total score following correct code: {}", total);

        },
        3 => {
            let file_path = &args[2];

            let contents = fs::read_to_string(file_path)
                .expect("Should have been able to read the file");

            let map = assign_letters_numbers();

            let mut priority_sum = parse_list_rucksacks(&contents, &map);

            println!("The sum of the priorities of the mismatched item types are: {}", priority_sum);

            priority_sum = parse_groups_rucksacks(&contents, &map);

            println!("The sum of the priorities of the badges' item types are: {}", priority_sum);
        },
        _ => {
            println!("This day has not been completed yet");
        },
    }
}

