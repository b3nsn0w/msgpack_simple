//! Simplified, easy to use, pure Rust [MessagePack](https://msgpack.org)
//! implementation focused on handling dynamic data structures.
//! 
//! Example usage:
//! 
//!     use msgpack_simple::{MsgPack, MapElement, Extension};
//! 
//!     let message = MsgPack::Map(vec![
//!         MapElement {
//!             key: MsgPack::String(String::from("hello")),
//!             value: MsgPack::Int(42)
//!         },
//!         MapElement {
//!             key: MsgPack::String(String::from("world")),
//!             value: MsgPack::Array(vec![
//!                 MsgPack::Boolean(true),
//!                 MsgPack::Nil,
//!                 MsgPack::Binary(vec![0x42, 0xff]),
//!                 MsgPack::Extension(Extension {
//!                     type_id: 2,
//!                     value: vec![0x32, 0x4a, 0x67, 0x11]
//!                 })
//!             ])
//!         }
//!     ]);
//! 
//!     let encoded = message.encode(); // encoded is a Vec<u8>
//!     let decoded = MsgPack::parse(&encoded).unwrap();
//! 
//!     println!("{}", decoded);
//!     assert_eq!(message, decoded);
//!     assert!(message.is_map());
//! 
//!     let mut map = message.as_map().unwrap(); // map is a Vec<MapElement>
//!     let second_element = map.remove(1);
//! 
//!     assert!(second_element.key.is_string());
//!     assert_eq!(second_element.key.as_string().unwrap(), "world".to_string());
//! 
//!     assert!(second_element.value.is_array());
//! 
//!     let mut array = second_element.value.as_array().unwrap(); // array is a Vec<MsgPack>
//!     let nil = array.remove(1);
//! 
//!     assert!(nil.is_nil());
//! 
//! Data is abstracted with the [MsgPack enum](enum.MsgPack.html), which can
//! contain any kind of data encodable with MessagePack. This is designed for
//! dynamic data, for static models,
//! [mneumann's rust-msgpack](https://github.com/mneumann/rust-msgpack) or 
//! [3Hren's RMP](https://github.com/3Hren/msgpack-rust) crates are recommended.
//! 
//! # Decoding MsgPack
//! 
//! msgpack_simple provides two functions for decoding data. For general use,
//! `MsgPack::parse()` is recommended:
//! 
//!     use msgpack_simple::MsgPack;
//! 
//!     let data = vec![0xaa, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x52, 0x75, 0x73, 0x74];
//!     let decoded = MsgPack::parse(&data);
//!     assert!(decoded.is_ok());
//! 
//!     let decoded = decoded.unwrap();
//!     assert!(decoded.is_string());
//!     assert_eq!(decoded.as_string().unwrap(), "Hello Rust".to_string());
//! 
//! `MsgPack::parse()` takes a byte array slice (`&[u8]`) and returns an
//! [MsgPack enum](enum.MsgPack.html) wrapped in a result. The error type is
//! [ParseError](struct.ParseError.html), which can show the byte where the
//! parser encountered an error if needed.
//! 
//! If you need more control, you can use the `parser` module directly:
//! 
//!     use msgpack_simple::parser;
//! 
//!     let data = vec![0xaa, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x52, 0x75, 0x73, 0x74, 0x00];
//!     let (decoded, length) = parser::parse(&data).unwrap();
//! 
//!     assert!(decoded.is_string());
//!     assert_eq!(decoded.as_string().unwrap(), "Hello Rust".to_string());
//!     assert_eq!(length, 11);
//! 
//! `parser::parse()` behaves identically, but it also returns the length of the
//! MessagePack data parsed.
//! 
//! # Encoding MsgPack
//! 
//! msgpack_simple provides the `MsgPack.encode()` function for encoding data:
//! 
//!     use msgpack_simple::MsgPack;
//! 
//!     let message = MsgPack::String("Hello Rust".to_string());
//!     let encoded = message.encode();
//! 
//!     let data = vec![0xaa, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x52, 0x75, 0x73, 0x74];
//!     assert_eq!(encoded, data);
//! 
//! # Accessing MsgPack
//! 
//! For every variant of the enum (String, Float, Array, etc.) MsgPack provides
//! two convenience functions, one of which checks the data against the type 
//! (`is_string()`, `is_float()`, `is_array(`), etc.), and the other one
//! transforms the MsgPack enum into the value it contains (`as_string()`,
//! `as_float()`, `as_array()`, etc.).
//! 
//!     use msgpack_simple::MsgPack;
//! 
//!     let message = MsgPack::String("Hello Rust".to_string());
//! 
//!     assert_eq!(message.is_float(), false);
//!     assert_eq!(message.is_string(), true);
//! 
//!     let float = message.clone().as_float(); // the as_type functions consume the MsgPack
//!     let string = message.as_string();
//! 
//!     assert!(float.is_err());
//!     assert!(string.is_ok());
//! 
//! There are two special cases: `as_nil()` does not exist because the Nil
//! variant holds no data, and there is an `is_some_int()` and `as_some_int()`
//! pair, which matches both Int and Uint and returns `i64`.
//! 
//! # Arrays, Maps, and Extensions
//! 
//! One of MessagePack's greatest strengths is a compact representation of
//! dynamic, nested hierarchies. To access that, msgpack_simple provides simple
//! Rust abstractions for these types:
//! 
//!   - Array is represented with `Vec<MsgPack>`
//!   - Map is represented with `Vec<MapElement>`
//!   - Extension is represented with `Extension`
//! 
//! [MapElement](struct.MapElement.html) and [Extension](struct.Extension.html)
//! are two custom structs with simple representations of their respective
//! types. MapElement simply has a `key` and a `value`, both with the `MsgPack`
//! type, and Extension has a `type_id` (`i8`) and a `value` (`Vec<u8>`).
//! 
//!     use msgpack_simple::{MsgPack, MapElement, Extension};
//! 
//!     let message = MsgPack::Array(vec![
//!         MsgPack::Map(vec![
//!             MapElement {
//!                 key: MsgPack::String("foo".to_string()),
//!                 value: MsgPack::Int(42)
//!             },
//!             MapElement {
//!                 key: MsgPack::Extension(Extension {
//!                     type_id: 27,
//!                     value: vec![0x32]
//!                 }),
//!                 value: MsgPack::Binary(vec![0x2a, 0xf4])
//!             }
//!         ])
//!     ]);
//! 
//!     let mut array = message.as_array().unwrap();
//!     let mut map = array.remove(0).as_map().unwrap();
//! 
//!     let first = map.remove(0);
//!     let second = map.remove(0);
//! 
//!     assert_eq!(first.value.as_some_int().unwrap(), 42);
//!     assert_eq!(second.key.as_extension().unwrap().type_id, 27);

