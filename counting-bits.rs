impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans = vec![];
        for i in 0..=n {
            let mut j = i;
            let mut cnt = 0;
            while(j > 0){
                if j&1 == 1 {
                    cnt += 1;
                }
                j = (j >> 1);
            }
            ans.push(cnt);
        }
        ans
    }
}
