use std::{ptr::NonNull, marker::PhantomData};

pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len:  usize,
    marker: PhantomData<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len:  0,
            marker: PhantomData,
        }
    }

    #[inline]
    unsafe fn push_front_node(&mut self, node: NonNull<Node<T>>) {
        // This method takes care not to create mutable references to whole nodes,
        // to maintain validity of aliasing pointers into `element`.
        unsafe {
            (*node.as_ptr()).next = self.head;
            (*node.as_ptr()).prev = None;
            let node = Some(node);

            match self.head {
                None       => self.tail = node,
                Some(head) => (*head.as_ptr()).prev = node,
            }

            self.head = node;
            self.len += 1;
        }
    }

    #[inline]
    fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.map(|node| unsafe {
            // This method takes care not to create mutable references to whole nodes,
            // to maintain validity of aliasing pointers into `element`.
            let node = Box::from_raw(node.as_ptr());
            self.head = node.next;

            match self.head {
                None       => self.tail = None,
                Some(head) => (*head.as_ptr()).prev = None,
            }

            self.len -= 1;
            node
        })
    }

    #[inline]
    unsafe fn push_back_node(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            (*node.as_ptr()).next = None;
            (*node.as_ptr()).prev = self.tail;
            let node = Some(node);

            match self.tail {
                None       => self.head = node,
                Some(tail) => (*tail.as_ptr()).next = node,
            }

            self.head = node;
            self.len  += 1;
        }
    }

    #[inline]
    fn pop_back_node(&mut self) -> Option<Box<Node<T>>> {
        // This method takes care not to create mutable references to whole nodes,
        // to maintain validity of aliasing pointers into `element`.
        self.tail.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.tail = node.prev;

            match self.tail {
                None       => self.head = None,
                Some(tail) => (*tail.as_ptr()).next = None,
            }

            node
        })
    }
}

impl<T> LinkedList<T> {
    pub fn push_front(&mut self, element: T) {
        let node = Box::new(Node::new(element));
        let node_ptr = NonNull::from(Box::leak(node));

        // SAFETY: node_ptr is a unique pointer to a node
        unsafe {
            self.push_front_node(node_ptr)
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node().map(Node::into_element)
    }

    pub fn push_back(&mut self, element: T) {
        let node = Box::new(Node::new(element));
        let node_ptr = NonNull::from(Box::leak(node));

        // SAFETY: node_ptr is a unique pointer to a node
        unsafe {
            self.push_back_node(node_ptr)
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_back_node().map(Node::into_element)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        struct DropGuard<'a, T>(&'a mut LinkedList<T>);
        impl<'a, T> Drop for DropGuard<'a, T> {
            fn drop(&mut self) {
                // Continue the same loop we do below. This only runs when a destructor has
                // panicked. If another one panics this will abort.
                while self.0.pop_front_node().is_some() { }
            }
        }

        // Wrap self so that if a destructor panics, we can try to keep looping
        let guard = DropGuard(self);
        while guard.0.pop_front_node().is_some() {}
        std::mem::forget(guard)
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}


// ----------------------------------------------------------------------------

pub struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
    element: T,
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        Self { next: None, prev: None, element, }
    }

    fn into_element(self: Box<Self>) -> T {
        self.element
    }
}


// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop_front() {
        let mut list = LinkedList::<i32>::new();
        list.push_front(1);
        assert_eq!(list.pop_front().unwrap(), 1);
    }
}
