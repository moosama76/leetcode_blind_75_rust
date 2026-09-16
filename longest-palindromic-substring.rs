impl Solution {
    pub fn okay(s: &[u8], l: usize, r: usize) -> bool {
        let mut l = l;
        let mut r = r;

        while l < r {
            if s[l] != s[r] {
                return false;
            }
            l += 1;
            r -= 1;
        }

        true
    }

    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();

        let mut best_l = 0;
        let mut best_r = 0;

        for i in 0..bytes.len() {
            for j in i..bytes.len() {
                if j - i > best_r - best_l
                    && Solution::okay(bytes, i, j)
                {
                    best_l = i;
                    best_r = j;
                }
            }
        }

        s[best_l..=best_r].to_string()
    }
}
