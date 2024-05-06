use pest::Parser;
use pest::iterators::Pairs;
use pest_derive::Parser;

use std::fs;

fn concatable(token_rule: Rule) -> bool {
    match token_rule {
        Rule::non_terminal | Rule::token | Rule::name | Rule::regex_chars 
        | Rule::regex_expression | Rule::regex_pattern => {
            return true;
        }
        _ => {return false;}
    }
}

fn regex_to_str(pairs: Pairs<Rule>) -> String {
    let mut res:Vec<String> = Vec::new();
    let mut pairs = pairs.into_iter();
    while let Some(pair) = pairs.next() {
        let pair_rule = pair.as_rule();
        match pair_rule {
            Rule::regex_chars => {
                let inner_chars = pair.into_inner();
                if inner_chars.len() == 1 {
                    res.push("\"".to_string() + inner_chars.as_str() + "\"");
                }
                else {
                    res.push(base_to_str(inner_chars));
                }
                
            }
            Rule::regex_chars_inner => {
            }
            Rule::not => {
                res.push("!".to_string());
            }
            Rule::dash => {
                res.push("..".to_string());
            }
            _ => {
                // fails quitely
            }
        }
    }
    return "".to_string();
}


fn base_to_str(pairs: Pairs<Rule>) -> String {
    let mut res:Vec<String> = Vec::new();
    let mut pairs = pairs.into_iter();
    while let Some(pair) = pairs.next() {
        let pair_rule = pair.as_rule();
        match pair_rule {
            Rule::non_terminal => {
                res.push(pair.clone().into_inner().next().unwrap().as_str().to_string());
            }
            //Rule::regex_expression => {
            //    println!("notice me!");
            //}
            Rule::pattern | Rule::regex_pattern | Rule::regex_expression | Rule::regex_expansion => {
                res.push(base_to_str(pair.into_inner()));
            }
            Rule::token => {
                res.push("\"".to_string() + pair.as_str() + "\"");
            }
            Rule::or | Rule::star | Rule::name | Rule::lparen | Rule::rparen => {
                res.push(pair.as_str().to_string());
            }
            Rule::regex_chars => {
                res.push(regex_to_str(pair.into_inner()));
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
                print!("rule: {} \n", rule.clone());

                let non_terminal = rule.next().unwrap();
                let name = base_to_str(non_terminal.into_inner());

                let expansion = rule.skip(1).next().unwrap().into_inner();

                print!("\n{} = ", name);
                print!("{{ {} }}\n", base_to_str(expansion));
            }
            Rule::file | Rule::rule => {
                generate_pest(pair.into_inner());
            }
            Rule::pattern | Rule::regex_pattern => {
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
