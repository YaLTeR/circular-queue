extern crate serde;
extern crate serde_test;

use super::*;

use self::serde::de::{Error, SeqAccess, Visitor};
use self::serde::export::fmt;
use self::serde::export::PhantomData;
use self::serde::ser::{Serialize, SerializeSeq, SerializeTupleStruct, Serializer};
use self::serde::{Deserialize, Deserializer};

/// Struct used to hold the iterator over the data for serialization
struct CircularQueueSerialize<'a, T> {
    length: usize,
    values: AscIter<'a, T>,
}

impl<'a, T> Serialize for CircularQueueSerialize<'a, T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.length))?;
        for e in self.values.clone() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

impl<T> Serialize for CircularQueue<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let to_ser = CircularQueueSerialize {
            length: self.len(),
            values: self.asc_iter(),
        };

        let mut seq = serializer.serialize_tuple_struct("CircularQueue", 2)?;
        seq.serialize_field(&self.capacity)?;
        seq.serialize_field(&to_ser)?;
        seq.end()
    }
}

#[derive(Debug)]
/// `serde::de::Visitor` for the internals (data) of a circular queue
struct CircularQueueDataVisitor<T> {
    capacity: usize,
    marker: PhantomData<fn() -> CircularQueue<T>>,
}

impl<T> CircularQueueDataVisitor<T> {
    pub fn new(capacity: usize) -> CircularQueueDataVisitor<T> {
        CircularQueueDataVisitor {
            capacity,
            marker: PhantomData,
        }
    }
}

#[derive(Debug)]
/// `serde::de::Visitor` for a circular queue
struct CircularQueueVisitor<T> {
    marker: PhantomData<fn() -> CircularQueueDataVisitor<T>>,
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
        let capacity = match seq.next_element()? {
            Some(val) => val,
            None => return Err(Error::missing_field("capacity")),
        };
        let values = match seq.next_element::<Vec<T>>()? {
            Some(val) => val,
            None => return Err(Error::missing_field("values")),
        };
        let mut ret = CircularQueue::with_capacity(capacity);
        for x in values {
            ret.push(x);
        }
        Ok(ret)
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
        deserializer.deserialize_tuple_struct("CircularQueue", 2, CircularQueueVisitor::new())
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
                Token::TupleStruct {
                    name: "CircularQueue",
                    len: 2,
                },
                Token::U64(8),
                Token::Seq { len: Some(2) },
                Token::I32(1),
                Token::I32(2),
                Token::SeqEnd,
                Token::TupleStructEnd,
            ],
        );
    }

    #[test]
    fn serialization_with_empty_queue() {
        let q = CircularQueue::<()>::with_capacity(0);

        assert_tokens(
            &q,
            &[
                Token::TupleStruct {
                    name: "CircularQueue",
                    len: 2,
                },
                Token::U64(0),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::TupleStructEnd,
            ],
        );
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
                Token::TupleStruct {
                    name: "CircularQueue",
                    len: 2,
                },
                Token::U64(4),
                Token::Seq { len: Some(4) },
                Token::I32(2),
                Token::I32(3),
                Token::I32(4),
                Token::I32(5),
                Token::SeqEnd,
                Token::TupleStructEnd,
            ],
        );
    }
}
