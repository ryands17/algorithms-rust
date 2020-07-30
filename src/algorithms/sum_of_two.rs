use std::collections::HashMap;

pub fn does_match(vec1: &Vec<i32>, vec2: &Vec<i32>, sum: i32) -> Option<(i32, i32)> {
  let mut difference: HashMap<i32, i32> = HashMap::new();
  for &number in vec1 {
    difference.insert(sum - number, number);
  }

  for number in vec2 {
    if difference.contains_key(number) {
      return Some((*difference.get(number).unwrap(), *number));
    }
  }

  None
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sum_of_two_matches() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![10, 20, 30, 40];
    assert_eq!(does_match(&vec1, &vec2, 42), Some((2, 40)));

    let vec1 = vec![0, 0, -5, 30212];
    let vec2 = vec![-10, 40, -3, 9];
    assert_eq!(does_match(&vec1, &vec2, -8), Some((-5, -3)));
  }

  #[test]
  fn sum_of_two_doesnt_match() {
    let vec1 = vec![0, 0, -6, 30212];
    let vec2 = vec![-10, 40, -3, 9];
    assert_eq!(does_match(&vec1, &vec2, -8), None);
  }
}
