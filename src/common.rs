use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn read_input_for_day(day: i32) -> Vec<String> {
    let filename = format!("src/day_{}/input.txt", day);
    let file = File::open(filename)
        .expect("File does not exist");

    BufReader::new(file)
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn read_input_as_integers_for_day(day: i32) -> Vec<i32> {
    let filename = format!("src/day_{}/input.txt", day);
    let file = File::open(filename)
        .expect("File does not exist");

    BufReader::new(file)
        .lines()
        .map(|l| l.expect("Could not parse line").parse().unwrap())
        .collect()
}