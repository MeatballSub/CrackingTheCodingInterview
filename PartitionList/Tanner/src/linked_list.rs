pub struct Node<T> {
    pub next: Option<std::ptr::NonNull<Node<T>>>,
    pub element: T,
}

pub struct LinkedList<T> {
    head: Option<std::ptr::NonNull<Node<T>>>,
    tail: Option<std::ptr::NonNull<Node<T>>>,
    len: usize,
    marker: std::marker::PhantomData<Box<Node<T>>>,
}

#[derive(Clone)]
pub struct IntoIter<T> {
    list: LinkedList<T>,
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Iter<'a, T: 'a> {
    head: Option<std::ptr::NonNull<Node<T>>>,
    len: usize,
    marker: std::marker::PhantomData<&'a Node<T>>,
}

#[derive(Clone)]
pub struct UnsafeCursor<T> {
    list_head: *mut Option<std::ptr::NonNull<Node<T>>>,
    list_tail: *mut Option<std::ptr::NonNull<Node<T>>>,
    list_len: *mut usize,
    previous: Option<std::ptr::NonNull<Node<T>>>,
    current: Option<std::ptr::NonNull<Node<T>>>,
}

#[derive(Clone)]
pub struct UnsafeCursorMut<T> {
    list_head: *mut Option<std::ptr::NonNull<Node<T>>>,
    list_tail: *mut Option<std::ptr::NonNull<Node<T>>>,
    list_len: *mut usize,
    previous: Option<std::ptr::NonNull<Node<T>>>,
    current: Option<std::ptr::NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        Node {
            next: None,
            element,
        }
    }

    fn into_element(self: Box<Self>) -> T {
        self.element
    }
}

impl<T> LinkedList<T> {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
            marker: std::marker::PhantomData,
        }
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push_back(&mut self, element: T) {
        let _ = self.push_back_mut(element);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node().map(Node::into_element)
    }

    pub fn head_node(&self) -> Option<std::ptr::NonNull<Node<T>>> {
        self.head
    }

    #[inline]
    fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.head = node.next;
            self.len -= 1;
            node
        })
    }

    #[must_use = "if you don't need a reference to the value, use `LinkedList::push_back` instead"]
    pub fn push_back_mut(&mut self, element: T) -> &mut T {
        let mut node = unsafe {
            std::ptr::NonNull::new_unchecked(Box::into_raw(Box::new(Node::new(element))))
        };
        unsafe {
            self.push_back_node(node);
            &mut node.as_mut().element
        }
    }

    #[inline]
    unsafe fn push_back_node(&mut self, node: std::ptr::NonNull<Node<T>>) {
        unsafe {
            (*node.as_ptr()).next = None;
            let node_opt = Some(node);
            match self.tail {
                None => self.head = node_opt,
                Some(tail) => (*tail.as_ptr()).next = node_opt,
            }
            self.tail = node_opt;
            self.len += 1;
        }
    }

    pub fn append(&mut self, other: &mut Self) {
        match self.tail {
            None => std::mem::swap(self, other),
            Some(mut tail) => {
                if let Some(other_head) = other.head.take() {
                    unsafe {
                        tail.as_mut().next = Some(other_head);
                    }

                    self.tail = other.tail.take();
                    self.len += std::mem::replace(&mut other.len, 0);
                }
            }
        }
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head,
            len: self.len,
            marker: std::marker::PhantomData,
        }
    }

    #[inline]
    #[must_use]
    pub fn unsafe_cursor_front(&mut self) -> UnsafeCursor<T> {
        UnsafeCursor {
            list_head: &mut self.head,
            list_tail: &mut self.tail,
            list_len: &mut self.len,
            current: self.head,
            previous: None,
        }
    }

    #[inline]
    #[must_use]
    pub fn unsafe_cursor_front_mut(&mut self) -> UnsafeCursorMut<T> {
        UnsafeCursorMut {
            list_head: &mut self.head,
            list_tail: &mut self.tail,
            list_len: &mut self.len,
            current: self.head,
            previous: None,
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        struct DropGuard<'a, T>(&'a mut LinkedList<T>);

        impl<'a, T> Drop for DropGuard<'a, T> {
            fn drop(&mut self) {
                while self.0.pop_front_node().is_some() {}
            }
        }

        let guard = DropGuard(self);
        while guard.0.pop_front_node().is_some() {}
        std::mem::forget(guard);
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut list = Self::new();
        self.iter()
            .for_each(|element| list.push_back(element.clone()));
        list
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other)
    }

    fn ne(&self, other: &Self) -> bool {
        self.len() != other.len() || self.iter().ne(other)
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;

    #[inline]
    fn into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.list.pop_front()
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for IntoIter<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("IntoIter").field(&self.list).finish()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| unsafe {
                let node = &*node.as_ptr();
                self.len -= 1;
                self.head = node.next;
                &node.element
            })
        }
    }
}

impl<T: Clone> UnsafeCursor<T> {
    #[must_use]
    pub fn current(&mut self) -> Option<T> {
        unsafe {
            self.current
                .map(|current| (*current.as_ptr()).element.clone())
        }
    }

    pub fn move_next(&mut self) {
        match self.current.take() {
            None => {
                self.previous = None;
                self.current = unsafe { *self.list_head };
            }

            Some(current) => unsafe {
                self.previous = Some(current);
                self.current = current.as_ref().next;
            },
        }
    }
}

impl<T> UnsafeCursorMut<T> {
    #[must_use]
    pub fn current(&mut self) -> Option<&mut T> {
        unsafe { self.current.map(|current| &mut (*current.as_ptr()).element) }
    }

    pub fn remove_current(&mut self) -> Option<T> {
        let unlinked_node = self.current?;
        unsafe {
            let node = unlinked_node.as_ref();
            self.current = node.next;

            match self.previous {
                Some(prev_ptr) => (*prev_ptr.as_ptr()).next = node.next,
                None => {
                    *self.list_head = node.next;
                }
            }

            if node.next.is_none() {
                *self.list_tail = self.previous
            }

            *self.list_len -= 1;

            let unlinked_node = Box::from_raw(unlinked_node.as_ptr());
            Some(unlinked_node.element)
        }
    }

    pub fn move_next(&mut self) {
        match self.current.take() {
            None => {
                self.previous = None;
                self.current = unsafe { *self.list_head };
            }
            Some(current) => unsafe {
                self.previous = Some(current);
                self.current = current.as_ref().next;
            },
        }
    }

    pub fn reset(&mut self, other: &UnsafeCursor<T>) {
        self.list_head = other.list_head;
        self.list_tail = other.list_tail;
        self.list_len = other.list_len;
        self.current = other.current;
        self.previous = other.previous;
    }
}
