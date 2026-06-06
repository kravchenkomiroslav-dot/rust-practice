pub fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut primary_diagonal_sum = 0;
    let mut secondary_diagonal_sum = 0;

    for i in 0..n {
        primary_diagonal_sum += arr[i][i];
        secondary_diagonal_sum += arr[i][n - 1 - i];
    }

    (primary_diagonal_sum - secondary_diagonal_sum).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal_difference_example() {
        let matrix = vec![
            vec![11, 2, 4],
            vec![4, 5, 6],
            vec![10, 8, -12],
        ];
        assert_eq!(diagonal_difference(&matrix), 15);
    }

    #[test]
    fn test_diagonal_difference_small() {
        let matrix = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        assert_eq!(diagonal_difference(&matrix), 0);
    }
}