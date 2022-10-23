mod search;

#[cfg(test)]
mod tests {
    use crate::search::linear_search;

    #[test]
    fn linear_search_test() {
        let result = &[1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420];
        assert_eq!(linear_search(result, 69), true);
        assert_eq!(linear_search(result, 1336), false);
        assert_eq!(linear_search(result, 69420), true);
        assert_eq!(linear_search(result, 69421), false);
        assert_eq!(linear_search(result, 1), true);
        assert_eq!(linear_search(result, 0), false);
    }
}
