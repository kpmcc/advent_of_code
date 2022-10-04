use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), std::num::ParseIntError> {
    if let Ok(lines) = read_lines("../../../input/03.txt") {
        let mut gamma_rate = 0;
        let mut epsilon_rate = 0;
        let mut counts = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut num_lines = 0;
        for line in lines {
            if let Ok(ip) = line {
                //println!("Parsed line: {}", ip);
                for (i, x) in ip.chars().enumerate() {
                    if x == '1' {
                        //println!("got 1");
                        counts[i] += 1;
                    } else {
                        //println!("got 0");
                    }
                }
            }
            num_lines += 1;
        }
        for count in counts {
            gamma_rate = gamma_rate << 1;
            epsilon_rate = epsilon_rate << 1;
            if count >= (num_lines / 2) {
                gamma_rate += 1;
            } else {
                epsilon_rate += 1;
            }
        }
        let power_consumption = gamma_rate * epsilon_rate;
        println!("gamma rate: {}", gamma_rate);
        println!("epsilon rate: {}", epsilon_rate);
        println!("Power consumption: {}", power_consumption);
    } else {
        println!("Failed to read input")
    }
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
