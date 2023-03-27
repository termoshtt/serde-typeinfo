use serde::Serialize;

/// Type tag based on [serde data model](https://serde.rs/data-model.html)
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TypeTag {
    Primitive(Primitive),
    String,
    ByteArray,
    Some(Box<Self>),
    None,
    Unit,
    UnitStruct {
        name: &'static str,
    },
    UnitVariant {
        name: &'static str,
        variant: &'static str,
    },
    NewTypeStruct {
        name: &'static str,
        inner: Box<Self>,
    },
    NewTypeVariant {
        name: &'static str,
        variant: &'static str,
        inner: Box<Self>,
    },
    Struct {
        name: &'static str,
        fields: Vec<(&'static str, Self)>,
    },
    StructVariant {
        name: &'static str,
        variant: &'static str,
        fields: Vec<(&'static str, Self)>,
    },
    // TODO more entries
}

impl TypeTag {
    pub fn from_value<T: Serialize>(value: &T) -> Self {
        let serializer = crate::serializer::TypeTagSerializer {};
        Serialize::serialize(value, serializer).unwrap()
    }

    pub fn from_default<T: Serialize + Default>() -> Self {
        let value = T::default();
        Self::from_value(&value)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Primitive {
    Bool,
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
    Char,
}

impl From<Primitive> for TypeTag {
    fn from(p: Primitive) -> Self {
        TypeTag::Primitive(p)
    }
}
