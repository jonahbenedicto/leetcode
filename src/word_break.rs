// 139. Word Break
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let n = s.len();
        let mut dp = vec![false; n + 1];
        dp[0] = true;
        
        let word_set: std::collections::HashSet<String> = word_dict.into_iter().collect();
        
        for i in 1..=n {
            for j in 0..i {
                if dp[j] && word_set.contains(&s[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }
        
        dp[n]
    }
}