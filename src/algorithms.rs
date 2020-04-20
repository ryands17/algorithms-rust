pub mod palindrome;
pub mod reverse_string;
pub mod reverse_uint;

pub fn call() {
  // reversing a string
  println!("Reversed: {}", reverse_string::reverse_string("A new string!"));

  // palindrome checker
  println!("Reverse: {}", palindrome::is_palindrome("abcba"));
  println!("Reverse: {}", palindrome::is_palindrome("TomHanks"));

  // reverse integers
  println!("Integer Reverse: {}", reverse_uint::reverse_int(5));
  println!("Integer Reverse: {}", reverse_uint::reverse_int(152));
}
