#![recursion_limit = "128"]

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate web_sys;
extern crate wasm_bindgen;

mod common;

pub use common::*;

#[cfg(not(target_arch = "wasm32"))]
extern crate ws;

#[cfg(target_arch = "wasm32")]
mod stdw;
#[cfg(target_arch = "wasm32")]
pub use stdw::*;

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use native::*;
