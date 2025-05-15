/// LeetCode 4. Median of Two Sorted Arrays
pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // Brute force method
        // Time complexity: O(n + m)
        // Space complexity: O(n + m)
        let mut i = 0;
        let mut j = 0;
        let mut v = vec![];
        while i < nums1.len() || j < nums2.len() {
            if i == nums1.len() {
                v.push(nums2[j]);
                j += 1;
            } else if j == nums2.len() {
                v.push(nums1[i]);
                i += 1;
            } else if nums1[i] < nums2[j] {
                v.push(nums1[i]);
                i += 1;
            } else if nums1[i] > nums2[j] {
                v.push(nums2[j]);
                j += 1;
            } else {
                v.push(nums1[i]);
                v.push(nums2[j]);
                i += 1;
                j += 1;
            }
        }
        return if v.len() % 2 == 0 {
            (v[v.len() / 2] + v[v.len() / 2 - 1]) as f64 / 2.0
        } else {
            v[v.len() / 2] as f64
        }
    }
}