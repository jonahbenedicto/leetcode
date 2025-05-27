// 4. Median of Two Sorted Arrays
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (mut nums1, mut nums2) = if nums1.len() > nums2.len() {
            (nums2, nums1)
        } else {
            (nums1, nums2)
        };

        let total_len = nums1.len() + nums2.len();
        let half = total_len / 2;

        let mut left = 0;
        let mut right = nums1.len();

        while left <= right {
            let mid1 = (left + right) / 2;
            let mid2 = half - mid1;

            let l1 = if mid1 == 0 { i32::MIN } else { nums1[mid1 - 1] };
            let r1 = if mid1 == nums1.len() { i32::MAX } else { nums1[mid1] };
            let l2 = if mid2 == 0 { i32::MIN } else { nums2[mid2 - 1] };
            let r2 = if mid2 == nums2.len() { i32::MAX } else { nums2[mid2] };

            if l1 > r2 {
                right = mid1 - 1;
            } else if l2 > r1 {
                left = mid1 + 1;
            } else {
                return if total_len % 2 == 0 {
                    (l1.max(l2) as f64 + r1.min(r2) as f64) / 2.0
                } else {
                    r1.min(r2) as f64
                };
            }
        }
        unreachable!()
    }
}