/// [242: Valid Anagram](https://leetcode.com/problems/valid-anagram/).
///
/// Determines if a given string is an anagram of another given string.
///
/// Given two strings `s` and `t`.
/// If `t` is an anagram of `s` return `true`.
/// Otherwise, return `false`.
///
/// # Constraints:
/// - 1 <= s.length, t.length <= 5 * 10^4.
/// - `s` and `t` consist of lowercase english letters.
///
/// # Complexity:
/// - Time: O(n).
/// - Space: O(1).
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false; // `s` and `t` cannot be anagrams if their lengths are not equivalent.
    }

    let mut occurences = vec![0; 26]; // Init 26 element vector with 0 values to store the frequency of each character.

    for (char_from_s, char_from_t) in s.chars().zip(t.chars()) { // Pair `s` and `t`'s characters into two element tuples and iterate over each tuple pair.
        occurences[(char_from_s as u8 - b'a') as usize] += 1; // Cast char to its ASCII value, calculate the ASCII value difference between the char and 'a', cast the difference to usize to allow for vector indexing, and increment the count of char in `occurences` by 1.
        occurences[(char_from_t as u8 - b'a') as usize] -= 1; // Decrement the count of char in `occurences` by 1, such that if `t` is an anagram of `s`, the count each char in `occurences` should balance to 0 via equal increments and decrements.
    }

    occurences.iter().all(|&element_value| element_value == 0) // Iterate over each element in the `occurences` collection and use dereferencing to unpack each element from a reference to a direct value to enable a zero check.
}