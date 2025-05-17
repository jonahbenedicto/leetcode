/// LeetCode 15. Three Sum
pub struct Solution;

use::std::collections::HashSet;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // Brute force method // Not a solution on leetcode
        let n = nums.len();
        let mut v_list = HashSet::new();
        for i in 0..n {
            for j in (i + 1)..n {
                for k in (j + 1)..n {
                    if &nums[i] + &nums[j] + &nums[k] == 0 {
                        let mut v = vec![nums[i], nums[j], nums[k]];
                        v.sort_unstable();
                        v_list.insert(v);
                    }
                }
            }
        }
        let v_list = v_list.into_iter().collect();
        v_list
    }
}