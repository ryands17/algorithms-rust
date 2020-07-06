pub fn steps(number: u64) -> u32 {
  let mut number = number;
  let mut i = 0;

  loop {
    if number % 2 == 0 {
      number = number / 2;
    } else {
      number = number * 3 + 1;
    }
    i += 1;

    if number == 1 {
      break;
    }
  }

  i
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn gets_correct_steps_for_an_odd_number() {
    assert_eq!(steps(7), 16);
  }

  #[test]
  fn gets_correct_steps_for_an_even_number() {
    assert_eq!(steps(28), 18);
  }
}
