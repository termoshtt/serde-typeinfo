# serde-typeinfo

"Serialize" type info to a runtime tag based on [serde data model](https://serde.rs/data-model.html).

Examples
---------

- `u8` integer will be "serialized" into `Primitive::U8` enum **without** its value

```rust
use serde_typeinfo::{TypeTag, Primitive};

assert_eq!(
    TypeTag::from_value(&32_u8),
    TypeTag::Primitive(Primitive::U8), // only tag, not includes 32
);
```

- User defined struct with `serde::Serialize` trait implementation
  will be "serialized" into `TypeTag::Struct` as its name and its fields' names and types,
  not includes values.

```rust
use serde_typeinfo::{TypeTag, Primitive};
use serde::Serialize;

#[derive(Serialize)]
struct A {
    a: u8,
    b: u8,
}

assert_eq!(
    TypeTag::from_value(&A { a: 2, b: 3 }),
    TypeTag::Struct {
        name: "A",
        fields: vec![
            ("a", Primitive::U8.into()),
            ("b", Primitive::U8.into()),
        ]
    }
);
```

License
--------

© 2023 Toshiki Teramura (@termoshtt)

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.
