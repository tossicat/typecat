use crate::markdown::md_parser::Rule;

#[derive(Debug)]
pub enum FragmentType {
    WORD(Word),
    LINK(Link),
    Newline,
}

#[derive(Debug)]
pub struct Word {
    pub(crate) kind: Rule,
    pub(crate) text: String,
}

impl Word {
    pub fn new() -> Self {
        return Word {
            kind: Rule::CONTENTS,
            text: String::new(),
        };
    }
}

#[derive(Debug)]
pub struct Link {
    pub(crate) text: Vec<Word>,
    pub(crate) url: String,
}

impl Link {
    pub fn new() -> Self {
        let word = vec![Word::new()];
        return Link {
            text: word,
            url: String::new(),
        };
    }
}
