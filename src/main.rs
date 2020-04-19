mod reverse_string;
mod palindrome;

fn main() {
  // reversing a string
  println!("Reversed: {}", reverse_string::reverse_string("A new string"));

  // palindrome checker
  println!("Reverse: {}", palindrome::is_palindrome("abcba"));
  println!("Reverse: {}", palindrome::is_palindrome("TomHanks"));
}
