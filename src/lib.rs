use std::iter::{Chain, Rev};
use std::slice::Iter as SliceIter;

/// A circular buffer-like queue.
///
/// Once the capacity is reached, pushing new items will overwrite old ones.
#[derive(Clone, Debug)]
pub struct CircularQueue<T> {
    data: Vec<T>,
    insertion_index: usize,
}

/// An iterator over `CircularQueue<T>`.
pub type Iter<'a, T> = Chain<Rev<SliceIter<'a, T>>, Rev<SliceIter<'a, T>>>;

impl<T> CircularQueue<T> {
    /// Constructs a new, empty `CircularQueue<T>`.
    ///
    /// # Panics
    ///
    /// Panics if the requested capacity is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_queue::CircularQueue;
    ///
    /// let mut queue: CircularQueue<i32> = CircularQueue::new(5);
    /// ```
    #[inline]
    pub fn new(capacity: usize) -> Self {
        if capacity == 0 {
            panic!("capacity must be greater than 0");
        }

        Self {
            data: Vec::with_capacity(capacity),
            insertion_index: 0,
        }
    }

    /// Returns the current number of elements in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_queue::CircularQueue;
    ///
    /// let mut queue = CircularQueue::new(5);
    /// queue.push(1);
    /// queue.push(2);
    /// queue.push(3);
    ///
    /// assert_eq!(queue.len(), 3);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Clears the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_queue::CircularQueue;
    ///
    /// let mut queue = CircularQueue::new(5);
    /// queue.push(1);
    /// queue.push(2);
    /// queue.push(3);
    ///
    /// queue.clear();
    /// assert_eq!(queue.len(), 0);
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
        self.insertion_index = 0;
    }

    /// Pushes a new element into the queue.
    ///
    /// Once the capacity is reached, pushing new items will overwrite old ones.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_queue::CircularQueue;
    ///
    /// let mut queue = CircularQueue::new(3);
    /// queue.push(1);
    /// queue.push(2);
    /// queue.push(3);
    /// queue.push(4);
    ///
    /// assert_eq!(queue.len(), 3);
    /// ```
    pub fn push(&mut self, x: T) {
        if self.data.len() < self.data.capacity() {
            self.data.push(x);
        } else {
            self.data[self.insertion_index] = x;
        }

        self.insertion_index = (self.insertion_index + 1) % self.data.capacity();
    }

    /// Returns an iterator over the queue's contents.
    ///
    /// The iterator goes from the most recently pushed items to the oldest ones.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_queue::CircularQueue;
    ///
    /// let mut queue = CircularQueue::new(3);
    /// queue.push(1);
    /// queue.push(2);
    /// queue.push(3);
    /// queue.push(4);
    ///
    /// let mut iter = queue.iter();
    ///
    /// assert_eq!(iter.next(), Some(&4));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), Some(&2));
    /// ```
    #[inline]
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        self.data[0..self.insertion_index]
            .iter()
            .rev()
            .chain(self.data[self.insertion_index..].iter().rev())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn zero_capacity() {
        let _ = CircularQueue::<i32>::new(0);
    }

    #[test]
    fn empty_queue() {
        let q = CircularQueue::<i32>::new(5);

        assert_eq!(q.iter().next(), None);
    }

    #[test]
    fn partially_full_queue() {
        let mut q = CircularQueue::new(5);
        q.push(1);
        q.push(2);
        q.push(3);

        assert_eq!(q.len(), 3);

        let res: Vec<_> = q.iter().map(|&x| x).collect();
        assert_eq!(res, [3, 2, 1]);
    }

    #[test]
    fn full_queue() {
        let mut q = CircularQueue::new(5);
        q.push(1);
        q.push(2);
        q.push(3);
        q.push(4);
        q.push(5);

        assert_eq!(q.len(), 5);

        let res: Vec<_> = q.iter().map(|&x| x).collect();
        assert_eq!(res, [5, 4, 3, 2, 1]);
    }

    #[test]
    fn over_full_queue() {
        let mut q = CircularQueue::new(5);
        q.push(1);
        q.push(2);
        q.push(3);
        q.push(4);
        q.push(5);
        q.push(6);
        q.push(7);

        assert_eq!(q.len(), 5);

        let res: Vec<_> = q.iter().map(|&x| x).collect();
        assert_eq!(res, [7, 6, 5, 4, 3]);
    }

    #[test]
    fn clear() {
        let mut q = CircularQueue::new(5);
        q.push(1);
        q.push(2);
        q.push(3);
        q.push(4);
        q.push(5);
        q.push(6);
        q.push(7);

        q.clear();

        assert_eq!(q.len(), 0);
        assert_eq!(q.iter().next(), None);

        q.push(1);
        q.push(2);
        q.push(3);

        assert_eq!(q.len(), 3);

        let res: Vec<_> = q.iter().map(|&x| x).collect();
        assert_eq!(res, [3, 2, 1]);
    }
}
