#![allow(dead_code)]
mod common;
mod day_1;
mod day_2;
mod day_3;
mod day_4;

fn main() {
    match day_4::solution::part_1() {
        Ok(answer) => println!("Answer is: {}", answer),
        Err(e) => println!("Error: {}", e)
    }
}