extern crate byteorder;
extern crate hex;
use byteorder::{BigEndian, WriteBytesExt};

mod error;
pub mod parser;

pub use self::error::{ConversionError, ParseError};

/// A piece of MessagePack-compatible data
/// 
///     use msgpack_simple::{MsgPack, MapElement, Extension};
/// 
///     let message = MsgPack::Map(vec![
///         MapElement {
///             key: MsgPack::String(String::from("hello")),
///             value: MsgPack::Int(42)
///         },
///         MapElement {
///             key: MsgPack::String(String::from("world")),
///             value: MsgPack::Array(vec![
///                 MsgPack::Boolean(true),
///                 MsgPack::Nil,
///                 MsgPack::Binary(vec![0x42, 0xff]),
///                 MsgPack::Extension(Extension {
///                     type_id: 2,
///                     value: vec![0x32, 0x4a, 0x67, 0x11]
///                 })
///             ])
///         }
///     ]);
#[derive(Debug, PartialEq, Clone)]
pub enum MsgPack {
    /// Empty value
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     let nil = MsgPack::Nil;
    ///     assert!(nil.is_nil());
    Nil,
    /// Signed integer, not much magic here
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     let int = MsgPack::Int(42);
    ///     assert!(int.is_int());
    ///     assert!(int.is_some_int());
    ///     assert_eq!(int.clone().as_int().unwrap(), 42);
    ///     assert_eq!(int.as_some_int().unwrap(), 42);
    Int(i64),
    /// Unsigned integer
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     let uint = MsgPack::Uint(42);
    ///     assert!(uint.is_uint());
    ///     assert!(uint.is_some_int());
    ///     assert_eq!(uint.clone().as_uint().unwrap(), 42);
    ///     assert_eq!(uint.as_some_int().unwrap(), 42);
    Uint(u64),
    /// Floating-point number
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     let float = MsgPack::Float(42.0);
    ///     assert!(float.is_float());
    ///     assert_eq!(float.as_float().unwrap(), 42.0);
    Float(f64),
    /// Boolean (wait, really?)
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     let boolean = MsgPack::Boolean(true);
    ///     assert!(boolean.is_boolean());
    ///     assert_eq!(boolean.as_boolean().unwrap(), true);
    Boolean(bool),
    /// Unicode compatible string
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     let string = MsgPack::String(String::from("foo"));
    ///     assert!(string.is_string());
    ///     assert_eq!(string.as_string().unwrap(), "foo".to_string());
    String(String),
    /// Raw binary value
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     let binary = MsgPack::Binary(vec![0x42]);
    ///     assert!(binary.is_binary());
    ///     assert_eq!(binary.as_binary().unwrap(), vec![0x42]);
    Binary(Vec<u8>),
    /// An array of other MsgPack fields
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     let array = MsgPack::Array(vec![
    ///         MsgPack::Int(42)
    ///     ]);
    ///     assert!(array.is_array());
    ///     assert_eq!(array.as_array().unwrap(), vec![MsgPack::Int(42)]);
    Array(Vec<MsgPack>),
    /// A map with key-value pairs, both being MsgPack data fields
    /// 
    ///     use msgpack_simple::{MsgPack, MapElement};
    /// 
    ///     let map = MsgPack::Map(vec![
    ///         MapElement {
    ///             key: MsgPack::String("foo".to_string()),
    ///             value: MsgPack::String("bar".to_string())
    ///         }
    ///     ]);
    ///     assert!(map.is_map());
    ///     assert_eq!(map.as_map().unwrap(), vec![MapElement {
    ///         key: MsgPack::String("foo".to_string()),
    ///         value: MsgPack::String("bar".to_string())
    ///     }]);
    Map(Vec<MapElement>),
    /// A tuple of an extension type and a raw data value
    /// 
    ///     use msgpack_simple::{MsgPack, Extension};
    /// 
    ///     let extension = MsgPack::Extension(Extension {
    ///         type_id: 42,
    ///         value: vec![0x42]
    ///     });
    ///     assert!(extension.is_extension());
    ///     assert_eq!(extension.as_extension().unwrap(), Extension { type_id: 42, value: vec![0x42] });
    Extension(Extension),
}

/// Represents an element in a MessagePack map
/// 
///     use msgpack_simple::{MsgPack, MapElement};
/// 
///     let map = MsgPack::Map(vec![
///         MapElement {
///             key: MsgPack::String("foo".to_string()),
///             value: MsgPack::String("bar".to_string())
///         }
///     ]);
#[derive(Debug, PartialEq, Clone)]
pub struct MapElement {
    pub key: MsgPack,
    pub value: MsgPack
}

/// Represents an extension field
/// 
///     use msgpack_simple::{MsgPack, Extension};
/// 
///     let extension = MsgPack::Extension(Extension {
///         type_id: 42,
///         value: vec![0x42]
///     });
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Extension {
    /// Type of the extension field. 0-127 are free to set by the application,
    /// but MessagePack reserves the negative type IDs for predefined types.
    pub type_id: i8,
    /// Raw binary value of the extension field
    pub value: Vec<u8>
}

impl MsgPack {
    /// Parses binary data as MessagePack
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     let data = vec![0xaa, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x52, 0x75, 0x73, 0x74];
    ///     let decoded = MsgPack::parse(&data);
    ///     assert!(decoded.is_ok());
    /// 
    ///     let decoded = decoded.unwrap();
    ///     assert!(decoded.is_string());
    ///     assert_eq!(decoded.as_string().unwrap(), "Hello Rust".to_string());
    pub fn parse (raw: &[u8]) -> Result<MsgPack, ParseError> {
        let (result, _) = parser::parse(raw)?;
        Ok(result)
    }

    /// Encodes a MsgPack enum into binary format
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     let message = MsgPack::String("Hello Rust".to_string());
    ///     let encoded = message.encode();
    /// 
    ///     let data = vec![0xaa, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x52, 0x75, 0x73, 0x74];
    ///     assert_eq!(encoded, data);
    pub fn encode (&self) -> Vec<u8> {
        match self {
            MsgPack::Nil => vec![0xc0],
            MsgPack::Boolean(value) => vec![if *value { 0xc3 } else { 0xc2 }],
            MsgPack::Int(value) => {
                let value = *value;
                if value >= 0 && value < 128 { return vec![value as u8] }
                if value < 0 && value > -32 {
                    let raw = unsafe { std::mem::transmute::<i8, u8>(value as i8) };
                    return vec![raw];
                };

                let mut result = vec![];

                if value >= -0x80 && value < 0x80 {
                    result.push(0xd0);
                    result.write_i8(value as i8).unwrap();
                } else if value >= -0x8000 && value < 0x8000 {
                    result.push(0xd1);
                    result.write_i16::<BigEndian>(value as i16).unwrap();
                } else if value >= -0x8000_0000 && value < 0x8000_0000 {
                    result.push(0xd2);
                    result.write_i32::<BigEndian>(value as i32).unwrap();
                } else {
                    result.push(0xd3);
                    result.write_i64::<BigEndian>(value).unwrap();
                }

                result
            },
            MsgPack::Uint(value) => {
                let value = *value;
                // not writing Uint as fixint retains integer types in the decoded value
                // if value < 128 { return vec![value as u8] }

                let mut result = vec![];

                if value <= 0x88 {
                    result.push(0xcc);
                    result.write_u8(value as u8).unwrap();
                } else if value <= 0x8888 {
                    result.push(0xcd);
                    result.write_u16::<BigEndian>(value as u16).unwrap();
                } else if value <= 0x8888_8888 {
                    result.push(0xce);
                    result.write_u32::<BigEndian>(value as u32).unwrap();
                } else {
                    result.push(0xcf);
                    result.write_u64::<BigEndian>(value).unwrap();
                }

                result
            },
            MsgPack::Float(value) => {
                // since it's nontrivial when float32 is enough and when it's not, we're just going to always use float64
                let mut result = vec![0xcb];
                let int_value = unsafe { std::mem::transmute::<f64, u64>(*value) };

                result.write_u64::<BigEndian>(int_value).unwrap();
                result
            },
            MsgPack::String(value) => {
                let bytes = value.as_bytes();
                let length = bytes.len();
                let mut result = Vec::with_capacity(length + 5);

                // encode length
                if length < 32 {
                    result.push(0xa0 | length as u8);
                } else if length <= 0x88 {
                    result.push(0xd9);
                    result.write_u8(length as u8).unwrap();
                } else if length <= 0x8888 {
                    result.push(0xda);
                    result.write_u16::<BigEndian>(length as u16).unwrap();
                } else {
                    result.push(0xdb);
                    result.write_u32::<BigEndian>(length as u32).unwrap();
                }

                // now that length is encoded, time to add the actual string
                result.extend_from_slice(bytes);
                result
            }
            MsgPack::Binary(value) => {
                let length = value.len();
                let mut result = Vec::with_capacity(length + 5);

                // encode length
                if length <= 0x88 {
                    result.push(0xc4);
                    result.write_u8(length as u8).unwrap();
                } else if length <= 0x8888 {
                    result.push(0xc5);
                    result.write_u16::<BigEndian>(length as u16).unwrap();
                } else {
                    result.push(0xc6);
                    result.write_u32::<BigEndian>(length as u32).unwrap();
                }

                // after length is encoded, add the actual value
                result.extend_from_slice(value);
                result
            },
            MsgPack::Extension(extension) => {
                let value = &extension.value;
                let type_id = unsafe { std::mem::transmute::<i8, u8>(extension.type_id) };

                let length = value.len();
                let mut result = Vec::with_capacity(length + 6);

                // encode length (wow there are a lot of options here)
                if length == 1 {
                    result.push(0xd4);
                } else if length == 2 {
                    result.push(0xd5);
                } else if length == 4 {
                    result.push(0xd6);
                } else if length == 8 {
                    result.push(0xd7);
                } else if length == 16 {
                    result.push(0xd8);
                } else if length <= 0x88 {
                    result.push(0xc7);
                    result.write_u8(length as u8).unwrap();
                } else if length <= 0x8888 {
                    result.push(0xc8);
                    result.write_u16::<BigEndian>(length as u16).unwrap();
                } else {
                    result.push(0xc9);
                    result.write_u32::<BigEndian>(length as u32).unwrap();
                }

                // with length encoded now we can add the tuple
                result.push(type_id);
                result.extend_from_slice(value);
                result
            },
            MsgPack::Array(value) => {
                let length = value.len();
                let mut result = vec![];

                // encode length
                if length < 16 {
                    result.push(0x90 | length as u8);
                } else if length <= 0x8888 {
                    result.push(0xdc);
                    result.write_u16::<BigEndian>(length as u16).unwrap();
                } else {
                    result.push(0xdd);
                    result.write_u32::<BigEndian>(length as u32).unwrap();
                }

                // now just add all the values
                for item in value {
                    result.append(&mut item.encode());
                }

                result
            },
            MsgPack::Map(value) => {
                let length = value.len();
                let mut result = vec![];

                // encode length
                if length < 16 {
                    result.push(0x80 | length as u8);
                } else if length <= 0x8888 {
                    result.push(0xde);
                    result.write_u16::<BigEndian>(length as u16).unwrap();
                } else {
                    result.push(0xdf);
                    result.write_u32::<BigEndian>(length as u32).unwrap();
                }

                // and add the values
                for item in value {
                    result.append(&mut item.key.encode());
                    result.append(&mut item.value.encode());
                }

                result
            }
        }
    }

    // convenience functions

    /// Checks if the MsgPack is an int variant
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     assert_eq!(MsgPack::Int(42).is_int(), true);
    ///     assert_eq!(MsgPack::Float(42.0).is_int(), false);
    pub fn is_int (&self) -> bool {
        match self {
            MsgPack::Int(_) => true,
            _ => false
        }
    }
    /// Consumes the MsgPack as int
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     assert_eq!(MsgPack::Int(42).as_int().unwrap(), 42);
    pub fn as_int (self) -> Result<i64, ConversionError> {
        match self {
            MsgPack::Int(value) => Ok(value),
            _ => Err(ConversionError { original: self, attempted: "int" })
        }
    }
    /// Checks if the MsgPack is a uint variant
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     assert_eq!(MsgPack::Uint(42).is_uint(), true);
    ///     assert_eq!(MsgPack::Float(42.0).is_uint(), false);
    pub fn is_uint (&self) -> bool {
        match self {
            MsgPack::Uint(_) => true,
            _ => false
        }
    }
    /// Consumes the MsgPack as uint
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     assert_eq!(MsgPack::Uint(42).as_uint().unwrap(), 42);
    pub fn as_uint (self) -> Result<u64, ConversionError> {
        match self {
            MsgPack::Uint(value) => Ok(value),
            _ => Err(ConversionError { original: self, attempted: "uint" })
        }
    }
    /// Checks if the MsgPack is one of the integer variants
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     assert_eq!(MsgPack::Int(42).is_some_int(), true);
    ///     assert_eq!(MsgPack::Uint(42).is_some_int(), true);
    ///     assert_eq!(MsgPack::Float(42.0).is_some_int(), false);
    pub fn is_some_int (&self) -> bool {
        match self {
            MsgPack::Uint(_) => true,
            MsgPack::Int(_) => true,
            _ => false
        }
    }
    /// Consumes the MsgPack as an int, even if it's a uint
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     assert_eq!(MsgPack::Int(42).as_some_int().unwrap(), 42);
    ///     assert_eq!(MsgPack::Uint(42).as_some_int().unwrap(), 42);
    pub fn as_some_int (self) -> Result<i64, ConversionError> {
        match self {
            MsgPack::Int(value) => Ok(value),
            MsgPack::Uint(value) => Ok(value as i64),
            _ => Err(ConversionError { original: self, attempted: "int" })
        }
    }
    /// Checks if the MsgPack is a float
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     assert_eq!(MsgPack::Float(42.0).is_float(), true);
    ///     assert_eq!(MsgPack::Int(42).is_float(), false);
    pub fn is_float (&self) -> bool {
        match self {
            MsgPack::Float(_) => true,
            _ => false
        }
    }
    /// Consumes the MsgPack as a float
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     assert_eq!(MsgPack::Float(42.0).as_float().unwrap(), 42.0);
    pub fn as_float (self) -> Result<f64, ConversionError> {
        match self {
            MsgPack::Float(value) => Ok(value),
            _ => Err(ConversionError { original: self, attempted: "float" })
        }
    }
    /// Checks if the MsgPack is a boolean
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     assert_eq!(MsgPack::Boolean(true).is_boolean(), true);
    ///     assert_eq!(MsgPack::Int(1).is_boolean(), false);
    pub fn is_boolean (&self) -> bool {
        match self {
            MsgPack::Boolean(_) => true,
            _ => false
        }
    }
    /// Consumes the MsgPack as a boolean
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     assert_eq!(MsgPack::Boolean(true).as_boolean().unwrap(), true);
    pub fn as_boolean (self) -> Result<bool, ConversionError> {
        match self {
            MsgPack::Boolean(value) => Ok(value),
            _ => Err(ConversionError { original: self, attempted: "boolean" })
        }
    }
    /// Checks if the MsgPack is a nil
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     assert_eq!(MsgPack::Nil.is_nil(), true);
    ///     assert_eq!(MsgPack::Boolean(false).is_nil(), false);
    pub fn is_nil (&self) -> bool {
        match self {
            MsgPack::Nil => true,
            _ => false
        }
    }
    /// Checks if the MsgPack is a string
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     assert_eq!(MsgPack::String("foo".to_string()).is_string(), true);
    ///     assert_eq!(MsgPack::Binary(vec![0x66, 0x6f, 0x6f]).is_string(), false);
    pub fn is_string (&self) -> bool {
        match self {
            MsgPack::String(_) => true,
            _ => false
        }
    }
    /// Consumes the MsgPack as a string
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     assert_eq!(MsgPack::String("foo".to_string()).as_string().unwrap(), "foo".to_string());
    pub fn as_string (self) -> Result<String, ConversionError> {
        match self {
            MsgPack::String(value) => Ok(value),
            _ => Err(ConversionError { original: self, attempted: "string" })
        }
    }
    /// Checks if the MsgPack is a binary
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     assert_eq!(MsgPack::Binary(vec![0x66, 0x6f, 0x6f]).is_binary(), true);
    ///     assert_eq!(MsgPack::String("foo".to_string()).is_binary(), false);
    pub fn is_binary (&self) -> bool {
        match self {
            MsgPack::Binary(_) => true,
            _ => false
        }
    }
    /// Consumes the MsgPack as a binary
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     assert_eq!(MsgPack::Binary(vec![0x66, 0x6f, 0x6f]).as_binary().unwrap(), vec![0x66, 0x6f, 0x6f]);
    pub fn as_binary (self) -> Result<Vec<u8>, ConversionError> {
        match self {
            MsgPack::Binary(value) => Ok(value),
            _ => Err(ConversionError { original: self, attempted: "binary" })
        }
    }
    /// Checks if the MsgPack is an array
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     assert_eq!(MsgPack::Array(vec![]).is_array(), true);
    ///     assert_eq!(MsgPack::Map(vec![]).is_array(), false);
    pub fn is_array (&self) -> bool {
        match self {
            MsgPack::Array(_) => true,
            _ => false
        }
    }
    /// Consumes the MsgPack as an array
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     assert_eq!(MsgPack::Array(vec![]).as_array().unwrap(), vec![]);
    pub fn as_array (self) -> Result<Vec<MsgPack>, ConversionError> {
        match self {
            MsgPack::Array(value) => Ok(value),
            _ => Err(ConversionError { original: self, attempted: "array" })
        }
    }
    /// Checks if the MsgPack is a map
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     assert_eq!(MsgPack::Map(vec![]).is_map(), true);
    ///     assert_eq!(MsgPack::Array(vec![]).is_map(), false);
    pub fn is_map (&self) -> bool {
        match self {
            MsgPack::Map(_) => true,
            _ => false
        }
    }
    /// Consumes the MsgPack as a map
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     assert_eq!(MsgPack::Map(vec![]).as_map().unwrap(), vec![]);
    pub fn as_map (self) -> Result<Vec<MapElement>, ConversionError> {
        match self {
            MsgPack::Map(value) => Ok(value),
            _ => Err(ConversionError { original: self, attempted: "map" })
        }
    }
    /// Checks if the MsgPack is an extension
    /// 
    ///     use msgpack_simple::{MsgPack, Extension};
    ///     let value = Extension { type_id: 42, value: vec![0x42] };
    /// 
    ///     assert_eq!(MsgPack::Extension(value).is_extension(), true);
    ///     assert_eq!(MsgPack::Binary(vec![0x42]).is_extension(), false);
    pub fn is_extension (&self) -> bool {
        match self {
            MsgPack::Extension(_) => true,
            _ => false
        }
    }
    /// Consumes the MsgPack as an extension
    /// 
    ///     use msgpack_simple::{MsgPack, Extension};
    ///     let value = Extension { type_id: 42, value: vec![0x42] };
    /// 
    ///     assert_eq!(MsgPack::Extension(value.clone()).as_extension().unwrap(), value);
    pub fn as_extension (self) -> Result<Extension, ConversionError> {
        match self {
            MsgPack::Extension(value) => Ok(value),
            _ => Err(ConversionError { original: self, attempted: "extension" })
        }
    }
}

impl std::fmt::Display for MsgPack {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MsgPack::Nil => write!(f, "nil"),
            MsgPack::Boolean(value) => write!(f, "{}", value),
            MsgPack::Int(value) => write!(f, "{}", value),
            MsgPack::Uint(value) => write!(f, "{}", value),
            MsgPack::Float(value) => write!(f, "{}", value),
            MsgPack::String(value) => write!(f, "\"{}\"", value),
            MsgPack::Binary(value) => write!(f, "bin:{}", hex::encode(value)),
            MsgPack::Extension(value) => write!(f, "ext:{}:{}", value.type_id, hex::encode(&value.value)),
            MsgPack::Array(value) => {
                write!(f, "[")?;

                let mut first = true;
                for item in value {
                    if !first { write!(f, ", ")? }
                    first = false;
                    write!(f, "{}", item)?;
                }

                write!(f, "]")
            },
            MsgPack::Map(value) => {
                write!(f, "{{")?;

                let mut first = true;
                for item in value {
                    if !first { write!(f, ", ")? }
                    first = false;
                    write!(f, "{}: ", item.key)?;
                    write!(f, "{}", item.value)?;
                }

                write!(f, "}}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_from_json () {
        let data = &vec![0x82, 0xa7, 0x63, 0x6f, 0x6d, 0x70, 0x61, 0x63, 0x74, 0xc3, 0xa6, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x93, 0x01, 0x02, 0xcb, 0x3f, 0xf5, 0x1e, 0xb8, 0x51, 0xeb, 0x85, 0x1f];

        let parsed = MsgPack::parse(data).unwrap();
        println!("{}", parsed);

        assert!(parsed.is_map());
        let map = parsed.as_map().unwrap();

        assert_eq!(map.len(), 2);

        let mut map = map.into_iter();
        let first = map.next().unwrap();
        let second = map.next().unwrap();

        assert!(first.key.is_string());
        assert!(first.value.is_boolean());

        assert!(second.key.is_string());
        assert!(second.value.is_array());

        assert_eq!(first.key.as_string().unwrap(), "compact");
        assert_eq!(first.value.as_boolean().unwrap(), true);

        assert_eq!(second.key.as_string().unwrap(), "schema");

        let mut array = second.value.as_array().unwrap().into_iter();

        let first = array.next().unwrap();
        let second = array.next().unwrap();
        let third = array.next().unwrap();

        assert!(array.next().is_none());

        assert!(first.is_some_int());
        assert_eq!(first.as_some_int().unwrap(), 1);

        assert!(second.is_some_int());
        assert_eq!(second.as_some_int().unwrap(), 2);

        assert!(third.is_float());
        assert_eq!(third.as_float().unwrap(), 1.32);
    }

    #[test]
    fn encode () {
        let message = MsgPack::Map(vec![
            MapElement {
                key: MsgPack::String(String::from("hello")),
                value: MsgPack::Int(0x424242)
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

        let encoded = message.encode();
        let decoded = MsgPack::parse(&encoded).unwrap();

        println!("{}", decoded);
        assert_eq!(message, decoded);
    }
}
