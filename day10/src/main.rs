use regex::Regex;
use shared::read_file;

static DAY: &str = "day10/";
pub static DEMO_FILENAME: &str = "src/input_demo.txt";
pub static PUZZLE_FILENAME: &str = "src/input.txt";

fn main() {
    let data = read_file((DAY.to_owned() + PUZZLE_FILENAME).as_str());

    let res = part1(data.clone());
    println!("PART 1 : {}", res);

    println!("PART 2 :");
    let res = part2(data.clone());
}

fn part1(input: Vec<String>) -> i32 {
    let check_cycle = vec![20, 60, 100, 140, 180, 220];
    let reg_addx = Regex::new(r"addx (-?\d+)").unwrap();

    let mut data = vec![];

    let mut cycle = 0;
    let mut x = 1;
    for line in input {
        cycle = cycle + 1;
        if check_cycle.contains(&cycle) {
            data.push(x);
        }

        if reg_addx.is_match(&line) {
            cycle = cycle + 1;

            if check_cycle.contains(&cycle) {
                data.push(x);
            }

            for cap in reg_addx.captures_iter(&line) {
                x = x + &cap[1].to_string().parse::<i32>().unwrap();
            }
        }
    }

    let mut cpt = 0;
    data.iter()
        .map(|v| {
            let res = v * check_cycle[cpt];
            cpt = cpt + 1;
            // println!("x:{} c: {} = {}", v, check_cycle[cpt - 1], res);
            res
        })
        .sum()
}

fn part2(input: Vec<String>) -> u32 {
    let mut ctr_lines = vec![vec![".".to_string(); 40]; 6];
    let col: usize = 40;
    let reg_addx = Regex::new(r"addx (-?\d+)").unwrap();

    let mut cycle: usize = 0;
    let mut x: i32 = 1;
    let mut cur_line: usize = 0;
    for line in input {

        let x_pos = cycle % col;


        if is_on_sprite(x_pos as i32, x) {
            draw(&mut ctr_lines, x_pos, cur_line);
        }

        cycle = cycle + 1;
        if cycle % col == 0 {
            cur_line = cur_line + 1;
        }

        if reg_addx.is_match(&line) {
            let x_pos = cycle % col;
            if is_on_sprite(x_pos as i32, x) {
                draw(&mut ctr_lines, x_pos, cur_line);
            }

            cycle = cycle + 1;

            if cycle % col == 0 {
                cur_line = cur_line + 1;
            }

            for cap in reg_addx.captures_iter(&line) {
                x = x + cap[1].to_string().parse::<i32>().unwrap();
            }
        }
    }

    for line in ctr_lines {
        println!("{}", line.join(""));
    }

    0
}

fn draw(ctr_lines: &mut Vec<Vec<String>>, x: usize, y: usize) {
    let ctr_line = ctr_lines.get_mut(y).unwrap();
    let pixel = ctr_line.get_mut(x).unwrap();
    *pixel = "#".to_string();
}

fn is_on_sprite(pixel_x: i32, sprite_x: i32) -> bool {
    pixel_x == sprite_x || pixel_x == sprite_x - 1 || pixel_x == sprite_x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_part1_demo() {
        let res = part1(read_file(DEMO_FILENAME));

        assert_eq!(res, 13140);
    }

    #[test]
    fn should_pass_part2_demo() {
        let res = part2(read_file(DEMO_FILENAME));

        assert_eq!(res, 0);
    }
}
