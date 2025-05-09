use std::collections::HashMap;
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut result = vec![];
        let mut seq_count: HashMap<&str, i32> = HashMap::new();
        let (mut l, mut r) = (0, 10);
        while r <= s.len() {
            let mut seq = &s[l..r];
            seq_count.entry(seq).and_modify(|count| *count += 1).or_insert(1);
            l += 1;
            r += 1;
        }
        //consumes the map
        for (seq, count) in seq_count {
            if count > 1 {
                result.push(seq.to_string())
            }
        }
        result
    }
}

//TC: O(n)
//SC: O(n)
