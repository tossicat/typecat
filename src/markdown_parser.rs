use std::fs::read_to_string;
extern crate pest;
use pest::Parser;
use pest::iterators::Pair;

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
                println!("################################");
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
                println!("###############################");
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
            println!("{:?}", line.as_rule());
        }
    }
}

fn parse_line(content: Pair<Rule>) {
    //STYLED, LINK
    println!("{:?}", content);
}

fn _parse_styled(content: Pair<Rule>) {
    //BOLD, ITALIC, QUOTE_CODE, STRIKETHROUGH, BOLDITALIC ... CLOSED_SUPERSCRIPT
    println!("{:?}", content);
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
