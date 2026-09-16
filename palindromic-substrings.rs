impl Solution {
    pub fn okay(s: &Vec<u8>) -> bool {
        for i in 0..s.len()/2{
            if s[i] != s[s.len()-i-1] {
                return false;
            }
        }
        true
    }
    pub fn count_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ans = 0;
        for i in 0..s.len(){
            let mut cur = vec![];
            for j in i..s.len() {
                cur.push(s[j]);
                if Solution::okay(&cur){
                    ans += 1;
                }
            }
        }
        ans
    }
}
