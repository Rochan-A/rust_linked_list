use std::mem;

pub struct List<T> {
    head: Link<T>,
}

// Type aliasing
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, None),
        });
        self.head = Some(new_node);
    }

    // pub fn pop(&mut self) -> Option<i32> {
    //     match mem::replace(self.head, None) {
    //         None => None,
    //         Link::Some(node) => {
    //             self.head = node.next;
    //             Some(node.elem)
    //         }
    //     }
    // }

    // x = mem::replace(option, None) can be x = option.take()

    /*
    match option {None => None, Some(x) => Some(y)} can be

    using Rust closure
    */
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    // Taking a shared reference and returning a shared reference.
    pub fn peek(&self) -> Option<&T> {
        // NOTE: as_ref()
        self.head.as_ref().map(|node| &node.elem)
    }

    // Mutable version of peek.
    // Taking a mutable reference and returning a mutable reference.
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        // NOTE: as_mut()
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None);
        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
        }
    }
}

/*
 Collections are iterated using the iterator trait

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
every implementation of Iterator has an associated type called Item.

    IntoIter - T
    IterMut - &mut T
    Iter - &T
*/

// Tuple structs are an alternative form of struct,
// useful for trivial wrappers around other types.

// IntoIter<T> is a wrapper for List<T> and we are implementing the Iterator for this.
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // 0 is indexing the first item in the struct (?) i.e., List<T>
        self.0.pop()
    }
}

// Generic over *some* lifetime, it doesn't care
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

// No need for lifetimes here
impl<T> List<T> {
    // declare fresh lifetime for the *exact* borrow that creates the iter.
    // Now &self needs to be valid as long as the Iter is around.
    // But lifetime elision can occur so no need to be explicit about it.
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

// we do have a lifetime here, because Iter has one that we need to define
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    // No need to change this because the above handles it
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
    /*
    This is basically

    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;

        fn next<'b>(&'b mut self) -> Option<&'a T> { /* stuff */ }
    }
    */
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // Need to use take() because this is mutable reference which is not Copy.
        // take() gives us exclusive ownership of the mut reference.
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_linked_list() {
        let mut list = List::new();

        // ! at the end is a macro, not logical not
        assert_eq!(list.pop(), None);
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        assert_eq!(list.pop(), Some(3));
        list.peek_mut().map(|value| *value = 4);
        assert_eq!(list.pop(), Some(4));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
