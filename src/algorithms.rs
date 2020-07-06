mod anagram;
mod capitalize;
mod chunks;
mod collatz;
mod matrix;
mod max_chars;
mod palindrome;
mod reverse_string;
mod reverse_uint;
mod stack;
mod vowels;

pub fn call() {
  // reversing a string
  println!(
    "Reversed: {}\n",
    reverse_string::reverse_string("A new string!")
  );

  // palindrome checker
  println!("Reverse: {}\n", palindrome::is_palindrome("radar"));

  // reverse integers
  println!("Integer Reverse: {}\n", reverse_uint::reverse_int(152));

  // max character occurring in a string
  println!(
    "Max occurring character: {}\n",
    max_chars::max_chars("ab1c1d1e1f1g1").unwrap()
  );

  // chunk the vector!
  println!(
    "Chunk the array: {:?}\n",
    chunks::chunk_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13], 5)
  );

  // are the two strings anagrams of each other!
  println!(
    "Are anagrams: {}\n",
    anagram::is_anagram("Whoa! Hi!", "Hi! Whoa!")
  );

  // capitalize every word!
  println!(
    "Capitalize: {}\n",
    capitalize::capitalize_every_word("i love breakfast at bill miller bbq")
  );

  println!(
    "Vowel count: {}\n",
    vowels::count_vowels("A simple statement")
  );

  // stack em up
  let mut st = stack::Stack::new(21);
  st.push(30);
  st.push(45);
  st.push(56);
  st.pop();
  println!("Top value: {}", st.peek().unwrap());
  println!("Stack length: {}\n", st.len());

  // transposing matrices
  let v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
  println!("Transpose: {:?}", matrix::transpose(&v));

  let v = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]];
  println!("Transpose: {:?}\n", matrix::transpose(&v));

  // collatz count steps
  let n = 7;
  println!("Steps for {}: {}", n, collatz::steps(n));

  let n = 28;
  println!("Steps for {}: {}\n", n, collatz::steps(n));
}
