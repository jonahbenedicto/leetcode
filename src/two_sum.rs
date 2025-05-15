/// LeetCode 1. Two Sum
pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Brute force solution
        // Time complexity: O(n^2)
        // Space complexity: O(1)
        let n = nums.len();
        for i in 0..n {
            for j in (i + 1)..n {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        Vec::new()
    }
}