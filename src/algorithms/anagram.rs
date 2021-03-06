use std::collections::HashMap;

pub fn is_anagram(text1: &str, text2: &str) -> bool {
  let mut text1_map: HashMap<char, u32> = HashMap::new();
  let mut text2_map: HashMap<char, u32> = HashMap::new();

  for char in text1.chars() {
    let val = text1_map.entry(char).or_insert(0);
    *val += 1;
  }

  for char in text2.chars() {
    let val = text2_map.entry(char).or_insert(0);
    *val += 1;
  }

  for (key, value) in text1_map {
    if value != *text2_map.get(&key).unwrap_or(&0) {
      return false;
    }
  }

  true
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn is_an_anagram() {
    assert!(is_anagram("Whoa! Hi!", "Hi! Whoa!"));
  }

  #[test]
  fn is_not_an_anagram() {
    assert!(
      !is_anagram("A tree, a life, a bench", "A tree, a fence, a yard"),
      "Shouldn't be anagrams!"
    );
  }
}
