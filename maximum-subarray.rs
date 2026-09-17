impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];
        let mut mx = nums[0];
        for &i in nums.iter().skip(1) {
            let prev_mx = mx;
            mx = i.max(prev_mx+i);
            ans = ans.max(mx);
        }
        ans
    }
}
