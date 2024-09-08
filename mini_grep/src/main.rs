use std::io::{self, BufRead};

fn main() {
    let pattern_string = std::env::args()
        .nth(1)
        .expect("missing required command-line argument: <pattern>");

    let pattern = &pattern_string;

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.expect("failed to read line");
        
        if line.contains(pattern) {
            println!("{}", line);
        }
    }
}
