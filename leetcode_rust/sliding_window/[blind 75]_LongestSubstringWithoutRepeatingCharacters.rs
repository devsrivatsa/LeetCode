use std::collections::HashSet;
use std::cmp;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut sv:Vec<char> = s.chars().collect();
        let (mut maxlen, mut l, mut r) = (0,0,0);
        let mut res:HashSet<char> = HashSet::new();
        while r < s.len() {
            while res.contains(&sv[r]) {
                res.remove(&sv[l]);
                l += 1;
            }
            res.insert(sv[r].clone());
            maxlen = cmp::max(maxlen, r-l+1);
            r+=1
        }
        return maxlen as i32
    }
}

//TC: O(n)
//SC: O(n) - for the hashset
