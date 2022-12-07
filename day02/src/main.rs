use shared::read_file;

static DAY: &str = "day02/";
static DEMO_FILENAME: &str = "src/input_demo.txt";
static PUZZLE_FILENAME: &str = "src/input.txt";

fn main() {
    let data = read_file((DAY.to_owned() + PUZZLE_FILENAME).as_str());

    let res = part1(data.clone());
    println!("PART 1 : {}", res);

    let res = part2(data.clone());
    println!("PART 2 : {}", res);
}

fn part1(input: Vec<String>) -> u16 {
    let mut scores = vec![];
    for line in input {
        scores.push(round_score(to_moves(line)));
    }

    scores.iter().sum()
}

fn part2(input: Vec<String>) -> u16 {
    let mut scores = vec![];
    for line in input {
        scores.push(round_score2(to_moves2(line)));
    }

    scores.iter().sum()
}

fn to_moves(line: String) -> Vec<char> {
    let vec = line.split(' ').collect::<Vec<&str>>();
    vec.into_iter()
        .map(|s| match s {
            "A" | "X" => 'R',
            "B" | "Y" => 'P',
            "C" | "Z" => 'S',
            _ => 'E',
        })
        .collect::<Vec<char>>()
}

fn to_moves2(line: String) -> Vec<char> {
    let vec = line.split(' ').collect::<Vec<&str>>();
    vec.into_iter()
        .map(|s| match s {
            "A" => 'R',
            "B" => 'P',
            "C" => 'S',
            "X" => 'L',
            "Y" => 'D',
            "Z" => 'W',
            _ => 'E',
        })
        .collect::<Vec<char>>()
}

fn round_score(round: Vec<char>) -> u16 {
    let move_score = move_score(*round.get(1).unwrap());
    let match_score = match_score(*round.get(1).unwrap(), *round.get(0).unwrap());

    // println!("{:?}", round);
    // println!("{:?}", match_score);

    move_score + match_score
}

fn round_score2(round: Vec<char>) -> u16 {
    let other_move = *round.get(0).unwrap();
    let match_result = *round.get(1).unwrap();
    let my_move = match match_result {
        'L' => match other_move {
            'R' => 'S',
            'P' => 'R',
            'S' => 'P',
            _ => 'E',
        },
        'D' => match other_move {
            'R' => 'R',
            'P' => 'P',
            'S' => 'S',
            _ => 'E',
        },
        'W' => match other_move {
            'R' => 'P',
            'P' => 'S',
            'S' => 'R',
            _ => 'E',
        },
        _ => 'E',
    };

    let move_score = move_score(my_move);
    let match_score = match_score(my_move, other_move);

    move_score + match_score
}

fn move_score(me: char) -> u16 {
    match me {
        'R' => 1,
        'P' => 2,
        'S' => 3,
        _ => 0,
    }
}

fn match_score(me: char, other: char) -> u16 {
    if me == other {
        3
    } else if (me == 'R' && other == 'S')
        || (me == 'P' && other == 'R')
        || (me == 'S' && other == 'P')
    {
        6
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_part1_demo() {
        let res = part1(read_file(DEMO_FILENAME));

        assert_eq!(res, 15);
    }

    #[test]
    fn should_pass_part2_demo() {
        let res = part2(read_file(DEMO_FILENAME));

        assert_eq!(res, 12);
    }
}
