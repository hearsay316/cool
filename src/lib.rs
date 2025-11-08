#![deny(clippy::all)]

use napi_derive::napi;

#[napi]
pub fn plus_1000(input: u32) -> u32 {
  input + 1000
}
