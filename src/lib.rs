#![allow(unknown_lints, verbose_bit_mask)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![feature(untagged_unions)]
extern crate libc;

#[macro_use]
mod macros;
mod bindings;
pub mod ulong_extras;
pub mod fmpz;
pub mod fmpq;
pub mod fmpz_mat;
