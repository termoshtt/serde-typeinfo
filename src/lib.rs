pub mod error;
pub mod serializer;

mod tag;
pub use tag::*;

use serde::Serialize;

pub fn type_of_value<T: Serialize>(value: &T) -> tag::TypeTag {
    let serializer = crate::serializer::TypeTagSerializer {};
    Serialize::serialize(value, serializer).unwrap()
}

pub fn type_info<T: Serialize + Default>() -> tag::TypeTag {
    let value = T::default();
    type_of_value(&value)
}

#[cfg_attr(doc, doc = include_str!("../README.md"))]
mod readme {}

#[cfg(test)]
mod test {
    use crate::*;
    use serde::Serialize;

    #[test]
    fn u8() {
        assert_eq!(type_of_value(&0_u8), TypeTag::Primitive(Primitive::U8));
    }

    #[derive(Serialize)]
    struct A {
        a: u8,
        b: u8,
    }

    #[test]
    fn struct_a() {
        assert_eq!(
            type_of_value(&A { a: 0, b: 0 }),
            TypeTag::Struct {
                name: "A",
                fields: vec![("a", Primitive::U8.into()), ("b", Primitive::U8.into()),]
            }
        );
    }
}
