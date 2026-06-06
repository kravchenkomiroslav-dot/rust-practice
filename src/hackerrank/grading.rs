pub fn grading_students(grades: &[i32]) -> Vec<i32> {
    let mut rounded_grades = Vec::new();

    for &grade in grades {
        if grade < 38 {
            rounded_grades.push(grade);
        } else {
            let next_multiple_of_5 = ((grade / 5) + 1) * 5;
            if next_multiple_of_5 - grade < 3 {
                rounded_grades.push(next_multiple_of_5);
            } else {
                rounded_grades.push(grade);
            }
        }
    }

    rounded_grades
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grading_students_example() {
        let input = vec![73, 67, 38, 33];
        let expected = vec![75, 67, 40, 33];
        assert_eq!(grading_students(&input), expected);
    }

    #[test]
    fn test_grading_no_change() {
        let input = vec![100, 0, 37, 42];
        let expected = vec![100, 0, 37, 42];
        assert_eq!(grading_students(&input), expected);
    }
}