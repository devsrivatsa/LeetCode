function findRepeatedDnaSequences(s: string): string[] {
    let result = [];
    let seqCount = {};
    let [l, r] =  [0, 10];
    while (r<=s.length) {
        let seq = s.slice(l,r);
        if (seq in seqCount) {
            seqCount[seq] += 1;
        } else {
            seqCount[seq] = 1;
        }
        l += 1;
        r += 1;
    }
    for (let [k, v] of Object.entries(seqCount)) {
        if (seqCount[k] > 1) {
            result.push(k)
        }
    }
    return result
};

//TC: O(n)
//SC: O9n)
