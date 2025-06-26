use longest_path_solver::parse_input;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut input = String::new();
    for line in stdin.lock().lines() {
        input.push_str(&line?);
        input.push('\n');
    }
    let graph = parse_input(&input)?;
    let (path, _distance) = graph.find_longest_path();
    for node in path {
        print!("{}\r\n", node);
    }
    Ok(())
}
