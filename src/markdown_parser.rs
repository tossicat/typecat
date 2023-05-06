use std::fs::read_to_string;
extern crate pest;
use pest::Parser;

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
                println!("{:?}", line);
                println!("{:?}", line.into_inner().count());
                println!("###############################")
            }
            else {
                println!("{:?}", line);
                println!("{:?}", line.into_inner().count());
            }
        }
}
