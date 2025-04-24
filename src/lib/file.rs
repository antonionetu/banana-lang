pub fn get() -> String {
    let file = std::env::args().nth(1).expect("No file path provided");
    is_banana(file.clone());
    file
}

fn is_banana(file: String) {
    if &file[file.len()-7 ..] != ".banana"{
        panic!("Invalid file extension");
    };
}

fn to_string(file: String) -> String {
    std::fs::read_to_string(file)
        .expect("Something went wrong reading the file")
}

pub fn get_tokens(file: String) -> Vec<char> {
    let content = to_string(file);
    content.chars().filter(|c| !c.is_whitespace()).collect()
}
