import "fmt"

func areSame(a, b map[byte]int) bool {
    if len(a) != len(b) { return false }
    for k,v := range a {
        if val, ok := b[k]; !ok || val != v {
            return false
        }
    }
    return true
}

func checkInclusion(s1 string, s2 string) bool {
    ls1, ls2 := len(s1), len(s2)
    if ls1 > ls2 { return false } //edge case

    m1 := make(map[byte]int,3)
    m2 := make(map[byte]int,3)
    l,r := 0, len(s1)
    
    //populating m1 from s1 and m2 from first window of s2
    for i := 0; i < ls1; i++ {
        m1[s1[i]]++
        m2[s2[i]]++
    }
    //check if the first window is a permutation of s1
    if areSame(m1, m2) { return true }
    
    for r < ls2 {
        // Remove the character s2[l] that is sliding out of the window and increment l
        m2[s2[l]] -= 1
        if m2[s2[l]] == 0 { 
            delete(m2, s2[l]) 
        }
        l++
        //add new character s2[r] into the window and increment
        m2[s2[r]]++
        //check if the new window is a permutation of s1, and increment r
        if areSame(m1, m2) {
            return true
        }
        r++
    }
    return false
}

//TC: O(n) where n is length of s2
//SC: O(2*m) where m is length of s1 - for the hashmaps
