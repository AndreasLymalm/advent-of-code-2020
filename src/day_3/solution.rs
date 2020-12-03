use crate::common;

use std::io::{Error, ErrorKind};

pub fn part_1() -> Result<i32, Error>  {
    let lines = common::read_input_for_day(3);
    
    let mut tree_count = 0;
    let line_length = lines[0].len();
    let mut pos = 0;
    for i in 0..lines.len() {
        let line = &lines[i];
        let terrain = line.chars().nth(pos).unwrap();

        if terrain == '#' {
            tree_count += 1;
        }
        pos += 3;
        pos %= line_length;
    }
    
    return Ok(tree_count);
}

pub fn part_2() -> Result<i32, Error>  {
    let lines = common::read_input_for_day(3);
    
    
    return Err(Error::new(ErrorKind::Other, "Solution not found"));
}
