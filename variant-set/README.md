# `variant-set`

`variant-set` provides a set-like data structure for enum variants. It allows you to store at most one value for each variant of an enum,
providing efficient storage and retrieval based on enum variants.

## Use-case

Suppose you have a large, complex enum such as the following:

```rust
enum MyEnum {
    Variant1(u32),
    Variant2(String),
    Variant3(f64),
    Variant4(bool),
    Variant5(Vec<u8>),
    Variant6 {
        field1: u32,
        field2: String,
    },
    Variant7(SomeLargeStruct),
}
```

You could store the data of this enum like so:

```rust
struct MyStruct {
    variant1: Option<u32>,
    variant2: Option<String>,
    variant3: Option<f64>,
    variant4: Option<bool>,
    variant5: Option<Vec<u8>>,
    variant6: Option<(u32, String)>,
    variant7: Option<SomeLargeStruct>,
}
```

However, the size of this struct is the sum of the sizes of all the fields, even if only one field is actually used. For large
enough structs, this can be inefficient.

You effectively want a `HashSet<MyEnum>` where you can store at most one value for each variant. However, you do not care about the
specific value stored for each variant, only that you can store and retrieve it efficiently based on the variant itself.
This is what `variant-set` provides. Just derive the `VariantEnum` trait for your enum, and you can use the `VariantSet` data structure:

```rust
use variant_set::{VariantSet, VariantEnum};

#[derive(VariantEnum)]
enum MyEnum {
    Variant1(u32),
    Variant2(String),
    Variant3(f64),
    Variant4(bool),
    Variant5(Vec<u8>),
    Variant6 {
        field1: u32,
        field2: String,
    },
    Variant7(SomeLargeStruct),
}

fn main() {
    let mut set = VariantSet::new();
    set.set(MyEnum::Variant1(42));
    set.set(MyEnum::Variant2("hello".to_string()));

    assert!(set.contains(MyEnumVariant::Variant1));
    assert!(set.contains(MyEnumVariant::Variant2));
    assert!(!set.contains(MyEnumVariant::Variant3));

    let value = set.get(MyEnumVariant::Variant1);
    assert_eq!(value, Some(&MyEnum::Variant1(42)));

    let value = set.get(MyEnumVariant::Variant2);
    assert_eq!(value, Some(&MyEnum::Variant2("hello".to_string())));
}
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
variant-set = "0.1.0"
```

## API Documentation

See the `docs.rs` documentation [here](https://docs.rs/variant-set/0.1.0/variant_set/).

## Contributing

Contributions are welcome! Please feel free to open an issue or a pull request on GitHub.

## License

This project is licensed under the CC0 License - see the [LICENSE](LICENSE) file for details.
