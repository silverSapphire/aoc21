use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashSet;

fn main() {
    let filename = "input.txt";
    let (balls, mut boards, mut sums) = load_input(filename);

    for ball in balls {
        let sum = mark_boards(&ball, &mut boards, &mut sums);
        if sum != -1 {
            println!("A board won on ball {} with sum {}", ball, sum);
            println!("Thus, the answer is {}", ball as i32 * sum);
            break;
        }
    }
}

/*
 * Returns the sum of the winning board
 */
fn mark_boards(ball: &u32, boards: &mut Vec<Vec<HashSet<u32>>>, sums: &mut Vec<u32>) -> i32 {

    for (board_i, board) in boards.iter_mut().enumerate() {
        let mut removed = false;
        let mut winner = false;
        for strip in board {
            if strip.remove(ball) {
                removed = true;
            }
            if strip.len() == 0 {
                winner = true;
                break;
            }
        }

        //Decrease the board sum
        if removed {
            sums[board_i] = sums[board_i] - ball;
        }

        //Winner
        if winner {
            return sums[board_i] as i32;
        }
    }

    -1
}

fn load_input(filename: &str) -> (Vec<u32>, Vec<Vec<HashSet<u32>>>, Vec<u32>) {
    let mut balls: Vec<u32> = Vec::new();
    let mut boards: Vec<Vec<HashSet<u32>>> = Vec::new();
    let mut sums: Vec<u32> = Vec::new();

    let f = fs::File::open(filename).expect("Unable to open file");
    let reader = io::BufReader::new(f);

    //Grab out the balls
    let mut lines_iter = reader.lines();
    if let Some(s) = lines_iter.next() {
        let b: Vec<&str> = s.as_ref().unwrap().split(",").collect();
        balls = b.iter().map(|x| x.parse::<u32>().unwrap()).collect();
    }

    let mut board: Vec<HashSet<u32>> = Vec::new();
    let mut init = false;
    let mut sum = 0;
    for line in lines_iter.skip(1) {
        let line = line.unwrap();
        if line.is_empty()  {
            boards.push(board);
            board = Vec::new();
            sums.push(sum);
            sum = 0;

            init = false;
            continue;
        }

        let line_numbers: Vec<u32> = line
            .split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

        //Construct the columns
        if !init {
            for _ in 0..line_numbers.len() {
                board.push(HashSet::new());
            }
            init = true;
        }

        //Add to the columns
        for (i, n) in line_numbers.iter().enumerate() {
            board[i].insert(*n);
            sum = sum + n;
        }

        //Add the row
        board.push(HashSet::from_iter(line_numbers));
    }

    boards.push(board);
    sums.push(sum);

    (balls, boards, sums)
}
