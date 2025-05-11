//logic: you can replace upto k characters - any characters.

use std::cmp;
use std::collections::HashMap;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        if s.len() == 1 { return 1 }
        let sv:Vec<char> = s.chars().collect();
        let (mut l, mut r, mut most_freq_char_count, mut max_win_size) = (0usize, 0usize, 0i32, 0usize);
        let mut most_freq_char:char = sv[l];
        let mut hm:HashMap<char, i32> = HashMap::new();
        while r < s.len() {
            //expand window
            *hm.entry(sv[r]).or_insert(0) += 1;
            
            //condition 1: check if the current char is the most_freq one
            if hm[&sv[r]] > most_freq_char_count { 
                most_freq_char_count = hm[&sv[r]];
                most_freq_char = sv[r];
            }

            //condition 2: check if the max replacable characters in the current window is exceeding the allowed value = k
            if ((r-l) as i32 + 1) - most_freq_char_count > k {
            //reduce the window size
                //1. adjust hm
                hm.entry(sv[l]).and_modify(|count| *count -= 1);
                //2. adjust most_freq_char_count
                if sv[l] == most_freq_char { most_freq_char_count -= 1; }
                //3. adjust window
                l += 1;
            }
            //calculate current window size and expand
            max_win_size = cmp::max(max_win_size, r-l+ 1);
            r += 1;
            // println!("max win size {}", max_win_size);
        }
        return max_win_size as i32
    }
}

//tc: O(n)
//sc: O(n) for the hashmap
