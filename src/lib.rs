//! A circular buffer-like queue.
//!
//! The `CircularQueue<T>` is created with a set capacity, then items are pushed in. When the queue
//! runs out of capacity, newer items start overwriting the old ones, starting from the oldest.
//!
//! There are built-in iterators that go from the newest items to the oldest ones and from the
//! oldest items to the newest ones.
//!
//! Two queues are considered equal if iterating over them with `iter()` would yield the same
//! sequence of elements.
//!
//! Enable the `serde_support` feature for [Serde](https://serde.rs/) support.
//!
//! # Examples
//!
//! ```
//! use circular_queue::CircularQueue;
//!
//! let mut queue = CircularQueue::with_capacity(3);
//! queue.push(1);
//! queue.push(2);
//! queue.push(3);
//! queue.push(4);
//!
//! assert_eq!(queue.len(), 3);
//!
//! let mut iter = queue.iter();
//!
//! assert_eq!(iter.next(), Some(&4));
//! assert_eq!(iter.next(), Some(&3));
//! assert_eq!(iter.next(), Some(&2));
//! ```

#![cfg_attr(has_extern_crate_alloc, no_std)]
#![doc(html_root_url = "https://docs.rs/circular-queue/0.2.7")]

#[cfg(has_extern_crate_alloc)]
extern crate alloc;

#[cfg(has_extern_crate_alloc)]
use alloc::vec::Vec;
#[cfg(has_extern_crate_alloc)]
use core::iter::{Chain, Rev};
#[cfg(has_extern_crate_alloc)]
use core::mem::replace;
#[cfg(has_extern_crate_alloc)]
use core::slice::{Iter as SliceIter, IterMut as SliceIterMut};

#[cfg(not(has_extern_crate_alloc))]
use std::iter::{Chain, Rev};
#[cfg(not(has_extern_crate_alloc))]
use std::mem::replace;
#[cfg(not(has_extern_crate_alloc))]
use std::slice::{Iter as SliceIter, IterMut as SliceIterMut};

#[cfg(feature = "serde_support")]
mod serde_support;

/// A circular buffer-like queue.
#[derive(Clone, Debug)]
pub struct CircularQueue<T> {
    data: Vec<T>,
    // Using our own capacity instead of the one stored in Vec to ensure consistent behavior with
    // zero-sized types.
    capacity: usize,
    insertion_index: usize,
}

/// An iterator over `CircularQueue<T>`.
pub type Iter<'a, T> = Chain<Rev<SliceIter<'a, T>>, Rev<SliceIter<'a, T>>>;

/// A mutable iterator over `CircularQueue<T>`.
pub type IterMut<'a, T> = Chain<Rev<SliceIterMut<'a, T>>, Rev<SliceIterMut<'a, T>>>;

/// An ascending iterator over `CircularQueue<T>`.
pub type AscIter<'a, T> = Chain<SliceIter<'a, T>, SliceIter<'a, T>>;

/// An mutable ascending iterator over `CircularQueue<T>`.
pub type AscIterMut<'a, T> = Chain<SliceIterMut<'a, T>, SliceIterMut<'a, T>>;

/// A value popped from `CircularQueue<T>` as the result of a push operation.
pub type Popped<T> = Option<T>;

impl<T> CircularQueue<T> {
    /// Constructs a new, empty `CircularQueue<T>` with the requested capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_queue::CircularQueue;
    ///
    /// let mut queue: CircularQueue<i32> = CircularQueue::with_capacity(5);
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            capacity,
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
    /// let mut queue = CircularQueue::with_capacity(5);
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

    /// Returns `true` if the queue contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_queue::CircularQueue;
    ///
    /// let mut queue = CircularQueue::with_capacity(5);
    /// assert!(queue.is_empty());
    ///
    /// queue.push(1);
    /// assert!(!queue.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns `true` if the queue is full.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_queue::CircularQueue;
    ///
    /// let mut queue = CircularQueue::with_capacity(5);
    ///
    /// assert!(!queue.is_full());
    ///
    /// queue.push(1);
    /// queue.push(2);
    /// queue.push(3);
    /// queue.push(4);
    /// queue.push(5);
    ///
    /// assert!(queue.is_full());
    /// ```
    #[inline]
    pub fn is_full(&self) -> bool {
        self.capacity() == self.len()
    }

