mod search;

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::search::*;

    #[rstest]
    #[case(69, true)]
    #[case(1336, false)]
    #[case(69420, true)]
    #[case(69421, false)]
    #[case(1, true)]
    #[case(0, false)]
    fn linear_search_test(#[case] x: i32, #[case] y: bool) {
        let result = &[1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420];
        assert_eq!(linear_search(result, x), y);
    }

    #[rstest]
    #[case(69, true)]
    #[case(1336, false)]
    #[case(69420, true)]
    #[case(69421, false)]
    #[case(1, true)]
    #[case(0, false)]
    fn binary_search_test(#[case] x: i32, #[case] y: bool) {
        let result = &[1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420];
        assert_eq!(binary_search(result, x), y);
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
