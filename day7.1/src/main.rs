use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    let filename = "input.txt";
    let positions = load_input(filename);
    let min = positions.iter().min().unwrap().clone();
    let max = positions.iter().max().unwrap().clone();
    let mut targets: Vec<i32> = Vec::new();
    let mut costs: Vec<i32> = Vec::new();

    for target in min..(max + 1) {
        let mut diffs: Vec<i32> = Vec::new();
        for &p in &positions {
            diffs.push((p - target).abs());
        }
        costs.push(diffs.iter().sum());
        targets.push(target);
    }

    let mut min_cost = costs[0];
    let mut min_target = targets[0];
    for (i, &cost) in costs.iter().enumerate() {
        if cost < min_cost {
            min_cost = cost;
            min_target = targets[i];
        }
    }

    println!(
        "The minimum amount of fuel aligns us at {} spending {}",
        min_target, min_cost
    );
}

fn load_input(filename: &str) -> Vec<i32> {
    let f = fs::File::open(filename).expect("Unable to open file");
    let reader = io::BufReader::new(f);

    let mut positions: Vec<i32> = Vec::new();

    for line in reader.lines() {
        positions = line
            .unwrap()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
    }

    positions
}
