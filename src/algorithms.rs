mod anagram;
mod capitalize;
mod chunks;
mod max_chars;
mod palindrome;
mod reverse_string;
mod reverse_uint;

pub fn call() {
  // reversing a string
  println!(
    "Reversed: {}",
    reverse_string::reverse_string("A new string!")
  );

  // palindrome checker
  println!("Reverse: {}", palindrome::is_palindrome("abcba"));
  println!("Reverse: {}", palindrome::is_palindrome("TomHanks"));

  // reverse integers
  println!("Integer Reverse: {}", reverse_uint::reverse_int(5));
  println!("Integer Reverse: {}", reverse_uint::reverse_int(152));

  // max character occurring in a string
  println!(
    "Max occurring character: {}",
    max_chars::max_chars("a").unwrap()
  );
  println!(
    "Max occurring character: {}",
    max_chars::max_chars("abcdefghijklmnaaa").unwrap()
  );
  println!(
    "Max occurring character: {}",
    max_chars::max_chars("ab1c1d1e1f1g1").unwrap()
  );

  // chunk the vector!
  println!(
    "Chunking array: {:?}",
    chunks::chunk_vec(vec![1, 2, 3, 4, 5], 2)
  );
  println!(
    "Chunking array: {:?}",
    chunks::chunk_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13], 5)
  );

  // are the two strings anagrams of each other!
  println!(
    "Are anagrams: {}",
    anagram::is_anagram("Whoa! Hi!", "Hi! Whoa!")
  );
  println!(
    "Are anagrams: {}",
    anagram::is_anagram("A tree, a life, a bench", "A tree, a fence, a yard")
  );

  // capitalize every word!
  println!(
    "Capitalize: {}",
    capitalize::capitalize_every_word(
      "hi there, \
    how is it going?"
    )
  );
  println!(
    "Capitalize: {}",
    capitalize::capitalize_every_word("i love breakfast at bill miller bbq")
  );
}
