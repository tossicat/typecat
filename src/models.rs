use crate::markdown_parser::Rule;

#[derive(Debug)]
pub enum CONTENT {
    Word(String),
    Style {kind: Rule, word: String},
    Link {text: String, url: String},
    Newline
}

