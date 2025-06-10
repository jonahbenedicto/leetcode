// 3442. Maximum Difference Between Even and Odd Frequency I
impl Solution {
    pub fn max_difference(s: String) -> i32 {
        use std::collections::HashMap;
        
        let mut freq_map = HashMap::new();
        for ch in s.chars() {
            *freq_map.entry(ch).or_insert(0) += 1;
        }
        
        let mut max_odd_freq = 0;
        let mut min_even_freq = i32::MAX;
        
        for &freq in freq_map.values() {
            if freq % 2 == 1 { 
                max_odd_freq = max_odd_freq.max(freq);
            } else { 
                min_even_freq = min_even_freq.min(freq);
            }
        }
        
        max_odd_freq - min_even_freq
    }
}