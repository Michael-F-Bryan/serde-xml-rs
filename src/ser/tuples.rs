use std::io::Write;
use serde::ser::{Error as SerError, Serialize, SerializeTuple};

use ser::Serializer;
use ser::helpers;
use error::Error;

pub struct Tuple<'a, W: 'a + Write> {
    parent: &'a mut Serializer<W>,
}


impl<'w, W> Tuple<'w, W>
where
    W: 'w + Write,
{
    pub fn new(parent: &'w mut Serializer<W>) -> Tuple<'w, W> {
        Tuple { parent }
    }
}

impl<'w, W> SerializeTuple for Tuple<'w, W>
where
    W: 'w + Write
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error> 
    where 
        T: Serialize + ?Sized 
    {
        if helpers::is_wrapped(value) {
            value.serialize(&mut *self.parent)
        } else {
            Err(SerError::custom(
                "Tuples can't contain primitive types. Please wrap primitives in a newtype.",
            ))
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}