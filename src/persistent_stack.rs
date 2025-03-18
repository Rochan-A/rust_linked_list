/*
Goal:

list1 = A -> B -> C -> D
list2 = tail(list1) = B -> C -> D
list3 = push(list2, X) = X -> B -> C -> D

list1 -> A ---+
              |
              v
list2 ------> B -> C -> D
              ^
              |
list3 -> X ---+

Can't use Box because ownership is shared. If we drop list2, should B,C,D be freed??

Garbage collection would have saved us in high-level PLs.
Rust has reference counting (Rc which is like Box) but we can only take shared reference
to its interals.
*/

use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem,
                // clone() trait: generic way to get "another one like this one" that is logically
                // disjoint, given only a shared reference. It's like a copy constructor in C++.
                // Rc in particular uses Clone as the way to increment the reference count.
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    // Like peek
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

// Iter like mutable list
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

// Previously, we did this for drop. We can't now because this is mutating inside the Box, we now
// have shared reference.
// impl<T> Drop for List<T> {
//     fn drop(&mut self) {
//         let mut cur_link = mem::replace(&mut self.head, None);
//         while let Some(mut boxed_node) = cur_link {
//             cur_link = mem::replace(&mut boxed_node.next, None);
//         }
//     }
// }

// Use try_unwrap
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take(); // instead of mem::replace
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

/*
 Some notes on thread safety

In order to be thread-safe, we need to update with reference counts atomically.
Arc is completely identical to Rc except for the fact that reference counts are modified atomically

A type is Send if it's safe to move to another thread. A type is Sync if it's safe to share between
multiple threads.

marker traits, they're traits that provide absolutely no interface. You either are Send, or you
aren't. It's just a property other APIs can require.

something about interior (and inherited) mutability...

*/

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
