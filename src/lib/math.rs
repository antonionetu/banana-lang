use crate::lib;

pub enum MathOperations {
    Sum,
    Sub,
    Mul,
    Div,
}

pub fn find_math_operation(op: &str) -> MathOperations {
    match op {
        "plus" => MathOperations::Sum,
        "minus" => MathOperations::Sub,
        "times" => MathOperations::Mul,
        "divided" => MathOperations::Div,
        _ => {
            lib::functions::console::smash("Invalid operator", op.to_string());
            panic!("No operation found.")
        },
    }
}

pub fn calculate_math_operation(op: MathOperations, a: i32, b: i32) -> i32 {
    match op {
        MathOperations::Sum => a + b,
        MathOperations::Sub => a - b,
        MathOperations::Mul => a * b,
        MathOperations::Div => a / b,
    }
}
