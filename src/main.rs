#![no_std]
#![cfg_attr(not(target_os = "linux"), no_main)]

use noli::prelude::*;

fn main() {
    println!("Hello, world!");
}

entry_point!(main);