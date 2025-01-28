#![allow(unknown_lints, unused_doc_comments)]
use base64::DecodeError;
use serde::{de, ser};
use std::fmt::{self, Debug, Display};
pub use quick_error::quick_error;

quick_error! {
    pub enum Error {
        Fmt(err: fmt::Error) {
            from()
            description("Issue while formatting")
            display("Issue while formatting: {}", err)
        }
        Decoding(t: String) {
            description("Issue while decoding data structure")
            display("Issue while decoding data structure: {}", t)
        }
        Encoding(t: String) {
            description("Issue while encoding data structure")
            display("Issue while encoding data structure: {}", t)
        }
        UnsupportedData(t: String) {
            description("Given structure is not supported")
            display("Given structure is not supported: {}", t)
        }
        IoError(err : std::io::Error) {
            from()
        }
        XmlError(err : serde_xml_rs::Error) {
            from()
        }
        ParseFloatError(err : std::num::ParseFloatError) {
            from()
        }
        Base64DecodeError(err : DecodeError) {
            from()
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self, f)
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Error {
        Error::Decoding(format!("{}", msg)).into()
    }

    fn invalid_type(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self {
        Error::custom(format_args!("invalid type: {}, expected {}", unexp, exp))
    }
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Error {
        Error::Encoding(format!("{}", msg)).into()
    }
}

pub type Result<T> = std::result::Result<T, Error>;