use serde::Serialize;

/// Type tag based on [serde data model](https://serde.rs/data-model.html)
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum TypeTag {
    /// 14 primitive types,
    /// `bool`,
    /// `i8`, `i16`, `i32`, `i64`, `i128`,
    /// `u8`, `u16`, `u32`, `u64`, `u128`,
    /// `f32`, `f64`, and `char`
    ///
    /// ```
    /// # use serde_typeinfo::*;
    /// assert_eq!(TypeTag::from_default::<u8>(), TypeTag::Primitive(Primitive::U8));
    /// assert_eq!(TypeTag::from_value(&3_u32), TypeTag::Primitive(Primitive::U32));
    /// ```
    Primitive(Primitive),

    /// ```
    /// # use serde_typeinfo::*;
    /// assert_eq!(TypeTag::from_value(&"homhom"), TypeTag::String);
    /// ```
    String,

    /// byte array, `[u8]`
    ByteArray,

    Some(Box<Self>),
    None,

    /// The type of `()` in Rust.
    /// It represents an anonymous value containing no data.
    ///
    /// ```
    /// # use serde_typeinfo::*;
    /// assert_eq!(TypeTag::from_value(&()), TypeTag::Unit);
    /// ```
    Unit,

    /// unit_struct
    ///
    /// For example `struct Unit` or `PhantomData<T>`.
    /// It represents a named value containing no data.
    ///
    /// ```
    /// # use serde_typeinfo::*;
    /// # use serde::Serialize;
    /// #[derive(Serialize)]
    /// struct Unit;
    ///
    /// assert_eq!(
    ///     TypeTag::from_value(&Unit {}),
    ///     TypeTag::UnitStruct { name: "Unit" },
    /// );
    /// ```
    UnitStruct {
        name: &'static str,
    },

    /// For example the E::A and E::B in enum E { A, B }.
    ///
    /// ```
    /// # use serde_typeinfo::*;
    /// # use serde::Serialize;
    /// #[derive(Serialize)]
    /// enum E { A, B };
    ///
    /// assert_eq!(
    ///     TypeTag::from_value(&E::A),
    ///     TypeTag::UnitVariant { name: "E", variant: "A" },
    /// );
    /// ```
    UnitVariant {
        name: &'static str,
        variant: &'static str,
    },

    /// For example `struct Millimeters(u8)`.
    ///
    /// ```
    /// # use serde_typeinfo::*;
    /// # use serde::Serialize;
    /// #[derive(Serialize)]
    /// struct Millimeters(u8);
    ///
    /// assert_eq!(
    ///     TypeTag::from_value(&Millimeters(100_u8)),
    ///     TypeTag::NewTypeStruct {
    ///         name: "Millimeters",
    ///         inner: Box::new(TypeTag::Primitive(Primitive::U8)),
    ///     },
    /// );
    /// ```
    NewTypeStruct {
        name: &'static str,
        inner: Box<Self>,
    },

    /// For example the `E::N` in `enum E { N(u8) }`.
    ///
    /// ```
    /// # use serde_typeinfo::*;
    /// # use serde::Serialize;
    /// #[derive(Serialize)]
    /// enum E { N(u8) };
    ///
    /// assert_eq!(
    ///     TypeTag::from_value(&E::N(1_u8)),
    ///     TypeTag::NewTypeVariant {
    ///         name: "E",
    ///         variant: "N",
    ///         inner: Box::new(TypeTag::Primitive(Primitive::U8)),
    ///     },
    /// );
    /// ```
    NewTypeVariant {
        name: &'static str,
        variant: &'static str,
        inner: Box<Self>,
    },

    /// ```
    /// # use serde_typeinfo::*;
    /// # use serde::Serialize;
    /// #[derive(Serialize)]
    /// enum Value {
    ///     Bool(bool),
    ///     Char(char),
    /// }
    ///
    /// // heterogeneous sequence
    /// assert_eq!(
    ///     TypeTag::from_value(&vec![Value::Bool(true), Value::Char('c')]),
    ///     TypeTag::Seq(Seq::Hetero(vec![
    ///         TypeTag::from_value(&Value::Bool(true)),
    ///         TypeTag::from_value(&Value::Char('c')),
    ///     ]))
    /// );
    ///
    /// // homogeneous sequence
    /// assert_eq!(
    ///     TypeTag::from_value(&vec![Value::Bool(true), Value::Bool(false)]),
    ///     TypeTag::Seq(Seq::Homo {
    ///         tag: Box::new(TypeTag::from_value(&Value::Bool(true))),
    ///         size: 2,
    ///     })
    /// );
    /// ```
    Seq(Seq),

    /// ```
    /// # use serde_typeinfo::*;
    /// # use serde::Serialize;
    ///
    /// // heterogeneous tuple
    /// assert_eq!(
    ///     TypeTag::from_value(&(true, 'c')),
    ///     TypeTag::Tuple(Seq::Hetero(vec![
    ///         TypeTag::Primitive(Primitive::Bool),
    ///         TypeTag::Primitive(Primitive::Char),
    ///     ]))
    /// );
    ///
    /// // homogeneous tuple
    /// assert_eq!(
    ///     TypeTag::from_value(&[1_u8; 10]),
    ///     TypeTag::Tuple(Seq::Homo {
    ///         tag: Box::new(TypeTag::Primitive(Primitive::U8)),
    ///         size: 10,
    ///     })
    /// );
    /// ```
    Tuple(Seq),

    /// ```
    /// # use serde_typeinfo::*;
    /// # use serde::Serialize;
    /// #[derive(Serialize)]
    /// struct S(u8, u8);
    ///
    /// assert_eq!(
    ///     TypeTag::from_value(&S(1, 2)),
    ///     TypeTag::TupleStruct {
    ///         name: "S",
    ///         fields: vec![
    ///             TypeTag::Primitive(Primitive::U8),
    ///             TypeTag::Primitive(Primitive::U8),
    ///         ]
    ///     },
    /// );
    /// ```
    TupleStruct {
        name: &'static str,
        fields: Vec<Self>,
    },

    /// ```
    /// # use serde_typeinfo::*;
    /// # use serde::Serialize;
    /// #[derive(Serialize)]
    /// enum E { S(u8, u8) };
    ///
    /// assert_eq!(
    ///     TypeTag::from_value(&E::S(1, 2)),
    ///     TypeTag::TupleVariant {
    ///         name: "E",
    ///         variant: "S",
    ///         fields: vec![
    ///             TypeTag::Primitive(Primitive::U8),
    ///             TypeTag::Primitive(Primitive::U8),
    ///         ]
    ///     },
    /// );
    /// ```
    TupleVariant {
        name: &'static str,
        variant: &'static str,
        fields: Vec<Self>,
    },

    /// ```
    /// # use serde_typeinfo::*;
    /// # use serde::Serialize;
    /// #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
    /// enum Value {
    ///     Bool(bool),
    ///     Char(char),
    /// }
    ///
    /// assert_eq!(
    ///     TypeTag::from_value(&maplit::btreemap!{ // Must be ordered
    ///         Value::Bool(true) => Value::Char('c'),
    ///         Value::Char('a') => Value::Bool(false),
    ///     }),
    ///     TypeTag::Map(Map::Hetero(vec![
    ///         (
    ///             TypeTag::from_value(&Value::Bool(true)),
    ///             TypeTag::from_value(&Value::Char('c')),
    ///         ),
    ///         (
    ///             TypeTag::from_value(&Value::Char('a')),
    ///             TypeTag::from_value(&Value::Bool(false)),
    ///         ),
    ///     ]))
    /// );
    /// ```
    Map(Map),

    /// ```
    /// # use serde_typeinfo::*;
    /// # use serde::Serialize;
    /// #[derive(Serialize)]
    /// struct S { r: u8, g: u8, b: u8 };
    ///
    /// assert_eq!(
    ///     TypeTag::from_value(&S { r: 0, g: 1, b: 2 }),
    ///     TypeTag::Struct {
    ///         name: "S",
    ///         fields: vec![
    ///             ("r", TypeTag::Primitive(Primitive::U8)),
    ///             ("g", TypeTag::Primitive(Primitive::U8)),
    ///             ("b", TypeTag::Primitive(Primitive::U8)),
    ///         ]
    ///     },
    /// );
    /// ```
    Struct {
        name: &'static str,
        fields: Vec<(&'static str, Self)>,
    },

    /// ```
    /// # use serde_typeinfo::*;
    /// # use serde::Serialize;
    /// #[derive(Serialize)]
    /// enum E { S { r: u8, g: u8, b: u8 } }
    ///
    /// assert_eq!(
    ///     TypeTag::from_value(&E::S { r: 0, g: 1, b: 2 }),
    ///     TypeTag::StructVariant {
    ///         name: "E",
    ///         variant: "S",
    ///         fields: vec![
    ///             ("r", TypeTag::Primitive(Primitive::U8)),
    ///             ("g", TypeTag::Primitive(Primitive::U8)),
    ///             ("b", TypeTag::Primitive(Primitive::U8)),
    ///         ]
    ///     },
    /// );
    /// ```
    StructVariant {
        name: &'static str,
        variant: &'static str,
        fields: Vec<(&'static str, Self)>,
    },
}

impl TypeTag {
    pub fn from_value<T: ?Sized + Serialize>(value: &T) -> Self {
        let serializer = crate::serializer::TypeTagSerializer {};
        Serialize::serialize(value, serializer).unwrap()
    }

    pub fn from_default<T: ?Sized + Serialize + Default>() -> Self {
        let value = T::default();
        Self::from_value(&value)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
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

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Serialize)]
pub enum Seq {
    /// Homogeneous sequence of types
    Homo { tag: Box<TypeTag>, size: usize },
    /// Heterogeneous sequence of types
    Hetero(Vec<TypeTag>),
    /// No elements
    #[default]
    Empty,
}

impl Seq {
    pub fn push(&mut self, new_tag: TypeTag) {
        match self {
            Self::Hetero(ref mut tags) => tags.push(new_tag),
            Self::Homo { tag, size } => {
                if tag.as_ref() == &new_tag {
                    *size += 1;
                } else {
                    let mut hetero = vec![*tag.clone(); *size];
                    hetero.push(new_tag);
                    *self = Self::Hetero(hetero);
                }
            }
            Self::Empty => {
                *self = Self::Homo {
                    tag: Box::new(new_tag),
                    size: 1,
                };
            }
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Serialize)]
pub enum Map {
    /// Homogeneous map
    Homo {
        key: Box<TypeTag>,
        value: Box<TypeTag>,
        size: usize,
    },
    /// Heterogeneous map
    Hetero(Vec<(TypeTag, TypeTag)>),
    /// No elements
    #[default]
    Empty,
}

impl Map {
    pub fn push(&mut self, new_key: TypeTag, new_value: TypeTag) {
        match self {
            Self::Hetero(ref mut tags) => tags.push((new_key, new_value)),
            Self::Homo { key, value, size } => {
                if key.as_ref() == &new_key && value.as_ref() == &new_value {
                    *size += 1;
                } else {
                    let mut hetero = vec![(*key.clone(), *value.clone()); *size];
                    hetero.push((new_key, new_value));
                    *self = Self::Hetero(hetero);
                }
            }
            Self::Empty => {
                *self = Self::Homo {
                    key: Box::new(new_key),
                    value: Box::new(new_value),
                    size: 1,
                };
            }
        }
    }
}
