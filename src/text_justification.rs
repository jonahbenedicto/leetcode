// 68. Text Justification
impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut result = Vec::new();
        let mut i = 0;
        
        while i < words.len() {
            let mut j = i;
            let mut line_length = 0;
            
            while j < words.len() {
                let word_len = words[j].len();
                let spaces_needed = if j == i { 0 } else { 1 };
                
                if line_length + spaces_needed + word_len <= max_width {
                    line_length += spaces_needed + word_len;
                    j += 1;
                } else {
                    break;
                }
            }
            
            let num_words = j - i;
            let is_last_line = j == words.len();
            
            if is_last_line || num_words == 1 {
                let mut line = String::new();
                for k in i..j {
                    if k > i {
                        line.push(' ');
                    }
                    line.push_str(&words[k]);
                }
                while line.len() < max_width {
                    line.push(' ');
                }
                result.push(line);
            } else {
                let total_word_chars: usize = (i..j).map(|k| words[k].len()).sum();
                let total_spaces = max_width - total_word_chars;
                let gaps = num_words - 1;
                let spaces_per_gap = total_spaces / gaps;
                let extra_spaces = total_spaces % gaps;
                
                let mut line = String::new();
                for k in i..j {
                    line.push_str(&words[k]);
                    if k < j - 1 {
                        for _ in 0..spaces_per_gap {
                            line.push(' ');
                        }
                        if k - i < extra_spaces {
                            line.push(' ');
                        }
                    }
                }
                result.push(line);
            }
            
            i = j;
        }
        
        result
    }
}