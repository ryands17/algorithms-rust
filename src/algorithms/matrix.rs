pub fn transpose(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
  let m_size = matrix.len();
  let n_size = matrix[0].len();
  let mut vec = vec![vec![0; m_size]; n_size];

  for i in 0..n_size {
    for j in 0..m_size {
      vec[i][j] = matrix[j][i];
    }
  }
  vec
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_on_square_matrices() {
    let v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    assert_eq!(
      transpose(&v),
      vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
    );
  }

  #[test]
  fn works_on_rectangular_matrices() {
    let v = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]];
    assert_eq!(
      transpose(&v),
      vec![vec![1, 5], vec![2, 6], vec![3, 7], vec![4, 8]]
    );
  }
}
