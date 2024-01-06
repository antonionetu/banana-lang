enum Operation {
    Sum,
    Sub,
    Mul,
    Div,
}

fn op32_to_i32(option: Option<u32>) -> i32 {
    option.unwrap_or(0) as i32
}

fn get_operation(op: Operation, a: i32, b: i32) -> i32 {
    match op {
        Operation::Sum => a + b,
        Operation::Sub => a - b,
        Operation::Mul => a * b,
        Operation::Div => a / b,
    }
}

fn main() {
    let file = std::env::args().nth(1).expect("No file path provided");
    if &file[file.len()-7 ..] != ".banana"{
        panic!("Invalid file extension");
    }

    let content = std::fs::read_to_string(file)
        .expect("Something went wrong reading the file");

    let tokens: Vec<char> = content.chars().collect();

    let operation = match tokens[0] {
        '+' => Operation::Sum,
        '-' => Operation::Sub,
        '*' => Operation::Mul,
        '/' => Operation::Div,
        _ => panic!("Invalid operator"),
    };

    let num1 = op32_to_i32(tokens[2].to_digit(10));
    let num2 = op32_to_i32(tokens[4].to_digit(10));
    let result = get_operation(operation, num1, num2);
    println!("{}", result);
}
