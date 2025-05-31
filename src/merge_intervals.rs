// 56. Merge Intervals
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }
        
        intervals.sort_by_key(|interval| interval[0]);
        
        let mut result = Vec::new();
        let mut current = intervals[0].clone();
        
        for i in 1..intervals.len() {
            let interval = &intervals[i];
            
            if current[1] >= interval[0] {
                current[1] = current[1].max(interval[1]);
            } else {
                result.push(current);
                current = interval.clone();
            }
        }
        
        result.push(current);
        
        result
    }
}