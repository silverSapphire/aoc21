use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    let filename = "input1a.txt";
    let numbers: Vec<i32> = load_input(filename);

    let mut num_inc = 0;
    for i in 3..numbers.len() {
        let diff = numbers[i] - numbers[i-3];
        if diff > 0 {
            num_inc = num_inc + 1;
        }
    }

    println!("We increased {} times", num_inc);
}

fn load_input(filename: &str) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();

    let f = fs::File::open(filename).expect("Unable to open file");
    let reader = io::BufReader::new(f);

	for line in reader.lines() {
		for word in line.unwrap().split_whitespace() {
            let n = word.parse::<i32>().unwrap();
            vec.push(n);
		}
	}
    vec
}
