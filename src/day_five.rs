pub fn initialize_stacks(list: &str) -> Vec<Vec<&str>> {
    let drawing: Vec<&str> = list
        .lines()
        .filter(|x| !x.contains("move") && !x.is_empty() && !x.contains('1'))
        .map(|x| {
            x.strip_prefix(' ')
                .expect("bad things have happened")
                .strip_suffix(' ')
                .expect("bad things have happened")
        })
        .collect();
    let num_crates: i32 = drawing[drawing.len() - 1_usize]
        .chars()
        .filter(|x| !x.is_whitespace())
        .count() as i32;

    let mut crates: Vec<Vec<&str>> = Vec::new();

    for i in 0..num_crates {
        crates.push(Vec::new());
        for j in drawing.iter().rev() {
            let binding = j.get((i as usize * 4_usize)..(i as usize * 4_usize + 1_usize));
            match binding {
                Some(value) => {
                    if value != " " {
                        crates[i as usize].push(value);
                    }
                }
                None => {
                    println!("i: {} \n j: {}", i, j);
                }
            }
        }
    }

    crates
}

pub fn move_crates(instruction: &str, crates: &mut [Vec<&str>]) {
    let nums: Vec<i32> = instruction
        .split(' ')
        .filter(|x| !x.contains('o'))
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    for _i in 0_usize..(nums[0] as usize) {
        let binding: Option<&str> = crates[nums[1] as usize - 1_usize].pop();
        if let Some(value) = binding {
            crates[nums[2] as usize - 1_usize].push(value)
        };
    }
}

pub fn move_mult_crates(instruction: &str, crates: &mut [Vec<&str>]) {
    let nums: Vec<i32> = instruction
        .split(' ')
        .filter(|x| !x.contains('o'))
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut binding_stack: Vec<&str> = Vec::new();
    for _i in 0_usize..(nums[0] as usize) {
        let binding: Option<&str> = crates[nums[1] as usize - 1_usize].pop();
        if let Some(value) = binding {
            binding_stack.push(value)
        };
    }

    for v in binding_stack.iter().rev() {
        crates[nums[2] as usize - 1_usize].push(v);
    }
}

#[cfg(test)]
mod day_five_tests {
    use crate::day_five::*;
    #[test]
    fn parse_crates_test() {
        let mut example = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2".to_string();

        example = example.replace(&['[', ']'][..], " ");
        let directions: Vec<&str> = example.lines().filter(|x| x.contains("move")).collect();

        assert_eq!(
            directions,
            vec![
                "move 1 from 2 to 1",
                "move 3 from 1 to 3",
                "move 2 from 2 to 1",
                "move 1 from 1 to 2"
            ]
        );

        let crates: Vec<Vec<&str>> = initialize_stacks(&example);

        assert_eq!(crates, vec![vec!["Z", "N"], vec!["M", "C", "D"], vec!["P"]]);
    }

    #[test]
    fn move_crates_test() {
        let mut example = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2".to_string();

        example = example.replace(&['[', ']'][..], " ");

        let directions: Vec<&str> = example.lines().filter(|x| x.contains("move")).collect();

        let mut crates: Vec<Vec<&str>> = initialize_stacks(&example);

        move_crates(directions[0], &mut crates);

        assert_eq!(crates, vec![vec!["Z", "N", "D"], vec!["M", "C"], vec!["P"]]);
    }

    #[test]
    fn move_many_crates_test() {
        let mut example = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2".to_string();

        example = example.replace(&['[', ']'][..], " ");

        let directions: Vec<&str> = example.lines().filter(|x| x.contains("move")).collect();

        let mut crates: Vec<Vec<&str>> = initialize_stacks(&example);

        for l in directions.iter() {
            move_crates(l, &mut crates);
        }

        assert_eq!(crates, vec![vec!["C"], vec!["M"], vec!["P", "D", "N", "Z"]]);
    }

    #[test]
    fn move_mult_crates_test() {
        let mut example = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2".to_string();

        example = example.replace(&['[', ']'][..], " ");

        let directions: Vec<&str> = example.lines().filter(|x| x.contains("move")).collect();

        let mut crates: Vec<Vec<&str>> = initialize_stacks(&example);

        move_mult_crates(directions[0], &mut crates);

        assert_eq!(crates, vec![vec!["Z", "N", "D"], vec!["M", "C"], vec!["P"]]);

        move_mult_crates(directions[1], &mut crates);

        assert_eq!(
            crates,
            vec![vec![], vec!["M", "C"], vec!["P", "Z", "N", "D"]]
        );
    }
}
