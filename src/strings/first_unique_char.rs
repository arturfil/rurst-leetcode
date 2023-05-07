use std::collections::HashMap;
// ex: "leetcode", 0
// ex2: "loveleetcode", 2
pub fn first_uniq_char(s: String) -> i32 {
    let  mut map: HashMap<char, i32> = HashMap::new(); 

    for c in s.chars() {
        if !map.contains_key(&c) {
            map.insert(c, 0);
        } else {
            let val = map.get(&c).unwrap();
            map.insert(c, val + 1);
        }
    }

    for (c, i) in s.chars().zip(0..s.len()) {
        if map.get(&c) == Some(&0) { return i as i32; }
    }
    -1
}
