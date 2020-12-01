use crate::common;

use std::io::{Error, ErrorKind};

pub fn part_1() -> Result<i32, Error>  {
    let lines = common::read_input_as_integers_for_day(1);
    for i in 0..lines.len()-1 {
        for k in i+1..lines.len() {
            if lines[i] + lines[k] == 2020 {
                return Ok(lines[i] * lines[k]);
            }
        }
    }
    
    return Err(Error::new(ErrorKind::Other, "Solution not found"));
}
