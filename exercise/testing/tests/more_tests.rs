#[cfg(test)]
mod test {
    use testing::*;

    #[test]
    fn test_complex() {
        assert_eq!(sploosh(splish(-1, 0), splish(1, 1), splish(3, 2)), 4);
    }
}
