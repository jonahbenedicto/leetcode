// 66. Plus One
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result = digits;
        
        for i in (0..result.len()).rev() {
            if result[i] < 9 {
                result[i] += 1;
                return result;
            }
            result[i] = 0;
        }
        
        let mut new_result = vec![1];
        new_result.extend(result);
        new_result
    }
}