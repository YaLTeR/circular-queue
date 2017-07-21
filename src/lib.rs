/// A circular buffer-like queue.
///
/// Once the capacity is reached, pushing new items will overwrite old ones.
#[derive(Clone, Debug)]
pub struct CircularQueue<T> {
    data: Vec<T>,
    insertion_index: usize,
}

impl<T> CircularQueue<T> {
    /// Creates a new `CircularQueue`.
    #[inline]
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            insertion_index: 0,
        }
    }

    /// Returns the current length of the queue.
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Clears the queue.
    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
        self.insertion_index = 0;
    }

    /// Pushes a new element into the queue.
    ///
    /// Once the capacity is reached, pushing new items will overwrite old ones.
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
    #[inline]
    pub fn iter<'a>(&'a self) -> CircularQueueIter<'a, T> {
        CircularQueueIter::new(self)
    }
}

/// Iterator over the `CircularQueue`.
#[derive(Clone, Debug)]
pub struct CircularQueueIter<'a, T: 'a> {
    data: &'a [T],
    index: usize,
    output: usize,
}

impl<'a, T: 'a> CircularQueueIter<'a, T> {
    #[inline]
    fn new(queue: &'a CircularQueue<T>) -> Self {
        Self {
            data: &queue.data,
            index: queue.insertion_index,
            output: 0,
        }
    }
}

impl<'a, T: 'a> Iterator for CircularQueueIter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.output == self.data.len() {
            None
        } else {
            self.index = circular_decrease(self.index, self.data.len());
            self.output += 1;

            Some(&self.data[self.index])
        }
    }
}

#[inline]
fn circular_decrease(value: usize, max: usize) -> usize {
    if max == 0 {
        0
    } else if value == 0 {
        max - 1
    } else {
        value - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_queue() {
        let q = CircularQueue::<i32>::new(5);

        assert_eq!(None, q.iter().next());
    }

    #[test]
    fn partially_full_queue() {
        let mut q = CircularQueue::new(5);
        q.push(1);
        q.push(2);
        q.push(3);

        assert_eq!(3, q.len());
        assert_eq!(&[3, 2, 1], q.iter().map(|&x| x).collect::<Vec<_>>().as_slice());
    }

    #[test]
    fn full_queue() {
        let mut q = CircularQueue::new(5);
        q.push(1);
        q.push(2);
        q.push(3);
        q.push(4);
        q.push(5);

        assert_eq!(5, q.len());
        assert_eq!(&[5, 4, 3, 2, 1], q.iter().map(|&x| x).collect::<Vec<_>>().as_slice());
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

        assert_eq!(5, q.len());
        assert_eq!(&[7, 6, 5, 4, 3], q.iter().map(|&x| x).collect::<Vec<_>>().as_slice());
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

        assert_eq!(0, q.len());
        assert_eq!(None, q.iter().next());

        q.push(1);
        q.push(2);
        q.push(3);

        assert_eq!(3, q.len());
        assert_eq!(&[3, 2, 1], q.iter().map(|&x| x).collect::<Vec<_>>().as_slice());
    }
}
