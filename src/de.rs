use std::collections::VecDeque;

use serde_core::de;

use crate::{
    Error,
    map::Map,
    value::{Dict, Value},
};

impl<'de> de::Deserializer<'de> for Value {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Self::None => visitor.visit_unit(),
            Self::Bool(b) => visitor.visit_bool(b),
            Self::I64(i) => visitor.visit_i64(i),
            Self::I128(i) => visitor.visit_i128(i),
            Self::U64(u) => visitor.visit_u64(u),
            Self::U128(u) => visitor.visit_u128(u),
            Self::Float(f) => visitor.visit_f64(f),
            Self::String(s) => visitor.visit_string(s),
            Self::Dict(map) => visitor.visit_map(MapAccess::new(map)),
            Self::List(values) => visitor.visit_seq(SeqAccess::new(values)),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_bool(bool::try_from(self)?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i8(i8::try_from(self)?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i16(i16::try_from(self)?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i32(i32::try_from(self)?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i64(i64::try_from(self)?)
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i128(i128::try_from(self)?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u8(u8::try_from(self)?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u16(u16::try_from(self)?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u32(u32::try_from(self)?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u64(u64::try_from(self)?)
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u128(u128::try_from(self)?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f32(f32::try_from(self)?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f64(f64::try_from(self)?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let string = String::try_from(self)?;
        let mut chars = string.chars();
        match (chars.next(), chars.next()) {
            (Some(ch), None) => visitor.visit_char(ch),
            _ => Err(Error::Message(
                "invalid type for char: expected a single character string".to_string(),
            )),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let string = String::try_from(self)?;
        visitor.visit_string(string)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_string(String::try_from(self)?)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Value::String(s) => visitor.visit_byte_buf(s.into_bytes()),
            Value::List(values) => {
                let bytes = values
                    .into_iter()
                    .map(u8::try_from)
                    .collect::<Result<Vec<_>, _>>()?;
                visitor.visit_byte_buf(bytes)
            }
            other => Err(Error::Message(format!(
                "invalid type for bytes: expected string or sequence, got {other}"
            ))),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Value::None => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Value::None => visitor.visit_unit(),
            other => Err(Error::Message(format!(
                "invalid type for unit: expected none, got {other}"
            ))),
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Value::List(values) => visitor.visit_seq(SeqAccess::new(values)),
            other => Err(Error::Message(format!(
                "invalid type for sequence: expected list, got {other}"
            ))),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Value::Dict(map) => visitor.visit_map(MapAccess::new(map)),
            other => Err(Error::Message(format!(
                "invalid type for map: expected dict, got {other}"
            ))),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_enum(EnumAccess {
            value: self,
            name,
            variants,
        })
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }
}

struct StrDeserializer<'a>(&'a str);

impl<'de, 'a> de::Deserializer<'de> for StrDeserializer<'a> {
    type Error = Error;

    #[inline]
    fn deserialize_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_str(self.0)
    }

    serde_core::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128
        u8 u16 u32 u64 u128
        f32 f64 char str string seq
        bytes byte_buf map struct unit enum newtype_struct
        identifier ignored_any unit_struct tuple_struct tuple option
    }
}

struct MapAccess {
    elements: VecDeque<(String, Value)>,
}

impl MapAccess {
    fn new(map: Map<String, Value>) -> Self {
        Self {
            elements: map.into_iter().collect(),
        }
    }
}

impl<'de> de::MapAccess<'de> for MapAccess {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        if let Some((key_s, _)) = self.elements.front() {
            let key_de = StrDeserializer(key_s);
            let key = seed.deserialize(key_de)?;
            Ok(Some(key))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let (_key, value) = self
            .elements
            .pop_front()
            .ok_or_else(|| Error::Message("missing map value".to_string()))?;
        seed.deserialize(value)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.elements.len())
    }
}

struct SeqAccess {
    elements: std::vec::IntoIter<Value>,
}

impl SeqAccess {
    fn new(elements: Vec<Value>) -> Self {
        Self {
            elements: elements.into_iter(),
        }
    }
}

impl<'de> de::SeqAccess<'de> for SeqAccess {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.elements.next() {
            Some(value) => seed.deserialize(value).map(Some),
            None => Ok(None),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        let (lower, upper) = self.elements.size_hint();
        match upper {
            Some(upper) if lower == upper => Some(upper),
            _ => None,
        }
    }
}

struct EnumAccess {
    value: Value,
    name: &'static str,
    variants: &'static [&'static str],
}

impl EnumAccess {
    fn variant_deserializer(&self, name: &str) -> Result<StrDeserializer<'_>, Error> {
        self.variants
            .iter()
            .find(|&&s| s == name)
            .map(|&s| StrDeserializer(s))
            .ok_or_else(|| {
                Error::Message(format!(
                    "enum {} does not have variant constructor {}",
                    self.name, name
                ))
            })
    }

    fn table_deserializer(&self, table: &Dict) -> Result<StrDeserializer<'_>, Error> {
        if table.len() == 1 {
            self.variant_deserializer(table.iter().next().unwrap().0)
        } else {
            Err(Error::Message(format!(
                "value of enum {} should be represented by either string or table with exactly one key",
                self.name
            )))
        }
    }
}

impl<'de> de::EnumAccess<'de> for EnumAccess {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let value = {
            let deserializer = match self.value {
                Value::String(ref s) => self.variant_deserializer(s),
                Value::Dict(ref t) => self.table_deserializer(t),
                _ => Err(Error::Message(format!(
                    "value of enum {} should be represented by either string or table with exactly one key",
                    self.name
                ))),
            }?;
            seed.deserialize(deserializer)?
        };

        Ok((value, self))
    }
}

impl<'de> de::VariantAccess<'de> for EnumAccess {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        match self.value {
            Value::String(_) => Ok(()),
            Value::Dict(ref t) if t.len() == 1 => Ok(()),
            _ => Err(Error::Message(format!(
                "invalid unit variant representation for enum {}",
                self.name
            ))),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.value {
            Value::Dict(t) => {
                let (_name, value) = t.into_iter().next().ok_or_else(|| {
                    Error::Message(format!(
                        "missing newtype variant value for enum {}",
                        self.name
                    ))
                })?;
                seed.deserialize(value)
            }
            _ => Err(Error::Message(format!(
                "invalid newtype variant representation for enum {}",
                self.name
            ))),
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            Value::Dict(t) => {
                let (_name, value) = t.into_iter().next().ok_or_else(|| {
                    Error::Message(format!(
                        "missing tuple variant value for enum {}",
                        self.name
                    ))
                })?;
                de::Deserializer::deserialize_seq(value, visitor)
            }
            _ => Err(Error::Message(format!(
                "invalid tuple variant representation for enum {}",
                self.name
            ))),
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            Value::Dict(t) => {
                let (_name, value) = t.into_iter().next().ok_or_else(|| {
                    Error::Message(format!(
                        "missing struct variant value for enum {}",
                        self.name
                    ))
                })?;
                de::Deserializer::deserialize_map(value, visitor)
            }
            _ => Err(Error::Message(format!(
                "invalid struct variant representation for enum {}",
                self.name
            ))),
        }
    }
}
