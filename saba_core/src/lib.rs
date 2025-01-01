#![no_std]

extern crate alloc;

pub mode url;

use alloc::string::String;

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    url: String,
    host: String,
    port: String,
    path: String,
    searchPath: String,
}

impl Url{
    pub fn new(url: String) -> Self{
        Self{
            url,
            host: "".to_string(), // ""だと&strになっちゃう。Stringにすることで所有権を持つし、可変にできる。
            port: "".to_string(),
            path: "".to_string(),
            searchPath: "".to_string(),
        }
    }

    pub fn parse(&mut self)->Result<Self,String>{
        // Results Ok(T) Err(E) にくるまった値を返す
        // T: 成功時の戻り値の型
        // E: エラー時の戻り値の型
        // Result<T,E> は、TかEをそれぞれ設定
        // 実際に返ってくるのは、Ok(T)かErr(E)のいずれか


        pass
    }
}