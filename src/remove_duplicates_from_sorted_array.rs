// 26. Remove Duplicates from Sorted Array
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut unique_count = 1;
        let mut last_unique = nums[0];

        for i in 1..nums.len() {
            if nums[i] != last_unique {
                nums[unique_count] = nums[i];
                last_unique = nums[i];
                unique_count += 1;
            }
        }

        nums.truncate(unique_count);
        unique_count as i32
    }
}