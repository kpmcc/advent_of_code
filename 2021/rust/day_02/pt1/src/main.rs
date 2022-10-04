use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::num::ParseIntError;

fn main() -> Result<(), std::num::ParseIntError> {
    if let Ok(lines) = read_lines("../../input/02.txt") {
        let mut x_pos = 0;
        let mut y_pos = 0;
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let v: Vec<&str> = ip.split(' ').collect();
                assert_eq!(v.len(), 2);
                let direction = v[0];
                let magnitude_string = v[1];
                let magnitude_num = match magnitude_string.parse::<i32>() {
                    Ok(number) => number,
                    Err(e) => return Err(e),
                };                
                match direction {
                    "forward" => x_pos += magnitude_num,
                    "up" => y_pos -= magnitude_num,
                    "down" => y_pos += magnitude_num,
                    &_ => todo!(),
                }

                //println!("{}", ip);
            }
        }

        let final_pos = x_pos * y_pos;
        println!("final position {}", final_pos);
    }
    Ok(())
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
