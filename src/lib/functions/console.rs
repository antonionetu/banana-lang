use std::process;


pub fn say(message: String){
    println!("🍌 {}", message);
}

pub fn smash(message: &str, at: String) -> ! {
    eprintln!(
        "\n🍌 Ur banana was smashed :(\n\nError: {}\nAt: {}\n",
        message, at
    );
    process::exit(1);
}
