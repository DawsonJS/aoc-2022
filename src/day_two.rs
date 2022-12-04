#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum HandShape {
    Rock,
    Paper,
    Scissors,
    Invalid,
}

pub enum PuzzlePart {
    One,
    Two,
}

impl HandShape {
    pub fn win_against(&self) -> HandShape {
        match self {
            HandShape::Rock => HandShape::Paper,
            HandShape::Paper => HandShape::Scissors,
            HandShape::Scissors => HandShape::Rock,
            _ => HandShape::Invalid,
        }
    }

    pub fn lose_against(&self) -> HandShape {
        match self {
            HandShape::Rock => HandShape::Scissors,
            HandShape::Paper => HandShape::Rock,
            HandShape::Scissors => HandShape::Paper,
            _ => HandShape::Invalid,
        }
    }

    pub fn decode_hand_shape(&self, code: &str) -> HandShape {
        match code {
            "X" => self.lose_against(),
            "Y" => *self,
            "Z" => self.win_against(),
            _ => HandShape::Invalid,
        }
    }
}

pub fn get_hand_shape(code: &str) -> HandShape {
    match code {
        "A" => HandShape::Rock,
        "B" => HandShape::Paper,
        "C" => HandShape::Scissors,
        "X" => HandShape::Rock,
        "Y" => HandShape::Paper,
        "Z" => HandShape::Scissors,
        _ => HandShape::Invalid,
    }
}

pub fn calc_points(code_line: &str, part: PuzzlePart) -> i32 {
    let opponent = get_hand_shape(&code_line[..1]);
    let player = match part {
        PuzzlePart::One => get_hand_shape(&code_line[2..]),
        PuzzlePart::Two => opponent.decode_hand_shape(&code_line[2..]),
    };
    let mut points: i32 = 0;

    match player {
        HandShape::Rock => {
            points += 1;
        }
        HandShape::Paper => points += 2,
        HandShape::Scissors => points += 3,
        _ => {}
    }

    if player == opponent.win_against() {
        points += 6;
    } else if player == opponent {
        points += 3;
    }

    points
}

#[cfg(test)]
mod day_two_tests {
    use crate::day_two::*;

    #[test]
    fn get_hand_shape_test() {
        let example = "A Y\nB X\nC Z\n";
        let test = get_hand_shape(&example[..1]);
        assert_eq!(test, HandShape::Rock);
    }

    #[test]
    fn against_test() {
        let example = HandShape::Rock;
        assert_eq!(example.win_against(), HandShape::Paper);
        assert_eq!(example.lose_against(), HandShape::Scissors);
    }

    #[test]
    fn calc_points_test() {
        let example = "A Y\nB X\nC Z\n";
        let mut test = calc_points(&example[..3], PuzzlePart::One);

        assert_eq!(test, 8);
        test = calc_points(&example[..3], PuzzlePart::Two);
        assert_eq!(test, 4);
    }
}
