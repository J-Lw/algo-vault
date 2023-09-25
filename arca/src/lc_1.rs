/// [1: Two Sum](https://leetcode.com/problems/two-sum/).
///
/// Solution desc.
///
/// # Problem Statement:
/// - Given an array of integers 'numbers' and an integer 'target'.
/// - Assume each input has exactly one solution and that each element can only be used once.
/// - Return the indices of the two numbers in 'numbers' that add up to 'target'.
///
/// # Constraints:
/// - 2 <= numbers.length <= 10^4.
/// - -10^9 <= numbers[i] <= 10^9.
/// - -10^9 <= target <= 10^9.
/// 
/// # Complexity:
/// - 
/// - 
/// 
/// # Intuition:
/// - Initialize a hashmap to store seen elements and their indices to enable O(1) lookups.
/// - Iterate over entire array to perform per-element operations.
/// - Calculate 'target - current_element' to produce the current elements complementary element,
/// which when summed would produce the target value.
/// - By knowing the the number that is needed to achieve the target with the current element,
/// The hashmap can be queried to obtain the complementary values index if it exists.
/// - If the complementary value does not exist, the current element and its index can be stored
/// in the hashmap to enable it to be looked-up as a potential complement element to future current elements.
/// - Once the pair of elements that equate to the target value are found, their indices can be returned by
/// returning the current loop iteration index along with the index associated with the complement value in the hashmap.
use std::collections::HashMap;
   
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut checked_elements = HashMap::new();

    for (index, current_element) in numbers.iter().enumerate() {
        let mut needed_element = target - current_element;

        if checked_elements.contains_key(&needed_element) {
            return index, checked_elements[&needed_element]; 
        }

        checked_elements.insert(current_element, index);        
    }
}