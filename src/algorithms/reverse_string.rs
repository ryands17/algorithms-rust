pub fn reverse_string(string: &str) -> String {
  let mut reversed = String::new();
  for ch in string.chars().rev() {
    reversed.push(ch);
  }

  reversed
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_reverse_string() {
    assert_eq!(
      reverse_string("This is great"),
      "taerg si sihT",
      "Didn't reverse the strings correctly"
    );
  }
}
