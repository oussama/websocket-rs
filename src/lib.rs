#![recursion_limit = "128"]

#[macro_use]
extern crate serde_derive;

mod common;

pub use crate::common::*;

#[cfg(target_arch = "wasm32")]
mod stdw;
#[cfg(target_arch = "wasm32")]
pub use crate::stdw::*;

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use native::*;
