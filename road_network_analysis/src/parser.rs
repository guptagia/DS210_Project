use std::fs::File;
use std::io::{BufRead, BufReader};

pub type ListOfEdges = Vec<(usize, usize)>;

pub fn read_file() -> ListOfEdges {
    let file_path = "./data/roadNet-CA.txt"; 
    let mut result = ListOfEdges::new();
    
    let file = File::open(file_path).expect(&format!("Could not open file: {}", file_path));
    let buf_reader = BufReader::new(file).lines();
    
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        if line_str.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line_str.trim().split_whitespace().collect();
        if parts.len() == 2 {
            let x = parts[0].parse::<usize>().unwrap();
            let y = parts[1].parse::<usize>().unwrap();
            result.push((x, y));
        }
    }
    
    result
}
