use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "md_table.pest"]
pub struct TableParser;

pub fn parse_table() {
    let successful_parse =
        TableParser::parse(Rule::table, "| Command | Description | 3 Description | \n");

    println!("{:?}", successful_parse);

    let unsuccessful_parse =
        TableParser::parse(Rule::table, "| --- | :------ | ------: | :------: | \n");
    println!("{:?}", unsuccessful_parse);
}
