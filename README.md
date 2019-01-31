Simplified, easy to use, pure Rust [MessagePack][msgpack] implementation focused
on handling dynamic data structures.

[Documentation][docs]

Example usage:

```rust
use msgpack_simple::{MsgPack, MapElement, Extension};

let message = MsgPack::Map(vec![
    MapElement {
        key: MsgPack::String(String::from("hello")),
        value: MsgPack::Int(42)
    },
    MapElement {
        key: MsgPack::String(String::from("world")),
        value: MsgPack::Array(vec![
            MsgPack::Boolean(true),
            MsgPack::Nil,
            MsgPack::Binary(vec![0x42, 0xff]),
            MsgPack::Extension(Extension {
                type_id: 2,
                value: vec![0x32, 0x4a, 0x67, 0x11]
            })
        ])
    }
]);

let encoded = message.encode(); // encoded is a Vec<u8>
let decoded = MsgPack::parse(&encoded).unwrap();

println!("{}", decoded);
assert_eq!(message, decoded);
assert!(message.is_map());

let mut map = message.as_map().unwrap(); // map is a Vec<MapElement>
let second_element = map.remove(1);

assert!(second_element.key.is_string());
assert_eq!(second_element.key.as_string().unwrap(), "world".to_string());

assert!(second_element.value.is_array());

let mut array = second_element.value.as_array().unwrap(); // array is a Vec<MsgPack>
let nil = array.remove(1);

assert!(nil.is_nil());
```

This library abstracts MessagePack data into a single [MsgPack enum][msgpack-enum]
which can correspond to any encodable data type and handle nested data
structures dynamically. It's not as performant as static solutions, for that,
[mneumann's rust-msgpack][rust-msgpack] and [3Hren's RMP][rmp] crates are
recommended, but it is able to parse messages without full prior knowledge of
their structure.

For more details, check out [the documentation][docs].

# Contributing, license, and other stuff

As always, pull requests, bug reports, suggestions, and other kinds of
improvements are welcome. Just be respectful towards each other, and maybe run
or create tests as appropriate.

msgpack_simple is available under the MIT license.

[msgpack]: https://msgpack.org
[docs]: https://docs.rs/msgpack_simple
[msgpack-enum]: https://docs.rs/msgpack_simple/latest/msgpack_simple/enum.MsgPack.html
[rust-msgpack]: https://github.com/mneumann/rust-msgpack
[rmp]: https://github.com/3Hren/msgpack-rust