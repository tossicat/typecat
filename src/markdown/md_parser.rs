use std::fs::read_to_string;
extern crate pest;
use pest::Parser;
use crate::markdown::datatypes::{FragmentType, Link, Word};
use crate::markdown::functions::*;

#[derive(Parser)]
#[grammar = "markdown/markdown.pest"]
pub struct MarkdownParser;

pub fn parse() -> Vec<(Rule, Vec<FragmentType>)> {
    let unparsed_file = read_to_string("test/pdf.md").expect("cannot read file");
    let file = MarkdownParser::parse(Rule::FILE, &unparsed_file)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    let mut results = vec![];

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::HEADER => {
                let data = parse_header(line);
                results.push(data);
            }
            Rule::TABLE_ALIGN | Rule::TABLE_LINE => {
                println!("###############################");
                parse_table(line);
            }
            Rule::LIST | Rule::ORDERD_LIST => {
                println!("###############################");
                parse_list(line);
            }
            Rule::CODE_BLOCK => {
                println!("###############################");
                parse_code(line);
            }
            Rule::LINE => {
                let data = parse_line(line);
                results.push((Rule::LINE, data));
            }
            _ => {}
        }
    }
    return results;
}
