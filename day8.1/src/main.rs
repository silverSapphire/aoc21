use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    let filename = "input.txt";
    let (patterns, digits) = load_input(filename);

    let unique_lens: [usize; 4] = [2, 4, 3, 7];

    let mut num_unique = 0;
    for line in digits {
        num_unique = num_unique
            + line
                .iter()
                .filter(|&x| unique_lens.contains(&x.len()))
                .count();
    }

    println!("We have {} digits of unique length", num_unique);
}

fn load_input(filename: &str) -> (Vec<Vec<String>>, Vec<Vec<String>>) {
    let mut patterns: Vec<Vec<String>> = Vec::new();
    let mut digits: Vec<Vec<String>> = Vec::new();

    let f = fs::File::open(filename).expect("Unable to open file");
    let reader = io::BufReader::new(f);

    for line in reader.lines() {
        let line: Vec<String> = line.unwrap().split('|').map(str::to_string).collect();
        let pattern: Vec<String> = line[0].split_whitespace().map(str::to_string).collect();
        let digit: Vec<String> = line[1].split_whitespace().map(str::to_string).collect();

        patterns.push(pattern);
        digits.push(digit);
    }

    (patterns, digits)
}
