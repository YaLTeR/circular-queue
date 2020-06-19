extern crate serde;

use super::*;

use self::serde::de::Error;
use self::serde::ser::{Serialize, SerializeSeq, SerializeStruct, Serializer};
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

        let mut seq = serializer.serialize_struct("CircularQueueData", 2)?;
        seq.serialize_field("capacity", &self.capacity)?;
        seq.serialize_field("values", &to_ser)?;
        seq.end()
    }
}

#[derive(Deserialize)]
struct CircularQueueData<T> {
    capacity: usize,
    values: Vec<T>,
}

impl<'de, T> Deserialize<'de> for CircularQueue<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<CircularQueue<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut data = CircularQueueData::deserialize(deserializer)?;
        // We do not allow the vector to exceed the capacity.
        if data.values.len() > data.capacity {
            return Err(Error::invalid_length(
                data.values.len(),
                &"serialized vector exceeds capacity of the circular queue",
            ));
        }
        // Grow the vec if needed
        if data.values.len() < data.capacity {
            data.values.reserve(data.capacity - data.values.len());
        }
        Ok(CircularQueue {
            data: data.values,
            capacity: data.capacity,
            insertion_index: 0,
        })
    }
}

#[cfg(feature = "serde_support_test")]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate bincode;
    extern crate serde_json;
    extern crate serde_test;

    use self::serde_test::{assert_tokens, Token};

    #[test]
    fn serialization_works() {
        let mut q = CircularQueue::with_capacity(8);
        q.push(1);
        q.push(2);

        assert_tokens(
            &q,
            &[
                Token::Struct {
                    name: "CircularQueueData",
                    len: 2,
                },
                Token::Str("capacity"),
                Token::U64(8),
                Token::Str("values"),
                Token::Seq { len: Some(2) },
                Token::I32(1),
                Token::I32(2),
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn serialization_with_empty_queue() {
        let q = CircularQueue::<()>::with_capacity(0);

        assert_tokens(
            &q,
            &[
                Token::Struct {
                    name: "CircularQueueData",
                    len: 2,
                },
                Token::Str("capacity"),
                Token::U64(0),
                Token::Str("values"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::StructEnd,
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
                Token::Struct {
                    name: "CircularQueueData",
                    len: 2,
                },
                Token::Str("capacity"),
                Token::U64(4),
                Token::Str("values"),
                Token::Seq { len: Some(4) },
                Token::I32(2),
                Token::I32(3),
                Token::I32(4),
                Token::I32(5),
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn serialization_to_json() {
        let mut q = CircularQueue::with_capacity(4);
        q.push(3);
        q.push(7);
        assert_eq!(
            serde_json::to_string(&q).unwrap(),
            r#"{"capacity":4,"values":[3,7]}"#
        );
    }

    #[test]
    fn serialization_from_json() {
        let mut q = CircularQueue::with_capacity(4);
        q.push(3);
        q.push(7);
        let p =
            serde_json::from_str::<CircularQueue<i32>>(r#"{"capacity":4,"values":[3,7]}"#).unwrap();
        assert_eq!(p, q);
    }

    #[test]
    fn serialization_from_json_empty() {
        let q = CircularQueue::with_capacity(4);
        let p =
            serde_json::from_str::<CircularQueue<i32>>(r#"{"capacity":4,"values":[]}"#).unwrap();
        assert_eq!(p, q);
    }

    #[test]
    fn serialization_to_bincode() {
        let mut q = CircularQueue::with_capacity(4);
        q.push(3);
        q.push(9);
        let v = bincode::serialize(&q).unwrap();
        let p = bincode::deserialize::<CircularQueue<i32>>(&v).unwrap();
        assert_eq!(p, q);
    }

    #[test]
    fn serialization_with_oversized_vector_fails() {
        let oversize =
            serde_json::from_str::<CircularQueue<i32>>(r#"{"capacity":2,"values":[3,7,8]}"#);
        assert!(oversize.is_err());
    }
}
