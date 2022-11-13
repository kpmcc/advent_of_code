use std::fs;
use std::path::Path;

fn read_file<P>(filename: P) -> String
where
    P: AsRef<Path>,
{
    let contents = fs::read_to_string(filename).expect("Should have been able to open file");
    contents
}

fn distance(a: i64, b: i64) -> i64 {
    let x = a - b;
    x.abs()
}

fn main() {

    let file_contents = read_file("../../../input/07.txt");
    let lines: Vec<&str> = file_contents.split("\n").collect();
    let mut crabs: Vec<i64> = Vec::new();
    let mut furthest_crab = 0;
    //let _positions: Vec<&str> = lines[0].split(",").collect();
    for p in lines[0].split(",") {
        //println!("reading: {}", p);
        let x = p
            .to_string()
            .parse::<i64>()
            .expect("Could not parse into i64");
        crabs.push(x);
        if x > furthest_crab {
            furthest_crab = x;
        }
    }

    let mut costs: Vec<i64> = Vec::new();
    costs.push(0);
    for i in 1..(furthest_crab+1) {
        let new_cost = i + costs[(i-1) as usize];
        costs.push(new_cost);
    }
    
    let mut min_distance = std::i64::MAX;
    let mut optimal_position = 0;
    for d in 0..furthest_crab {
        let mut curr_distance = 0;
        for i in 0..crabs.len() {
            let a = crabs[i];
            curr_distance += costs[distance(a, d) as usize];
        }
        if curr_distance < min_distance {
            min_distance = curr_distance;
            optimal_position = d;
        }
    }
    println!("Optimal position: {}, min distance: {}", optimal_position, min_distance);
}
