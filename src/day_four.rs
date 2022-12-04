pub fn list_pairs_cmp(list: &str) -> i32 {
    let mut count: i32 = 0;
    for l in list.lines() {
        if pair_compare(l) {
            count += 1;
        }
    }

    count
}

pub fn list_pairs_cmp_any_overlap(list: &str) -> i32 {
    let mut count: i32 = 0;
    for l in list.lines() {
        if pair_cmp_any_overlap(l) {
            count += 1;
        }
    }

    count
}

pub fn pair_compare(line: &str) -> bool {
    let v: Vec<i32> = line
        .split(&['-', ','])
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    (v[0] >= v[2] && v[1] <= v[3]) || (v[2] >= v[0] && v[3] <= v[1])
}

pub fn pair_cmp_any_overlap(line: &str) -> bool {
    let v: Vec<i32> = line
        .split(&['-', ','])
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    !((v[1] < v[2]) || (v[0] > v[3]))
}

#[cfg(test)]
mod day_four_tests {
    use crate::day_four::*;

    #[test]
    fn parse_line_test() {
        let binding = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8".to_string();
        let example: Vec<&str> = binding.split('\n').collect();

        assert_eq!(pair_compare(example[0]), false);
        assert_eq!(pair_compare(example[3]), true);
        assert_eq!(pair_compare(example[2]), false);

        assert_eq!(pair_cmp_any_overlap(example[0]), false);
        assert_eq!(pair_cmp_any_overlap(example[2]), true);
    }

    #[test]
    fn parse_list_test() {
        let example = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8".to_string();

        assert_eq!(list_pairs_cmp(&example), 2);
        assert_eq!(list_pairs_cmp_any_overlap(&example), 4);
    }
}
