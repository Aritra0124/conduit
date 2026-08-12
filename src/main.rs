fn main() {
    println!("Hello, world! for earth");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_main() {
        assert_eq!(1, 1);
    }
}
