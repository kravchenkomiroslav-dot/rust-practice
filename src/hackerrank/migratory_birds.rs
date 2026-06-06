pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = vec![0; 6];

    for &bird in arr {
        if bird >= 1 && bird <= 5 {
            counts[bird as usize] += 1;
        }
    }

    let mut max_count = 0;
    let mut result_bird = 1;

    for bird_id in 1..=5 {
        if counts[bird_id] > max_count {
            max_count = counts[bird_id];
            result_bird = bird_id as i32;
        }
    }

    result_bird
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migratory_birds_example() {
        let arr = vec![1, 4, 4, 4, 5, 3];
        assert_eq!(migratory_birds(&arr), 4);
    }

    #[test]
    fn test_migratory_birds_tie() {
        let arr = vec![1, 2, 2, 3, 3, 4, 5];
        assert_eq!(migratory_birds(&arr), 2);
    }
}