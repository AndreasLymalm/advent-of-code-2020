use crate::common;

use std::io::{Error};

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

pub fn part_2() -> Result<i64, Error>  {
    let lines = common::read_input_for_day(3);
    
    let slopes = vec![
        (1, 1), (1, 3), (1, 5), (1, 7), (2, 1)
    ];

    let mut multiplied_tree_count: i64 = 1;
    for slope in slopes.iter() {
        let mut tree_count = 0;
        let line_length = lines[0].len();
        let mut pos = 0;
        for i in (0..lines.len()).step_by(slope.0) {
            let line = &lines[i];
            let terrain = line.chars().nth(pos).unwrap();

            if terrain == '#' {
                tree_count += 1;
            }
            pos += slope.1;
            pos %= line_length;
        }
        multiplied_tree_count *= tree_count;
    }
    
    return Ok(multiplied_tree_count);
}
