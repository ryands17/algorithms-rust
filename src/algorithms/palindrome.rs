pub fn is_palindrome(string: &str) -> bool {
  let str_vec: Vec<char> = string.chars().collect();
  let vec_len = str_vec.len();
  let to_iter = if vec_len % 2 == 0 {
    vec_len / 2
  } else {
    (vec_len / 2) + 1
  };

  for i in 0..to_iter {
    if str_vec.get(i).unwrap().to_ascii_lowercase()
      != str_vec.get(vec_len - i - 1).unwrap().to_ascii_lowercase()
    {
      return false;
    }
  }
  true
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_be_palindrome() {
    assert!(is_palindrome("Malayalam"));
  }

  #[test]
  fn should_not_be_palindrome() {
    assert!(!is_palindrome("TomHanks"));
  }
}
