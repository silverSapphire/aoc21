use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    let filename = "input.txt";
    let words = load_input(filename);

    let mut oxy = words.to_vec();
    let mut co2 = words.to_vec();

    //Find oxygen rating
    let mut bit_loc = 0;
    while oxy.len() > 1 {

        let (gamma, _) = tally(&oxy);
        oxy.retain(|x| x.chars().nth(bit_loc).unwrap() == gamma.chars().nth(bit_loc).unwrap());
        bit_loc = bit_loc + 1;
    }

    //Find co2 rating
    bit_loc = 0;
    while co2.len() > 1 {
        let (_, epsilon) = tally(&co2);
        co2.retain(|x| x.chars().nth(bit_loc).unwrap() == epsilon.chars().nth(bit_loc).unwrap());
        bit_loc = bit_loc + 1;
    }


    let oxy = u32::from_str_radix(&oxy[0], 2).unwrap();
    let co2 = u32::from_str_radix(&co2[0], 2).unwrap();

    println!("Life support rating is {}", oxy * co2);
}

fn tally(words: &Vec<String>) -> (String, String) {

    let len = words[0].len();

    let mut tallies = vec![0; len];

    for word in words {
        for (i, c) in word.chars().enumerate() {
            if c == '1' {
                tallies[i] = tallies[i] + 1; 
            }
        }
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();

    //Find majority bits
    for tally in &tallies {
        if (*tally as f32) >= (words.len() as f32) / 2.0 {
            gamma.push('1') ;
            epsilon.push('0');
        }
        else {
            gamma.push('0');
            epsilon.push('1') ;
        }
    }

    (gamma, epsilon)
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
