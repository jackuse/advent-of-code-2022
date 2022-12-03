use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file(filename: &str) -> Vec<String> {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut res = vec![];

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
                                  // Show the line and its number.
                                  // println!("{}. {}", index + 1, line);
        res.push(line)
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_file() {
        let filename = "src/report.txt";
        let res_str = vec![
            "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
        ];
        let res = res_str
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        assert_eq!(read_file(filename), res);
    }
}
