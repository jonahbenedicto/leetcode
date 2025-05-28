// 39. Combination Sum
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut current = vec![];
        
        let mut sorted_candidates = candidates.clone();
        sorted_candidates.sort();
        
        Self::backtrack(&sorted_candidates, target, 0, &mut current, &mut result);
        
        result
    }
    
    fn backtrack(candidates: &[i32], target: i32, start: usize, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if target == 0 {
            result.push(current.clone());
            return;
        }
        
        for i in start..candidates.len() {
            if candidates[i] > target {
                break;
            }
            current.push(candidates[i]);
            Self::backtrack(candidates, target - candidates[i], i, current, result);
            current.pop();
        }
    }
}