lengthOfLongestSubstring(s) {
    let longest = 0;
    let tracker = new Set();
    let [l, r] = [0,0];
    while (r < s.length) {
        while (tracker.has(s[r])) {
            tracker.delete(s[l]);
            l += 1;
        }
        tracker.add(s[r]);
        longest = Math.max(longest, r-l+1);
        r += 1;
    }
    return longest;
}

//TC: O(n)
//SC: O(n) - for tracker 
