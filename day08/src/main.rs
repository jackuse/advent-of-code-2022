use shared::read_file;

static DAY: &str = "day08/";
pub static DEMO_FILENAME: &str = "src/input_demo.txt";
pub static PUZZLE_FILENAME: &str = "src/input.txt";

fn main() {
    let data = read_file((DAY.to_owned() + PUZZLE_FILENAME).as_str());

    let res = part1(data.clone());
    println!("PART 1 : {}", res);

    let res = part2(data.clone());
    println!("PART 2 : {}", res);
}

fn part1(input: Vec<String>) -> u32 {
    let forest = init_matrix(input);
    // println!("init");
    // pretty_print(&forest);
    count_visible(&forest)
}
fn part2(input: Vec<String>) -> u32 {
    let forest = init_matrix(input);
    // pretty_print(&forest);
    count_best_scenic(&forest)
}

fn count_best_scenic(forest: &Vec<Vec<u32>>) -> u32 {
    let row_size = forest.get(0).unwrap().len();
    let col_size = forest.len();
    let mut top_score = 0;
    for x in 0..row_size  {
        for y in 0..col_size  {
        // let x = 2;
        // let y = 3;
            // println!("test: {}x{}", x, y);
            // println!("up: {}", scenic_up(x, y, &forest));
            // println!("left: {}", scenic_left(x, y, &forest));
            // println!("down: {}", scenic_down(x, y, &forest));
            // println!("right: {}", scenic_right(x, y, &forest));
            let score = scenic_right(x, y, &forest)
            * scenic_left(x, y, &forest)
            * scenic_up(x, y, &forest)
            * scenic_down(x, y, &forest);
            if score > top_score
            {
                // println!("top: {}x{} : {}", x, y, score);
                top_score = score;
            }
        }
    }
    top_score
}

fn scenic_right(x: usize, y: usize, forest: &Vec<Vec<u32>>) -> u32 {
    let row_size = forest.get(0).unwrap().len();
    let cur = forest[y][x];
    let mut res = 0;
    // println!("{} to {}", x+1, row_size);
    for xd in x+1..row_size {
        if cur > forest[y][xd] {
            res = res +1;
        } else if cur <= forest[y][xd] {
            res = res +1;
            break;
        }
    }

    res
}

fn scenic_left(x: usize, y: usize, forest: &Vec<Vec<u32>>) -> u32 {
    let cur = forest[y][x];
    let mut res = 0;
    // println!("{} to {}", 0, x);
    for xd in (0..x).rev() {
        if cur > forest[y][xd] {
            res = res +1;
        } else if cur <= forest[y][xd] {
            res = res +1;
            break;
        }
    }

    res
}

fn scenic_up(x: usize, y: usize, forest: &Vec<Vec<u32>>) -> u32 {
    let cur = forest[y][x];
    let mut res = 0;
    // println!("{} to {}", 0, y);
    for yd in (0..y).rev() {
        if cur > forest[yd][x] {
            res = res +1;
        } else if cur <= forest[yd][x] {
            res = res +1;
            break;
        }
    }

    res
}

fn scenic_down(x: usize, y: usize, forest: &Vec<Vec<u32>>) -> u32 {
    let col_size = forest.len();
    let cur = forest[y][x];
    let mut res = 0;
    // println!("{} to {}", y+1, col_size-1);
    for yd in y+1..col_size {
        if cur > forest[yd][x] {
            res = res +1;
        } else if cur <= forest[yd][x] {
            res = res +1;
            break;
        }
    }

    res
}

fn count_visible(forest: &Vec<Vec<u32>>) -> u32 {
    let row_size = forest.get(0).unwrap().len();
    let col_size = forest.len();
    let mut visible = 0;
    for x in 0..row_size  {
        for y in 0..col_size  {
            // println!("test: {}x{}", x, y);
            if is_visible_right(x, y, &forest)
                || is_visible_left(x, y, &forest)
                || is_visible_top(x, y, &forest)
                || is_visible_bottom(x, y, &forest)
            {
                // println!("visible: {}x{} : {}", x, y, forest[y][x]);
                visible = visible + 1;
            }
        }
    }

    // 1x1 / 2x1 / 1x2 / . 3x2 / 2x3
    // let x = 0;
    // let y = 0;
    // println!("test: {}x{} : {}", x, y, forest[y][x]);
    // println!("right {}", is_visible_right(x, y, &forest));
    // println!("left {}", is_visible_left(x, y, &forest));
    // println!("top {}", is_visible_top(x, y, &forest));
    // println!("bottom {}", is_visible_bottom(x, y, &forest));
    // println!("total {}", is_visible_right(x, y, &forest)
    //             || is_visible_left(x, y, &forest)
    //             || is_visible_top(x, y, &forest)
    //             || is_visible_bottom(x, y, &forest));

    visible
}

fn is_visible_right(x: usize, y: usize, forest: &Vec<Vec<u32>>) -> bool {
    let row_size = forest.get(0).unwrap().len();
    let cur = forest[y][x];
    let mut res = true;
    for xd in x+1..row_size {
        if cur <= forest[y][xd] {
            res = false;
            break;
        }
    }

    res
}

fn is_visible_left(x: usize, y: usize, forest: &Vec<Vec<u32>>) -> bool {
    let cur = forest[y][x];
    let mut res = true;
    // println!("{} to {}", 0, x);
    for xd in (0..x).rev() {
        if cur <= forest[y][xd] {
            res = false;
            break;
        }
    }

    res
}

fn is_visible_top(x: usize, y: usize, forest: &Vec<Vec<u32>>) -> bool {
    let cur = forest[y][x];
    let mut res = true;
    // println!("{} to {}", 0, y);
    for yd in (0..y).rev() {
        if cur <= forest[yd][x] {
            res = false;
            break;
        }
    }

    res
}

fn is_visible_bottom(x: usize, y: usize, forest: &Vec<Vec<u32>>) -> bool {
    let col_size = forest.len();
    let cur = forest[y][x];
    let mut res = true;
    // println!("{} to {}", y+1, col_size-1);
    for yd in y+1..col_size {
        // println!("lol: {} / {}: {}", cur, yd,forest[yd][x]);
        if cur <= forest[yd][x] {
            res = false;
            break;
        }
    }

    res
}

fn init_matrix(input: Vec<String>) -> Vec<Vec<u32>> {
    let row_size = input.get(0).unwrap().len();
    let col_size = input.len();
    // println!("{:?}x{:?}", col_size, row_size);
    let mut forest = vec![vec![0; row_size]; col_size];
    let mut y = 0;
    for line in &input {
        let lol: Vec<u32> = line
            .chars()
            .into_iter()
            .map(|c| match c.to_digit(10) {
                Some(v) => v,
                _ => 0,
            })
            .collect();
        let mut x = 0;
        for tree in lol {
            forest[y][x] = tree;
            x = x + 1;
        }
        y = y + 1;
    }
    forest
}

pub fn pretty_print(matrix: &Vec<Vec<u32>>) {
    for line in matrix {
        println!("{:?}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_part1_demo() {
        let res = part1(read_file(DEMO_FILENAME));

        assert_eq!(res, 21);
    }

    #[test]
    fn should_pass_part2_demo() {
        let res = part2(read_file(DEMO_FILENAME));

        assert_eq!(res, 8);
    }
}
