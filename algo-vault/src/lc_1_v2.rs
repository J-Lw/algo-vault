/// [1: Two Sum](https://leetcode.com/problems/two-sum/).
///
/// # Constraints:
/// - 2 <= numbers.length <= 10^4.
/// - -10^9 <= numbers[i] <= 10^9.
/// - -10^9 <= target <= 10^9.
///
/// # Complexity:
/// - Time: O(n).
/// - Space: O(n).
use std::collections::HashMap;

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut previous_elements = HashMap::new();

    for (index, element) in numbers.iter().enumerate() {
        let complement = target - element;

        if previous_elements.contains_key(&complement) {
            return vec![previous_elements[&complement], index as i32];
        }

        previous_elements.insert(element, index as i32);
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![1, 2, 3, 4, 5], 9), vec![3, 4]);
    }
}