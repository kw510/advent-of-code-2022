use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let file_path = "input.txt";
    let file = File::open(file_path).expect("Should have been able to read the file");
    let reader = BufReader::new(file);
    let mut acc = 0;
    for line in reader.lines() {
        let line = line.expect("should be a line");
        let vec:Vec<char> = line.chars().filter(|x| x.is_ascii_alphabetic()).collect();
        //println!("{}, {}", vec[0], vec[1])
        match vec[..] {
            [opponent, 'X'] => { // Lose
                let score = match opponent {
                    'A' => 3,
                    'B' => 1,
                    'C' => 2,
                    _ => 0,
                } + 0;
                print!("score: {} | ", score);
                acc = acc + score;
            },
            [opponent, 'Y'] => { // Draw
                let score = match opponent {
                    'A' => 1,
                    'B' => 2,
                    'C' => 3,
                    _ => 0,
                } + 3;
                print!("score: {} | ", score);
                acc = acc + score;
            },
            [opponent, 'Z'] => { // Win
                let score = match opponent {
                    'A' => 2,
                    'B' => 3,
                    'C' => 1,
                    _ => 0,
                } + 6;
                print!("score: {} | ", score);
                acc = acc + score;
            }
            _ => print!("")
        }
        println!("{},{}",vec[0], vec[1])
    }


    println!("{}", acc)
}
