use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    let filename = "input.txt";
    let points = load_input(filename);


}

fn plot_lines(points: Vec<((u32, u32), (u32, u32))>) {

    for point in points {
        if point.0.0 != point.1.0 && point.0.1 != point.1.1 {
            continue;
        }

        
    }
}

fn load_input(filename: &str) -> Vec<((u32, u32), (u32, u32))> {

    let f = fs::File::open(filename).expect("Unable to open file");
    let reader = io::BufReader::new(f);

    let mut points: Vec<((u32, u32), (u32, u32))> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut words: Vec<&str> = line.split_whitespace().collect();
        words.remove(1); //remove the arrow

        let start_nums: Vec<u32> = words[0].split(',').map(|x| x.parse::<u32>().unwrap()).collect();
        let end_nums: Vec<u32> = words[1].split(',').map(|x| x.parse::<u32>().unwrap()).collect();
        points.push(((start_nums[0], start_nums[1]), (end_nums[0], end_nums[1])));
    }

    points
}
