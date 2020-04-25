use std::collections::HashMap;

pub fn max_chars(str: &str) -> Option<char> {
  let mut chars: HashMap<char, i32> = HashMap::new();
  for ch in str.chars() {
    let val = chars.entry(ch).or_insert(0);
    *val += 1;
  }

  let mut max_number = &0;
  let mut char: Option<char> = None;
  for (key, val) in &chars {
    if val > max_number {
      max_number = val;
      char = Some(*key);
    }
  }

  char
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn max_single_char() {
    assert_eq!(
      max_chars("a").unwrap(),
      'a',
      "Should return the single char"
    );
  }

  #[test]
  fn max_char_long_str() {
    assert_eq!(
      max_chars(
        "Lorem \
    Ipsumissimplydummytextoftheprintingandtypesettingindustry."
      )
      .unwrap(),
      't',
      "A really long string"
    );
  }
}
