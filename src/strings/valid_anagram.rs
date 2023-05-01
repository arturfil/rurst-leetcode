
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() { return false; }
    let mut arr: [u32 ; 26] = [0 ; 26 ];

    for c in s.chars() {
        arr[c as usize - 'a' as usize] += 1;
    }
    for c in t.chars() {
        if arr[c as usize - 'a' as usize] == 0 { return false }
        arr[c as usize - 'a' as usize] -= 1
    }
    
    true
}
