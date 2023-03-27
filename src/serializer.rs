use crate::{error::*, tag::*};
use serde::{ser, Serialize};

pub struct TypeTagSerializer {}

macro_rules! serialize_primitive {
    ($f:ident, $t:ty, $p:expr) => {
        fn $f(self, _v: $t) -> Result<Self::Ok> {
            Ok(TypeTag::Primitive($p))
        }
    };
}

impl ser::Serializer for TypeTagSerializer {
    type Ok = TypeTag;

    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = TypeTagStruct;
    type SerializeStructVariant = Self;

    serialize_primitive!(serialize_bool, bool, Primitive::Bool);
    serialize_primitive!(serialize_i8, i8, Primitive::I8);
    serialize_primitive!(serialize_i16, i16, Primitive::I16);
    serialize_primitive!(serialize_i32, i32, Primitive::I32);
    serialize_primitive!(serialize_i64, i64, Primitive::I64);
    serialize_primitive!(serialize_u8, u8, Primitive::U8);
    serialize_primitive!(serialize_u16, u16, Primitive::U16);
    serialize_primitive!(serialize_u32, u32, Primitive::U32);
    serialize_primitive!(serialize_u64, u64, Primitive::U64);
    serialize_primitive!(serialize_f32, f32, Primitive::F32);
    serialize_primitive!(serialize_f64, f64, Primitive::F64);
    serialize_primitive!(serialize_char, char, Primitive::Char);

    fn serialize_str(self, _v: &str) -> Result<Self::Ok> {
        Ok(TypeTag::String)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok> {
        Ok(TypeTag::ByteArray)
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Ok(TypeTag::None)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        let tt_serializer = TypeTagSerializer {};
        let tag = T::serialize(value, tt_serializer)?;
        Ok(TypeTag::Some(Box::new(tag)))
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(TypeTag::Unit)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        Ok(TypeTag::UnitStruct { name })
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        Ok(TypeTag::UnitVariant { name, variant })
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        let tt_serializer = TypeTagSerializer {};
        let tag = T::serialize(value, tt_serializer)?;
        Ok(TypeTag::NewTypeStruct {
            name,
            inner: Box::new(tag),
        })
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        let tt_serializer = TypeTagSerializer {};
        let tag = T::serialize(value, tt_serializer)?;
        Ok(TypeTag::NewTypeVariant {
            name,
            variant,
            inner: Box::new(tag),
        })
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        todo!()
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        todo!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        todo!()
    }

    fn serialize_struct(self, name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(TypeTagStruct {
            name,
            fields: Vec::new(),
        })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        todo!()
    }
}

impl ser::SerializeSeq for TypeTagSerializer {
    type Ok = TypeTag;
    type Error = Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl ser::SerializeTuple for TypeTagSerializer {
    type Ok = TypeTag;
    type Error = Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl ser::SerializeTupleStruct for TypeTagSerializer {
    type Ok = TypeTag;
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl ser::SerializeTupleVariant for TypeTagSerializer {
    type Ok = TypeTag;
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl ser::SerializeMap for TypeTagSerializer {
    type Ok = TypeTag;
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

#[derive(Debug)]
pub struct TypeTagStruct {
    name: &'static str,
    fields: Vec<(&'static str, TypeTag)>,
}

impl ser::SerializeStruct for TypeTagStruct {
    type Ok = TypeTag;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let tt_serializer = TypeTagSerializer {};
        let tag = T::serialize(value, tt_serializer)?;
        self.fields.push((key, tag));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(TypeTag::Struct {
            name: self.name,
            fields: self.fields,
        })
    }
}

impl ser::SerializeStructVariant for TypeTagSerializer {
    type Ok = TypeTag;
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}
