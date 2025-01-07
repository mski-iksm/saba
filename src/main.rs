#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::{String, ToString};
use net_wasabi::http::HttpClient;
use noli::prelude::*;

fn main() -> u64 {
    let client: HttpClient = HttpClient::new();
    match client.get("host.test".to_string(), 8000, "test.html".to_string()) {
        Ok(response) => {
            print!("Response: \n{:#?}", response);
        }
        Err(e) => {
            print!("Error: \n{:#?}", e);
        }
    }
    0
}

entry_point!(main);
