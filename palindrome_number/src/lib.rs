// 121 -> true
// 241 -> false

struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let char_stream = x.to_string();

        let reversed: String = char_stream.chars().rev().collect();
        reversed == char_stream
    }
}

#[cfg(test)]
#[test]
pub fn true_test() {
    assert!(Solution::is_palindrome(121));
}

#[test]
pub fn false_test() {
    assert!(Solution::is_palindrome(122));
}
