// 47. Permutations II
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut result = Vec::new();
        let mut current = Vec::new();
        let mut used = vec![false; nums.len()];
        
        nums.sort();
        
        Self::backtrack(&nums, &mut current, &mut used, &mut result);
        
        result
    }
    
    fn backtrack(
        nums: &Vec<i32>,
        current: &mut Vec<i32>,
        used: &mut Vec<bool>,
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
            
            if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
                continue;
            }
            
            used[i] = true;
            current.push(nums[i]);
            
            Self::backtrack(nums, current, used, result);
            
            current.pop();
            used[i] = false;
        }
    }
}