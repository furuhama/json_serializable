## JSON Serializable(WIP)

Simple JSON format serializer for Rust

## Usage

```rust
serializable! {
    struct Person {
        name: String,
        age: u8,
    }
}

let person = Person { name: "Julien Sorel".to_string(), age: 16 };
let serialized = person.serialize().unwrap();

assert_eq!(r#"{"Person":{"name":"Julien Sorel","age":"16",}}"#, serialized);
```

## Test

```
$ cargo test
```
