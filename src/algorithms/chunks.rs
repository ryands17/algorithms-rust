pub fn chunk_vec(arr: Vec<i32>, size: Option<u32>) -> Vec<Vec<i32>> {
  let usize = size.unwrap_or(2);
  let len = arr.len() as u32;
  if usize >= len {
    return vec![arr];
  }

  let mut chunks = Vec::new();

  let mut i = 0;
  let mut vec: Vec<i32> = Vec::new();
  for item in arr {
    vec.push(item);
    i += 1;

    if i == usize {
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
