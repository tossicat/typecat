use std::fs::read_to_string;
extern crate pest;
use pest::Parser;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "markdown.pest"]
pub struct MarkdownParser;

pub fn parse_markdown() {
    let unparsed_file = read_to_string("test/test.md").expect("cannot read file");
    let file = MarkdownParser::parse(Rule::FILE, &unparsed_file)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
    for line in file.into_inner() {
            if line.as_rule() == Rule::HEADER {
                parse_header(line);
                println!("###############################")
            }
            else {
                parse_sentence(line);
            }
        }
}

fn parse_header(header: Pair<Rule>) {
    println!("{:?}", header);
}

fn parse_sentence(sentence: Pair<Rule>) {
    println!("{:?}", sentence);
}