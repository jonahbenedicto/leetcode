// 84. Largest Rectangle in Histogram
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = Vec::new();
        let mut max_area = 0;
        let n = heights.len();
        
        for i in 0..=n {
            let current_height = if i == n { 0 } else { heights[i] };
            
            while !stack.is_empty() && current_height < heights[*stack.last().unwrap()] {
                let height_index = stack.pop().unwrap();
                let height = heights[height_index];
                
                let width = if stack.is_empty() {
                    i as i32
                } else {
                    (i - stack.last().unwrap() - 1) as i32
                };
                
                let area = height * width;
                max_area = max_area.max(area);
            }
            
            if i < n {
                stack.push(i);
            }
        }
        
        max_area
    }
}