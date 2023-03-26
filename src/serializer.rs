use crate::{error::*, tag::*};
use serde::{ser, Serialize};

pub struct TypeTagSerializer {}

impl<'a> ser::Serializer for TypeTagSerializer {
    type Ok = TypeTag;

    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        Ok(TypeTag::Primitive(Primitive::U8))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        todo!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        todo!()
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        todo!()
    }
}

impl<'a> ser::SerializeSeq for TypeTagSerializer {
    type Ok = TypeTag;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl<'a> ser::SerializeTuple for TypeTagSerializer {
    type Ok = TypeTag;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl<'a> ser::SerializeTupleStruct for TypeTagSerializer {
    type Ok = TypeTag;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl<'a> ser::SerializeTupleVariant for TypeTagSerializer {
    type Ok = TypeTag;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl<'a> ser::SerializeMap for TypeTagSerializer {
    type Ok = TypeTag;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl<'a> ser::SerializeStruct for TypeTagSerializer {
    type Ok = TypeTag;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl<'a> ser::SerializeStructVariant for TypeTagSerializer {
    type Ok = TypeTag;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}
