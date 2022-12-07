use shared::read_file;

static DAY: &str = "day06/";
static DEMO_FILENAME: &str = "src/input_demo.txt";
static PUZZLE_FILENAME: &str = "src/input.txt";

fn main() {
    let data = read_file((DAY.to_owned() + PUZZLE_FILENAME).as_str());

    let res = part1(data.clone());
    println!("PART 1 : {}", res);

    let res = part2(data.clone());
    println!("PART 2 : {}", res);
}

fn part1(input: Vec<String>) -> usize {
    let mut res = 0;

    for line in input {
        res = last_char_for_uniq_size(line, 4);
    }

    res
}

fn part2(input: Vec<String>) -> usize {
    let mut res = 0;

    for line in input {
        res = last_char_for_uniq_size(line, 14);
    }

    res
}

fn last_char_for_uniq_size(text: String, size: usize) -> usize {
    let mut res = 0;
    for i in 0..text.len() - (size + 1) {
        let candidate = &text[i..i + (size)];
        if all_different(candidate.to_string()) {
            res = i + size;
            break;
        }
    }
    res
}

fn all_different(text: String) -> bool {
    let mut uniq = vec![];
    let mut res = true;
    text.chars().for_each(|c| {
        if uniq.contains(&c) {
            res = false
        } else {
            uniq.push(c);
        }
    });

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_part1_demo() {
        let res = part1(read_file(DEMO_FILENAME));

        assert_eq!(res, 7);
    }

    #[test]
    fn should_pass_part2_demo() {
        let res = part2(read_file(DEMO_FILENAME));

        assert_eq!(res, 19);
    }
}
