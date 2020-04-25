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
