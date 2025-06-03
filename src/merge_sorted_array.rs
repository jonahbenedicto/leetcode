// 88. Merge Sorted Array
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m as usize;
        let mut j = n as usize;
        let mut k = (m + n) as usize;
        while i > 0 && j > 0 {
            k -= 1;
            if nums1[i - 1] > nums2[j - 1] {
                nums1[k] = nums1[i - 1];
                i -= 1;
            } else {
                nums1[k] = nums2[j - 1];
                j -= 1;
            }
        }
        while j > 0 {
            k -= 1;
            j -= 1;
            nums1[k] = nums2[j];
        }
    }
}