use std::fmt::{self, Display, Debug, Formatter};
use std::error::Error;

use super::{MsgPack};

/// An error that occurred when trying to access a field as a different type
/// 
/// The "as_type" functions of [MsgPack](enum.MsgPack.html) can throw this
/// error. It contains the original object and a string representation of the
/// attempted conversion.
pub struct ConversionError {
    /// The original object, owned.
    pub original: MsgPack,
    /// A string that contains which type conversion was attempted.
    pub attempted: &'static str,
}

impl Display for ConversionError {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        let original_type = match self.original {
            MsgPack::Nil => "nil",
            MsgPack::Int(_) => "int",
            MsgPack::Uint(_) => "uint",
            MsgPack::Float(_) => "float",
            MsgPack::Boolean(_) => "boolean",
            MsgPack::String(_) => "string",
            MsgPack::Binary(_) => "binary",
            MsgPack::Array(_) => "array",
            MsgPack::Map(_) => "map",
            MsgPack::Extension(_) => "extension",
        };
        write!(f, "MsgPack conversion error: cannot use {} as {}", original_type, self.attempted)
    }
}

impl Debug for ConversionError {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        let original_type = match self.original {
            MsgPack::Nil => "nil",
            MsgPack::Int(_) => "int",
            MsgPack::Uint(_) => "uint",
            MsgPack::Float(_) => "float",
            MsgPack::Boolean(_) => "boolean",
            MsgPack::String(_) => "string",
            MsgPack::Binary(_) => "binary",
            MsgPack::Array(_) => "array",
            MsgPack::Map(_) => "map",
            MsgPack::Extension(_) => "extension",
        };
        write!(f, "MsgPack conversion error: cannot use {} as {} (original value: {:?})", original_type, self.attempted, self.original)
    }
}

impl Error for ConversionError {}

impl ConversionError {
    /// Recovers the MsgPack object from the error
    /// 
    ///     use msgpack_simple::MsgPack;
    /// 
    ///     let float = MsgPack::Float(42.0);
    ///     let error = float.as_int().unwrap_err(); // trigger and capture an error
    ///     let recovered = error.recover();
    /// 
    ///     assert!(recovered.is_float());
    ///     assert_eq!(recovered.as_float().unwrap(), 42.0);
    pub fn recover (self) -> MsgPack {
        self.original
    }
}

/// An error that occurred while parsing a binary as MsgPack
pub struct ParseError {
    /// The byte where the error was found
    pub byte: usize
}

impl Display for ParseError {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "MsgPack parse error at byte {}", self.byte)
    }
}

impl Debug for ParseError {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "MsgPack parse error at byte {}", self.byte)
    }
}

impl Error for ParseError {}

impl ParseError {
    /// Creates another error of the same type with a byte offset
    /// 
    ///     use msgpack_simple::ParseError;
    /// 
    ///     let error = ParseError { byte: 5 };
    ///     let other = error.offset(3);
    /// 
    ///     assert_eq!(other.byte, 8);
    pub fn offset (&self, value: usize) -> ParseError {
        ParseError { byte: self.byte + value }
    }

    /// Takes a result with ParseError as its error type and returns the same
    /// with a byte offset on the error
    /// 
    ///     use msgpack_simple::ParseError;
    /// 
    ///     let result: Result<(), ParseError> = Err(ParseError { byte: 39 });
    ///     let other = ParseError::offset_result(result, 3);
    /// 
    ///     let error = other.unwrap_err();
    ///     assert_eq!(error.byte, 42);
    pub fn offset_result <T> (result: Result<T, ParseError>, value: usize) -> Result<T, ParseError> {
        result.map_err(|err| err.offset(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_error () {
        let error = ConversionError { original: MsgPack::Float(4.2), attempted: "int" };
        let error_message = format!("{}", error);
        assert_eq!(error_message, "MsgPack conversion error: cannot use float as int");

        let recovered = error.recover();
        assert!(recovered.is_float());
        assert_eq!(recovered.as_float().unwrap(), 4.2);
    }

    #[test]
    fn parse_error () {
        let error = ParseError { byte: 42 };
        let error_message = format!("{}", error);
        assert_eq!(error_message, "MsgPack parse error at byte 42");
    }
}