// 45. Jump Game II
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 1 {
            return 0;
        }
        
        let mut jumps = 0;
        let mut current_jump_end = 0;
        let mut farthest = 0;
        
        for i in 0..n - 1 {
            farthest = farthest.max(i + nums[i] as usize);
            
            if i == current_jump_end {
                jumps += 1;
                current_jump_end = farthest;
                
                if current_jump_end >= n - 1 {
                    break;
                }
            }
        }
        
        jumps
    }
}