fn main() {
    println!("Hello, world!");
}

fn add(int x, y) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    #[test]
    fn add() {
        assert_eq!(add(1, 2), 3);
    }
}