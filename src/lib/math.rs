pub enum MathOperations {
    Sum,
    Sub,
    Mul,
    Div,
}

pub fn find_math_operation(op: char) -> MathOperations {
    match op {
        '+' => MathOperations::Sum,
        '-' => MathOperations::Sub,
        '*' => MathOperations::Mul,
        '/' => MathOperations::Div,
        _ => panic!("Invalid operator"),
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
