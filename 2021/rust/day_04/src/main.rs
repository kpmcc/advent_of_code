use std::env;
use std::fs;
//use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Copy)]
struct Tile {
    called: bool,
    number: u8,
}

impl Tile {
    fn call(&mut self) -> bool {
        self.called = true;
        return true;
    }
}

const BOARD_ROW_WIDTH: usize = 5;

#[derive(Clone, Copy)]
struct Board {
    tiles: [[Tile; BOARD_ROW_WIDTH]; BOARD_ROW_WIDTH],
    has_won: bool,
}

impl Board {
    fn apply_number(&mut self, num: u8) -> bool {
        let mut rv = false;
        (0..BOARD_ROW_WIDTH).for_each(|i| {
            (0..BOARD_ROW_WIDTH).for_each(|j| {
                let t = &mut self.tiles[i][j];
                if t.number == num {
                    t.call();
                    rv = true;
                }
            });
        });
        return rv;
    }

    fn check_win(&self) -> bool {
        // Check for row win condition
        for i in 0..BOARD_ROW_WIDTH {
            //println!("\tChecking row condition");
            let mut winning = true;
            for j in 0..BOARD_ROW_WIDTH {
                let t = self.tiles[i][j];
                //println!("\t\t t: {} {}", t.number, t.called);
                winning = winning & self.tiles[i][j].called;
            }
            if winning {
                return winning;
            }
        }
        // Check for col win condition
        for j in 0..BOARD_ROW_WIDTH {
            let mut winning = true;
            for i in 0..BOARD_ROW_WIDTH {
                winning = winning & self.tiles[i][j].called;
            }
            if winning {
                return winning;
            }
        }

        return false;
    }

    fn calculate_score(&self) -> u16 {
        let mut score: u16 = 0;
        for i in 0..BOARD_ROW_WIDTH {
            for j in 0..BOARD_ROW_WIDTH {
                let t = self.tiles[i][j];
                if t.called == false {
                    score += t.number as u16;
                }
            }
        }
        return score;
    }
}

fn read_file<P>(filename: P) -> String
where
    P: AsRef<Path>,
{
    //let file = fs::File::open(filename);
    let contents = fs::read_to_string(filename).expect("Should have been able to open file");
    contents
}

fn populate_row_from_string(row: &mut [Tile; 5], vals: String) {
    let parsed: Vec<&str> = vals.split_ascii_whitespace().collect();

    for (i, p) in parsed.into_iter().enumerate() {
        //println!("Parsed: {}", p);
        let x = p.parse::<u8>().expect("Couldn't parse");
        row[i].number = x;
    }

    //println!("populating row");
    return;
}

fn get_board_from_string(s: String) -> Board {
    let mut board = Board {
        tiles: [[Tile {
            called: false,
            number: 0,
        }; BOARD_ROW_WIDTH]; BOARD_ROW_WIDTH],
        has_won: false,
    };
    //let rows: Vec<&str> = s.split("\n").collect();
    let rows = s.split("\n");
    for (i, r) in rows.enumerate() {
        if i < BOARD_ROW_WIDTH {
            populate_row_from_string(&mut board.tiles[i], r.to_string());
        }
    }
    return board;
}

fn main() {
    let file_contents = read_file("../advent_of_code/2021/input/04.txt");
    let v: Vec<&str> = file_contents.split("\n\n").collect();
    let nums = v[0];
    let nums: Vec<&str> = nums.split(',').collect();
    let mut parsed_nums: Vec<u8> = Vec::new();
    for n in nums {
        //println!("Trying to parse: {}", n);
        let p = n.parse::<u8>().expect("Couldn't parse num");
        parsed_nums.push(p);
    }
    let board_strings = &v[1..];

    let mut boards: Vec<Board> = Vec::new();

    for bs in board_strings {
        let board = get_board_from_string(bs.to_string());
        boards.push(board)
    }

    let mut winning_boards: Vec<Board> = Vec::new();
    let mut last_winning_num = 0;

    for n in parsed_nums {
        //println!("Processing {}", n);
        let mut board_index = 0;
        for b in &mut boards {
            //println!("\t Board {}", board_index);
            if !b.has_won {
                let board_has_number = b.apply_number(n);
                if board_has_number {
                    //println!("\t Has number!");
                    let has_win = b.check_win();
                    if has_win {
                        //println!("\t HAS WIN!");
                        //let board_score = b.calculate_score();
                        winning_boards.push(b.clone());
                        last_winning_num = n;
                        b.has_won = true;
                    }
                }
            }
            board_index += 1;
        }
    }

    if winning_boards.len() != 0 {
        let last_winner_score = winning_boards
            .pop()
            .expect("Tried to pop from empty")
            .calculate_score();
        let final_score = last_winner_score * last_winning_num as u16;
        println!("last Winning num : {}", last_winning_num);
        println!("last Winning Board score: {}", last_winner_score);
        println!("final score: {}", final_score);
    }
}
