use crate::lib;
use crate::peel::analyzers::lexical::list;

pub fn throw(peel: String) {
    let mut can = String::new();
    let mut trash = String::new();
    let mut throwing = false;
    let mut open_parentheses = 0;

    for p in peel.chars() {
        if p == '(' {
            if throwing {
                open_parentheses += 1;
                trash.push(p);
            } else {
                throwing = true;
            }
            continue;
        } else if p == ')' {
            if open_parentheses > 0 {
                open_parentheses -= 1;
                trash.push(p);
            } else {
                throwing = false;
            }
            continue;
        }

        if throwing {
            trash.push(p);
        } else {
            can.push(p);
        }
    }

    organic_waste_bin(can.trim().to_string(), trash.trim().to_string());
}

fn organic_waste_bin(can: String, trash: String) {
    if list::INSTRUCTIONS.contains(&can.as_str()) {
        match can.as_str() {
            "say" => {
                if trash.contains('(') {
                    throw(trash);
                } else {
                    crate::lib::functions::console::say(trash);
                }
            }
            "plus" | "minus" | "times" | "divided" => {
                let args: Vec<&str> = trash.split(',').map(|s| s.trim()).collect();

                if args.len() == 2 {
                    let left_operand = args[0].parse::<i32>().unwrap_or_else(|_| panic!("Invalid left operand: {}", args[0]));
                    let right_operand = args[1].parse::<i32>().unwrap_or_else(|_| panic!("Invalid right operand: {}", args[1]));

                    let operation = crate::lib::math::find_math_operation(&can);
                    crate::lib::math::calculate_math_operation(operation, left_operand, right_operand);
                } else {
                    crate::lib::functions::console::smash("Invalid math arguments", trash);
                }
            }
            _ => {
                crate::lib::functions::console::smash("Unknown instruction", can);
            }
        }
    } else {
        crate::lib::functions::console::smash("Instruction not found", can);
    }
}
