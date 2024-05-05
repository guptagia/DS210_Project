use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub type ListOfEdges = Vec<(usize, usize)>;

pub fn read_file() -> Result<ListOfEdges, Error> {
    let path = "/Users/giagupta/Downloads/roadNet-CA.txt";  // File path
    let max_index = 1965206;  // Maximum valid index

    let mut result = ListOfEdges::new();
    let file = File::open(path)?;  // Handle potential file read errors
    let buf_reader = BufReader::new(file).lines();

    for line in buf_reader {
        let line_str = line.expect("Error reading line");

        if line_str.starts_with('#') {  // Skip comments
            continue;
        }

        let parts: Vec<&str> = line_str.trim().split_whitespace().collect();
        if parts.len() == 2 {
            let x = parts[0].parse::<usize>().unwrap();
            let y = parts[1].parse::<usize>().unwrap();

            // Ensure indices are within the valid range
            if x < max_index && y < max_index {
                result.push((x, y));
            } else {
                // Ignore invalid indices
                continue;
            }
        }
    }

    Ok(result)
}
