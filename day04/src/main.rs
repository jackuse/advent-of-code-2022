use std::vec;

use shared::read_file;

static day: &str = "day04/";
static demo_filename: &str = "src/input_demo.txt";
static puzzle_filename: &str = "src/input.txt";

#[derive(Debug)]
struct Assignment {
    pub worker1: String,
    pub worker2: String,
}

impl Assignment {
    pub fn fully_contain(&self) -> bool {
        let range1 = self.range(&self.worker1);
        let range2 = self.range(&self.worker2);

        if (range1.0 >= range2.0 && range1.1 <= range2.1)
            || (range2.0 >= range1.0 && range2.1 <= range1.1)
        {
            true
        } else {
            false
        }
    }

    pub fn overlap(&self) -> bool {
        let range1 = self.range(&self.worker1);
        let range2 = self.range(&self.worker2);

        if (range1.0 >= range2.0 && range1.0 <= range2.1)
            || (range1.1 >= range2.0 && range1.1 <= range2.1)
            || (range2.0 >= range1.0 && range2.0 <= range1.1)
            || (range2.1 >= range1.0 && range2.1 <= range1.1)
        {
            true
        } else {
            false
        }
    }

    fn range(&self, worker: &String) -> (u32, u32) {
        let list = worker
            .split('-')
            .into_iter()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        (*list.get(0).unwrap(), *list.get(1).unwrap())
    }
}

fn main() {
    let data = read_file((day.to_owned() + puzzle_filename).as_str());

    let res = part1(data.clone());
    println!("PART 1 : {}", res);

    let res = part2(data.clone());
    println!("PART 2 : {}", res);

}

fn part1(input: Vec<String>) -> u32 {
    let mut groups = vec![];
    for line in input {
        groups.push(create_groups_by_colon(line));
    }
    // println!("{:?}", groups);

    let mut assignments = vec![];
    for group in groups {
        assignments.push(Assignment {
            worker1: group.get(0).unwrap().to_string(),
            worker2: group.get(1).unwrap().to_string(),
        });
    }
    // // println!("{:?}", assignments);

    let mut fullyContaines = 0;
    for assignment in assignments {
        if assignment.fully_contain() {
            fullyContaines = fullyContaines + 1;
        }
    }
    fullyContaines
}

fn part2(input: Vec<String>) -> u32 {
    let mut groups = vec![];
    for line in input {
        groups.push(create_groups_by_colon(line));
    }

    let mut assignments = vec![];
    for group in groups {
        assignments.push(Assignment {
            worker1: group.get(0).unwrap().to_string(),
            worker2: group.get(1).unwrap().to_string(),
        });
    }
    // // println!("{:?}", assignments);

    let mut overlap = 0;
    for assignment in assignments {
        if assignment.overlap() {
            overlap = overlap + 1;
        }
    }
    overlap
}

fn create_groups_by_colon(line: String) -> Vec<String> {
    line.split(',').into_iter().map(|s| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_part1_demo() {
        let res = part1(read_file(demo_filename));

        assert_eq!(res, 2);
    }

    #[test]
    fn should_pass_part2_demo() {
        let res = part2(read_file(demo_filename));

        assert_eq!(res, 4);
    }
}
