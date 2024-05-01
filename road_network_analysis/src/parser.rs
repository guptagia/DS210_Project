use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

type ListOfEdges = Vec<(usize, usize)>;

// Read a list of edges from a file
pub fn read_file(path: &str) -> ListOfEdges {
    let mut result = ListOfEdges::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        if line_str.starts_with('#') {
            continue; // Skip comment lines
        }
        let v: Vec<&str> = line_str.trim().split_whitespace().collect();
        if v.len() == 2 {
            let x = v[0].parse::<usize>().unwrap();
            let y = v[1].parse::<usize>().unwrap();
            result.push((x, y));
        }
    }
    result
}
