use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let filename = "input.txt";
    let points = load_input(filename);
    let map = plot_lines(points);

    //Find danger points
    let mut num_danger = 0;
    for (_, &count) in &map {
        if count >= 2 {
            num_danger = num_danger + 1;
        }
    }

    println!("There are {} danger points", num_danger);
}

fn plot_lines(points: Vec<((u32, u32), (u32, u32))>) -> HashMap<(u32, u32), u32> {

    let mut map: HashMap<(u32, u32), u32> = HashMap::new();

    for point in points {
        let x1 = point.0.0;
        let x2 = point.1.0;
        let y1 = point.0.1;
        let y2 = point.1.1;

        //Diagonal
        if x1 != x2 && y1 != y2 {
            //If we have positive slope, we add to the starting point
            let y_increment = (y2 as i32 - y1 as i32) / (x2 as i32 - x1 as i32) > 0; 
            let start_point = if x1 < x2 {point.0} else {point.1};
            let end_point = if x2 < x1 {point.0} else {point.1};

            let mut x = start_point.0;
            let mut y = start_point.1;
            for _ in start_point.0..end_point.0 + 1 {
                let new_point = (x, y);
                let count = map.get(&new_point).unwrap_or(&0).clone();
                map.insert(new_point, count+1);

                x = x + 1;
                y = if y_increment {y + 1} else {y - 1};
            }

        }
        //Vertical
        else if x1 == x2 {
            let lower = if y1 < y2 {y1} else {y2};
            let higher = if y1 < y2 {y2} else {y1};

            for i in lower..higher+1 {
                let new_point = (x1, i);
                let count = map.get(&new_point).unwrap_or(&0).clone();
                map.insert(new_point, count+1);
            }
        }
        //Horizontal
        else {
            let lower = if x1 < x2 {x1} else {x2};
            let higher = if x1 < x2 {x2} else {x1};

            for i in lower..higher+1 {
                let new_point = (i, y1);
                let count = map.get(&new_point).unwrap_or(&0).clone();
                map.insert(new_point, count+1);
            }
        }
    }

    map
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
