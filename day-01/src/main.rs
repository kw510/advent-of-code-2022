use std::fs::File;
use std::io::{BufReader, BufRead};


fn main() {
    let file_path = "input.txt";

    println!("In file {}", file_path);

    let file = File::open(file_path).expect("Should have been able to read the file");
    let reader = BufReader::new(file);

    let mut i = 0;
    let mut acc = 0;
    let mut vec = Vec::new();
    for line in reader.lines() {
        let line = line.expect("msg");
        if line.is_empty() {
            vec.push(acc);
            i = i + 1;
            acc = 0;
            continue;
        }
        acc = acc + line.parse::<i32>().expect("should be a number");
    }
    vec.sort();
    match vec[..] {
        [.., third, second, first] => {
            println!("Most: {}", first);
            println!("Top three total: {}", first + second + third);
        },
        _  => print!("Not enough elves :(")
    }
}