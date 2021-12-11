use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    let filename = "input.txt";
    let mut fish = load_input(filename);

    let mut num_new = 0;
    let days = 80;
    for _ in 0..days {
        for f in fish.iter_mut() {
            *f = *f - 1;
            if *f < 0 {
                *f = 6;
                num_new = num_new + 1;
            }
        }
        for _ in 0..num_new {
            fish.push(8);
        }
        num_new = 0;
    }

    println!("After {} days, we have {} fish", days, fish.len());
}

fn load_input(filename: &str) -> Vec<i32> {
    let f = fs::File::open(filename).expect("Unable to open file");
    let reader = io::BufReader::new(f);

    let mut fish: Vec<i32> = Vec::new();

    for line in reader.lines() {
        fish = line
            .unwrap()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
    }

    fish
}
