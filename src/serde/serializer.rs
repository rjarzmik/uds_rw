use serde::{ser, Serialize};
use std::io::Write;

#[derive(Debug)]
pub enum EncodeError {
    Custom(String),
    Io(std::io::Error),
}

#[allow(dead_code)]
/// Serialize a type, outputing it to a vector
///
/// This function serializes a type into a vector, and returns it. The
/// serialization is a flat serialization, ie. each complex type is encoded
/// without its length (for serqences or maps), only the contained basic uint*
/// values ares encoded. The only meta information encoded is for enums where
/// the variant index is encoded into a single byte.
///
/// If the serialization succees, it returns [`Ok(Vec<u8>)`].
///
/// # Errors
///
/// If a basic rust type other that u8, u16, u32 or u64 is passed, a
/// [`EncodeError::Custom()`] is returned. If a compound type other than a
/// struct or a sequence is in the type T or one of its subtypes, the same error
/// is returned.
pub(super) fn to_bytes<T>(value: &T) -> Result<Vec<u8>, EncodeError>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        output: Sink::<Vec<u8>>::Vec(vec![]),
        big_endian: true,
    };
    value.serialize(&mut serializer)?;
    if let Sink::Vec(v) = serializer.output {
        Ok(v)
    } else {
        Ok(vec![])
    }
}

/// Serialize a type, outputing it to a vector
///
/// This function serializes a type into a vector, and returns it. The
/// serialization is a flat serialization, ie. each complex type is encoded
/// without its length (for serqences or maps), only the contained basic uint*
/// values ares encoded. The only meta information encoded is for enums where
/// the variant index is encoded into a single byte.
///
/// If the serialization succees, it returns [`Ok()`].
///
/// # Errors
///
/// If a basic rust type other that u8, u16, u32 or u64 is passed, a
/// [`EncodeError::Custom()`] is returned. If a compound type other than a
/// struct or a sequence is in the type T or one of its subtypes, the same error
/// is returned.
///
/// If there is an I/O error due to a writer failure, it is returned in
/// [`EncodeError::Io`].
pub(super) fn to_writer<W: Write, T>(writer: &mut W, value: &T) -> Result<(), EncodeError>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        output: Sink::Writer(writer),
        big_endian: true,
    };
    value.serialize(&mut serializer)?;
    Ok(())
}

impl core::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl serde::ser::Error for EncodeError {
    fn custom<T>(msg: T) -> Self
    where
        T: core::fmt::Display,
    {
        EncodeError::Custom(msg.to_string())
    }
}
impl serde::de::StdError for EncodeError {}

enum Sink<'w, W: Write> {
    Vec(Vec<u8>),
    Writer(&'w mut W),
}

struct Serializer<'s, W: Write> {
    output: Sink<'s, W>,
    big_endian: bool,
}

impl<'a, W: Write> ser::Serializer for &'a mut Serializer<'_, W> {
    // The output type produced by this `Serializer` during successful
    // serialization. Most serializers that produce text or binary output should
    // set `Ok = ()` and serialize into an `io::Write` or buffer contained
    // within the `Serializer` instance, as happens here. Serializers that build
    // in-memory data structures may be simplified by using `Ok` to propagate
    // the data structure around.
    type Ok = ();

    // The error type when some error occurs during serialization.
    type Error = EncodeError;

    // Associated types for keeping track of additional state while serializing
    // compound data structures like sequences and maps. In this case no
    // additional state is required beyond what is already stored in the
    // Serializer struct.
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        let b = if v { 0x01 } else { 0x00 };
        serialize_slice_u8(self, &[b])
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u8(v as u8)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u16(v as u16)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(v as u32)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        match self.big_endian {
            true => serialize_slice_u8(self, &v.to_be_bytes()),
            false => serialize_slice_u8(self, &v.to_le_bytes()),
        }
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        match self.big_endian {
            true => serialize_slice_u8(self, &v.to_be_bytes()),
            false => serialize_slice_u8(self, &v.to_le_bytes()),
        }
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        match self.big_endian {
            true => serialize_slice_u8(self, &v.to_be_bytes()),
            false => serialize_slice_u8(self, &v.to_le_bytes()),
        }
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        match self.big_endian {
            true => serialize_slice_u8(self, &v.to_be_bytes()),
            false => serialize_slice_u8(self, &v.to_le_bytes()),
        }
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        match self.big_endian {
            true => serialize_slice_u8(self, &v.to_be_bytes()),
            false => serialize_slice_u8(self, &v.to_le_bytes()),
        }
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        match self.big_endian {
            true => serialize_slice_u8(self, &v.to_be_bytes()),
            false => serialize_slice_u8(self, &v.to_le_bytes()),
        }
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        serialize_slice_u8(self, &[v as u8])
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        serialize_slice_u8(self, v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        serialize_slice_u8(self, v)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        (variant_index as u8).serialize(&mut *self) // If more than 255 variants, code will crash on purpose
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        (variant_index as u8).serialize(&mut *self)?; // If more than 255 variants, code will crash on purpose
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        variant.serialize(&mut *self)?;
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(EncodeError::Custom("Map not supported".to_string()))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        variant.serialize(&mut *self)?;
        Ok(self)
    }
}

