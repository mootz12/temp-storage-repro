#![no_std]

#[cfg(any(test, feature = "testutils"))]
extern crate std;

pub mod contract;
pub mod storage;

#[cfg(test)]
mod test;

pub use contract::*;
