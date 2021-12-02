use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        let mut prev = 0;
        let mut increased_count = 0;
        for line in lines {
            if let Ok(ip) = line {
                if prev != 0 && ip.parse::<i32>().unwrap() > prev{
                    increased_count += 1;
                }
                prev = ip.parse::<i32>().unwrap()
            }
        }
        println!("increased_count: {}", increased_count);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
