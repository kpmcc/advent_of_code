use std::fs;
use std::path::Path;

const GRID_SIZE: usize = 1000;

struct Coord {
    x: u16,
    y: u16,
}

fn valid_segment(a: &Coord, b: &Coord) -> bool {
    return a.x == b.x || a.y == b.y;
}

fn parse_coord(s: String) -> Coord {
    let vals: Vec<&str> = s.split(",").collect();
    assert!(vals.len() == 2, "parse_coord: vals len not 2");
    let x = vals[0]
        .to_string()
        .parse::<u16>()
        .expect("parse_coord: couldn't parse x into u16");
    let y = vals[1]
        .parse::<u16>()
        .expect("parse_coord: couldn't parse y into u16");
    let c = Coord { x, y };
    return c;
}

fn parse_segment(s: String) -> (Coord, Coord) {
    let coords: Vec<&str> = s.split(" -> ").collect();
    //println!("s: {} coords len{}", s, coords.len());
    assert!(coords.len() == 2, "Coords len not 2");
    let a: Coord = parse_coord(coords[0].to_string());
    let b: Coord = parse_coord(coords[1].to_string());
    return (a, b);
}

fn read_file<P>(filename: P) -> String
where
    P: AsRef<Path>,
{
    let contents = fs::read_to_string(filename).expect("Should have been able to open file");
    contents
}

fn apply_segment(grid: &mut [[u16; GRID_SIZE]; GRID_SIZE], a: Coord, b: Coord) -> u16 {
    let mut new_danger_points = 0;
    if a.x == b.x {
        let start: &Coord;
        let stop: &Coord;
        if a.y < b.y {
            start = &a;
            stop = &b;
        } else {
            start = &b;
            stop = &a;
        }
        for i in start.y..(stop.y + 1) {
            let v = grid[i as usize][a.x as usize];
            grid[i as usize][a.x as usize] = v + 1;
            if v == 1 {
                new_danger_points += 1;
            }
        }
    }

    if a.y == b.y {
        let start: &Coord;
        let stop: &Coord;
        if a.x < b.x {
            start = &a;
            stop = &b;
        } else {
            start = &b;
            stop = &a;
        }
        for i in start.x..(stop.x + 1) {
            let v = grid[a.y as usize][i as usize];
            grid[a.y as usize][i as usize] = v + 1;
            if v == 1 {
                new_danger_points += 1;
            }
        }
    }

    return new_danger_points;
}

fn main() {
    let file_contents = read_file("../advent_of_code/2021/input/05.txt");
    let lines: Vec<&str> = file_contents.split("\n").collect();
    let mut valid_segments: Vec<(Coord, Coord)> = Vec::new();
    let num_lines = lines.len();
    for l in lines {
        if l != "" {
            let (a, b) = parse_segment(l.to_string());
            if valid_segment(&a, &b) {
                valid_segments.push((a, b))
            }
        }
    }
    let l = valid_segments.len();
    let mut grid: [[u16; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];
    let mut num_danger_points = 0;

    for v in valid_segments {
        let (a, b) = v;
        num_danger_points += apply_segment(&mut grid, a, b);
    }

    //for i in 0..GRID_SIZE {
    //    for j in 0..GRID_SIZE {
    //        let x = grid[i][j];
    //        if x == 0 {
    //            print!(".");
    //        } else {
    //            print!("{}", x);
    //        }
    //    }
    //    println!("");
    //}

    println!("num lines: {}, num valid segments: {}", num_lines, l);
    println!("num danger points: {}", num_danger_points);
}
