use crate::markdown::datatypes::{FragmentType, Link, Word};
use crate::markdown::parser::Rule;
use pest::iterators::Pair;

pub fn parse_header(header: Pair<Rule>) -> (Rule, Vec<FragmentType>) {
    let mut texts: Vec<FragmentType> = vec![];
    let mut size = Rule::HEADER;
    for line in header.into_inner() {
        if line.as_rule() == Rule::LINE {
            texts = parse_line(line);
        } else {
            let header_size = line.as_rule();
            size = header_size;
        }
    }
    return (size, texts);
}

pub fn parse_line(content: Pair<Rule>) -> Vec<FragmentType> {
    //STYLED, LINK

    let mut results: Vec<FragmentType> = vec![];

    if content.as_str() == "\n" {
        results.push(FragmentType::Newline);
        return results;
    }

    for line in content.into_inner() {
        if line.as_rule() == Rule::CONTENTS {
            let piece = FragmentType::WORD(Word {
                kind: Rule::CONTENTS,
                text: line.as_str().to_string(),
            });
            results.push(piece);
        } else if line.as_rule() == Rule::STYLED {
            let (k, w) = _parse_styled(line);
            let style = FragmentType::WORD(Word { kind: k, text: w });
            results.push(style);
        } else if line.as_rule() == Rule::LINK {
            let l = _parse_link(line);
            let link = FragmentType::LINK(l);
            results.push(link);
        }
    }
    return results;
}

fn _parse_styled(content: Pair<Rule>) -> (Rule, String) {
    //BOLDITALIC | BOLD | ITALIC | QUOTE | SUBSCRIPT | SUPERSCRIPT
    let mut k = Rule::STYLED;
    let mut w = "";
    for line in content.into_inner() {
        k = line.as_rule();
        w = line.into_inner().as_str();
    }
    return (k, w.to_string());
}

fn _parse_link(content: Pair<Rule>) -> Link {
    let mut results = Link::new();

    for link in content.into_inner() {
        if link.as_rule() == Rule::LINK_TEXT {
            let text = _parse_link_text(link);
            results.text = text;
        } else if link.as_rule() == Rule::LINK_URL {
            let url = _parse_link_url(link);
            results.url = url;
        }
    }
    return results;
}

fn _parse_link_text(link: Pair<Rule>) -> Vec<Word> {
    let mut texts: Vec<Word> = vec![];
    for word in link.into_inner() {
        if word.as_rule() == Rule::CONTENTS {
            let piece = Word {
                kind: Rule::CONTENTS,
                text: word.as_str().to_string(),
            };
            texts.push(piece);
        } else if word.as_rule() == Rule::STYLED {
            let (k, w) = _parse_styled(word);
            let style = Word { kind: k, text: w };
            texts.push(style);
        }
    }
    return texts;
}

fn _parse_link_url(link: Pair<Rule>) -> String {
    let mut urls = String::new();
    for url in link.into_inner() {
        urls.push_str(url.as_str());
    }
    return urls;
}

pub fn parse_table(content: Pair<Rule>) {
    //TABLE_WORD, LEFT_ALIGNED, CENTER_ALIGNED, RIGHT_ALIGNED
    println!("{:?}", content);
}

pub fn parse_code(content: Pair<Rule>) {
    //CONTENTS, NEWLINES
    println!("{:?}", content);
}

// NESTED 구현 로직 설계 필요
pub fn parse_list(content: Pair<Rule>) {
    //LIST, ORDERD_LIST
    println!("{:?}", content);
}
