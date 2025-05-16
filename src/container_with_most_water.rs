/// LeetCode 11. Container With Most Water
pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        // Brute force solution // Not a solution on leetcode
        let mut max_area = 0;
        for (i, h1) in height.iter().enumerate() {
            for (j, h2) in height.iter().enumerate() {
                if i == j {
                    continue;
                }
                if h1 <= h2 && h1 * (j as i32 - i as i32) > max_area {
                    max_area = h1 * (j as i32 - i as i32);
                } else if h2 < h1 && h2 * (j as i32 - i as i32) > max_area {
                    max_area = h2 * (j as i32 - i as i32);
                }
            }
        }
        max_area
        // 
    }
}