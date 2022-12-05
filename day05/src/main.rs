use shared::read_file;

static day: &str = "day05/";
static demo_filename: &str = "src/input_demo.txt";
static puzzle_filename: &str = "src/input.txt";

#[derive(Debug)]
struct Cargo {
    pub stack: Vec<Vec<char>>,
}

impl Cargo {
    pub fn move_order_9000(&mut self, quantity: usize, from: usize, to: usize) {
        self.move_order(quantity, from, to, false)
    }

    pub fn move_order_9001(&mut self, quantity: usize, from: usize, to: usize) {
        self.move_order(quantity, from, to, true)
    }

    fn move_order(&mut self, quantity: usize, from: usize, to: usize, is9001: bool) {
        let from = self.stack.get_mut(from - 1).unwrap();
        let mut lol = vec![];
        for _ in 0..quantity {
            lol.push(from.pop().unwrap());
        }
        if (is9001) {
            lol.reverse();
        }

        let to = self.stack.get_mut(to - 1).unwrap();
        to.append(&mut lol);
    }

    pub fn get_top(&self) -> String {
        let mut res = "".to_string();
        for stack in &self.stack {
            res.push_str(stack.last().unwrap().to_string().as_str());
        }
        res
    }
}

fn main() {
    let data = read_file((day.to_owned() + puzzle_filename).as_str());

    let res = part1(data.clone());
    println!("PART 1 : {}", res);

    let res = part2(data.clone());
    println!("PART 2 : {}", res);
}

fn part1(input: Vec<String>) -> String {
    let mut cargo = make_cargo(&input);
    // println!("cargo : {:?}", cargo);

    // get order section
    let mut orders = vec![];
    let mut start = false;
    for line in &input {
        if start {
            orders.push(line);
        }
        if line.len() == 0 {
            start = true;
        }
    }
    // println!("orders : {:?}", orders);

    for order in orders {
        let split_order = order.split_whitespace().collect::<Vec<&str>>();
        let quantity = split_order.get(1).unwrap().parse::<usize>().unwrap();
        let from = split_order.get(3).unwrap().parse::<usize>().unwrap();
        let to = split_order.get(5).unwrap().parse::<usize>().unwrap();

        cargo.move_order_9000(quantity, from, to);
    }

    println!("cargo : {:?}", cargo);

    // let mut lol = Cargo {
    //     stack: vec![vec!['N', 'Z'], vec!['P']],
    // };

    // println!("a : {:?}", lol);

    // lol.move_order(2, 1, 2);

    // println!("b : {:?}", lol);

    cargo.get_top()
}

fn part2(input: Vec<String>) -> String {
    let mut cargo = make_cargo(&input);
    println!("cargo : {:?}", cargo);

    // get order section
    let mut orders = vec![];
    let mut start = false;
    for line in &input {
        if start {
            orders.push(line);
        }
        if line.len() == 0 {
            start = true;
        }
    }

    for order in orders {
        let split_order = order.split_whitespace().collect::<Vec<&str>>();
        let quantity = split_order.get(1).unwrap().parse::<usize>().unwrap();
        let from = split_order.get(3).unwrap().parse::<usize>().unwrap();
        let to = split_order.get(5).unwrap().parse::<usize>().unwrap();

        cargo.move_order_9001(quantity, from, to);
    }

    cargo.get_top()
}

fn make_cargo(input: &Vec<String>) -> Cargo {
    // get cargo section
    let mut section = vec![];
    for line in input {
        if line.len() == 0 {
            break;
        }
        section.push(line);
    }
    // println!("section : {:?}", section);

    // get vec number
    let size = section
        .last()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    // println!("size : {:?}", size);

    let mut res = vec![vec![]; size];

    // from bottom to top add cargo
    section.pop();
    section.reverse();
    // println!("section : {:?}", section);
    for line in section {
        for colomn in 0..size {
            let from = 1 + colomn * 4;
            let to = from + 1;
            let a = line.get(from..to);
            if a.is_some() && !a.unwrap().contains(" ") {
                res.get_mut(colomn)
                    .unwrap()
                    .push(a.unwrap().to_string().chars().nth(0).unwrap());
            }
        }
    }

    Cargo { stack: res }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_part1_demo() {
        let res = part1(read_file(demo_filename));

        assert_eq!(res, "CMZ".to_string());
    }

    #[test]
    fn should_pass_part2_demo() {
        let res = part2(read_file(demo_filename));

        assert_eq!(res, "MCD".to_string());
    }
}
