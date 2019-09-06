fn main() {
    println!("Hello, world!");
    println!("{}", add2(2, 3))
}

fn add2(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add2(1, 2), 3);
    }
}
