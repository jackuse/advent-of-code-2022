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

fn part1(input: Vec<String>) -> u32 {
    let elves_items = split_by_space(input)
        .into_iter()
        .map(|s| vec_string_to_u32(s))
        .collect::<Vec<Vec<u32>>>();
    let elves_calories = elves_items
        .into_iter()
        .map(|e| sum(e))
        .collect::<Vec<u32>>();

    let max_colories = elves_calories
        .into_iter()
        .reduce(|acc, n| if acc > n { acc } else { n });

    let res = match max_colories {
        Some(v) => v,
        None => 0,
    };

    res
}

fn part2(input: Vec<String>) -> u32 {
    let elves_items = split_by_space(input)
        .into_iter()
        .map(|s| vec_string_to_u32(s))
        .collect::<Vec<Vec<u32>>>();
    let mut elves_calories = elves_items
        .into_iter()
        .map(|e| {
            let calories = e.into_iter().reduce(|acc, c| acc + c);
            match calories {
                Some(v) => v,
                None => 0,
            }
        })
        .collect::<Vec<u32>>();
    elves_calories.sort_by(|a, b| b.cmp(a));
    let top_3 = &elves_calories[0..3];

    sum(top_3.to_vec())
}
fn sum(vec: Vec<u32>) -> u32 {
    let calories = vec.into_iter().reduce(|acc, c| acc + c);
    match calories {
        Some(v) => v,
        None => 0,
    }
}

fn vec_string_to_u32(vector: Vec<String>) -> Vec<u32> {
    vector
        .into_iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn split_by_space(vector: Vec<String>) -> Vec<Vec<String>> {
    let mut res = vec![];
    let mut cur = vec![];
    vector.into_iter().for_each(|s| {
        if s.len() > 0 {
            cur.push(s);
        } else {
            res.push(cur.clone());
            cur = vec![];
        }
    });

    if cur.len() > 0 {
        res.push(cur.clone());
    }

    res
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
