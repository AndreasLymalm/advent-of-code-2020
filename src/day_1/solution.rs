use crate::common;

use std::io;

pub fn solution_1() -> io::Result<()>  {
    for line in common::read_input_for_day(1) {
        println!("{}", line);
    }
    
    Ok(())
}