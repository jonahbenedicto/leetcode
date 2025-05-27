// 16. 3Sum Closest
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        let mut closest_sum = i32::MAX;

        for i in 0..n {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut left = i + 1;
            let mut right = n - 1;

            while left < right {
                let current_sum = nums[i] + nums[left] + nums[right];
                if (current_sum - target).abs() < (closest_sum - target).abs() {
                    closest_sum = current_sum;
                }

                if current_sum < target {
                    left += 1;
                } else if current_sum > target {
                    right -= 1;
                } else {
                    return current_sum;
                }
            }
        }
        closest_sum
    }
}