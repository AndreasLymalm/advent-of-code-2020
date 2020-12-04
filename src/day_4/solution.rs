use crate::common;

use std::collections::HashSet;
use std::io::{Error};

pub fn part_1() -> Result<i32, Error>  {
    let lines = common::read_input_for_day(4);
    
    let mut valid_passports = 0;
    let mut available_required_fields: HashSet<&str> = vec![].into_iter().collect();
    let required_fields = vec![
        "iyr", "ecl", "eyr", "pid", "hcl", "byr", "hgt"
    ];

    for i in 0..lines.len() {
        if lines[i] == "" {
            if available_required_fields.len() == 7 {
                valid_passports += 1;
            }
            available_required_fields.clear();
        }
        else {
            let field_sets: Vec<&str> = lines[i].split(" ").collect();
            for fs in 0..field_sets.len() {
                let field_set: Vec<&str> = field_sets[fs].split(":").collect();
                let field = field_set[0];
                for rf in 0..required_fields.len() {
                    if field == required_fields[rf] {
                        available_required_fields.insert(field);
                    }
                }
            }
        }
    }

    // Last element
    if available_required_fields.len() == 7 {
        valid_passports += 1;
    }

    return Ok(valid_passports);
}

pub fn part_2() -> Result<i64, Error>  {
    let lines = common::read_input_for_day(4);
    


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
