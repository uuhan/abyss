use crate::atomic::*;
use std::{
    fmt::{self, Debug},
    ops::Deref,
};

type CompareAndSwapResult<'g, T> =
    std::result::Result<Shared<'g, Node<T>>, (Shared<'g, Node<T>>, Owned<Node<T>>)>;

/// A node in the lock-free `Stack`.
#[derive(Debug)]
pub struct Node<T: Send + 'static> {
    pub(crate) inner: T,
    pub(crate) next: Atomic<Node<T>>,
}

impl<T: Send + 'static> Drop for Node<T> {
    fn drop(&mut self) {
        unsafe {
            let next = self.next.load(Relaxed, unprotected());
            if !next.as_raw().is_null() {
                drop(next.into_owned());
            }
        }
    }
}

/// A simple lock-free stack, with the ability to atomically
/// append or entirely swap-out entries.
pub struct Stack<T: Send + 'static> {
    head: Atomic<Node<T>>,
}

impl<T: Send + 'static> Default for Stack<T> {
    fn default() -> Self {
        Self {
            head: Atomic::null(),
        }
    }
}

impl<T: Send + 'static> Drop for Stack<T> {
    fn drop(&mut self) {
        unsafe {
            let curr = self.head.load(Relaxed, unprotected());
            if !curr.as_raw().is_null() {
                drop(curr.into_owned());
            }
        }
    }
}

impl<T> Debug for Stack<T>
where
    T: Clone + Debug + Send + 'static + Sync,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        let guard = crossbeam_epoch::pin();
        let head = self.head(&guard);
        let iter = StackIter::from_ptr(head, &guard);

        formatter.write_str("Stack [")?;
        let mut written = false;
        for node in iter {
            if written {
                formatter.write_str(", ")?;
            }
            formatter.write_str(&*format!("({:?}) ", &node as *const _))?;
            node.fmt(formatter)?;
            written = true;
        }
        formatter.write_str("]")?;
        Ok(())
    }
}

impl<T: Send + 'static> Deref for Node<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T: Clone + Send + Sync + 'static> Stack<T> {
    /// Add an item to the stack, spinning until successful.
    pub fn push(&self, inner: T) {
        let node = Owned::new(Node {
            inner,
            next: Atomic::null(),
        });

        unsafe {
            let node = node.into_shared(unprotected());

            loop {
                let head = self.head(unprotected());
                node.deref().next.store(head, Release);
                if self
                    .head
                    .compare_exchange(head, node, Release, Relaxed, unprotected())
                    .is_ok()
                {
                    return;
                }
            }
        }
    }

    /// Pop the next item off the stack. Returns None if nothing is there.
    #[allow(unused)]
    fn pop(&self, guard: &Guard) -> Option<T> {
        use std::ptr;
        let mut head = self.head(&guard);
        loop {
            match unsafe { head.as_ref() } {
                Some(h) => {
                    let next = h.next.load(Acquire, &guard);
                    match self
                        .head
                        .compare_exchange(head, next, Release, Relaxed, &guard)
                    {
                        Ok(_) => unsafe {
                            // NB It's important to unset the next pointer before destruction
                            // to avoid **DOUBLE-FREE**
                            h.next.store(Shared::default(), SeqCst);
                            guard.defer_destroy(head);

                            return Some(ptr::read(&h.inner));
                        },
                        Err(h) => head = h.current,
                    }
                }
                None => return None,
            }
        }
    }

