//! The actual parser under the hood

use super::{MsgPack, MapElement, Extension};
use super::error::ParseError;

fn read_8 (raw: &[u8]) -> u64 {
    raw[0] as u64
}

fn read_16 (raw: &[u8]) -> u64 {
    raw[1] as u64 | (raw[0] as u64) << 8
}

fn read_32 (raw: &[u8]) -> u64 {
    raw[3] as u64 | (raw[2] as u64) << 8 | (raw[1] as u64) << 16 | (raw[0] as u64) << 24
}

fn read_64 (raw: &[u8]) -> u64 {
    raw[7] as u64 | (raw[6] as u64) << 8 | (raw[5] as u64) << 16 | (raw[4] as u64) << 24
    | (raw[3] as u64) << 32 | (raw[2] as u64) << 40 | (raw[1] as u64) << 48 | (raw[0] as u64) << 56
}

/// Parses binary data as MsgPack, returning both the result and the length of
/// the data. Useful when you have other data directly following in the slice.
/// 
///     use msgpack_simple::parser;
/// 
///     let data = vec![0xaa, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x52, 0x75, 0x73, 0x74, 0x00];
///     let (decoded, length) = parser::parse(&data).unwrap();
/// 
///     assert!(decoded.is_string());
///     assert_eq!(decoded.as_string().unwrap(), "Hello Rust".to_string());
///     assert_eq!(length, 11);
pub fn parse (raw: &[u8]) -> Result<(MsgPack, usize), ParseError> {
    if raw.len() < 1 { return Err(ParseError { byte: 0 }) }
    let first_byte = raw[0];

    if first_byte <= 0x7f { // positive fixint
        return Ok((MsgPack::Int(first_byte as i64), 1))
    }
    if first_byte >= 0xe0 { // negative fixint
        return Ok((MsgPack::Int(first_byte as i64 - 256), 1))
    }

    if first_byte >= 0x80 && first_byte <= 0x8f { // fixmap
        let len = (first_byte & 0x0f) as usize;
        let (value, size) = ParseError::offset_result(parse_map(&raw[1..], len), 1)?;

        return Ok((MsgPack::Map(value), 1 + size));
    }

    if first_byte >= 0x90 && first_byte <= 0x9f { // fixarray
        let len = (first_byte & 0x0f) as usize;
        let (value, size) = ParseError::offset_result(parse_array(&raw[1..], len), 1)?;

        return Ok((MsgPack::Array(value), 1 + size));
    }

    if first_byte >= 0xa0 && first_byte <= 0xbf { // fixstr
        let len = (first_byte & 0x1f) as usize;
        if raw.len() < 1 + len { return Err(ParseError { byte: 1 }) }

        let value = String::from_utf8(raw[1..1 + len].to_vec()).map_err(|_| ParseError { byte: 1 })?;
        return Ok((MsgPack::String(value), 1 + len));
    }

    if first_byte == 0xc0 { return Ok((MsgPack::Nil, 1)) } // nil
    if first_byte == 0xc1 { return Err(ParseError { byte: 0 }) } // never used
    if first_byte == 0xc2 { return Ok((MsgPack::Boolean(false), 1)) } // false
    if first_byte == 0xc3 { return Ok((MsgPack::Boolean(true), 1)) } // true

    if first_byte == 0xc4 { // bin 8
        if raw.len() < 2 { return Err(ParseError { byte: 1 }) }
        let len = read_8(&raw[1..]) as usize;

        if raw.len() < 2 + len { return Err(ParseError {byte: 2}) }
        let value = raw[2..2 + len].to_vec();

        return Ok((MsgPack::Binary(value), 2 + len))
    }

    if first_byte == 0xc5 { // bin 16
        if raw.len() < 3 { return Err(ParseError { byte: 1 }) }
        let len = read_16(&raw[1..]) as usize;

        if raw.len() < 3 + len { return Err(ParseError {byte: 3}) }
        let value = raw[3..3 + len].to_vec();

        return Ok((MsgPack::Binary(value), 3 + len))
    }

    if first_byte == 0xc6 { // bin 32
        if raw.len() < 5 { return Err(ParseError { byte: 1 }) }
        let len = read_32(&raw[1..]) as usize;

        if raw.len() < 5 + len { return Err(ParseError { byte: 5 }) }
        let value = raw[5..5 + len].to_vec();

        return Ok((MsgPack::Binary(value), 5 + len))
    }

    if first_byte == 0xc7 { // ext 8
        if raw.len() < 3 { return Err(ParseError { byte: 1 }) }
        let len = read_8(&raw[1..]) as usize;
        let type_id = unsafe { std::mem::transmute::<u8, i8>(raw[2]) };

        if raw.len() < 3 + len { return Err(ParseError { byte: 3 }) }
        let value = raw[3..3 + len].to_vec();

        return Ok((MsgPack::Extension(Extension { type_id, value }), 3 + len))
    }

    if first_byte == 0xc8 { // ext 16
        if raw.len() < 4 { return Err(ParseError { byte: 1 }) }
        let len = read_16(&raw[1..]) as usize;
        let type_id = unsafe { std::mem::transmute::<u8, i8>(raw[3]) };

        if raw.len() < 4 + len { return Err(ParseError { byte: 4 }) }
        let value = raw[4..4 + len].to_vec();

        return Ok((MsgPack::Extension(Extension { type_id, value }), 4 + len))
    }

    if first_byte == 0xc9 { // ext 32
        if raw.len() < 6 { return Err(ParseError { byte: 1 }) }
        let len = read_32(&raw[1..]) as usize;
        let type_id = unsafe { std::mem::transmute::<u8, i8>(raw[5]) };

        if raw.len() < 6 + len { return Err(ParseError { byte: 6 }) }
        let value = raw[6..6 + len].to_vec();

        return Ok((MsgPack::Extension(Extension { type_id, value }), 6 + len))
    }

    if first_byte == 0xca { // float 32
        if raw.len() < 5 { return Err(ParseError { byte: 1 }) }

        let raw_value = read_32(&raw[1..]) as u32;
        let value = unsafe { std::mem::transmute::<u32, f32>(raw_value) };

        return Ok((MsgPack::Float(value as f64), 5));
    }

    if first_byte == 0xcb { // float 64
        if raw.len() < 9 { return Err(ParseError { byte: 1 }) }

        let raw_value = read_64(&raw[1..]);
        let value = unsafe { std::mem::transmute::<u64, f64>(raw_value) };

        return Ok((MsgPack::Float(value), 9));
    }

    if first_byte == 0xcc { // uint 8
        if raw.len() < 2 { return Err(ParseError { byte: 1 }) }

        let value = read_8(&raw[1..]);
        return Ok((MsgPack::Uint(value), 2));
    }

    if first_byte == 0xcd { // uint 16
        if raw.len() < 3 { return Err(ParseError { byte: 1 }) }

        let value = read_16(&raw[1..]);
        return Ok((MsgPack::Uint(value), 3));
    }

    if first_byte == 0xce { // uint 32
        if raw.len() < 5 { return Err(ParseError { byte: 1 }) }

        let value = read_32(&raw[1..]);
        return Ok((MsgPack::Uint(value), 5));
    }
    
    if first_byte == 0xcf { // uint 64
        if raw.len() < 9 { return Err(ParseError { byte: 1 }) }

        let value = read_64(&raw[1..]);
        return Ok((MsgPack::Uint(value), 9));
    }

    if first_byte == 0xd0 { // int 8
        if raw.len() < 2 { return Err(ParseError { byte: 1 }) }

        let raw_value = read_8(&raw[1..]);
        let value = unsafe { std::mem::transmute::<u64, i64>(raw_value) };

        return Ok((MsgPack::Int(value), 2));
    }

    if first_byte == 0xd1 { // int 16
        if raw.len() < 3 { return Err(ParseError { byte: 1 }) }

        let raw_value = read_16(&raw[1..]);
        let value = unsafe { std::mem::transmute::<u64, i64>(raw_value) };

        return Ok((MsgPack::Int(value), 3));
    }

    if first_byte == 0xd2 { // int 32
        if raw.len() < 5 { return Err(ParseError { byte: 1 }) }

        let raw_value = read_32(&raw[1..]);
        let value = unsafe { std::mem::transmute::<u64, i64>(raw_value) };

        return Ok((MsgPack::Int(value), 5));
    }
    
    if first_byte == 0xd3 { // int 64
        if raw.len() < 9 { return Err(ParseError { byte: 1 }) }

        let raw_value = read_64(&raw[1..]);
        let value = unsafe { std::mem::transmute::<u64, i64>(raw_value) };

        return Ok((MsgPack::Int(value), 9));
    }

    if first_byte == 0xd4 { // fixext 1
        if raw.len() < 3 { return Err(ParseError { byte: 1 }) }

        let type_id = unsafe { std::mem::transmute::<u8, i8>(raw[1]) };
        let value = raw[2..3].to_vec();

        return Ok((MsgPack::Extension(Extension { type_id, value }), 3));
    }

    if first_byte == 0xd5 { // fixext 2
        if raw.len() < 4 { return Err(ParseError { byte: 1 }) }

        let type_id = unsafe { std::mem::transmute::<u8, i8>(raw[1]) };
        let value = raw[2..4].to_vec();

        return Ok((MsgPack::Extension(Extension { type_id, value }), 4));
    }

    if first_byte == 0xd6 { // fixext 4
        if raw.len() < 6 { return Err(ParseError { byte: 1 }) }

        let type_id = unsafe { std::mem::transmute::<u8, i8>(raw[1]) };
        let value = raw[2..6].to_vec();

        return Ok((MsgPack::Extension(Extension { type_id, value }), 6));
    }

    if first_byte == 0xd7 { // fixext 8
        if raw.len() < 10 { return Err(ParseError { byte: 1 }) }

        let type_id = unsafe { std::mem::transmute::<u8, i8>(raw[1]) };
        let value = raw[2..10].to_vec();

        return Ok((MsgPack::Extension(Extension { type_id, value }), 10));
    }

    if first_byte == 0xd8 { // fixext 16
        if raw.len() < 18 { return Err(ParseError { byte: 1 }) }

        let type_id = unsafe { std::mem::transmute::<u8, i8>(raw[1]) };
        let value = raw[2..18].to_vec();

        return Ok((MsgPack::Extension(Extension { type_id, value }), 18));
    }

    if first_byte == 0xd9 { // str 8
        if raw.len() < 2 { return Err(ParseError { byte: 1 }) }

        let len = read_8(&raw[1..]) as usize;
        if raw.len() < 2 + len { return Err(ParseError { byte: 2 }) }

        let value = String::from_utf8(raw[2..2 + len].to_vec()).map_err(|_| ParseError { byte: 2 })?;
        return Ok((MsgPack::String(value), 2 + len));
    }

    if first_byte == 0xda { // str 16
        if raw.len() < 3 { return Err(ParseError { byte: 1 }) }

        let len = read_16(&raw[1..]) as usize;
        if raw.len() < 3 + len { return Err(ParseError { byte: 3 }) }

        let value = String::from_utf8(raw[3..3 + len].to_vec()).map_err(|_| ParseError { byte: 3 })?;
        return Ok((MsgPack::String(value), 3 + len));
    }

    if first_byte == 0xdb { // str 32
        if raw.len() < 5 { return Err(ParseError { byte: 1 }) }

        let len = read_32(&raw[1..]) as usize;
        if raw.len() < 5 + len { return Err(ParseError { byte: 5 }) }

        let value = String::from_utf8(raw[5..5 + len].to_vec()).map_err(|_| ParseError { byte: 5 })?;
        return Ok((MsgPack::String(value), 5 + len));
    }

    if first_byte == 0xdc { // array 16
        if raw.len() < 3 { return Err(ParseError { byte: 1 }) }
        
        let len = read_16(&raw[1..]) as usize;
        let (value, size) = ParseError::offset_result(parse_array(&raw[3..], len), 3)?;

        return Ok((MsgPack::Array(value), 3 + size));
    }

    if first_byte == 0xdd { // array 32
        if raw.len() < 5 { return Err(ParseError { byte: 1 }) }
        
        let len = read_32(&raw[1..]) as usize;
        let (value, size) = ParseError::offset_result(parse_array(&raw[5..], len), 5)?;

        return Ok((MsgPack::Array(value), 5 + size));
    }

    if first_byte == 0xde { // map 16
        if raw.len() < 3 { return Err(ParseError { byte: 1 }) }

        let len = read_16(&raw[1..]) as usize;
        let (value, size) = ParseError::offset_result(parse_map(&raw[3..], len), 3)?;

        return Ok((MsgPack::Map(value), 3 + size));
    }

    if first_byte == 0xdf { // map 32
        if raw.len() < 5 { return Err(ParseError { byte: 1 }) }

        let len = read_32(&raw[1..]) as usize;
        let (value, size) = ParseError::offset_result(parse_map(&raw[5..], len), 5)?;

        return Ok((MsgPack::Map(value), 5 + size));
    }

    Err(ParseError { byte: 0 })
}

