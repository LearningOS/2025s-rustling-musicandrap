use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PartialOrd + Clone> LinkedList<T> {
    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self {
        let mut merged_list = LinkedList::new();
        let mut a_ptr = list_a.start;
        let mut b_ptr = list_b.start;

        while a_ptr.is_some() || b_ptr.is_some() {
            let next_val = match (a_ptr, b_ptr) {
                (Some(a), Some(b)) => {
                    let a_val = unsafe { &(*a.as_ptr()).val };
                    let b_val = unsafe { &(*b.as_ptr()).val };
                    if a_val <= b_val {
                        a_ptr = unsafe { (*a.as_ptr()).next };
                        a_val.clone()
                    } else {
                        b_ptr = unsafe { (*b.as_ptr()).next };
                        b_val.clone()
                    }
                }
                (Some(a), None) => {
                    let val = unsafe { &(*a.as_ptr()).val };
                    a_ptr = unsafe { (*a.as_ptr()).next };
                    val.clone()
                }
                (None, Some(b)) => {
                    let val = unsafe { &(*b.as_ptr()).val };
                    b_ptr = unsafe { (*b.as_ptr()).next };
                    val.clone()
                }
                (None, None) => break,
            };
            merged_list.add(next_val);
        }

        merged_list
    }
}