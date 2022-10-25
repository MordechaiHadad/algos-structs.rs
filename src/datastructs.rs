use std::ptr::NonNull;

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<NonNull<Node<T>>>,
}

#[derive(Debug)]
pub struct Queue<T> {
    pub length: i32,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
}

impl<T> Queue<T> where T: Clone {
    pub fn new() -> Queue<T> {
        Queue {
            length: 0,
            head: None,
            tail: None,
        }
    }
    pub fn enqueue(&mut self, item: T) {
        let node = Box::new(Node::new(item));
        self.length += 1;
        unsafe {
            let node = Some(Box::leak(node).into());
            match self.tail {
                Some(tail) => (*tail.as_ptr()).next = node,
                None => self.head = node,
            }
            self.tail = node;
        }
    }

    pub fn deque(&mut self) -> Option<T> {
        if let None = self.head {
            return None;
        }
        self.length -= 1;
        unsafe {
            let head = &mut *self.head?.as_ptr();
            self.head = (*self.head?.as_ptr()).next;

            head.next = None;
            return Some(head.value.clone());
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe { Some(&(*self.head?.as_ptr()).value) }
    }
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node { value, next: None }
    }
}
