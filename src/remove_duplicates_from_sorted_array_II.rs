// 80. Remove Duplicates from Sorted Array II
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }
        
        let mut write_pos = 2;
        
        for read_pos in 2..nums.len() {
            if nums[read_pos] != nums[write_pos - 2] {
                nums[write_pos] = nums[read_pos];
                write_pos += 1;
            }
        }
        
        write_pos as i32
    }
}