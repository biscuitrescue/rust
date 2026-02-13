// noprelude --extern references don't count
//@ edition:2018
//@ check-pass
//@ aux-crate:noprelude:bar=bar.rs
//@ compile-flags: -Zunstable-options
#![warn(unused_crate_dependencies)]
fn main() {}
