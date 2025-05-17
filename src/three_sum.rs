/// LeetCode 15. Three Sum
pub struct Solution;

use::std::collections::HashSet;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // Brute force method // Not a solution on leetcode: Time limit exceeded
        // let n = nums.len();
        // let mut v_list = HashSet::new();
        // for i in 0..n {
        //     for j in (i + 1)..n {
        //         for k in (j + 1)..n {
        //             if &nums[i] + &nums[j] + &nums[k] == 0 {
        //                 let mut v = vec![nums[i], nums[j], nums[k]];
        //                 v.sort_unstable();
        //                 v_list.insert(v);
        //             }
        //         }
        //     }
        // }
        // let v_list = v_list.into_iter().collect();
        // v_list
        
        // Hash map solution
        let n = nums.len();
        let mut v_list = HashSet::new();
        for i in 0..n { 
            let mut s = HashSet::new();
            for j in (i + 1)..n {
                let complement = -1 * (&nums[i] + &nums[j]);
                if s.contains(&complement) {
                    let mut v = vec![nums[i], nums[j], complement];
                    v.sort_unstable();
                    v_list.insert(v);
                }
                s.insert(nums[j]);
            }
        }
        let v_list = v_list.into_iter().collect();
        v_list
        
        // Two Pointer Solution
    }
}