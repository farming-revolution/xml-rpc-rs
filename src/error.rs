#![allow(unknown_lints, unused_doc_comments)]
pub use super::xmlfmt::error::Error as FmtError;

use quick_error::quick_error;

quick_error! {
    pub enum Error {
        XmlFormat(err: FmtError) {
            from()
            description("XmlFormat error")
            display("Error in xml formatting: {}", err)
        }
        BindFail(details: String) {
            description("Failed to bind XML-RPC server to port")
            display("Failed to bind XML-RPC server to port: {}", details)
        }
        HyperError(err: hyper::Error) {
            from()
        }
        IoError(err : std::io::Error) {
            from()
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

