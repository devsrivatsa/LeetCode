use std::collections::HashMap;

impl Solution {
    // pub fn check_inclusion(s1: String, s2: String) -> bool {
    //     //edge case
    //     if s1.len() > s2.len() { return false }
        
    //     let (mut l, mut r):(usize, usize) = (0, s1.len());
    //     let mut s2vec: Vec<char> = s2.chars().collect();
    //     let (mut hm1, mut hm2) = (HashMap::new(), HashMap::new());
        
    //     for i in s2vec[0..r].iter() { 
    //         *hm2.entry(*i).or_insert(0) += 1; 
    //     }
    //     for i in s1.chars() { 
    //         *hm1.entry(i).or_insert(0) += 1; 
    //     }
    //     while r < s2vec.len() {
    //         if hm2 == hm1 { 
    //             return true 
    //         }
            
    //         //update window
    //         hm2.entry(s2vec[l]).and_modify(|c| *c-=1);
    //         if hm2[&s2vec[l]] == 0 { 
    //             hm2.remove(&s2vec[l]); 
    //         }
    //         l+=1;
    //         hm2.entry(s2vec[r]).and_modify(|c| *c+=1 ).or_insert(1);
    //         r+=1;
    //     }
    //     hm2 == hm1 //after traversing the list, we want to check for the last window
    // }
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() { return false }
        let s1_chars:Vec<char> = s1.chars().collect();
        let s2_chars:Vec<char> = s2.chars().collect();
        let (mut l, mut r):(usize, usize) = (0, s1.len());
        let (mut m1, mut m2) = (HashMap::new(), HashMap::new());
        for i in 0..s1.len() {
            *m1.entry(s1_chars[i]).or_insert(0) += 1;
            *m2.entry(s2_chars[i]).or_insert(0) += 1;
        }
        //check if the first window is a permutation of s1
        if m1 == m2 { return true }
        while r < s2.len() {
            //slide the left most char out of window and make adjustments to the hashmap
            if let Some(count) = m2.get_mut(&s2_chars[l]) {
                *count -= 1;
                if *count == 0 {
                    m2.remove(&s2_chars[l]);
                }
            }
            l+=1;
            //add the next char to the window, and incement r for next iteration
            m2.entry(s2_chars[r]).and_modify(|c| *c+=1).or_insert(1);
            r+=1;
            
            //check if the window is a permutation of s1
            if m1 == m2 { return true }
        }
        return false
    }
}

//TC: O(n) where n is length of s2
//SC: O(2*m) where m is length of s1 - for the hashmaps
