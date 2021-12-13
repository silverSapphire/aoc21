use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    let filename = "input.txt";
    let (patterns, digits) = load_input(filename);

    let mut sum = 0;
    for (i, p) in patterns.iter().enumerate() {
        let translations = translate(&p);

        let mut output = String::new();
        for d in &digits[i] {
            for (key, val) in &translations {
                if val.eq(&d) {
                    output.push_str(key);
                }
            }
        }

        let number = output.parse::<u32>().unwrap();
        sum = sum + number;
    }
}

fn translate(p: &Vec<String>) -> HashMap<&str, &str> {
    let mut translations = HashMap::new();
    translations.insert("1", p.iter().find(|&x| x.len() == 2).unwrap().as_str());
    translations.insert("4", p.iter().find(|&x| x.len() == 4).unwrap().as_str());
    translations.insert("8", p.iter().find(|&x| x.len() == 7).unwrap().as_str());
    translations.insert("7", p.iter().find(|&x| x.len() == 3).unwrap().as_str());
    translations.insert(
        "9",
        p.iter()
            .find(|&x| {
                (translations.get("4").unwrap())
                    .chars()
                    .all(|c| x.contains(c))
                    && x.len() == 6
            })
            .expect("9 not found")
            .as_str(),
    );
    translations.insert(
        "0",
        p.iter()
            .find(|&x| {
                (translations.get("1").unwrap())
                    .chars()
                    .all(|c| x.contains(c))
                    && !x.eq(translations.get("9").unwrap() as &str)
                    && x.len() == 6
            })
            .expect("0 not found")
            .as_str(),
    );
    translations.insert(
        "6",
        p.iter()
            .find(|&x| {
                !x.eq(translations.get("0").unwrap() as &str)
                    && !x.eq(translations.get("9").unwrap() as &str)
                    && x.len() == 6
            })
            .expect("6 not found")
            .as_str(),
    );
    translations.insert(
        "3",
        p.iter()
            .find(|&x| {
                (translations.get("1").unwrap())
                    .chars()
                    .all(|c| x.contains(c))
                    && x.len() == 5
            })
            .expect("3 not found")
            .as_str(),
    );
    translations.insert(
        "5",
        p.iter()
            .find(|&x| {
                x.chars()
                    .all(|c| (translations.get("9").unwrap()).contains(c))
                    && !x.eq(translations.get("3").unwrap() as &str)
                    && x.len() == 5
            })
            .expect("5 not found")
            .as_str(),
    );
    translations.insert(
        "2",
        p.iter()
            .find(|&x| {
                !x.eq(translations.get("5").unwrap() as &str)
                    && !x.eq(translations.get("3").unwrap() as &str)
                    && x.len() == 5
            })
            .expect("2 not found")
            .as_str(),
    );

    translations
}

fn load_input(filename: &str) -> (Vec<Vec<String>>, Vec<Vec<String>>) {
    let mut patterns: Vec<Vec<String>> = Vec::new();
    let mut digits: Vec<Vec<String>> = Vec::new();

    let f = fs::File::open(filename).expect("Unable to open file");
    let reader = io::BufReader::new(f);

    //Collect input strings
    for line in reader.lines() {
        let line: Vec<String> = line.unwrap().split('|').map(str::to_string).collect();
        let pattern: Vec<String> = line[0].split_whitespace().map(str::to_string).collect();
        let digit: Vec<String> = line[1].split_whitespace().map(str::to_string).collect();

        //Sort for uniqueness
        let mut sorted_pattern: Vec<String> = Vec::new();
        let mut sorted_digit: Vec<String> = Vec::new();
        for p in pattern {
            let mut chars: Vec<char> = p.chars().collect();
            chars.sort();
            let word = chars.into_iter().collect();
            sorted_pattern.push(word);
        }
        for d in digit {
            let mut chars: Vec<char> = d.chars().collect();
            chars.sort();
            let word = chars.into_iter().collect();
            sorted_digit.push(word);
        }

        patterns.push(sorted_pattern);
        digits.push(sorted_digit);
    }

    (patterns, digits)
}
