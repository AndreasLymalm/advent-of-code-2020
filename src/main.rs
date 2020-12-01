mod common;
mod day_1;

fn main() {
    match day_1::solution::part_1() {
        Ok(answer) => println!("Answer is: {}", answer),
        Err(e) => println!("Error: {}", e)
    }
}
