pub fn chunk_vec(arr: Vec<i32>, size: usize) -> Vec<Vec<i32>> {
  let len = arr.len();
  if size >= len {
    return vec![arr];
  }

  let mut chunks = Vec::new();

  let mut i = 0;
  let mut vec: Vec<i32> = Vec::new();
  for item in arr {
    vec.push(item);
    i += 1;

    if i == size {
      i = 0;
      chunks.push(vec.clone());
      vec.clear();
    }
  }

  if vec.len() > 0 {
    chunks.push(vec.clone());
  }

  chunks
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn chunk_basic() {
    let v = vec![vec![1, 2], vec![3, 4], vec![5]];
    assert_eq!(chunk_vec(vec![1, 2, 3, 4, 5], 2), v);
  }

  #[test]
  fn chunk_size_greater_than_vec() {
    let v = vec![vec![1, 2, 3, 4, 5]];
    assert_eq!(chunk_vec(vec![1, 2, 3, 4, 5], 6), v);
  }
}
