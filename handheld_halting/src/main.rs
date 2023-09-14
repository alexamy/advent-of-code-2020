mod first;
mod interpreter;
mod parser;
mod reader;
mod second;

fn main() {
    println!("First solution: {}", first::solve());
    println!("Second solution: {}", second::solve());
}
