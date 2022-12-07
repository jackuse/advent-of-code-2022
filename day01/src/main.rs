use shared::read_file;

fn main() {
    let filename = "day01/src/input.txt";
    // let filename = "day01/src/input1_demo.txt";
    let data = read_file(filename);

    let res = part1(data.clone());
    println!("PART 1 : {}", res);

    let res = part2(data.clone());
    println!("PART 2 : {}", res);
}

fn part1(input: Vec<String>) -> u16 {
    let total_cal_by_elves = total_cal_by_elves(input);

    match total_cal_by_elves.first() {
        Some(v) => v.to_owned(),
        None => 0,
    }
}
fn part2(input: Vec<String>) -> u16 {
    let total_cal_by_elves = total_cal_by_elves(input);

    total_cal_by_elves[0..3].iter().sum()
}

fn total_cal_by_elves(input: Vec<String>) -> Vec<u16> {
    let mut total_cal_by_elves = vec![];
    let mut acc: u16 = 0;
    for line in input {
        if line.len() > 0 {
            acc = acc
                + match line.parse::<u16>() {
                    Ok(v) => v,
                    _ => 0,
                };
        } else {
            total_cal_by_elves.push(acc);
            acc = 0;
        }
    }

    total_cal_by_elves.sort();
    total_cal_by_elves.reverse();

    total_cal_by_elves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_part1_demo() {
        let filename = "src/input1_demo.txt";
        let res = part1(read_file(filename));

        assert_eq!(res, 24000);
    }

    #[test]
    fn should_pass_part2_demo() {
        let filename = "src/input1_demo.txt";
        let res = part2(read_file(filename));

        assert_eq!(res, 45000);
    }
}
