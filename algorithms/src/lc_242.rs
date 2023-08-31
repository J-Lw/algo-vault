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
///
/// # Intuition:
/// - Employ frequency counting to determine if strings `s` and `t` have the same number of each character.
/// - Increment counts for characters in `s` and decrement for `t`.
/// - If all character frequencies zero out, `s` and `t` are anagrams.
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false; // `s` and `t` cannot be anagrams if their lengths are not equivalent.
    }

    let mut occurrences = vec![0; 26]; // Init 26 element vector with 0 values to store the frequency of each character.

    for (char_from_s, char_from_t) in s.chars().zip(t.chars()) {
        // Pair `s` and `t`'s characters into two element tuples and iterate over each tuple pair.
        occurrences[(char_from_s as u8 - b'a') as usize] += 1; // Cast char to its ASCII value, calculate the ASCII value difference between the char and 'a', cast the difference to usize to allow for vector indexing, and increment the count of char in `occurences` by 1.
        occurrences[(char_from_t as u8 - b'a') as usize] -= 1; // Decrement the count of char in `occurrences` by 1, such that if `t` is an anagram of `s`, the count each char in `occurrences` should balance to 0 via equal increments and decrements.
    }

    occurrences.iter().all(|&element_value| element_value == 0) // Iterate over each element in the `occurrences` collection and use dereferencing to unpack each element from a reference to a direct value to enable a zero check.
}

#[cfg(test)]
mod tests {
    use super::is_anagram;

    #[test]
    fn test_is_anagram() {
        // Not anagrams: different lengths.
        assert!(!is_anagram("xyz".to_string(), "zyxz".to_string()));

        // Anagrams.
        assert!(is_anagram("silent".to_string(), "listen".to_string()));

        // Not anagrams: same lengths.
        assert!(!is_anagram("triangle".to_string(), "integral".to_string()));
    }
}
