/// LeetCode 16. Three Sum Closet
pub struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        // Brute force method // Not a solution on leetcode: Time limit exceeded
        let mut delta= i32::MAX;
        let mut sum = 0;
        let n = nums.len();
        for i in 0..n {
            for j in (i + 1)..n {
                for k in (j + 1)..n {
                    let new_sum = nums[i] + nums[j] + nums[k];
                    let new_delta = (new_sum - target).abs();
                    if new_delta < delta {
                        delta = new_delta;
                        sum = new_sum;
                    }
                }
            }
        }
        sum
    }
}