fn parse_array (raw: &[u8], length: usize) -> Result<(Vec<MsgPack>, usize), ParseError> {
    let mut cursor = 0usize;
    let mut result = Vec::with_capacity(length);

    for _ in 0..length {
        let (value, size) = ParseError::offset_result(parse(&raw[cursor..]), cursor)?;
        result.push(value);
        cursor += size;
    }

    Ok((result, cursor))
}

fn parse_map (raw: &[u8], length: usize) -> Result<(Vec<MapElement>, usize), ParseError> {
    let (elements, size) = parse_array(&raw, length * 2)?;
    let mut result = Vec::with_capacity(length);

    if elements.len() != length * 2 { unreachable!() }
    let mut iter = elements.into_iter();

    for _ in 0..length {
        let key = iter.next().unwrap();
        let value = iter.next().unwrap();

        let element = MapElement { key, value };
        result.push(element);
    }

    Ok((result, size))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endian_reader () {
        assert_eq!(read_8(&vec![0x32]), 0x32);
        assert_eq!(read_16(&vec![0x42, 0x58]), 0x4258);
        assert_eq!(read_32(&vec![0x3a, 0x9c, 0x64, 0x82]), 0x3a9c6482);
    }

    #[test]
    fn primitives () {
        let (parsed, length) = parse(&vec![0xc0]).unwrap();
        assert_eq!(length, 1);
        assert!(parsed.is_nil());

        let (parsed, length) = parse(&vec![0xc2]).unwrap();
        assert_eq!(length, 1);
        assert!(parsed.is_boolean());
        assert_eq!(parsed.as_boolean().unwrap(), false);

        let (parsed, length) = parse(&vec![0xc3]).unwrap();
        assert_eq!(length, 1);
        assert!(parsed.is_boolean());
        assert_eq!(parsed.as_boolean().unwrap(), true);
    }

    #[test]
    fn numbers () {
        let (parsed, length) = parse(&vec![0xcb, 0x3f, 0xf6, 0xb8, 0x51, 0xeb, 0x85, 0x1e, 0xb8]).unwrap();
        assert_eq!(length, 9);
        assert!(parsed.is_float());
        assert_eq!(parsed.as_float().unwrap(), 1.42);
    }

    #[test]
    fn objects () {
        let (parsed, length) = parse(&vec![0x82, 0xa7, 0x63, 0x6f, 0x6d, 0x70, 0x61, 0x63, 0x74, 0xc3, 0xa6, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x93, 0x01, 0x02, 0xcb, 0x3f, 0xf5, 0x1e, 0xb8, 0x51, 0xeb, 0x85, 0x1f]).unwrap();

        println!("{} {:?}", length, parsed);
    }
}