    /// Returns the capacity of the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_queue::CircularQueue;
    ///
    /// let queue: CircularQueue<i32> = CircularQueue::with_capacity(5);
    /// assert_eq!(queue.capacity(), 5);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Clears the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_queue::CircularQueue;
    ///
    /// let mut queue = CircularQueue::with_capacity(5);
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
    /// In case an old value is overwritten, it will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_queue::CircularQueue;
    ///
    /// let mut queue = CircularQueue::with_capacity(3);
    ///
    /// queue.push(1);
    /// queue.push(2);
    ///
    /// assert_eq!(queue.push(3), None);
    /// assert_eq!(queue.push(4), Some(1));
    ///
    /// assert_eq!(queue.len(), 3);
    ///
    /// let mut iter = queue.iter();
    ///
    /// assert_eq!(iter.next(), Some(&4));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), Some(&2));
    /// ```
    #[inline]
    pub fn push(&mut self, x: T) -> Popped<T> {
        let mut old = None;

        if self.capacity() == 0 {
            return old;
        }

        if !self.is_full() {
            self.data.push(x);
        } else {
            old = Some(replace(&mut self.data[self.insertion_index], x));
        }

        self.insertion_index = (self.insertion_index + 1) % self.capacity();

        old
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
    /// let mut queue = CircularQueue::with_capacity(3);
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
    pub fn iter(&self) -> Iter<T> {
        let (a, b) = self.data.split_at(self.insertion_index);
        a.iter().rev().chain(b.iter().rev())
    }

    /// Returns a mutable iterator over the queue's contents.
    ///
    /// The iterator goes from the most recently pushed items to the oldest ones.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_queue::CircularQueue;
    ///
    /// let mut queue = CircularQueue::with_capacity(3);
    /// queue.push(1);
    /// queue.push(2);
    /// queue.push(3);
    /// queue.push(4);
    ///
    /// let mut iter = queue.iter_mut();
    ///
    /// assert_eq!(iter.next(), Some(&mut 4));
    /// assert_eq!(iter.next(), Some(&mut 3));
    /// assert_eq!(iter.next(), Some(&mut 2));
    /// ```
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<T> {
        let (a, b) = self.data.split_at_mut(self.insertion_index);
        a.iter_mut().rev().chain(b.iter_mut().rev())
    }

    /// Returns an ascending iterator over the queue's contents.
    ///
    /// The iterator goes from the least recently pushed items to the newest ones.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_queue::CircularQueue;
    ///
    /// let mut queue = CircularQueue::with_capacity(3);
    /// queue.push(1);
    /// queue.push(2);
    /// queue.push(3);
    /// queue.push(4);
    ///
    /// let mut iter = queue.asc_iter();
    ///
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), Some(&4));
    /// ```
    #[inline]
    pub fn asc_iter(&self) -> AscIter<T> {
        let (a, b) = self.data.split_at(self.insertion_index);
        b.iter().chain(a.iter())
    }

    /// Returns a mutable ascending iterator over the queue's contents.
    ///
    /// The iterator goes from the least recently pushed items to the newest ones.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_queue::CircularQueue;
    ///
    /// let mut queue = CircularQueue::with_capacity(3);
    /// queue.push(1);
    /// queue.push(2);
    /// queue.push(3);
    /// queue.push(4);
    ///
    /// let mut iter = queue.asc_iter_mut();
    ///
    /// assert_eq!(iter.next(), Some(&mut 2));
    /// assert_eq!(iter.next(), Some(&mut 3));
    /// assert_eq!(iter.next(), Some(&mut 4));
    /// ```
    #[inline]
    pub fn asc_iter_mut(&mut self) -> AscIterMut<T> {
        let (a, b) = self.data.split_at_mut(self.insertion_index);
        b.iter_mut().chain(a.iter_mut())
    }

    /// Converts the queue into a `Vec<T>` going from the most recently pushed items to the oldest
    /// ones.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_queue::CircularQueue;
    ///
    /// let mut queue = CircularQueue::with_capacity(3);
    /// queue.push(1);
    /// queue.push(2);
    /// queue.push(3);
    /// queue.push(4);
    ///
    /// let v = queue.into_vec();
    ///
    /// assert_eq!(v, vec![4, 3, 2]);
    /// ```
    #[inline]
    pub fn into_vec(mut self) -> Vec<T> {
        self.data[self.insertion_index..].reverse(); // Reverse the upper part.
        self.data[..self.insertion_index].reverse(); // Reverse the lower part.
        self.data
    }
}

impl<T: PartialEq> PartialEq for CircularQueue<T> {
    #[inline]
    fn eq(&self, other: &CircularQueue<T>) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

impl<T: Eq> Eq for CircularQueue<T> {}

#[cfg(has_relaxed_orphan_rule)]
impl<T> From<CircularQueue<T>> for Vec<T> {
    #[inline]
    fn from(queue: CircularQueue<T>) -> Self {
        queue.into_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(has_extern_crate_alloc)]
    use alloc::vec;

    #[test]
    fn zero_capacity() {
        let mut q = CircularQueue::<i32>::with_capacity(0);
        assert_eq!(q.len(), 0);
        assert_eq!(q.capacity(), 0);
        assert!(q.is_empty());

        q.push(3);
        q.push(4);
        q.push(5);

        assert_eq!(q.len(), 0);
        assert_eq!(q.capacity(), 0);
        assert!(q.is_empty());

        assert_eq!(q.iter().count(), 0);
        assert_eq!(q.asc_iter().count(), 0);

        q.clear();
    }

    #[test]
    fn empty_queue() {
        let q = CircularQueue::<i32>::with_capacity(5);

        assert!(q.is_empty());
        assert_eq!(q.iter().next(), None);
    }

    #[test]
    fn partially_full_queue() {
        let mut q = CircularQueue::with_capacity(5);
        q.push(1);
        q.push(2);
        q.push(3);

        assert!(!q.is_empty());
        assert_eq!(q.len(), 3);

        let res: Vec<_> = q.iter().map(|&x| x).collect();
        assert_eq!(res, [3, 2, 1]);
    }

