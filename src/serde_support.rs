extern crate serde;
extern crate serde_test;

use super::*;

use self::serde::de::{SeqAccess, Visitor};
use self::serde::export::fmt;
use self::serde::export::PhantomData;
use self::serde::ser::{Serialize, SerializeSeq, Serializer};
use self::serde::{Deserialize, Deserializer};

impl<T> Serialize for CircularQueue<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.capacity()))?;
        for e in self.asc_iter() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

#[derive(Debug)]
/// `serde::de::Visitor` for a circular queue
struct CircularQueueVisitor<T> {
    marker: PhantomData<fn() -> CircularQueue<T>>,
}

impl<T> CircularQueueVisitor<T> {
    pub fn new() -> CircularQueueVisitor<T> {
        CircularQueueVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de, T> Visitor<'de> for CircularQueueVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = CircularQueue<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a circular queue")
    }

    #[inline]
    fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
    where
        S: SeqAccess<'de>,
    {
        let mut values = CircularQueue::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(value) = seq.next_element()? {
            values.push(value);
        }
        Ok(values)
    }
}

impl<'de, T> Deserialize<'de> for CircularQueue<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<CircularQueue<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(CircularQueueVisitor::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use self::serde_test::{assert_tokens, Token};

    #[test]
    fn serialization_works() {
        let mut q = CircularQueue::with_capacity(8);
        q.push(1);
        q.push(2);

        assert_tokens(
            &q,
            &[
                Token::Seq { len: Some(8) },
                Token::I32(1),
                Token::I32(2),
                Token::SeqEnd,
            ],
        );
    }

    #[test]
    fn serialization_with_empty_queue() {
        let q = CircularQueue::<()>::with_capacity(0);

        assert_tokens(&q, &[Token::Seq { len: Some(0) }, Token::SeqEnd]);
    }

    #[test]
    fn serialization_preserves_order() {
        let mut q = CircularQueue::with_capacity(4);
        // Fill the q
        for i in 0..4 {
            q.push(i);
        }
        // Add a couple of new values
        q.push(4);
        q.push(5);
        // At this point, the queue should be [2, 3, 4, 5]

        assert_tokens(
            &q,
            &[
                Token::Seq { len: Some(4) },
                Token::I32(2),
                Token::I32(3),
                Token::I32(4),
                Token::I32(5),
                Token::SeqEnd,
            ],
        );
    }
}
