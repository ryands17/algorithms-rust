pub struct Stack {
  top: Option<Box<Node>>,
  len: usize,
}

struct Node {
  value: i32,
  next: Option<Box<Node>>,
}

impl Stack {
  pub fn new(value: i32) -> Self {
    let node = Node { value, next: None };

    Stack {
      top: Some(Box::new(node)),
      len: 1,
    }
  }

  pub fn len(self) -> usize {
    self.len
  }

  pub fn push(&mut self, value: i32) {
    let node = Node {
      value,
      next: self.top.take(),
    };

    self.top = Some(Box::new(node));
    self.len += 1;
  }

  pub fn pop(&mut self) -> Option<i32> {
    match self.len {
      0 => None,
      _ => self.top.take().map(|node| {
        self.top = node.next;
        self.len -= 1;

        (*node).value
      }),
    }
  }

  pub fn peek(&mut self) -> Option<i32> {
    match self.len {
      0 => None,
      _ => self.top.take().map(|node| (*node).value),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn push_to_stack() {
    let mut s = Stack::new(21);
    s.push(22);

    assert_eq!(s.len, 2, "Length should be 2");
    assert_eq!(s.peek(), Some(22), "Length should be 2");
  }

  pub fn pop_from_stack() {
    let mut s = Stack::new(21);
    s.push(22);
    s.push(23);
    s.push(41);
    s.push(65);

    s.pop();

    assert_eq!(s.len, 4, "Length should be 4");
    assert_eq!(s.peek(), Some(41), "65 should be popped");
  }
}
