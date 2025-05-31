// 55. Jump Game
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut max_reach = 0;
        
        for i in 0..n {
            if i > max_reach {
                return false;
            }
            
            max_reach = max_reach.max(i + nums[i] as usize);
            
            if max_reach >= n - 1 {
                return true;
            }
        }
        
        false
    }
}