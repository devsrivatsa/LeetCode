function characterReplacement(s: string, k: number): number {
    
    if (s.length == 1) {return 1}
    let [maxWin, l, r, maxCharCount, maxChar] = [0, 0, 0, 0, s[0]];
    let charCountTracker = {};
    while (r < s.length) {
        // 1. add new char to window
        if (s[r] in charCountTracker) {
            charCountTracker[s[r]] += 1;
        } else {
            charCountTracker[s[r]] = 1;
        }
        //2. check if the added char impacts the macCharCount and update it
        if (charCountTracker[s[r]] > maxCharCount) {
            maxCharCount = charCountTracker[s[r]];
            maxChar = s[r];
        }
        //3. if window contains more chars than what we are allowed to replace, then decrease window size from left
        if ((r-l+1) - maxCharCount > k) {
            charCountTracker[s[l]] -= 1;
            if (s[l] === maxChar) {
                maxCharCount -= 1;
            }
            l += 1;
        }
        //4. update maxWin and increment r
        maxWin = Math.max(r-l+1, maxWin);
        r += 1;
    }
    return maxWin
};

//TC, SC: O(n)
