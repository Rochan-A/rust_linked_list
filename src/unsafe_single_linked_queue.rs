use std::mem;

pub struct List<T> {
    head: Link<T>,
    // Won't work. FIXME.
    tail: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List {
            head: None,
            tail: None,
        }
    }
}
