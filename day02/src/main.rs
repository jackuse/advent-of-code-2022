use shared::read_file;

static day: &str = "day02/";
static demo_filename: &str = "src/input_demo.txt";
static puzzle_filename: &str = "src/input.txt";

fn main() {
    let data = read_file((day.to_owned() + puzzle_filename).as_str());

    let res = part1(data.clone());
    println!("PART 1 : {}", res);

    let res = part2(data.clone());
    println!("PART 2 : {}", res);
}

fn part1(input: Vec<String>) -> u32 {
    let moves: Vec<Vec<char>> = input.into_iter().map(|l| to_moves(l)).collect();
    let scores: Vec<u32> = moves.into_iter().map(|m| round_score(m)).collect();

    sum(scores)
}
fn part2(input: Vec<String>) -> u32 {
    let moves: Vec<Vec<char>> = input.into_iter().map(|l| to_moves2(l)).collect();
    let scores: Vec<u32> = moves.into_iter().map(|m| round_score2(m)).collect();

    sum(scores)
}

fn sum(vec: Vec<u32>) -> u32 {
    let calories = vec.into_iter().reduce(|acc, c| acc + c);
    match calories {
        Some(v) => v,
        None => 0,
    }
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

fn round_score(round: Vec<char>) -> u32 {
    let move_score = move_score(*round.get(1).unwrap());
    let match_score = match_score(*round.get(1).unwrap(), *round.get(0).unwrap());

    // println!("{:?}", round);
    // println!("{:?}", match_score);

    move_score + match_score
}

fn round_score2(round: Vec<char>) -> u32 {
    let other_move = *round.get(0).unwrap();
    let match_result = *round.get(1).unwrap();
    let my_move = match match_result {
        'L' => {
            match other_move {
                'R' => 'S',
                'P' => 'R',
                'S' => 'P',
                _ => 'E',
            }
        },
        'D' => {
            match other_move {
                'R' => 'R',
                'P' => 'P',
                'S' => 'S',
                _ => 'E',
            }
        },
        'W' => {
            match other_move {
                'R' => 'P',
                'P' => 'S',
                'S' => 'R',
                _ => 'E',
            }
        },
        _ => 'E'
    };

    let move_score = move_score(my_move);
    let match_score = match_score(my_move,other_move);

    move_score + match_score
}


fn move_score(me: char) -> u32 {
    match me {
        'R' => 1,
        'P' => 2,
        'S' => 3,
        _ => 0,
    }
}

fn match_score(me: char, other: char) -> u32 {
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
        let res = part1(read_file(demo_filename));

        assert_eq!(res, 15);
    }

    // #[test]
    fn should_pass_part2_demo() {
        let res = part2(read_file(demo_filename));

        assert_eq!(res, 12);
    }
}
