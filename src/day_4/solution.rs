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
    
    let mut valid_passports = 0;
    let mut available_required_fields: HashSet<&str> = vec![].into_iter().collect();
    let hair_color_chars = vec![
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"
    ];
    let eye_colors = vec![
        "amb", "blu", "brn", "gry", "grn", "hzl", "oth"
    ];

    for i in 0..lines.len() {
        if lines[i] == "" {
            println!("{:?}", available_required_fields);
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
                let value = field_set[1];
                match field {
                    "byr" => if value.len() == 4 {
                        match value.parse::<i32>() {
                            Ok(int_value) => {
                                if 1920 <= int_value && int_value <= 2002 {
                                    available_required_fields.insert(field);
                                }
                            },
                            Err(_) => {}
                        };
                    },
                    "ecl" => {
                        if eye_colors.contains(&value) {
                            available_required_fields.insert(field);
                        }
                    },
                    "eyr" => if value.len() == 4 {
                        match value.parse::<i32>() {
                            Ok(int_value) => {
                                if 2020 <= int_value && int_value <= 2030 {
                                    available_required_fields.insert(field);
                                }
                            },
                            Err(_) => {}
                        };
                    },
                    "hcl" => if value.len() == 7 {
                        let start = &value[..1];
                        let characters = &value[1..];
                        if start == "#" {
                            let mut is_valid = true;
                            for i in 0..characters.len() {
                                if !hair_color_chars.contains(&&characters[i..i+1]) {
                                    is_valid = false;
                                }
                            }
                            if is_valid {
                                available_required_fields.insert(field);
                            }
                        }
                    },
                    "hgt" => {
                        let height = &value[..value.len()-2];
                        let unit = &value[value.len()-2..];
                        if unit == "cm" {
                            if height.len() == 3 {
                                match height.parse::<i32>() {
                                    Ok(int_value) => {
                                        if 150 <= int_value && int_value <= 193 {
                                            available_required_fields.insert(field);
                                        }
                                    },
                                    Err(_) => {}
                                };
                            }
                        }
                        if unit == "in" {
                            if height.len() == 2 {
                                match height.parse::<i32>() {
                                    Ok(int_value) => {
                                        if 59 <= int_value && int_value <= 76 {
                                            available_required_fields.insert(field);
                                        }
                                    },
                                    Err(_) => {}
                                };
                            }
                        }
                    },
                    "iyr" => if value.len() == 4 {
                        match value.parse::<i32>() {
                            Ok(int_value) => {
                                if 2010 <= int_value && int_value <= 2020 {
                                    available_required_fields.insert(field);
                                }
                            },
                            Err(_) => {}
                        };
                    },
                    "pid" => if value.len() == 9 {
                        match value.parse::<i32>() {
                            Ok(_) => {
                                available_required_fields.insert(field);
                            },
                            Err(_) => {}
                        };
                    },
                    _ => {},
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
