#![deny(clippy::all)]
mod algo;
use napi_derive::napi;

#[napi]
pub fn plus(a: u32, b: u32) -> u32 {
  a + b
}
