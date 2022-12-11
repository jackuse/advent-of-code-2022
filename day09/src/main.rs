use std::collections::{ HashSet};

use regex::Regex;
use shared::read_file;

static DAY: &str = "day09/";
pub static DEMO_FILENAME: &str = "src/input_demo.txt";
pub static DEMO_FILENAME2: &str = "src/input_demo2.txt";
pub static PUZZLE_FILENAME: &str = "src/input.txt";

fn main() {
    let data = read_file((DAY.to_owned() + PUZZLE_FILENAME).as_str());

    let res = part1(data.clone());
    println!("PART 1 : {}", res); // 6081

    let res = part2(data.clone());
    println!("PART 2 : {}", res);
}

fn part1(input: Vec<String>) -> usize {
    let mut visited_pos: HashSet<(isize, isize)> = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    visited_pos.insert(tail.to_owned());
    let reg_cmd = Regex::new(r"(\w) (\d+)").unwrap();

    for line in input {
        let mut dir = "N".to_string();
        let mut step = 0;
        for cap in reg_cmd.captures_iter(&line) {
            dir = cap[1].to_string();
            step = cap[2].to_string().parse::<isize>().unwrap();
        }

        // println!("== {} {} ==", dir, step);

        for _ in 0..step {
            if dir == "R".to_string() {
                head = (head.0 + 1, head.1);
            } else if dir == "L".to_string() {
                head = (head.0 - 1, head.1);
            } else if dir == "U".to_string() {
                head = (head.0, head.1 + 1);
            } else if dir == "D".to_string() {
                head = (head.0, head.1 - 1);
            }

            follow_tail(&head, &mut tail);
            // println!("head: {:?}", head);
            // println!("tail: {:?}", tail);
            // println!("");
            visited_pos.insert(tail.to_owned());
        }

        // println!("head: {:?}", head);
        // println!("tail: {:?}", tail);
    }

    visited_pos.len()
}

fn part2(input: Vec<String>) -> usize {
    let mut visited_pos: HashSet<(isize, isize)> = HashSet::new();
    let mut head = (0, 0);
    let mut knots = vec![(0,0);9];
    visited_pos.insert(head.to_owned());
    let reg_cmd = Regex::new(r"(\w) (\d+)").unwrap();

    for line in input {
        let mut dir = "N".to_string();
        let mut step = 0;
        for cap in reg_cmd.captures_iter(&line) {
            dir = cap[1].to_string();
            step = cap[2].to_string().parse::<isize>().unwrap();
        }

        // println!("== {} {} ==", dir, step);

        for _ in 0..step {
            if dir == "R".to_string() {
                head = (head.0 + 1, head.1);
            } else if dir == "L".to_string() {
                head = (head.0 - 1, head.1);
            } else if dir == "U".to_string() {
                head = (head.0, head.1 + 1);
            } else if dir == "D".to_string() {
                head = (head.0, head.1 - 1);
            }

            let mut prev = head;
            for mut knot in &mut knots {
                follow_tail(&prev, &mut knot);
                prev = *knot;
            }
           
            visited_pos.insert(prev.to_owned());
        }

        // println!("head: {:?}", head);
        // println!("tail: {:?}", knots.last().unwrap());
    }

    visited_pos.len()
}

fn follow_tail(head: &(isize, isize), tail: &mut (isize, isize)) {
    let distance_x = head.0 - tail.0;
    let distance_y = head.1 - tail.1;
    // println!("distance: x {:?} / y {:?}", distance_x, distance_y);

    if distance_x.abs() > 1 || distance_y.abs() > 1  {
        if distance_x > 0 {
            *tail = (tail.0 + 1, tail.1);
        } else if distance_x < 0 {
            *tail = (tail.0 - 1, tail.1);
        }
        if distance_y > 0  {
            *tail = (tail.0, tail.1 + 1);
        } else if distance_y < 0 {
            *tail = (tail.0, tail.1 - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_part1_demo() {
        let res = part1(read_file(DEMO_FILENAME));

        assert_eq!(res, 13);
    }

    #[test]
    fn should_pass_part2_demo() {
        let res = part2(read_file(DEMO_FILENAME2));

        assert_eq!(res, 36);
    }
}
