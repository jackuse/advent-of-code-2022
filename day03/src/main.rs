use shared::read_file;

static DAY: &str = "day03/";
static DEMO_FILENAME: &str = "src/input_demo.txt";
static PUZZLE_FILENAME: &str = "src/input.txt";

fn main() {
    let data = read_file((DAY.to_owned() + PUZZLE_FILENAME).as_str());

    let res = part1(data.clone());
    println!("PART 1 : {}", res);

    let res = part2(data.clone());
    println!("PART 2 : {}", res);
}

fn part1(input: Vec<String>) -> u32 {
    let mut res = 0;
    for s in input {
        let compt = to_compartments(s);
        let ch = found_common_char(compt.0, compt.1);
        res = res + to_priority(ch);
    }

    res
}

fn part2(input: Vec<String>) -> u32 {
    let groups = create_groups(input);
    let mut res = 0;
    for group in groups {
        let one = &*group.get(0).unwrap();
        let two = &*group.get(1).unwrap();
        let three = &*group.get(2).unwrap();
        let c1 = found_common_chars_in_str(one.to_string(), two.to_string());
        let c2 = found_common_chars_in_str(two.to_string(), three.to_string());
        let ch = found_common_char(c1.to_string(), c2.to_string());
        // println!("-----------");
        // println!("{:?}", one);
        // println!("{:?}", two);
        // println!("{:?}", three);
        // println!("{:?}", c1);
        // println!("{:?}", c2);
        // println!("{:?}", ch);
        res = res + to_priority(ch);
    }

    res
}

fn create_groups(input: Vec<String>) -> Vec<Vec<String>> {
    let mut groups = vec![];
    let mut cpt = 0;
    let mut cur_group = vec![];
    for s in input {
        if cpt >= 3 {
            groups.push(cur_group);
            cur_group = vec![];
            cpt = 0
        }
        cur_group.push(s);
        cpt = cpt + 1;
    }
    groups.push(cur_group);

    groups
}

fn found_common_char(s1: String, s2: String) -> char {
    let mut found = ' ';
    for c in s1.chars() {
        if s2.contains(c) {
            found = c;
        }
    }
    found
}

fn found_common_chars_in_str(s1: String, s2: String) -> String {
    let mut found = "".to_string();
    for c in s1.chars() {
        if s2.contains(c) {
            found.push(c)
        }
    }
    found
}

fn to_compartments(rucksacks: String) -> (String, String) {
    let size = rucksacks.len() / 2;
    let a = &rucksacks[0..size].to_string();
    let b = &rucksacks[size..rucksacks.len()].to_string();

    (a.to_string(), b.to_string())
}

fn to_priority(ch: char) -> u32 {
    let value = ch as u32;
    let mut shift = 38;
    if ch.is_lowercase() {
        shift = 96
    }

    value - shift
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_part1_demo() {
        let res = part1(read_file(DEMO_FILENAME));

        assert_eq!(res, 157);
    }

    // #[test]
    fn should_pass_part2_demo() {
        let res = part2(read_file(DEMO_FILENAME));

        assert_eq!(res, 70);
    }
}
