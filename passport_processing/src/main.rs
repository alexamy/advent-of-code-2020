mod first;
mod processor;
mod reader;
mod second;
mod solution;

fn main() {
    println!("First solution: {}", first::solve());
    println!("Second solution: {}", second::solve());
}
