use std::fs;
use std::io;
use std::io::BufRead;

const LEN: usize = 9;

fn main() {
    let filename = "input.txt";
    let mut fish = load_input(filename);

    let days = 256;
    for _ in 0..days {
        let just_spawned = fish[0];

        //Decrement all the timers
        for i in 1..LEN {
            fish[i - 1] = fish[i];
        }

        fish[6] = fish[6] + just_spawned;
        fish[8] = just_spawned;
    }

    let sum: u64 = fish.iter().sum();
    println!("After {} days, we have {} fish", days, sum);
}

fn load_input(filename: &str) -> [u64; LEN] {
    let f = fs::File::open(filename).expect("Unable to open file");
    let reader = io::BufReader::new(f);

    let mut fish = [0; LEN];
    let mut nums: Vec<usize> = Vec::new();

    for line in reader.lines() {
        nums = line
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
    }

    for num in nums {
        fish[num] = fish[num] + 1;
    }

    fish
}
