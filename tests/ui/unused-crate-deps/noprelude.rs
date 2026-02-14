// noprelude --extern references don't count
//@ edition:2018
//@ check-pass
//@ aux-crate:baz=baz.rs
//@ aux-crate:noprelude:bar=bar.rs
//@ compile-flags: -Zunstable-options
#![warn(unused_crate_dependencies)]
//~^ WARNING extern crate `baz` is unused in crate `noprelude`
// No warning from bar is expected since it's noprelude
fn main() {}
