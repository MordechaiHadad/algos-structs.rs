mod search;

#[cfg(test)]
mod tests {
    use crate::search::*;

    #[test]
    fn linear_search_test() {
        let result = &[1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420];
        assert_eq!(linear_search(result, 69), true);
        assert_eq!(linear_search(result, 1336), false); // rust doesnt support test cases
                                                        // unfortunate
        assert_eq!(linear_search(result, 69420), true);
        assert_eq!(linear_search(result, 69421), false);
        assert_eq!(linear_search(result, 1), true);
        assert_eq!(linear_search(result, 0), false);
    }

    #[test]
    fn binary_search_test() {
        let result = &[1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420];
        assert_eq!(binary_search(result, 69), true);
        assert_eq!(binary_search(result, 1336), false);
        assert_eq!(binary_search(result, 69420), true);
        assert_eq!(binary_search(result, 69421), false);
        assert_eq!(binary_search(result, 1), true);
        assert_eq!(binary_search(result, 0), false);
    }

    #[test]
    fn two_crystal_balls_test_equal() {
        let idx = (rand::random::<f64>() * (10000 as f64)) as i32;
        let mut data = [false; 10000];

        let mut i = idx;
        while i < 10000 {
            data[i as usize] = true;
            i += 1;
        }

        assert_eq!(two_crystal_balls(&data), idx);
    }

    #[test]
    fn two_crystal_balls_test_negative() {
        assert_eq!(two_crystal_balls(&[false; 821]), -1);
    }
}
