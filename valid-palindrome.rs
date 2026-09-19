impl Solution {
    pub fn is_palindrome(mut s: String) -> bool {
        //s.retain(|c| ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || ('0'..='9').contains(&c));
        s.retain(|c| c.is_ascii_alphabetic() || c.is_ascii_digit());
        s.make_ascii_lowercase();
        s.chars().eq(s.chars().rev())
    }
}
