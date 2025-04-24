mod lib;
mod peel;

fn main() {
    let file = lib::file::get();
    let tokens = lib::file::get_tokens(file);
    peel::to_peel(tokens);
}
