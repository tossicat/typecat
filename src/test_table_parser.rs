use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "markdown.pest"]
pub struct TableParser;

pub fn parse_table() {
    let successful_parse = TableParser::parse(
        Rule::TABLE_LINE,
        "| Command | Description | 3 Description | \n",
    );

    println!("{:?}", successful_parse);

    let unsuccessful_parse = TableParser::parse(
        Rule::TABLE_ALIGN,
        "| --- | :------ | ------: | :------: | \n",
    );
    println!("{:?}", unsuccessful_parse);
}
