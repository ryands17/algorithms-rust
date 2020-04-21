use std::collections::HashMap;

pub fn max_chars(str: &str) -> char {
  let mut chars : HashMap<char, i32> = HashMap::new();
  for ch in str.chars() {
    let val = chars.entry(ch).or_insert(0);
    *val += 1;
  }

  let mut max_number = &0;
  let mut char = 'a';
  for (key, val) in &chars {
    if val > max_number {
      max_number = val;
      char = *key;
    }
  }

  char
}