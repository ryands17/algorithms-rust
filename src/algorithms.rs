mod palindrome;
mod reverse_string;
mod reverse_uint;
mod max_chars;

pub fn call() {
  // reversing a string
  println!("Reversed: {}", reverse_string::reverse_string("A new string!"));

  // palindrome checker
  println!("Reverse: {}", palindrome::is_palindrome("abcba"));
  println!("Reverse: {}", palindrome::is_palindrome("TomHanks"));

  // reverse integers
  println!("Integer Reverse: {}", reverse_uint::reverse_int(5));
  println!("Integer Reverse: {}", reverse_uint::reverse_int(152));

  // max character occurring in a string
  println!("Max occurring character: {}", max_chars::max_chars("a").unwrap());
  println!("Max occurring character: {}", max_chars::max_chars
    ("abcdefghijklmnaaa").unwrap());
  println!("Max occurring character: {}", max_chars::max_chars
    ("ab1c1d1e1f1g1").unwrap());
}
