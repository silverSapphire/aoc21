use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    let filename = "input.txt";
    let balls, boards = load_input(filename);
}

fn load_input(filename: &str) -> Vec<u32>, (Vec<Vec<u32>> {
    let mut balls: Vec<u32> = Vec::new();
    let mut boards: Vec<Vec<u32>> = Vec::new();

    let f = fs::File::open(filename).expect("Unable to open file");
    let reader = io::BufReader::new(f);

    //Grab out the balls
    let mut lines_iter = reader.lines();
    let balls = lines_iter.next();
    println!("Balls are {}", balls);


    /*
    //Now grab out the boards
    for line in lines_iter {
        for word in line.unwrap().split(",").collect() {
            let ball = u32::from_str_radix(&co2[0], 2).unwrap();
            vec.push(n);
        }
    }
    */
    
    (balls, boards)
}
