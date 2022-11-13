use std::fs;
use std::path::Path;

struct Pair<'a> {
    seq: &'a str,
    output: &'a str,
}

fn read_file<P>(filename: P) -> String
where
    P: AsRef<Path>,
{
    let contents = fs::read_to_string(filename).expect("Should have been able to open file");
    contents
}

fn count_unique(entries: Vec<Pair>) -> u64 {
    let mut unique = 0;
    for e in entries {
        let nums: Vec<&str> = e.output.split(" ").collect();
        for n in nums {
            if matches!(n.len(), 2 | 3 | 4 | 7) {
                unique += 1;
            }
        }
    }
    unique
}

fn main() {
    let file_contents = read_file("../../../input/08.txt");

    let lines = file_contents.split("\n");
    let mut entries: Vec<Pair> = Vec::new();
    for l in lines {
        let e: Vec<&str> = l.split("|").collect();
        if e.len() == 2 {
            let p = Pair {
                seq: e[0],
                output: e[1],
            };
            entries.push(p);
        }
    }

    let unique = count_unique(entries);

    println!("Number of unique output strings: {}", unique);
}
