/// Type tag represented as serde data model
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TypeTag {
    Primitive(Primitive),
    String(String),
    Struct {
        name: String,
        fields: Vec<(String, Self)>,
    },
    // TODO more entries
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
