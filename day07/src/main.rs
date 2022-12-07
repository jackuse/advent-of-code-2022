use regex::Regex;
use shared::read_file;

static DAY: &str = "day07/";
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
    let mut files_by_folders: Vec<Vec<u32>> = vec![];
    let mut folders = vec![];
    let mut current_path = vec![];
    let reg_cmd = Regex::new(r"^\$").unwrap();
    let reg_cd = Regex::new(r"\$ cd (.*)").unwrap();
    let reg_ls = Regex::new(r"\$ ls").unwrap();
    let reg_file = Regex::new(r"(\d+) (.*)").unwrap();

    for line in input {
        if reg_cmd.is_match(&line) {
            if reg_cd.is_match(&line) {
                let mut folder = "".to_string();
                for cap in reg_cd.captures_iter(&line) {
                    // println!("{:?}", &cap[1]);
                    folder = (&cap[1]).to_string();
                }
                if folder == ".." {
                    let pos = get_current_pos(&folders, &current_path);
                    let current_size: u32 = files_by_folders.get(pos).unwrap().iter().sum::<u32>();
                    current_path.pop();

                    let pos2 = get_current_pos(&folders, &current_path);
                    let lol = files_by_folders.get_mut(pos2).unwrap();
                    lol.push(current_size);
                } else {
                    current_path.push(folder.to_owned());
                    let path = current_path.join("/");
                    if !folders.contains(&path) {
                        folders.push(path);
                        files_by_folders.push(vec![]);
                    }
                }
                // println!("current_path: {:?}", current_path);
            } else if reg_ls.is_match(&line) {
            }
        } else {
            if reg_file.is_match(&line) {
                let mut size = 0;
                for cap in reg_file.captures_iter(&line) {
                    // println!("{:?}", &cap[1]);
                    size = (&cap[1]).to_string().parse::<u32>().unwrap();
                }

                let pos = get_current_pos(&folders, &current_path);
                let lol = files_by_folders.get_mut(pos).unwrap();
                lol.push(size);
            }
        }
    }

    let total_by_folder: Vec<u32> = files_by_folders.iter().map(|f| f.iter().sum()).collect();

    // println!("folders: {:?}", folders);
    // println!("files_by_folders: {:?}", files_by_folders);
    // println!("total_by_folder: {:?}", total_by_folder);

    total_by_folder.iter().filter(|&&s| s <= 100000).sum()
}

fn part2(input: Vec<String>) -> u32 {
    let mut total_by_folder: Vec<u32> = do_it(input);
    // println!("files_by_folders: {:?}", &total_by_folder);
    let available_space = 70000000;
    let min_free_space = 30000000;

    let used_space = *total_by_folder.get(0).unwrap();

    total_by_folder.sort();
    total_by_folder.pop();

    let mut res = 0;

    // println!("total_by_folder: {:?}", total_by_folder);
    // println!("available_space: {:?}", available_space);
    // println!("used_space:      {:?}", used_space);
    // println!("free:            {:?}", available_space - used_space);
    for folder in total_by_folder {
        if available_space - used_space > min_free_space - folder {
            res = folder;
            break;
        }
    }

    res
}

fn get_current_pos(folders: &Vec<String>, current_path: &Vec<String>) -> usize {
    match folders.iter().position(|f| *f == current_path.join("/")) {
        Some(v) => v,
        _ => 0,
    }
}

fn do_it(input: Vec<String>) -> Vec<u32> {
    let mut files_by_folders: Vec<Vec<u32>> = vec![];
    let mut folders = vec![];
    let mut current_path = vec![];
    let reg_cmd = Regex::new(r"^\$").unwrap();
    let reg_cd = Regex::new(r"\$ cd (.*)").unwrap();
    let reg_ls = Regex::new(r"\$ ls").unwrap();
    let reg_file = Regex::new(r"(\d+) (.*)").unwrap();

    for line in input {
        if reg_cmd.is_match(&line) {
            if reg_cd.is_match(&line) {
                let mut folder = "".to_string();
                for cap in reg_cd.captures_iter(&line) {
                    // println!("{:?}", &cap[1]);
                    folder = (&cap[1]).to_string();
                }
                if folder == ".." {
                    cd_back(&folders, &mut current_path, &mut files_by_folders);
                } else {
                    current_path.push(folder.to_owned());
                    let path = current_path.join("/");
                    if !folders.contains(&path) {
                        folders.push(path);
                        files_by_folders.push(vec![]);
                    }
                }
                // println!("current_path: {:?}", current_path);
            } else if reg_ls.is_match(&line) {
            }
        } else {
            if reg_file.is_match(&line) {
                let mut size = 0;
                for cap in reg_file.captures_iter(&line) {
                    // println!("{:?}", &cap[1]);
                    size = (&cap[1]).to_string().parse::<u32>().unwrap();
                }

                let pos = get_current_pos(&folders, &current_path);
                let lol = files_by_folders.get_mut(pos).unwrap();
                lol.push(size);
            }
        }
    }

    for a in current_path.clone() {
        if (a != "/") {
            cd_back(&folders, &mut current_path, &mut files_by_folders);
        }
    }

    let total_by_folder: Vec<u32> = files_by_folders.iter().map(|f| f.iter().sum()).collect();

    total_by_folder
}

fn cd_back(
    folders: &Vec<String>,
    current_path: &mut Vec<String>,
    files_by_folders: &mut Vec<Vec<u32>>,
) {
    let pos = get_current_pos(folders, current_path);
    let current_size: u32 = files_by_folders.get(pos).unwrap().iter().sum::<u32>();
    current_path.pop();

    let pos2 = get_current_pos(folders, current_path);
    let lol = files_by_folders.get_mut(pos2).unwrap();
    lol.push(current_size);
}

// struct FileSystem {
//     files_by_folders: vec![],
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_part1_demo() {
        let res = part1(read_file(DEMO_FILENAME));

        assert_eq!(res, 95437);
    }

    #[test]
    fn should_pass_part2_demo() {
        let res = part2(read_file(DEMO_FILENAME));

        assert_eq!(res, 24933642);
    }
}
