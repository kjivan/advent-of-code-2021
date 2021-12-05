use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        let (mut x, mut y) = (0, 0);
        for line in lines {
            let command = line.unwrap();
            let mut command = command.split(' ');
            match command.next().unwrap() {
                "forward" =>  x += command.next().unwrap().parse::<i32>().unwrap(),
                "up" =>  y -= command.next().unwrap().parse::<i32>().unwrap(),
                "down" =>  y += command.next().unwrap().parse::<i32>().unwrap(),
                _ => panic!()
            }
        }
        dbg!(x*y);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
