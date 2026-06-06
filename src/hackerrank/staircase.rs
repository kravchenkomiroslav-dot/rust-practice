
pub fn generate_staircase(n: usize) -> String {
    let mut result = String::new();

    for i in 1..=n {
        let spaces = " ".repeat(n - i);
        let hashes = "#".repeat(i);
        result.push_str(&spaces);
        result.push_str(&hashes);
        if i < n {
            result.push('\n');
        }
    }

    result
}

pub fn staircase(n: i32) {
    if n <= 0 {
        return;
    }
    println!("{}", generate_staircase(n as usize));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staircase_size_4() {
        let expected = "   #\n  ##\n ###\n####";
        assert_eq!(generate_staircase(4), expected);
    }

    #[test]
    fn test_staircase_size_1() {
        let expected = "#";
        assert_eq!(generate_staircase(1), expected);
    }
}