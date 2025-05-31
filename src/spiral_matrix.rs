// 54. Spiral Matrix
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() {
            return vec![];
        }
        
        let mut result = Vec::new();
        let (mut top, mut right, mut bottom, mut left) = (0, matrix[0].len() - 1, matrix.len() - 1, 0);
        
        while top <= bottom && left <= right {
            for j in left..=right {
                result.push(matrix[top][j]);
            }
            top += 1;
            
            for i in top..=bottom {
                result.push(matrix[i][right]);
            }
            
            if right == 0 {
                break;
            }
            right -= 1;
            
            if top <= bottom {
                for j in (left..=right).rev() {
                    result.push(matrix[bottom][j]);
                }
                bottom -= 1;
            }
            
            if left <= right {
                for i in (top..=bottom).rev() {
                    result.push(matrix[i][left]);
                }
                left += 1;
            }
        }
        
        result
    }
}