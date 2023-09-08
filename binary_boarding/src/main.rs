mod first;
mod reader;
// mod second;

mod processor {
    mod interval;
    mod parser;
    pub mod seat;
}

fn main() {
    println!("First solution: {}", first::solve());
    // println!("Second solution: {}", second::solve());
}
