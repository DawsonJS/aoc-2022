pub fn num_calories(list: &str) -> Vec<i32> {
    let mut current_calories: i32 = 0;
    let mut elves: Vec<i32> = Vec::new();
    for l in list.lines() {
        if l.is_empty() {
            elves.push(current_calories);
            current_calories = 0;
        } 
        else {
            current_calories += l.parse::<i32>().unwrap();
        }
    }
    elves
}

pub fn greatest_num_calories(cal_list: &[i32]) -> usize {
    let mut greatest: usize = 0;
    for (index, item) in cal_list.iter().enumerate() {
        if item > &cal_list[greatest] {
            greatest = index;
        }
    }
    greatest
}

pub fn top_three_total(cal_list: &[i32]) -> i32 {
    let mut elves: Vec<i32> = cal_list.to_owned();
    elves.sort_by(|a, b| b.cmp(a));
    let mut sum: i32 = 0;
    for (index, item) in elves.iter().enumerate() {
        if index >= 3 {
            break;
        }
        sum += item;
    }
    sum
}

#[cfg(test)]
mod day_one_tests {
    use crate::day_one::*;
    #[test]
    fn num_calories_test() {
        let test_list = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n\n".to_string();
        let cal_list: Vec<i32> = num_calories(&test_list);
        assert_eq!(cal_list, vec![6000, 4000, 11000, 24000, 10000]);
    }

    #[test]
    fn greatest_cals_test() {
        let test_list = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n\n".to_string();
        let cal_list: Vec<i32> = num_calories(&test_list);
        let greatest = greatest_num_calories(&cal_list);
        assert_eq!(greatest, 3 as usize);
    }

    #[test]
    fn top_three_total_test() {
        let test_list = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n\n".to_string();
        let cal_list: Vec<i32> = num_calories(&test_list);
        let sum = top_three_total(&cal_list);
        assert_eq!(sum, 45000);
    }
}

