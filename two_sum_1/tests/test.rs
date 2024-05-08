extern crate two_sum;
use two_sum::two_sum;

// Simple tests
#[test]
fn test_two_sum_basic() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}

#[test]
fn test_two_sum_no_result() {
    assert_eq!(two_sum(vec![1, 2, 3], 5), vec![1, 2]);
}

#[test]
fn test_two_sum_negative_numbers() {
    assert_eq!(two_sum(vec![-3, 4, 3, 90], 94), vec![1, 3]);
}