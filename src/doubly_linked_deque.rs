/*
 RefCell

 - fn borrow(&self) -> Ref<'_, T>           (like &)
 - fn borrow_mut(&self) -> RefMut<'_, T>    (like &mut, requires exclusivity)
*/

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        // new node needs +2 links, everything else should be 0
        let new_head = Node::new(elem);

        match self.head.take() {
            Some(curr_head) => {
                // non-empty list, so connect new_head to curr_head
                curr_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(curr_head);
                self.head = Some(new_head);
            }
            None => {
                // empty list
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|curr_head| {
            match curr_head.borrow_mut().next.take() {
                Some(new_head) => {
                    // -1 ref count on old head
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    // -1 ref count to empty list
                    self.tail.take();
                }
            }
            /*
            1. need something that takes a RefCell<T> and gives us a T
                into_inner()
            2. into_inner wants to move out the RefCell, but we can't, because it's in an Rc, use
               Rc::try_unwrap(), which moves out the contents of an Rc if its refcount is 1
            3. Rc::try_unwrap returns a Result<T, Rc<T>>. Results are basically a generalized
               Option, where the None case has data associated with it.
            4. unwrap on Result requires that you can debug-print the error case. RefCell<T> only
               implements Debug if T does. Node doesn't implement Debug. Rather than doing that,
               let's just work around it by converting the Result to an Option with ok.
            */
            Rc::try_unwrap(curr_head).ok().unwrap().into_inner().elem
        })
    }

    /*
    Ref and RefMut implement Deref and DerefMut respectively. So for most intents and purposes they
    behave exactly like &T and &mut T. However, because of how those traits work, the reference
    that's returned is connected to the lifetime of the Ref, and not the actual RefCell. This means
    that the Ref has to be sitting around as long as we keep the reference around.
     */
    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            /* just like you can map over an Option, you can map over a Ref. */
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    /* --------------- *_back versions of the above methods ----------------- */

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);

        match self.tail.take() {
            Some(curr_tail) => {
                curr_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(curr_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                // order matters here.
                self.tail = Some(new_tail);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|curr_tail| {
            match curr_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(curr_tail).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }
}

// Destructor.
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // pop_front till list is empty.
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        // ---- back -----

        // Check empty list behaves right
        assert_eq!(list.pop_back(), None);

        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
    }
}
