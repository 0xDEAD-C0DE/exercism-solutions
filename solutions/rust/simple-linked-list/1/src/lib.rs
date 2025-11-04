#[derive(Debug)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, _element: T) {
        let new_node = Box::new(Node {
            data: _element,
            next: self.head.take(),
        });
        self.len += 1;
        self.head = Some(new_node)
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            match self.head.take() {
                // -> mem::replace(&mut self.head, None) {
                Some(node) => {
                    self.len -= 1;
                    self.head = node.next;
                    Some(node.data)
                }
                None => None,
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            match &self.head {
                // -> mem::replace(&mut self.head, None) {
                Some(node) => {
                    // self.head = node.next;
                    Some(&node.data)
                }
                None => None,
            }
        }
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();
        let temp_vec: Vec<_> = self.into();

        for i in temp_vec.into_iter().rev() {
            list.push(i);
        }
        list
    }
}

impl<T> Iterator for SimpleLinkedList<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = Self::new();
        for i in _iter {
            list.push(i);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut as_vec: Vec<T> = vec![];
        while !_linked_list.is_empty() {
            as_vec.push(_linked_list.pop().unwrap());
        }
        as_vec.into_iter().rev().collect()
    }
}
