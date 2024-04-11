use pest::Parser;
use pest::iterators::Pairs;
use pest_derive::Parser;

use std::fs;

fn generate_pest(pairs : Pairs<Rule>) -> &str {
    let mut result = "";
    for pair in pairs.clone() {
        match pair.as_rule() {
            Rule::syntax_rule | Rule::regex_rule => {
                let rule = pair.into_inner();
                println!("{} = {{}}\n", rule);
            }
            _ => {
                generate_pest(pair.into_inner());
            }
        }
    }
    return result;

}

#[derive(Parser)]
#[grammar = "bnf.pest"]
pub struct BNFParser;

fn main() {
    let thf_bnf = fs::read_to_string("src/thf.bnf").unwrap();
    let tptp_bnf = fs::read_to_string("src/tptp.bnf").unwrap();
    let thf_parse = BNFParser::parse(Rule::file, &thf_bnf);
    let tptp_parse = BNFParser::parse(Rule::file, &tptp_bnf);
    println!("{:?}", thf_parse.is_ok());
    //println!("{:?}", tptp_parse.is_ok());

    println!("{}", thf_parse.unwrap());
    //println!("{}", generate_pest(thf_parse.unwrap()));
}
