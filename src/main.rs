use pest::Parser;
use pest::iterators::Pairs;
use pest_derive::Parser;

use std::fs;

fn concatable(token_rule: Rule) -> bool {
    match token_rule {
        Rule::non_terminal | Rule::token | Rule::name => {
            return true;
        }
        _ => {return false;}
    }
}


fn base_to_str(pairs: Pairs<Rule>) -> String {
    let mut res:Vec<String> = Vec::new();
    let mut pairs = pairs.into_iter();
    while let Some(pair) = pairs.next() {
        let pair_rule = pair.as_rule();
        match pair_rule {
            Rule::non_terminal => {
                res.push(pair.into_inner().next().unwrap().as_str().to_string());
            }
            Rule::pattern => {
                res.push(base_to_str(pair.into_inner()));
            }
            Rule::token => {
                res.push("\"".to_string() + pair.as_str() + "\"");
            }
            Rule::or | Rule::star | Rule::name => {
                res.push(pair.as_str().to_string());
            }
            _ => {}
        }
        if concatable(pair_rule) && pairs.peek().is_some_and(|r| concatable(r.as_rule())) {
            res.push("~".to_string());
        }
    }
    return res.join(" ");
}

fn generate_pest(pairs : Pairs<Rule>) -> &str {
    for pair in pairs.clone() {
        match pair.as_rule() {
            Rule::syntax_rule | Rule::regex_rule => {
                let mut rule = pair.into_inner();

                let non_terminal = rule.next().unwrap();
                let name = base_to_str(non_terminal.into_inner());

                let expansion = rule.skip(1).next().unwrap().into_inner();


                print!("\n{} = ", name);
                print!("{{ {} }}\n", base_to_str(expansion));
            }
            Rule::file | Rule::rule => {
                generate_pest(pair.into_inner());
            }
            Rule::pattern => {
                println!("pattern: {}", base_to_str(pair.into_inner()));
            }
            _ => {}
        }
    }
    return "";

}

#[derive(Parser)]
#[grammar = "bnf.pest"]
pub struct BNFParser;

fn main() {
    let tptp_bnf = fs::read_to_string("src/tptp.bnf").unwrap();
    let tptp_parse = BNFParser::parse(Rule::file, &tptp_bnf);
    println!("{:?}", tptp_parse.is_ok());
    generate_pest(tptp_parse.unwrap());

}
