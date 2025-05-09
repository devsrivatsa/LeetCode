func lengthOfLongestSubstring(s string) int {
    l,r,longest := 0,0,0
    tracker := make(map[byte]byte)
    for {
        if r >= len(s) {
            break
        }
        for {
            if _, present := tracker[s[r]]; !present {
                break
            } else {
                delete(tracker, s[l])
                l += 1
            }
        }
        tracker[s[r]] = s[r]
        if longest < r - l + 1 {
            longest = r - l + 1
        }
        r += 1
    }
    return longest
}

//TC: O(n)
//SC: O(n) - for the hashset
