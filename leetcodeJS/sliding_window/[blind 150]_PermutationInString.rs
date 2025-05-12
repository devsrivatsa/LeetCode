function areSame(m1: Object, m2: Object): boolean {
    for (const k in m1) {
        if (!(k in m2) || m2[k] !== m1[k]) {
            return false;
        }
    }    
    return true;
}

function checkInclusion(s1: string, s2: string): boolean {
    let [l, r] = [0, s1.length];
    let s2a = s2.split(''); 
    let s1a = s1.split('');
    let [m1, m2] = [{}, {}];
    
    for (let i=0; i<s1.length; i++) {
        if (s1[i] in m1) {
            m1[s1[i]]  += 1;
        } else {
            m1[s1[i]] = 1;
        }
        
        if (s2[i] in m2) {
            m2[s2[i]]  += 1;
        } else {
            m2[s2[i]] = 1;
        }
    }

    if (areSame(m1, m2)) {
        return true
    }

    while (r < s2.length) {
        m2[s2[l]]--;
        if (m2[s2[l]] === 0) {
            delete m2[s2[l]];
        }
        l++;

        if (s2[r] in m2) {
            m2[s2[r]]++;
        } else {
            m2[s2[r]] = 1;
        }
        r++;

        if (areSame(m1, m2)) {
            return true;
        }
    }
    return false;
