//use std::fs;
//
//fn main() {
//    let file_name = "../../input/01.txt";
//    println!("Reading the file: {}", file_name);
//    let file_content = fs::read_to_string(file_name)
//        .expect("Failed to read the file");
//    println!("\nFile contents:\n--------------\n{}\n", file_content)
//}
//
//
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::num::ParseIntError;
use std::collections::VecDeque;

fn main() -> Result<(), std::num::ParseIntError> {
    let f = File::open("../../input/01.txt");
    let f = f.unwrap();
    let f = BufReader::new(f);

    // Depth Increases with window size of 1
    //let mut depth_increases = -1;
    //let mut previous_depth = 0;
    //for line in f.lines() {
    //    let number_str = line.unwrap();
    //    let number = match number_str.parse::<i32>() {
    //        Ok(number) => number, 
    //        Err(e) => return Err(e),
    //    };
    //    if number > previous_depth {
    //        depth_increases += 1;
    //    }
    //    previous_depth = number;
    //    //println!("{}", number);
    //}
    //
    let mut depth_increases = -3;
    let mut previous_depth = 0;
    let mut new_depth = 0;
    let mut v = VecDeque::from([0, 0, 0]);
    for line in f.lines() {
        let number_str = line.unwrap();
        let number = match number_str.parse::<i32>() {
            Ok(number) => number, 
            Err(e) => return Err(e),
        };

        new_depth = previous_depth;
        let oldest_depth = v.pop_front()
            .expect("tried to pop from empty VecDeque");
        new_depth -= oldest_depth;
        new_depth += number;
        v.push_back(number);

        if new_depth > previous_depth {
            depth_increases += 1;
        }
        previous_depth = new_depth;
        //println!("{}", number);
    }
 
    println!("Depth increases {}", depth_increases);
    Ok(())
}


//use std::num::ParseIntError;
//
//fn main() -> Result<(), ParseIntError> {
//    let number_str = "10";
//    let number = match number_str.parse::<i32>() {
//        Ok(number)  => number,
//        Err(e) => return Err(e),
//    };
//    println!("{}", number);
//    Ok(())
//}
