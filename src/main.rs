mod common;
mod day_1;

fn main() {
    match day_1::solution::solution_1() {
        Ok(_) => println!("Program successfully finished!"),
        Err(e) => println!("Error: {}", e)
    }
}
