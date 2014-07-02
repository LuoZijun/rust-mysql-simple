#![crate_id="mysql"]	
#![comment="Mysql client library writen in rust"]
#![license="MIT"]
#![crate_type="rlib"]
#![crate_type="dylib"]

#![feature(unsafe_destructor)]

#![allow(dead_code)]
#![feature(macro_rules)]

#[cfg(test)]
extern crate test;
extern crate sync;
extern crate core;
extern crate debug;
extern crate time;

mod scramble;
pub mod consts;
pub mod error;
pub mod packet;
pub mod io;
pub mod value;
pub mod conn;
