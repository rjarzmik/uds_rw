use serde::de::{
    self, DeserializeSeed, Deserializer, EnumAccess, IntoDeserializer, SeqAccess, VariantAccess,
    Visitor,
};
use serde::Deserialize;

#[derive(Debug)]
pub enum DecodeError {
    Custom(String),
}

/// Deserialize a type, outputting it to a vector
///
/// This function deserializes a type from bytes, and returns it. The
/// deserialization is a flat deserialization, i.e. each complex type is decoded
/// without its length (for sequences or maps), only the contained basic uint*
/// values ares decoded. The only meta information encoded is for enums where
/// the variant index is encoded into a single byte.
/// As a consequence, when a sequence is encountered (Vec), the deserializer
/// consumes as many bytes as possible to fulfill the sequence.
///
/// If the serialization success, it returns [`Ok(Vec<u8>)`].
///
/// # Errors
///
/// If a basic rust type other that u8, u16, u32 or u64 is passed, a
/// [`DecodeError::Custom()`] is returned. If a compound type other than a
/// struct or a sequence is in the type T or one of its subtypes, the same error
/// is returned.
pub(crate) fn from_bytes<'a, T>(s: &'a [u8]) -> Result<T, DecodeError>
where
    T: Deserialize<'a>,
{
    let mut deserializer = FlatDeserializer {
        input: s,
        big_endian: true,
    };
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(DecodeError::Custom("No more bytes to read".to_string()))
    }
}

impl de::Error for DecodeError {
    fn custom<T>(msg: T) -> Self
    where
        T: core::fmt::Display,
    {
        DecodeError::Custom(msg.to_string())
    }
}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl de::StdError for DecodeError {}

type DecResult<T> = Result<T, DecodeError>;

struct FlatDeserializer<'de> {
    // This string starts with the input data and characters are truncated off
    // the beginning as data is parsed.
    input: &'de [u8],
    big_endian: bool,
}

impl FlatDeserializer<'_> {
    fn assert_remaining(&self, minimum: usize) -> DecResult<()> {
        if self.input.len() < minimum {
            Err(de::Error::invalid_length(
                self.input.len(),
                &"not enough bytes left",
            ))
        } else {
            Ok(())
        }
    }

    fn take_u8(&mut self) -> DecResult<u8> {
        self.assert_remaining(1)?;
        let (first, cdr) = self.input.split_first().unwrap();
        self.input = cdr;
        Ok(*first)
    }
    fn take_u16(&mut self) -> DecResult<u16> {
        self.assert_remaining(2)?;
        let slice = [self.take_u8()?, self.take_u8()?];
        if self.big_endian {
            Ok(u16::from_be_bytes(slice))
        } else {
            Ok(u16::from_le_bytes(slice))
        }
    }

    fn take_u32(&mut self) -> DecResult<u32> {
        self.assert_remaining(4)?;
        let slice: [u8; 4] = [
            self.take_u8()?,
            self.take_u8()?,
            self.take_u8()?,
            self.take_u8()?,
        ];
        if self.big_endian {
            Ok(u32::from_be_bytes(slice))
        } else {
            Ok(u32::from_le_bytes(slice))
        }
    }
}

impl<'de> Deserializer<'de> for &mut FlatDeserializer<'de> {
    type Error = DecodeError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!();
    }

    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!();
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!();
    }

    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!();
    }

    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!();
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.take_u8()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.take_u16()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.take_u32()?)
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!();
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!();
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!();
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!();
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!();
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!();
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!();
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!();
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct Access<'a, 'de: 'a> {
            deserializer: &'a mut FlatDeserializer<'de>,
        }

        impl<'de> SeqAccess<'de> for Access<'_, 'de> {
            type Error = DecodeError;

            fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
            where
                T: DeserializeSeed<'de>,
            {
                let value = DeserializeSeed::deserialize(seed, &mut *self.deserializer);
                value.map(Some).or(Ok(None))
            }
        }
        visitor.visit_seq(Access { deserializer: self })
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct Access<'a, 'de: 'a> {
            deserializer: &'a mut FlatDeserializer<'de>,
            len: usize,
        }

        impl<'de> SeqAccess<'de> for Access<'_, 'de> {
            type Error = DecodeError;

            fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, DecodeError>
            where
                T: DeserializeSeed<'de>,
            {
                if self.len > 0 {
                    self.len -= 1;
                    let value = DeserializeSeed::deserialize(seed, &mut *self.deserializer)?;
                    Ok(Some(value))
                } else {
                    Ok(None)
                }
            }

            fn size_hint(&self) -> Option<usize> {
                Some(self.len)
            }
        }

        visitor.visit_seq(Access {
            deserializer: self,
            len,
        })
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct Access<'a, 'de: 'a> {
            deserializer: &'a mut FlatDeserializer<'de>,
        }
        impl<'de> VariantAccess<'de> for Access<'_, 'de> {
            type Error = DecodeError;

            fn unit_variant(self) -> Result<(), Self::Error> {
                Ok(())
            }

            fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
            where
                T: DeserializeSeed<'de>,
            {
                DeserializeSeed::deserialize(seed, &mut *self.deserializer)
            }

            fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
            where
                V: Visitor<'de>,
            {
                self.deserializer.deserialize_tuple(len, visitor)
            }

            fn struct_variant<V>(
                self,
                fields: &'static [&'static str],
                visitor: V,
            ) -> Result<V::Value, Self::Error>
            where
                V: Visitor<'de>,
            {
                self.deserializer.deserialize_tuple(fields.len(), visitor)
            }
        }

        impl<'de> EnumAccess<'de> for Access<'_, 'de> {
            type Error = DecodeError;
            type Variant = Self;

            fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
            where
                V: DeserializeSeed<'de>,
            {
                let idx = u8::deserialize(&mut *self.deserializer)?;
                let val = seed.deserialize(idx.into_deserializer())?;
                Ok((val, self))
            }
        }

        visitor.visit_enum(Access { deserializer: self })
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::serde::deserializer::from_bytes;
    use serde::Deserialize;

    #[test]
    fn deserialize_struct() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct S1 {
            status: u16,
            records: Vec<S2>,
        }
        #[derive(Deserialize, PartialEq, Debug)]
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
        let bytes = vec![0x00, 0xff, 0x01, 0x02, 0x03, 0x27, 0xae, 0x37, 0x04, 0x13];
        let s: S1 = from_bytes(&bytes).unwrap();
        assert_eq!(&s, &val);
    }

    #[test]
    fn deserialize_enum() {
        #[derive(Deserialize, PartialEq, Debug)]
        enum S1 {
            Status,
            Records(Vec<u8>),
        }
        let val = S1::Status;
        let bytes = vec![0x00];
        let s: S1 = from_bytes(&bytes).unwrap();
        assert_eq!(&s, &val);

        let val = S1::Records(vec![0x12, 0xab]);
        let bytes = vec![0x01, 0x12, 0xab];
        let s: S1 = from_bytes(&bytes).unwrap();
        assert_eq!(&s, &val);

        let val = S1::Records(vec![0x12, 0xab]);
        let bytes = vec![0x01, 0x12, 0xab, 0xdd];
        let s: S1 = from_bytes(&bytes).unwrap();
        assert_ne!(&s, &val);
    }
}
