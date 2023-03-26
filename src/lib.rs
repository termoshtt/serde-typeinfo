pub mod error;
pub mod serializer;
pub mod tag;

#[cfg(test)]
mod test {
    use crate::tag::*;

    #[test]
    fn u8() {
        let a = 0_u8;
        let serializer = crate::serializer::TypeTagSerializer {};
        let tag = serde::Serialize::serialize(&a, serializer).unwrap();
        assert_eq!(tag, TypeTag::Primitive(Primitive::U8));
    }
}
