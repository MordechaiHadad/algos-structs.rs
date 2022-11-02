mod datastructs;
mod search;
mod sort;

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{datastructs::Queue, search::*, sort::*};

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

    #[test]
    fn bubble_sort_test() {
        let mut array = [9, 3, 7, 4, 69, 420, 42];

        bubble_sort(&mut array);

        assert_eq!(array, [3, 4, 7, 9, 42, 69, 420]);
    }

    #[test]
    fn queue_enqueue_test() {
        let mut list: Queue<i32> = Queue::new();

        list.enqueue(1);
        list.enqueue(3);
        list.enqueue(4);

        assert_eq!(3, list.length);
    }

    #[test]
    fn queue_deque_test() {
        let mut list: Queue<i32> = Queue::new();
        list.enqueue(4);
        list.enqueue(1);
        list.enqueue(30);
        list.deque();
        list.deque();

        assert_eq!(list.deque(), Some(30));
    }

    #[test]
    fn quck_sort_test() {
        let mut array = [9, 3, 7, 4, 69, 420, 42];
        let sorted_array = [3, 4, 7, 9, 42, 69, 420];

        quick_sort(&mut array);

        assert_eq!(array, sorted_array);
    }
}
