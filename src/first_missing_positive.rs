// 41. First Missing Positive
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut nums = nums;

        for i in 0..n {
            while nums[i] > 0 && nums[i] <= n as i32 && nums[nums[i] as usize - 1] != nums[i] {
                let temp = nums[i];
                nums[i] = nums[temp as usize - 1];
                nums[temp as usize - 1] = temp;
            }
        }

        for i in 0..n {
            if nums[i] != (i as i32 + 1) {
                return (i as i32 + 1);
            }
        }

        (n as i32 + 1)
    }
}