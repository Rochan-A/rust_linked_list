use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    // Link is either empty or contains a pointer (Box) to a Node.
    Empty,
    More(Box<Node>),
}
struct Node {
    // Struct allows holding both values.
    elem: i32,
    next: Link,
}

// Associating code with type List using impl
impl List {
    // static function for the type
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    // self -> Value (true owenership)
    // &mut self -> mutable reference (temporary exclusive access to a value we don't own)
    //              can't move the value out without a replacement.
    // &self -> shared reference (*should't* mutate the value (but it is not immutable))

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        let result;
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => {
                result = None;
            }
            Link::More(node) => {
                result = Some(node.elem);
                self.head = node.next;
            }
        }
        return result;
    }
}

/*
 type has a destructor if it implements a trait called Drop.
 Traits are Rust's fancy term for interfaces.
*/

// ******************************
// Explicit implementation of Drop

// impl Drop for List {
//     fn drop(&mut self) {
//         // Not possible to call drop in Rust. This is an example.
//         self.head.drop();
//     }
// }

// impl Drop for Link {
//     fn drop(&mut self) {
//         match *self {
//             Link::Empty => {} // nothing to do
//             Link::More(ref mut boxed_node) => {
//                 boxed_node.drop(); // tail recursive -- good!
//             }
//         }
//     }
// }

// impl Drop for Box<Node> {
//     fn drop(&mut self) {
//         self.ptr.drop();
//         dealloc(self.ptr);
//     }
// }

// impl Drop for Node {
//     fn drop(&mut self) {
//         self.next.drop();
//     }
// }
// ******************************

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
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

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
