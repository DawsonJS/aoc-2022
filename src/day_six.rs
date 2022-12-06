use std::collections::VecDeque;

pub fn start_stream(stream: &str) -> i32 {
    let mut result: i32 = -1;
    let mut processed: VecDeque<char> = VecDeque::new();

    for (index, c) in stream.chars().enumerate() {
        if processed.len() <= 3 {
            if processed.contains(&c) {
                for _i in 0..processed.len() {
                    let popped = processed.pop_front().unwrap();
                    if popped == c {
                        break;
                    }
                }
            }
            processed.push_back(c);
        } else {
            result = index as i32;
            break;
        }
    }

    result
}

pub fn start_message(stream: &str) -> i32 {
    let mut result: i32 = -1;
    let mut processed: VecDeque<char> = VecDeque::new();

    for (index, c) in stream.chars().enumerate() {
        if processed.len() <= 13 {
            if processed.contains(&c) {
                for _i in 0..processed.len() {
                    let popped = processed.pop_front().unwrap();
                    if popped == c {
                        break;
                    }
                }
            }
            processed.push_back(c);
        } else {
            result = index as i32;
            break;
        }
    }

    result
}
#[cfg(test)]
mod day_six_tests {
    use crate::day_six::*;

    #[test]
    fn start_stream_test() {
        let example = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
        assert_eq!(start_stream(&example), 7);

        let example_2 = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        assert_eq!(start_stream(&example_2), 5);

        let example_3 = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
        assert_eq!(start_stream(&example_3), 6);

        let example_4 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
        assert_eq!(start_stream(&example_4), 10);

        let example_5 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();
        assert_eq!(start_stream(&example_5), 11);
    }

    #[test]
    fn start_message_test() {
        let example = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
        assert_eq!(start_message(&example), 19);

        let example_2 = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        assert_eq!(start_message(&example_2), 23);

        let example_3 = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
        assert_eq!(start_message(&example_3), 23);

        let example_4 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
        assert_eq!(start_message(&example_4), 29);

        let example_5 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();
        assert_eq!(start_message(&example_5), 26);
    }
}
