// 85. Maximal Rectangle
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }
        
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut heights = vec![0; cols];
        let mut max_area = 0;
        
        for row in 0..rows {
            for col in 0..cols {
                if matrix[row][col] == '1' {
                    heights[col] += 1;
                } else {
                    heights[col] = 0;
                }
            }
            
            max_area = max_area.max(Self::largest_rectangle_area(&heights));
        }
        
        max_area
    }
    
    fn largest_rectangle_area(heights: &[i32]) -> i32 {
        let mut stack = Vec::new();
        let mut max_area = 0;
        let n = heights.len();
        
        for i in 0..=n {
            let h = if i == n { 0 } else { heights[i] };
            
            while let Some(&top) = stack.last() {
                if heights[top] <= h {
                    break;
                }
                
                let height = heights[stack.pop().unwrap()];
                let width = if stack.is_empty() { i } else { i - stack.last().unwrap() - 1 };
                max_area = max_area.max(height * width as i32);
            }
            
            stack.push(i);
        }
        
        max_area
    }
}