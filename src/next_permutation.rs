// 31. Next Permutation
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        if n <= 1 {
            return;
        }

        let mut k = (n as i32) - 2;
        while k >= 0 && nums[k as usize] >= nums[(k + 1) as usize] {
            k -= 1;
        }

        if k < 0 {
            nums.reverse();
            return;
        }

        let k_usize = k as usize;
        let mut l = n - 1;
        while l > k_usize && nums[l] <= nums[k_usize] {
            l -= 1;
        }

        nums.swap(k_usize, l);

        nums[(k_usize + 1)..].reverse();
    }
}