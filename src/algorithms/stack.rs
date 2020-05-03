pub struct Stack<T> {
  top: Option<Box<Node<T>>>,
  len: usize,
}

struct Node<T> {
  value: T,
  next: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
  pub fn new(value: T) -> Self {
    let node = Node { value, next: None };

    Stack {
      top: Some(Box::new(node)),
      len: 1,
    }
  }

  pub fn len(self) -> usize {
    self.len
  }

  pub fn push(&mut self, value: T) {
    let node = Node {
      value,
      next: self.top.take(),
    };

    self.top = Some(Box::new(node));
    self.len += 1;
  }

  pub fn pop(&mut self) -> Option<T> {
    match self.len {
      0 => None,
      _ => self.top.take().map(|node| {
        self.top = node.next;
        self.len -= 1;

        (*node).value
      }),
    }
  }

  pub fn peek(&mut self) -> Option<T> {
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
    assert_eq!(s.peek().unwrap(), 22, "Top element should be 22");
  }

  #[test]
  pub fn pop_from_stack() {
    let mut s = Stack::new(21);
    s.push(22);
    s.push(23);
    s.push(41);
    s.push(65);

    s.pop();

    assert_eq!(s.len, 4, "Length should be 4");
    assert_eq!(s.peek().unwrap(), 41, "65 should be popped so top => 45");
  }
}
