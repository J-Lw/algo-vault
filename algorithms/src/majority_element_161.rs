pub fn majority_element(numbers: Vec<i32>) -> i32 {
    let (mut candidate, mut count) = (numbers[0], 1);

    for &number in &numbers[1..] {
        if count == 0 {
            candidate = number;
            count = 1;
        } else if number == candidate {
            count += 1;
        } else {
            count -= 1;
        }
    }
    candidate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_majority_element() {
        let numbers = vec![3,2,3];
        assert_eq!(majority_element(numbers), 3);

        let numbers = vec![2,2,1,1,1,2,2];
        assert_eq!(majority_element(numbers), 2);
    }
}
