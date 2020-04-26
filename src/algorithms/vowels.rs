pub fn count_vowels(text: &str) -> u32 {
  let mut count = 0;
  let vowels = vec!['a', 'e', 'i', 'o', 'u'];

  for ch in text.chars() {
    if vowels.contains(&ch.to_ascii_lowercase()) {
      count += 1;
    }
  }

  count
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn counts_vowels() {
    assert_eq!(count_vowels("The Great Gatsby!"), 4);
  }

  #[test]
  fn count_uppercase() {
    assert_eq!(count_vowels("ASTEROID"), 4)
  }

  #[test]
  fn no_match() {
    assert_eq!(count_vowels("Rhythm"), 0);
  }
}
