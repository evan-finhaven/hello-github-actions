fn add_one(x: usize) -> usize {
    x + 1
}

fn main() {
    println!("2 plus one is {}", add_one(2));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn adds_one_to_two() {
        let x = add_one(2);
        assert_eq!(x, 3);
    }
}