fn serialize_slice_u8<W: Write>(ser: &mut Serializer<W>, v: &[u8]) -> Result<(), EncodeError> {
    let output = &mut ser.output;

    if let Sink::Writer(w) = output {
        w.write_all(v).map_err(EncodeError::Io)?;
        return Ok(());
    }
    if let Sink::Vec(ref mut vec) = ser.output {
        vec.extend(v);
        return Ok(());
    }
    Ok(())
}

// The following 7 impls deal with the serialization of compound types like
// sequences and maps. Serialization of such types is begun by a Serializer
// method and followed by zero or more calls to serialize individual elements of
// the compound type and one call to end the compound type.
//
// This impl is SerializeSeq so these methods are called after `serialize_seq`
// is called on the Serializer.
impl<'a, W: Write> ser::SerializeSeq for &'a mut Serializer<'_, W> {
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = EncodeError;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    // Close the sequence.
    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

// Same thing but for tuples.
impl<'a, W: Write> ser::SerializeTuple for &'a mut Serializer<'_, W> {
    type Ok = ();
    type Error = EncodeError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

// Same thing but for tuple structs.
impl<'a, W: Write> ser::SerializeTupleStruct for &'a mut Serializer<'_, W> {
    type Ok = ();
    type Error = EncodeError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

// Tuple variants are a little different. Refer back to the
// `serialize_tuple_variant` method above:
//
//    self.output += "{";
//    variant.serialize(&mut *self)?;
//    self.output += ":[";
//
// So the `end` method in this impl is responsible for closing both the `]` and
// the `}`.
impl<'a, W: Write> ser::SerializeTupleVariant for &'a mut Serializer<'_, W> {
    type Ok = ();
    type Error = EncodeError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

// Some `Serialize` types are not able to hold a key and value in memory at the
// same time so `SerializeMap` implementations are required to support
// `serialize_key` and `serialize_value` individually.
impl<'a, W: Write> ser::SerializeMap for &'a mut Serializer<'_, W> {
    type Ok = ();
    type Error = EncodeError;

    // The Serde data model allows map keys to be any serializable type. JSON
    // only allows string keys so the implementation below will produce invalid
    // JSON if the key serializes as something other than a string.
    //
    // A real JSON serializer would need to validate that map keys are strings.
    // This can be done by using a different Serializer to serialize the key
    // (instead of `&mut **self`) and having that other serializer only
    // implement `serialize_str` and return an error on any other data type.
    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)
    }

    // It doesn't make a difference whether the colon is printed at the end of
    // `serialize_key` or at the beginning of `serialize_value`. In this case
    // the code is a bit simpler having it here.
    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

// Structs are like maps in which the keys are constrained to be compile-time
// constant strings.
impl<'a, W: Write> ser::SerializeStruct for &'a mut Serializer<'_, W> {
    type Ok = ();
    type Error = EncodeError;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

// Similar to `SerializeTupleVariant`, here the `end` method is responsible for
// closing both of the curly braces opened by `serialize_struct_variant`.
impl<'a, W: Write> ser::SerializeStructVariant for &'a mut Serializer<'_, W> {
    type Ok = ();
    type Error = EncodeError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::serde::serializer::to_bytes;
    use serde::Serialize;

    #[test]
    fn serialize_struct() {
        #[derive(Serialize)]
        struct S1 {
            status: u16,
            records: Vec<S2>,
        }
        #[derive(Serialize)]
        struct S2 {
            dtc: [u8; 3],
            status: u8,
        }
        let val = S1 {
            status: 0xff,
            records: vec![
                S2 {
                    dtc: [0x01, 0x02, 0x03],
                    status: 0x27,
                },
                S2 {
                    dtc: [0xae, 0x37, 0x04],
                    status: 0x13,
                },
            ],
        };
        let bytes = to_bytes(&val).unwrap();
        assert_eq!(
            bytes,
            &[0x00, 0xff, 0x01, 0x02, 0x03, 0x27, 0xae, 0x37, 0x04, 0x13]
        );
    }
}
