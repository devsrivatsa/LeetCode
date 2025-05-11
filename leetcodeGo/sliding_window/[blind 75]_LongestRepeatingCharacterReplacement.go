func characterReplacement(s string, k int) int {
    l, r, maxCount, maxWin := 0, 0, 0, 0
    maxChar := s[l]
    charCountTracker := make(map[byte]int)
    for i := range s { charCountTracker[s[i]] = 0 }
    for {
        if r >= len(s) { break }

        //1. expand windwow by adding the next char into it
        charCountTracker[s[r]] += 1
        //2. check if added char influences the maxChar and maxCount and update
        if charCountTracker[s[r]] > maxCount {
            maxCount = charCountTracker[s[r]]
            maxChar = s[r]
        }
        //3. if there are more than allowed characters to be replaced, decrese window from left
        if r-l+1 - maxCount > k {
            charCountTracker[s[l]] -= 1
            if s[l] == maxChar {
                maxCount -= 1
            }
            l += 1
        }
        //4. calculate window size and increment r
        if r-l+1 > maxWin {
            maxWin = r-l+1
        }
        r += 1
    }
    return maxWin
}

//TC: O(n)
//SC: O(n)
