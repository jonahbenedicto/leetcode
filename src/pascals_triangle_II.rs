// 119. Pascal's Triangle II
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut row = vec![1; (row_index + 1) as usize];
        
        for i in 1..=row_index {
            for j in (1..i as usize).rev() {
                row[j] = row[j] + row[j - 1];
            }
        }
        
        row
    }
}