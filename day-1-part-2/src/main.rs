use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        let lines: Vec<i32> = lines.map(|x| x.unwrap().parse::<i32>().unwrap()).collect();
        let mut increased = 0;
        for i in 3..lines.len() {
            if lines[i-2] + lines[i-1] + lines[i] > 
            lines[i-3] + lines[i-2] + lines[i-1] {
                increased += 1;
            }
        }
        println!("{}", increased);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
