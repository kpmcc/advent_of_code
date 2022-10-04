use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::num::ParseIntError;


fn main() -> Result<(), std::num::ParseIntError> {
    let mut x_pos = 0;
    let mut aim = 0;
    let mut depth = 0;
    if let Ok(lines) = read_lines("../../../input/02.txt") {
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
                    "forward" => {
                        x_pos += magnitude_num;
                        depth += aim * magnitude_num;
                    },
                    "up" => aim -= magnitude_num,
                    "down" => aim += magnitude_num,
                    &_ => todo!(),
                }
            }
        }
    }
    let final_pos = x_pos * depth;
    println!("final_pos: {}", final_pos);
    Ok(())
}

fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
