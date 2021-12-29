use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Display;

use serde::ser::{self, Impossible};
use serde_json::Number;

use crate::GqlValue;
#[derive(Debug)]
pub struct SerializerError(String);

impl Display for SerializerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}
impl Error for SerializerError {
    fn description(&self) -> &str {
        "GqlValue serializer error"
    }
}

impl ser::Error for SerializerError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        SerializerError(msg.to_string())
    }
}

struct Serializer;

pub fn serialize_to_gql_value<T: ser::Serialize>(value: T) -> Result<GqlValue, SerializerError> {
    value.serialize(Serializer)
}

impl ser::Serializer for Serializer {
    type Ok = GqlValue;

    type Error = SerializerError;

    type SerializeSeq = SerializeSeq;

    type SerializeTuple = SerializeTuple;

    type SerializeTupleStruct = SerializeTupleStruct;

    type SerializeTupleVariant = SerializeTupleVariant;

    type SerializeMap = SerializeMap;

    type SerializeStruct = SerializeStruct;

    type SerializeStructVariant = SerializeStructVariant;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::Boolean(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::Number(Number::from(v)))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::Number(Number::from(v)))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::Number(Number::from(v)))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::Number(Number::from(v)))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::Number(Number::from(v)))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::Number(Number::from(v)))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::Number(Number::from(v)))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::Number(Number::from(v)))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        match Number::from_f64(v as f64) {
            Some(v) => Ok(GqlValue::Number(v)),
            None => Err(SerializerError(format!("{}: f32 failed to serialize", v))),
        }
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        match Number::from_f64(v) {
            Some(v) => Ok(GqlValue::Number(v)),
            None => Err(SerializerError(format!("{}: f32 failed to serialize", v))),
        }
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Err(SerializerError("char cannot be serialized.".to_string()))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::String(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(SerializerError("bytes cannot be serialized.".to_string()))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::Null)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::Null)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::Null)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::String(variant.to_string()))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self).map(|v| {
            let mut obj = BTreeMap::new();
            obj.insert(variant.to_string(), v);
            GqlValue::Object(obj)
        })
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SerializeSeq(vec![]))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(SerializeTuple(vec![]))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(SerializeTupleStruct(vec![]))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(SerializeTupleVariant(
            variant.to_string(),
            Vec::with_capacity(len),
        ))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(SerializeMap {
            map: BTreeMap::new(),
            key: None,
        })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(SerializeStruct(BTreeMap::new()))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(SerializeStructVariant(variant.to_string(), BTreeMap::new()))
    }
}

struct SerializeSeq(Vec<GqlValue>);

impl ser::SerializeSeq for SerializeSeq {
    type Ok = GqlValue;

    type Error = SerializerError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        let value = value.serialize(Serializer)?;
        self.0.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::List(self.0))
    }
}

struct SerializeTuple(Vec<GqlValue>);

impl ser::SerializeTuple for SerializeTuple {
    type Ok = GqlValue;

    type Error = SerializerError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        let value = value.serialize(Serializer)?;
        self.0.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::List(self.0))
    }
}

struct SerializeTupleStruct(Vec<GqlValue>);

impl ser::SerializeTupleStruct for SerializeTupleStruct {
    type Ok = GqlValue;
    type Error = SerializerError;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let value = value.serialize(Serializer)?;
        self.0.push(value);
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::List(self.0))
    }
}

struct SerializeTupleVariant(String, Vec<GqlValue>);

impl ser::SerializeTupleVariant for SerializeTupleVariant {
    type Ok = GqlValue;
    type Error = SerializerError;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let value = value.serialize(Serializer)?;
        self.1.push(value);
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut map = BTreeMap::new();
        map.insert(self.0, GqlValue::List(self.1));
        Ok(GqlValue::Object(map))
    }
}

struct SerializeMap {
    map: BTreeMap<String, GqlValue>,
    key: Option<String>,
}

impl ser::SerializeMap for SerializeMap {
    type Ok = GqlValue;
    type Error = SerializerError;

    #[inline]
    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let key = key.serialize(KeySerializer)?;
        self.key = Some(key);
        Ok(())
    }

    #[inline]
    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let value = value.serialize(Serializer)?;
        self.map.insert(self.key.take().unwrap(), value);
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::Object(self.map))
    }
}

struct SerializeStruct(BTreeMap<String, GqlValue>);

impl ser::SerializeStruct for SerializeStruct {
    type Ok = GqlValue;
    type Error = SerializerError;

    #[inline]
    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let value = value.serialize(Serializer)?;
        self.0.insert(key.to_string(), value);
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::Object(self.0))
    }
}

struct SerializeStructVariant(String, BTreeMap<String, GqlValue>);

impl ser::SerializeStructVariant for SerializeStructVariant {
    type Ok = GqlValue;
    type Error = SerializerError;

    #[inline]
    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let value = value.serialize(Serializer)?;
        self.1.insert(key.to_string(), value);
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut map = BTreeMap::new();
        map.insert(self.0, GqlValue::Object(self.1));
        Ok(GqlValue::Object(map))
    }
}

fn cannot_serialize_except_string() -> SerializerError {
    SerializerError("key must be a string".to_string())
}
struct KeySerializer;

impl serde::Serializer for KeySerializer {
    type Ok = String;

    type Error = SerializerError;

    type SerializeSeq = Impossible<String, SerializerError>;

    type SerializeTuple = Impossible<String, SerializerError>;

    type SerializeTupleStruct = Impossible<String, SerializerError>;

    type SerializeTupleVariant = Impossible<String, SerializerError>;

    type SerializeMap = Impossible<String, SerializerError>;

    type SerializeStruct = Impossible<String, SerializerError>;

    type SerializeStructVariant = Impossible<String, SerializerError>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(cannot_serialize_except_string())
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(cannot_serialize_except_string())
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(cannot_serialize_except_string())
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(cannot_serialize_except_string())
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(cannot_serialize_except_string())
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(cannot_serialize_except_string())
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(cannot_serialize_except_string())
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(cannot_serialize_except_string())
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(cannot_serialize_except_string())
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(cannot_serialize_except_string())
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(cannot_serialize_except_string())
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(cannot_serialize_except_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(cannot_serialize_except_string())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(cannot_serialize_except_string())
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(cannot_serialize_except_string())
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(cannot_serialize_except_string())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(cannot_serialize_except_string())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(variant.to_string())
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(cannot_serialize_except_string())
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(cannot_serialize_except_string())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(cannot_serialize_except_string())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(cannot_serialize_except_string())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(cannot_serialize_except_string())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(cannot_serialize_except_string())
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(cannot_serialize_except_string())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(cannot_serialize_except_string())
    }
}
