use std::fs::File;
use std::io::{self, BufRead, BufReader, Error};
use std::str::FromStr;
use rayon::prelude::*;

pub type ListOfEdges = Vec<(usize, usize)>;

pub fn read_file(path: &str) -> Result<ListOfEdges, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file).lines();
    let mut edges = ListOfEdges::new();

    for line_result in reader.skip(4) { 
        let line = line_result?;
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if let (Ok(u), Ok(v)) = (usize::from_str(parts[0]), usize::from_str(parts[1])) {
            edges.push((u, v));
        } else {
            return Err(Error::new(io::ErrorKind::InvalidData, "Invalid line format"));
        }
    }

    Ok(edges)
}
pub fn find_max_index_parallel(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines: Vec<_> = reader.lines().collect::<Result<_, _>>()?;
 
    let max_index = lines.par_iter()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                let u = parts[0].parse::<usize>().ok()?;
                let v = parts[1].parse::<usize>().ok()?;
                Some(u.max(v))
            } else {
                None
            }
        })
        .max()
        .unwrap_or(0);

    Ok(max_index)
}