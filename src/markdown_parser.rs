use std::fs::read_to_string;
extern crate pest;
use pest::Parser;
use pest::iterators::Pair;
use crate::models::CONTENT;

#[derive(Parser)]
#[grammar = "markdown.pest"]
pub struct MarkdownParser;

pub fn parse_markdown() {
    let unparsed_file = read_to_string("test/pdf.md").expect("cannot read file");
    let file = MarkdownParser::parse(Rule::FILE, &unparsed_file)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
    for line in file.into_inner() {
        match line.as_rule() {
            Rule::HEADER => {
                parse_header(line);
            },
            Rule::TABLE_ALIGN | Rule::TABLE_LINE => {
                println!("###############################");
                parse_table(line);
            },
            Rule::LIST |  Rule::ORDERD_LIST => {
                println!("###############################");
                parse_list(line);
            },
            Rule::CODE_BLOCK => {
                println!("###############################");
                parse_code(line);
            },
            Rule::LINE => {
                parse_line(line);
            },
            _ => {}
        }
    }
}

fn parse_header(header: Pair<Rule>) {
    for line in header.into_inner(){
        if line.as_rule() == Rule::LINE {
            parse_line(line);
        }
        else {
            let header_size = line.as_rule();
            println!("{:?}", header_size);
        }
    }
}

fn parse_line(content: Pair<Rule>) {
    //STYLED, LINK
    let mut results: Vec<CONTENT> = vec![];

    for line in content.into_inner(){

        if line.as_rule() == Rule::CONTENTS {
            let piece = CONTENT::Word(line.as_str().to_string());
            results.push(piece);
        }
        else if line.as_rule() == Rule::STYLED {
            let (k, w) = _parse_styled(line);
            let style = CONTENT::Style {kind: k, word: w};
            results.push(style);
        }
        else if line.as_rule() == Rule::LINK {
        }
        else if line.as_str() == "\n" {
            println!("{:?}", line.as_rule());
            results.push(CONTENT::Newline);
        }
    }
    println!("{:?}", results);
}

fn _parse_styled(content: Pair<Rule>) -> (Rule, String) {
    //BOLDITALIC | BOLD | ITALIC | QUOTE | SUBSCRIPT | SUPERSCRIPT
    let mut k = Rule::STYLED;
    let mut w = "";
    for line in content.into_inner() {
        k = line.as_rule();
        w = line.into_inner().as_str();
    }
    return (k, w.to_string())
}

//styled만 들어가면 된다
fn _parse_link(content: Pair<Rule>) {
    //LINK_TEXT, LINK_URL
    println!("{:?}", content);
}

fn parse_table(content: Pair<Rule>) {
    //TABLE_WORD, LEFT_ALIGNED, CENTER_ALIGNED, RIGHT_ALIGNED
    println!("{:?}", content);
}

fn parse_code(content: Pair<Rule>) {
    //CONTENTS, NEWLINES
    println!("{:?}", content);
}

// NESTED 구현 로직 설계 필요
fn parse_list(content: Pair<Rule>) {
    //LIST, ORDERD_LIST
    println!("{:?}", content);
}
