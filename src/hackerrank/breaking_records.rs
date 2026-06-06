pub fn breaking_records(scores: &[i32]) -> Vec<i32> {
    if scores.is_empty() {
        return vec![0, 0];
    }

    let mut max_score = scores[0];
    let mut min_score = scores[0];
    let mut max_breaks = 0;
    let mut min_breaks = 0;

    for &score in scores.iter().skip(1) {
        if score > max_score {
            max_score = score;
            max_breaks += 1;
        } else if score < min_score {
            min_score = score;
            min_breaks += 1;
        }
    }

    vec![max_breaks, min_breaks]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breaking_records_example1() {
        let scores = vec![10, 5, 20, 20, 4];
        assert_eq!(breaking_records(&scores), vec![1, 2]);
    }

    #[test]
    fn test_breaking_records_example2() {
        let scores = vec![3, 4, 21, 36, 10, 28, 35, 5, 24, 42];
        assert_eq!(breaking_records(&scores), vec![4, 0]);
    }
}