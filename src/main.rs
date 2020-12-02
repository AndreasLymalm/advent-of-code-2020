#![allow(dead_code)]
mod common;
mod day_1;
mod day_2;

fn main() {
    match day_2::solution::part_1() {
        Ok(answer) => println!("Answer is: {}", answer),
        Err(e) => println!("Error: {}", e)
    }
}
