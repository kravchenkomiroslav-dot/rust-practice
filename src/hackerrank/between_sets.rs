pub fn getTotalX(a: &[i32], b: &[i32]) -> i32 {
    if a.is_empty() || b.is_empty() {
        return 0;
    }

    let min_x = *a.iter().max().unwrap();
    let max_x = *b.iter().min().unwrap();
    let mut count = 0;

    for x in min_x..=max_x {
        let is_a_factor = a.iter().all(|&element| x % element == 0);
        let is_factor_of_b = b.iter().all(|&element| element % x == 0);

        if is_a_factor && is_factor_of_b {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_between_two_sets_example() {
        let a = vec![2, 6];
        let b = vec![24, 36];
        assert_eq!(getTotalX(&a, &b), 2);
    }

    #[test]
    fn test_between_two_sets_another() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];
        assert_eq!(getTotalX(&a, &b), 3);
    }
}