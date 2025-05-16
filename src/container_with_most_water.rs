/// LeetCode 11. Container With Most Water
pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        // Brute force solution // Not a solution on leetcode
        // let mut max_area = 0;
        // for (i, h1) in height.iter().enumerate() {
        //     for (j, h2) in height.iter().enumerate() {
        //         if i == j {
        //             continue;
        //         }
        //         if h1 <= h2 && h1 * (j as i32 - i as i32) > max_area {
        //             max_area = h1 * (j as i32 - i as i32);
        //         } else if h2 < h1 && h2 * (j as i32 - i as i32) > max_area {
        //             max_area = h2 * (j as i32 - i as i32);
        //         }
        //     }
        // }
        // max_area
        
        // Sliding window solution
        let mut i = 0;
        let mut j = height.len() - 1;
        let mut max_area = 0;
        while i < j {
            if &height[i] <= &height[j] {
                if &height[i] * (j as i32 - i as i32) > max_area {
                    max_area = &height[i] * (j as i32 - i as i32);
                }
                i += 1;
            } else if &height[j] < &height[i] {
                if &height[j] * (j as i32 - i as i32) > max_area {
                    max_area = &height[j] * (j as i32 - i as i32);
                }
                j -= 1;
            }
        }
        max_area
    }
}