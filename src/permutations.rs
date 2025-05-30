// 46. Permutations
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut current = Vec::new();
        let mut used = vec![false; nums.len()];
        
        Self::backtrack(&nums, &mut used, &mut current, &mut result);
        
        result
    }
    
    fn backtrack(
        nums: &[i32],
        used: &mut [bool],
        current: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>
    ) {
        if current.len() == nums.len() {
            result.push(current.clone());
            return;
        }
        
        for i in 0..nums.len() {
            if used[i] {
                continue;
            }
            
            current.push(nums[i]);
            used[i] = true;
            
            Self::backtrack(nums, used, current, result);
            
            current.pop();
            used[i] = false;
        }
    }
}