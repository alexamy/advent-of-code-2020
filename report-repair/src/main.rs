mod first;

fn main() {
    println!("First solution: {}", first::solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_solution() {
        assert_eq!(first::solve(), 902451);
    }
}
