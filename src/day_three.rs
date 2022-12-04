use std::collections::HashMap;

pub fn assign_letters_numbers() -> HashMap<char, u32> {
    let mut c: u32 = 26;
    let letters: HashMap<char, u32> = ('A'..='z')
        .map(|letter| {
            c += 1;
            if letter == 'a' {
                c = 1;
            }
            (letter, c)
        })
        .collect();

    letters
}

pub fn parse_rucksack(first: &str, second: &str) -> char {
    let mut result: char = '\0';
    for ch in first.chars() {
        if second.contains(ch) {
            result = ch;
        }
    }
    result
}

pub fn parse_group(first: &str, second: &str, third: &str) -> char {
    let mut result: char = '\0';
    for ch in first.chars() {
        if second.contains(ch) && third.contains(ch) {
            result = ch;
        }
    }
    result
}

pub fn parse_list_rucksacks(list: &str, map: &HashMap<char, u32>) -> u32 {
    let mut midpoint: usize;
    let mut letters: Vec<char> = Vec::new();
    for l in list.lines() {
        midpoint = l.len() / 2;
        letters.push(parse_rucksack(&l[..=(midpoint - 1)], &l[(midpoint)..]));
    }

    let mut count: u32 = 0;

    for letter in letters {
        count += map.get(&letter).unwrap();
    }

    count
}

pub fn parse_groups_rucksacks(list: &str, map: &HashMap<char, u32>) -> u32 {
    let mut letters: Vec<char> = Vec::new();

    let mut count = 0;

    let groups: Vec<&str> = list
        .split(|c| {
            if c == '\n' {
                count += 1;
                count % 3 == 0
            } else {
                false
            }
        })
        .collect();

    for group in groups {
        let rucksacks: Vec<&str> = group.split('\n').collect();
        if rucksacks.len() >= 3 {
            letters.push(parse_group(rucksacks[0], rucksacks[1], rucksacks[2]));
        }
    }

    let mut count: u32 = 0;

    for letter in letters {
        count += map.get(&letter).unwrap();
    }

    count
}

#[cfg(test)]
mod day_three_tests {
    use crate::day_three::*;

    #[test]
    fn get_priority_test() {
        let map = assign_letters_numbers();
        let mut count = 0;
        for letter in 'a'..='z' {
            count += 1;
            assert_eq!(map.get(&letter), Some(&count));
        }

        for letter in 'A'..='Z' {
            count += 1;
            assert_eq!(map.get(&letter), Some(&count));
        }

        assert_eq!(map.get(&'d'), Some(&4));
        assert_eq!(map.get(&'D'), Some(&30));
        assert_eq!(map.get(&'z'), Some(&26));
        assert_eq!(map.get(&'A'), Some(&27));
    }

    #[test]
    fn parse_rucksack_test() {
        let example = "vJrwpWtwJgWrhcsFMMfFFhFp".to_string();

        let midpoint = example.len() / 2;

        assert_eq!(
            parse_rucksack(&example[..=(midpoint - 1)], &example[(midpoint)..]),
            'p'
        )
    }

    #[test]
    fn parse_list_test() {
        let example = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw\n".to_string();

        let map = assign_letters_numbers();

        assert_eq!(parse_list_rucksacks(&example, &map), 157);
        assert_eq!(parse_groups_rucksacks(&example, &map), 70);
    }
}
