use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    let filename = "input.txt";
    let words = load_input(filename);
    let len = words[0].len();

    let mut tallies = vec![0; len];

    for word in &words {
        for (i, c) in word.chars().enumerate() {
            if c == '1' {
                tallies[i] = tallies[i] + 1; 
            }
        }
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();
    for tally in &tallies {
        if tally > &(words.len() / 2) {
            gamma.push('1') ;
            epsilon.push('0');
        }
        else {
            gamma.push('0');
            epsilon.push('1') ;
        }
    }

    println!("Gamma is {}", gamma);
    println!("Epsilon is {}", epsilon);

    let gamma = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon, 2).unwrap();

    println!("Power consumption is {}", gamma * epsilon);
}

fn load_input(filename: &str) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();

    let f = fs::File::open(filename).expect("Unable to open file");
    let reader = io::BufReader::new(f);

    for line in reader.lines() {
		for word in line.unwrap().split_whitespace() {
            let n = word.clone().to_string();
            vec.push(n);
		}
    }

    vec
}
