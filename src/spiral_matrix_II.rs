// 59. Spiral Matrix II
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut matrix = vec![vec![0; n]; n];
        let (mut left, mut right) = (0, n - 1);
        let (mut top, mut bottom) = (0, n - 1);
        let mut num = 1;
        
        while left <= right && top <= bottom {
            for j in left..=right {
                matrix[top][j] = num;
                num += 1;
            }
            top += 1;
            
            for i in top..=bottom {
                matrix[i][right] = num;
                num += 1;
            }
            
            if right == 0 {
                break;
            }
            right -= 1;
            
            if top <= bottom {
                for j in (left..=right).rev() {
                    matrix[bottom][j] = num;
                    num += 1;
                }
                bottom -= 1;
            }
            
            if left <= right {
                for i in (top..=bottom).rev() {
                    matrix[i][left] = num;
                    num += 1;
                }
                left += 1;
            }
        }
        
        matrix
    }
}