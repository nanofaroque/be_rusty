use std::collections::HashMap;
use std::cmp;

fn main() {
    let res = max_vowels(String::from("abciiidef"), 3);
    println!("{}",res);
}


pub fn max_vowels(s: String, k: i32) -> i32 {
    let mut map = HashMap::new();
    map.insert('a', 'a');
    map.insert('A', 'A');
    map.insert('e', 'e');
    map.insert('E', 'E');
    map.insert('i', 'i');
    map.insert('I', 'I');
    map.insert('o', 'o');
    map.insert('O', 'O');
    map.insert('u', 'u');
    map.insert('U', 'U');

    if s.len() < k as usize { return 0; }
    let chars: Vec<char> = s.chars().collect();

    let mut count = 0;
    for i in 0..k as usize {
        if map.contains_key(&chars[i]) {
            count += 1;
        }
    }

    let mut start = 1;
    let mut end = start + k - 1;
    let mut max_count = count;

    while end < chars.len() as i32 {
        if map.contains_key(&chars[start as usize - 1]) {
            count -= 1;
        }
        if map.contains_key(&chars[end as usize]) {
            count += 1;
        }
        max_count = cmp::max(max_count, count);
        start += 1;
        end += 1;
    }
    return max_count;
}