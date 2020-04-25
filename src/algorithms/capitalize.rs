pub fn capitalize_every_word(text: &str) -> String {
  let mut cap_text = String::new();

  for word in text.split_whitespace() {
    for (i, char) in word.chars().enumerate() {
      if i == 0 {
        cap_text.push(char.to_ascii_uppercase());
      } else {
        cap_text.push(char);
      }
    }
    cap_text.push(' ');
  }
  cap_text.pop();

  cap_text
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn capitalize_words() {
    assert_eq!(
      capitalize_every_word(
        "hi there, \
    how is it going?"
      ),
      "Hi There, How Is It Going?",
      "Uppercase every first letter correctly"
    )
  }
}
