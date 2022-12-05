pub fn initialize_stacks<'a>(list: &'a str) -> Vec<Vec<&'a str>> {
    let mut drawing: Vec<&str> = list.lines().filter(|x| !x.contains("move") && !x.is_empty()).collect();
    let number = drawing.pop().unwrap();

    let num_of_crates = number.get((number.len() - 2)..(number.len() - 1)).unwrap().parse::<i32>().unwrap();

    let mut crates: Vec<Vec<&str>> = Vec::new();

    for i in 0..num_of_crates {
        crates.push(Vec::new());
        for j in drawing.iter().rev() {
            let binding: Vec<&str> = j.split(" ").collect();
            if !binding[i as usize].is_empty() {
                crates[i as usize].push(binding[i as usize]);
            }
        }
    }

    crates
}

pub fn move_crates(instruction: &str, crates: &mut Vec<Vec<&str>>) {
    let nums: Vec<i32> = instruction.split(' ').filter(|x| !x.contains('o')).map(|x| x.parse::<i32>().unwrap()).collect();

    for i in (0 as usize)..(nums[0] as usize) {
        let binding: Option<&str> = crates[nums[1] as usize - 1 as usize].pop();
        match binding {
            Some(value) => {crates[nums[2] as usize - 1 as usize].push(value)},
            None => {},
        }
    }
}


#[cfg(test)]
mod day_five_tests {
    use crate::day_five::*;
    #[test]
    fn parse_crates_test() {
        let mut example = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2".to_string();

        example = example.replace(&['[', ']', '\t'][..], "");

        assert_eq!(example, "    D    \nN C    \nZ M P\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2");

        let mut drawing: Vec<&str> = list.lines().filter(|x| !x.contains("move") && !x.is_empty()).collect();
        let number = drawing.pop().unwrap();

        let num_of_crates = number.get((number.len() - 2)..(number.len() - 1)).unwrap().parse::<i32>().unwrap();
//        let directions: Vec<&str> = example.lines().filter(|x| x.contains("move")).collect();

 //       assert_eq!(directions, vec!["move 1 from 2 to 1", "move 3 from 1 to 3", "move 2 from 2 to 1", "move 1 from 1 to 2"]);

  //      let crates: Vec<Vec<&str>> = initialize_stacks(&example);

   //     assert_eq!(crates, vec![vec!["Z", "N"], vec!["M", "C", "D"], vec!["P"]]);
    }
/*
    #[test]
    fn move_crates_test() {
        let mut example = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2".to_string();

        example = example.replace(&['[', ']', '\t'][..], "");

        let directions: Vec<&str> = example.lines().filter(|x| x.contains("move")).collect();


        let mut crates: Vec<Vec<&str>> = initialize_stacks(&example);

        move_crates(directions[0], &mut crates);

        assert_eq!(crates, vec![vec!["Z", "N", "D"], vec!["M", "C"], vec!["P"]]);
    }

    #[test]
    fn move_many_crates_test() {
        let mut example = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2".to_string();

        example = example.replace(&['[', ']', '\t'][..], "");

        let directions: Vec<&str> = example.lines().filter(|x| x.contains("move")).collect();


        let mut crates: Vec<Vec<&str>> = initialize_stacks(&example);

        for l in directions.iter() {

            move_crates(l, &mut crates);
        }

        assert_eq!(crates, vec![vec!["C"], vec!["M"], vec!["P", "D", "N", "Z"]]);
    }*/
}

