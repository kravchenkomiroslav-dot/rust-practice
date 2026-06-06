use std::collections::HashSet;

pub fn sock_merchant(n: i32, ar: &[i32]) -> i32 {
    if n <= 0 || ar.is_empty() {
        return 0;
    }

    let mut pairs = 0;
    let mut unmatched_socks = HashSet::new();

    for &sock in ar {
        if unmatched_socks.contains(&sock) {
            unmatched_socks.remove(&sock);
            pairs += 1;
        } else {
            unmatched_socks.insert(sock);
        }
    }

    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sock_merchant_example() {
        let ar = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];
        assert_eq!(sock_merchant(9, &ar), 3);
    }

    #[test]
    fn test_sock_merchant_no_pairs() {
        let ar = vec![1, 2, 3, 4, 5];
        assert_eq!(sock_merchant(5, &ar), 0);
    }
}