    #[test]
    fn full_queue() {
        let mut q = CircularQueue::with_capacity(5);
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
        let mut q = CircularQueue::with_capacity(5);
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
        let mut q = CircularQueue::with_capacity(5);
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

    #[test]
    fn mutable_iterator() {
        let mut q = CircularQueue::with_capacity(5);
        q.push(1);
        q.push(2);
        q.push(3);
        q.push(4);
        q.push(5);
        q.push(6);
        q.push(7);

        for x in q.iter_mut() {
            *x *= 2;
        }

        let res: Vec<_> = q.iter().map(|&x| x).collect();
        assert_eq!(res, [14, 12, 10, 8, 6]);
    }

    #[test]
    fn zero_sized() {
        let mut q = CircularQueue::with_capacity(3);
        assert_eq!(q.capacity(), 3);

        q.push(());
        q.push(());
        q.push(());
        q.push(());

        assert_eq!(q.len(), 3);

        let mut iter = q.iter();
        assert_eq!(iter.next(), Some(&()));
        assert_eq!(iter.next(), Some(&()));
        assert_eq!(iter.next(), Some(&()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn empty_queue_eq() {
        let q1 = CircularQueue::<i32>::with_capacity(5);
        let q2 = CircularQueue::<i32>::with_capacity(5);
        assert_eq!(q1, q2);

        let q3 = CircularQueue::<i32>::with_capacity(6);
        assert_eq!(q1, q3); // Capacity doesn't matter as long as the same elements are yielded.
    }

    #[test]
    fn partially_full_queue_eq() {
        let mut q1 = CircularQueue::with_capacity(5);
        q1.push(1);
        q1.push(2);
        q1.push(3);

        let mut q2 = CircularQueue::with_capacity(5);
        q2.push(1);
        q2.push(2);
        assert_ne!(q1, q2);

        q2.push(3);
        assert_eq!(q1, q2);

        q2.push(4);
        assert_ne!(q1, q2);
    }

    #[test]
    fn full_queue_eq() {
        let mut q1 = CircularQueue::with_capacity(5);
        q1.push(1);
        q1.push(2);
        q1.push(3);
        q1.push(4);
        q1.push(5);

        let mut q2 = CircularQueue::with_capacity(5);
        q2.push(1);
        q2.push(2);
        q2.push(3);
        q2.push(4);
        q2.push(5);

        assert_eq!(q1, q2);
    }

    #[test]
    fn over_full_queue_eq() {
        let mut q1 = CircularQueue::with_capacity(5);
        q1.push(1);
        q1.push(2);
        q1.push(3);
        q1.push(4);
        q1.push(5);
        q1.push(6);
        q1.push(7);

        let mut q2 = CircularQueue::with_capacity(5);
        q2.push(1);
        q2.push(2);
        q2.push(3);
        q2.push(4);
        q2.push(5);
        q2.push(6);
        assert_ne!(q1, q2);

        q2.push(7);
        assert_eq!(q1, q2);

        q2.push(8);
        assert_ne!(q1, q2);

        q2.push(3);
        q2.push(4);
        q2.push(5);
        q2.push(6);
        q2.push(7);
        assert_eq!(q1, q2);
    }

    #[test]
    fn clear_eq() {
        let mut q1 = CircularQueue::with_capacity(5);
        q1.push(1);
        q1.push(2);
        q1.push(3);
        q1.push(4);
        q1.push(5);
        q1.push(6);
        q1.push(7);
        q1.clear();

        let mut q2 = CircularQueue::with_capacity(5);
        assert_eq!(q1, q2);

        q2.push(1);
        q2.clear();
        assert_eq!(q1, q2);
    }

    #[test]
    fn zero_sized_eq() {
        let mut q1 = CircularQueue::with_capacity(3);
        q1.push(());
        q1.push(());
        q1.push(());
        q1.push(());

        let mut q2 = CircularQueue::with_capacity(3);
        q2.push(());
        q2.push(());
        assert_ne!(q1, q2);

        q2.push(());
        assert_eq!(q1, q2);

        q2.push(());
        assert_eq!(q1, q2);

        q2.push(());
        assert_eq!(q1, q2);
    }

    #[test]
    fn into_vec() {
        let mut q = CircularQueue::with_capacity(4);
        q.push(1);
        q.push(2);
        q.push(3);

        let v = q.clone().into_vec();
        assert_eq!(v, vec![3, 2, 1]);

        q.push(4);
        q.push(5);
        let v = q.clone().into_vec();
        assert_eq!(v, vec![5, 4, 3, 2]);

        q.push(6);
        let v = q.into_vec();
        assert_eq!(v, vec![6, 5, 4, 3]);
    }

    #[cfg(has_relaxed_orphan_rule)]
    #[test]
    fn vec_from() {
        let mut q = CircularQueue::with_capacity(3);
        q.push(1);
        q.push(2);
        q.push(3);
        q.push(4);

        let v = Vec::from(q);
        assert_eq!(v, vec![4, 3, 2]);
    }
}
