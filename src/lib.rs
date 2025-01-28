#![recursion_limit = "1024"]

extern crate base64;
extern crate futures;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate lazy_static;
extern crate regex;
#[macro_use]
extern crate serde;
#[cfg(test)]
extern crate serde_bytes;
#[macro_use]
extern crate serde_derive;
pub extern crate rouille;
extern crate serde_xml_rs;
extern crate xml;

#[macro_use]
extern crate quick_error;

pub mod client;
pub mod error;
pub mod server;
mod xmlfmt;

pub use client::{call, call_value, Client};
pub use hyper::Url;
pub use server::Server;
pub use xmlfmt::{from_params, into_params, Call, Fault, Params, Response, Value};
