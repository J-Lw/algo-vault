/// [217: Contains Duplicate](https://leetcode.com/problems/contains-duplicate/).
///
/// func desc.
///
/// Given a vector `nums`, where each element represents an integer value.
/// The vector is assessed to determine:
/// - Whether any integer value appears at least twice.
/// If any value is found to be repeated in the vector, `true` is returned.
/// Otherwise, if any integer in the vector is distinct, `false` is returned.
///
/// # Constraints:
/// - 1 <= nums.length <= 10^5.
/// - -10^9 <= nums[i] <= 10^9.
///
/// # Complexity:
/// -
/// -
use std::collection::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::new(); // Create hashset `seen`.

    for &num in &nums {
        if seen.contains(&num) {
            return true; // If current `num` is present in hashset `seen`, `true` is returned.
        }
        seen.insert(num); // Else, add current `num` to hashset `seen`.
    }

    false // And return `false`.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        // Regular case with duplicate values.
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);

        // Regular case with no duplicate values.
        asseert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);

        // Multiple duplicates in different parts of the vector.
        assert_eq!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);

        // Single element edge case, no possibility for duplicates.
        assert_eq!(contains_duplicate(vec![1]), false);

        // Minimum and maximum values without duplicates.
        assert_eq!(
            contains_duplicate(vec![-1_000_000_000, 1_000_000_000]),
            false
        );

        // Minimum value with duplicates.
        assert_eq!(
            contains_duplicate(vec![-1_000_000_000, -1_000_000_000]),
            true
        );

        // Maximum value with duplicates.
        assert_eq!(contains_duplicate(vec![1_000_000_000, 1_000_000_000]), true);

        // Mixed positive and negative numbers with duplicates.
        assert_eq!(contains_duplicate(vec![5, 7, 9, -9, -9, 5]), true);

        // All identical numbers; contains duplicates.
        assert_eq!(contains_duplicate(vec![0, 0, 0, 0]), true);

        // Large dataset with distinct values, testing the upper bound.
        let large_data: Vec<i32> = (1..=100_000).collect();
        assert_eq!(contains_duplicate(large_data), false);
    }
}
