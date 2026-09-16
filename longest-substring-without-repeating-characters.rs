impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() < 2 {
            return s.len() as i32;
        }
        let mut last_occurrence = [-1 as i32; 200];
        let s: Vec<char> = s.chars().collect(); 
        let mut ans = 0;
        let mut i = 0;
        let mut j = 0;
        while j < s.len() {
            if last_occurrence[s[j] as usize] >= i as i32 {
                i = last_occurrence[s[j] as usize] as usize + 1; 
            } 
            ans = ans.max(j-i+1);
            last_occurrence[s[j] as usize] = j as i32;
            j += 1;

        }
        ans as i32
    }
}
