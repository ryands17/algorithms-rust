pub fn reverse_string(string: &str) -> String {
  let mut reversed = String::new();
  for ch in string.chars().rev() {
    reversed.push(ch);
  }

  reversed
}
