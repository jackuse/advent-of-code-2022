use regex::Regex;
use shared::read_file;

static DAY: &str = "day11/";
pub static DEMO_FILENAME: &str = "src/input_demo.txt";
pub static PUZZLE_FILENAME: &str = "src/input.txt";

fn main() {
    let data = read_file((DAY.to_owned() + DEMO_FILENAME).as_str());

    let res = part1(data.clone());
    println!("PART 1 : {}", res);

    let res = part2(data.clone());
    println!("PART 2 : {}", res);
}

fn part1(input: Vec<String>) -> usize {
    let mut monkeys: Vec<Monkey> = make_monkeys(input);
    let rounds = 20;

    for _round in 0..rounds {
        // for monkey in monkeys.iter_mut() {
        for i in 0..monkeys.len() {
            // println!();
            // println!("Monkey: {}", i + 1);
            // print_state(&monkeys);

            let test;
            let mut res = vec![];

            let monkey = monkeys.get_mut(i).unwrap();
            test = monkey.test.clone();

            for item in &monkey.items {
                let mut worry_level = match monkey.op.op {
                    Op::Mult => {
                        if monkey.op.coef == 0 {
                            item * item
                        } else {
                            item * monkey.op.coef
                        }
                    }
                    _ => item + monkey.op.coef,
                };
                worry_level = worry_level / 3;
                res.push(worry_level);
            }
            monkey.inspected = monkey.inspected + monkey.items.len();
            monkey.items = vec![];

            let monkey_true = monkeys.get_mut(test.if_true).unwrap();

            for worry_level in &res {
                if worry_level % test.coef == 0 {
                    monkey_true.items.push(worry_level.clone());
                }
            }

            let monkey_false = monkeys.get_mut(test.if_false).unwrap();

            for worry_level in &res {
                if worry_level % test.coef != 0 {
                    monkey_false.items.push(worry_level.clone());
                }
            }
        }

        // println!("Round: {}", round + 1);
        // print_state(&monkeys);
    }

    // print_inspected(&monkeys);

    get_score(&mut monkeys)
}

fn part2(input: Vec<String>) -> usize {
    let mut monkeys: Vec<Monkey> = make_monkeys(input);
    let rounds = 10000;

    let least_common_multiple = monkeys.iter().map(|m| m.test.coef).product::<usize>();
    println!("{}", least_common_multiple);

    for _round in 0..rounds {
        // if round % 1000 == 0 {
        //     println!("Round: {}", round );
        //     print_inspected(&monkeys);
        // }
        for i in 0..monkeys.len() {

            let test;
            let mut res = vec![];

            let monkey = monkeys.get_mut(i).unwrap();
            test = monkey.test.clone();

            for item in &monkey.items {
                let mut worry_level = match monkey.op.op {
                    Op::Mult => {
                        if monkey.op.coef == 0 {
                            item * item
                        } else {
                            item * monkey.op.coef
                        }
                    }
                    _ => item + monkey.op.coef,
                };
                worry_level = worry_level % least_common_multiple;
                res.push(worry_level);
            }
            monkey.inspected = monkey.inspected + monkey.items.len();
            monkey.items = vec![];

            for worry_level in &res {
                let next;
                if worry_level % test.coef == 0 {
                    next = test.if_true
                } else {
                    next = test.if_false;
                }
                let next_mk = monkeys.get_mut(next).unwrap();
                next_mk.items.push(worry_level.clone());
            }

        }
    }

    print_inspected(&monkeys);

    get_score(&mut monkeys)
}

pub fn print_state(monkeys: &Vec<Monkey>) {
    let mut cpt = 0;
    for monkey in monkeys {
        println!("Monkey {}: {:?}", cpt, monkey.items);
        cpt = cpt + 1;
    }
    println!();
}

pub fn print_inspected(monkeys: &Vec<Monkey>) {
    let mut cpt = 0;
    for monkey in monkeys {
        println!("Monkey {} inspected {:?} times", cpt, monkey.inspected);
        cpt = cpt + 1;
    }
    println!();
}

fn get_score(monkeys: &mut Vec<Monkey>) -> usize {
    monkeys.sort_by(|a, b| a.inspected.cmp(&b.inspected.to_owned()));
    monkeys.reverse();

    let top_2 = &monkeys[0..2];

    top_2[0].inspected * top_2[1].inspected
}

fn make_monkeys(input: Vec<String>) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];

    let reg_items = Regex::new(r"items: (\d+(?:, \d+)*)").unwrap();
    let reg_op = Regex::new(r"new = .* (\+|\*) (\d+)").unwrap();
    let reg_test = Regex::new(r"divisible by (\d+)").unwrap();
    let reg_true = Regex::new(r"If true: throw to monkey (\d+)").unwrap();
    let reg_false = Regex::new(r"If false: throw to monkey (\d+)").unwrap();

    let mut cur = Monkey {
        items: vec![],
        op: Operation {
            op: Op::Mult,
            coef: 0,
        },
        test: Test {
            coef: 0,
            if_true: 0,
            if_false: 0,
        },
        inspected: 0,
    };
    for line in input {
        if line.len() == 0 {
            monkeys.push(cur);

            cur = Monkey {
                items: vec![],
                op: Operation {
                    op: Op::Mult,
                    coef: 0,
                },
                test: Test {
                    coef: 0,
                    if_true: 0,
                    if_false: 0,
                },
                inspected: 0,
            };
        }
        if reg_items.is_match(&line) {
            for cap in reg_items.captures_iter(&line) {
                let values: Vec<usize> = cap[1]
                    .split(", ")
                    .map(|n| n.to_string().parse::<usize>().unwrap())
                    .collect();
                cur.items = values;
            }
        }
        if reg_op.is_match(&line) {
            for cap in reg_op.captures_iter(&line) {
                if cap[1].to_string() == "*".to_owned() {
                    cur.op.op = Op::Mult;
                } else {
                    cur.op.op = Op::Plus;
                }

                cur.op.coef = cap[2].to_string().parse::<usize>().unwrap();
            }
        }

        if reg_test.is_match(&line) {
            for cap in reg_test.captures_iter(&line) {
                cur.test.coef = cap[1].to_string().parse::<usize>().unwrap();
            }
        }

        if reg_true.is_match(&line) {
            for cap in reg_true.captures_iter(&line) {
                cur.test.if_true = cap[1].to_string().parse::<usize>().unwrap();
            }
        }

        if reg_false.is_match(&line) {
            for cap in reg_false.captures_iter(&line) {
                cur.test.if_false = cap[1].to_string().parse::<usize>().unwrap();
            }
        }
    }
    monkeys.push(cur);

    monkeys
}

#[derive(Debug, Clone)]
pub enum Op {
    Plus,
    Mult,
}
#[derive(Debug, Clone)]
pub struct Operation {
    pub op: Op,
    pub coef: usize, // 0 is old
}

#[derive(Debug, Clone)]
pub struct Test {
    pub coef: usize,
    pub if_true: usize,
    pub if_false: usize,
}

#[derive(Debug, Clone)]
pub struct Monkey {
    pub items: Vec<usize>,
    pub op: Operation,
    pub test: Test,
    pub inspected: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_part1_demo() {
        let res = part1(read_file(DEMO_FILENAME));

        assert_eq!(res, 10605);
    }

    #[test]
    fn should_pass_part2_demo() {
        let res = part2(read_file(DEMO_FILENAME));

        assert_eq!(res, 0);
    }
}
