use crate::renderer::html::attribute::Attribute;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlTokenizer {
    state: State,
    pos: usize, // usize型=符号なし整数型
    reconsume: bool,
    latest_token: Option<HtmlToken>,
    input: Vec<char>,
    buf: String,
}

impl HtmlTokenizer {
    pub fn new(html: String) -> Self {
        return Self {
            state: State::Data,
            pos: 0,
            reconsume: false,
            latest_token: None,
            input: html.chars().collect(),
            buf: String::new(),
        };
    }
}

// htmlのトークンを定義
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HtmlToken {
    StartTag {
        tag_name: String,
        self_closing: bool, // 終了タグがない場合はtrue
        attributes: Vec<Attribute>,
    },
    EndTag {
        tag_name: String,
    },
    Char(char),
    Eof,
}
