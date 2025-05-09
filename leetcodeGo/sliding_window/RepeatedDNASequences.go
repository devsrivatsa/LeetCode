func findRepeatedDnaSequences(s string) []string {
    seqCount := make(map[string]int)
    var result []string
    l, r := 0, 10
    for {
        if r > len(s) {
            break
        }
        seq := s[l:r]
        seqCount[seq] += 1
        l += 1
        r += 1
    }
    for seq, count := range seqCount {
        if count > 1 {
            result = append(result, seq)
        }
    }
    return result
}

//TC: O(n)
//SC: O(n)
