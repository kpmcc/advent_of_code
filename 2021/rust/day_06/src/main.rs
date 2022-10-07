use std::fs;
use std::path::Path;

fn accumulate_fish(fish: &mut [u64; 9], num_iterations: u16) -> u64 {
    for _ in 0..num_iterations {
        let new_fish = fish[0];
        fish[0] = fish[1];
        fish[1] = fish[2];
        fish[2] = fish[3];
        fish[3] = fish[4];
        fish[4] = fish[5];
        fish[5] = fish[6];
        fish[6] = fish[7];
        fish[7] = fish[8];
        fish[8] = new_fish;
        fish[6] += new_fish;
    }
    let mut num_fish: u64 = 0;
    for f in fish {
        num_fish += *f as u64;
    }
    return num_fish;
}

fn read_file<P>(filename: P) -> String
where
    P: AsRef<Path>,
{
    let contents = fs::read_to_string(filename).expect("Should have been able to open file");
    contents
}

fn main() {
    let mut fish: [u64; 9] = [0; 9];

    let file_contents = read_file("../advent_of_code/2021/input/06.txt");
    let lines: Vec<&str> = file_contents.split("\n").collect();
    let l = lines[0];
    let nums: Vec<&str> = l.split(",").collect();
    for n in nums {
        let x = n
            .to_string()
            .parse::<u8>()
            .expect("couldn't parse num into u8");
        fish[x as usize] += 1;
    }

    let num_fish = accumulate_fish(&mut fish, 256);
    println!("Num fish: {}", num_fish);
}