    /// compare and push
    pub fn cap<'g>(
        &self,
        old: Shared<'_, Node<T>>,
        new: T,
        guard: &'g Guard,
    ) -> CompareAndSwapResult<'g, T> {
        let node = Owned::new(Node {
            inner: new,
            next: Atomic::from(old),
        });

        self.cap_node(old, node, guard)
    }

    /// compare and push
    pub fn cap_node<'g>(
        &self,
        old: Shared<'_, Node<T>>,
        mut node: Owned<Node<T>>,
        guard: &'g Guard,
    ) -> CompareAndSwapResult<'g, T> {
        // properly set next ptr
        node.next = Atomic::from(old);
        match self
            .head
            .compare_exchange(old, node, AcqRel, Acquire, guard)
        {
            Err(e) => {
                // we want to set next to null to prevent
                // the current shared head from being
                // dropped when we drop this node.
                let mut returned = e.new;
                returned.next = Atomic::null();
                Err((e.current, returned))
            }
            Ok(success) => Ok(success),
        }
    }

    /// compare and swap
    pub fn cas<'g>(
        &self,
        old: Shared<'g, Node<T>>,
        new: Owned<Node<T>>,
        guard: &'g Guard,
    ) -> CompareAndSwapResult<'g, T> {
        match self.head.compare_exchange(old, new, AcqRel, Acquire, guard) {
            Ok(success) => {
                if !old.is_null() {
                    unsafe {
                        guard.defer_destroy(old);
                    };
                }
                Ok(success)
            }
            Err(e) => Err((e.current, e.new)),
        }
    }

    /// Returns the current head pointer of the stack, which can
    /// later be used as the key for cas and cap operations.
    pub fn head<'g>(&self, guard: &'g Guard) -> Shared<'g, Node<T>> {
        self.head.load(Acquire, guard)
    }
}

/// An iterator over nodes in a lock-free stack.
pub struct StackIter<'a, T>
where
    T: Send + 'static + Sync,
{
    inner: Shared<'a, Node<T>>,
    guard: &'a Guard,
}

impl<'a, T> StackIter<'a, T>
where
    T: 'a + Send + 'static + Sync,
{
    /// Creates a StackIter from a pointer to one.
    pub fn from_ptr<'b>(ptr: Shared<'b, Node<T>>, guard: &'b Guard) -> StackIter<'b, T> {
        StackIter { inner: ptr, guard }
    }
}

impl<'a, T> Iterator for StackIter<'a, T>
where
    T: Send + 'static + Sync,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_null() {
            None
        } else {
            unsafe {
                let ret = &self.inner.deref().inner;
                self.inner = self.inner.deref().next.load(Acquire, self.guard);
                Some(ret)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let mut size = 0;
        let mut cursor = self.inner;

        while !cursor.is_null() {
            unsafe {
                cursor = cursor.deref().next.load(Acquire, self.guard);
            }
            size += 1;
        }

        (size, Some(size))
    }
}

/// Turns a vector of elements into a lock-free stack
/// of them, and returns the head of the stack.
pub fn node_from_frag_vec<T>(from: Vec<T>) -> Owned<Node<T>>
where
    T: Send + 'static + Sync,
{
    let mut last = None;

    for item in from.into_iter().rev() {
        last = if let Some(last) = last {
            Some(Owned::new(Node {
                inner: item,
                next: Atomic::from(last),
            }))
        } else {
            Some(Owned::new(Node {
                inner: item,
                next: Atomic::null(),
            }))
        }
    }

    last.expect("at least one frag was provided in the from Vec")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        use crate::{atomic::pin, sync::*};
        use std::thread;

        let guard = pin();
        let stack = Arc::new(Stack::default());
        assert_eq!(stack.pop(&guard), None);

        stack.push(1);
        let stack2 = Arc::clone(&stack);
        thread::spawn(move || {
            stack2.push(2);
            stack2.push(3);
            stack2.push(4);
        })
        .join()
        .unwrap();

        stack.push(5);
        assert_eq!(stack.pop(&guard), Some(5));
        assert_eq!(stack.pop(&guard), Some(4));
        let stack3 = Arc::clone(&stack);
        thread::spawn(move || {
            let guard = pin();
            assert_eq!(stack3.pop(&guard), Some(3));
            assert_eq!(stack3.pop(&guard), Some(2));
        })
        .join()
        .unwrap();
        assert_eq!(stack.pop(&guard), Some(1));

        let stack4 = Arc::clone(&stack);
        thread::spawn(move || {
            let guard = pin();
            assert_eq!(stack4.pop(&guard), None);
        })
        .join()
        .unwrap();
    }
}
