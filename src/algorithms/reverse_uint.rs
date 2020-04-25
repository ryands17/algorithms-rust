/// Reverses only **unsigned** integers
pub fn reverse_int(mut n: u32) -> u32 {
  let mut num = 0;
  while n != 0 {
    let md = n % 10;
    num = 10 * num + md;
    n = n / 10;
  }

  num
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn reverse_single_integer() {
    assert_eq!(
      reverse_int(5),
      5,
      "Should return the same single integer \
    correctly"
    );
  }

  #[test]
  fn reverse_large_integer() {
    assert_eq!(reverse_int(2438), 8342, "Should be correctly reversed");
  }
}
