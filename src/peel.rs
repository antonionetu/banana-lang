mod analyzers;
use crate::lib;

pub fn to_peel(tokens: Vec<char>) {
    let mut peels = Vec::new();
    let mut current = String::new();

    for p in tokens {
        if p == ';' {
            peels.push(current.clone());
            current.clear();
        } else {
            current.push(p);
        }
    }

    if !current.is_empty() {
        lib::functions::console::smash("Expected ';'", current);
    }

    for peel in peels {
        // analyzers::semantic::is_ok(peel);
        analyzers::lexical::peel::throw(peel);
    }
}
