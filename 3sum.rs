use std::collections::HashSet;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        const MAX: usize = 100_000;
        const SIZE: usize = MAX + 1;

        let mut positive = [false; SIZE];
        let mut negative = [false; SIZE];

        for &num in &nums {
            if num >= 0 {
                positive[num as usize] = true;
            } else {
                negative[(-num) as usize] = true;
            }
        }

        let contains = |num: i32| -> bool {
            if num.abs() > MAX as i32 {
                return false;
            }

            if num >= 0 {
                positive[num as usize]
            } else {
                negative[(-num) as usize]
            }
        };

        let mut result = HashSet::new();

        for i in 0..nums.len() {
            let left = nums[i];

            for j in i + 1..nums.len() {
                let right = nums[j];
                let complement = -left - right;

                // guarantees complement isn't a virtual count of either sides
                if complement == left || complement == right {
                    continue;
                }

                if contains(complement) {
                    let mut triplet = vec![left, right, complement];
                    triplet.sort_unstable();
                    result.insert(triplet);
                }
            }
        }

        if nums.iter().filter(|&&num| num == 0).count() >= 3 {
            result.insert(vec![0, 0, 0]);
        }

        result.into_iter().collect()
    }
}
