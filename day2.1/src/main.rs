use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    let filename = "input.txt";
    let (dirs, nums) = load_input(filename);

    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for i in 0..dirs.len() {
        let dir = &dirs[i];
        let num = &nums[i];

        match dir.as_str() {
            "forward" => {
                pos = pos + num; 
                depth = depth + aim * num;
            },
            "down" => aim = aim + num,
            "up" => aim = aim - num,
            _ => panic!("Bad dir"),
        };
    }

    println!("We ended up at position {} and depth {} for an answer of {}", pos, depth, pos * depth);
}

fn load_input(filename: &str) -> (Vec<String>, Vec<i32>) {
    let mut num_vec: Vec<i32> = Vec::new();
    let mut dir_vec: Vec<String> = Vec::new();

    let f = fs::File::open(filename).expect("Unable to open file");
    let reader = io::BufReader::new(f);

    for line in reader.lines() {
        let res: Vec<String> = line.unwrap().split(" ").map(|s| s.to_string()).collect();

        dir_vec.push(res[0].clone());
        num_vec.push(res[1].parse::<i32>().unwrap());
    }

    (dir_vec, num_vec)
}
