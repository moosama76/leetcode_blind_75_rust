impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];
        let mut mn = nums[0];
        let mut mx = nums[0];
        for &i in nums.iter().skip(1) {
            let (prev_mx, prev_mn) = (mx, mn);
            mn = i.min(prev_mn*i).min(prev_mx*i);
            mx = i.max(prev_mx*i).max(prev_mn*i);
            ans = ans.max(mx);
        }
        ans
    }
}
