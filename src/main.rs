use pest::Parser;
use pest::iterators::Pairs;
use pest_derive::Parser;

use std::fs;

fn concatable(token_rule: Rule) -> bool {
    match token_rule {
        Rule::non_terminal | Rule::token | Rule::name | Rule::regex_chars
        | Rule::non_terminal_reg
        | Rule::regex_expression | Rule::regex_pattern | Rule::regex_chars_inner
        | Rule::class_expression | Rule::class_pattern | Rule::class_bracketed_expansion 
        | Rule::regex_bracketed_expansion
        | Rule::class_expansion | Rule::regex_expansion => {
            return true;
        }
        _ => {return false;}
    }
}

fn escape_special_chars(string: String) -> String {
    let special_chars = vec!["\""];
    let mut escaped_string = string.clone();
    for char in special_chars {
        escaped_string = escaped_string.replace(char, &("\\".to_string() + char));
    }
    return escaped_string;
}

fn inner_regex_to_str(pairs: Pairs<Rule>) -> String {
    let mut res:Vec<String> = Vec::new();
    let mut pairs = pairs.into_iter();
    while let Some(pair) = pairs.next() {
        let pair_rule = pair.as_rule();
        match pair_rule {
            Rule::regex_chars_inner => {
                res.push("\"".to_string() + &escape_special_chars(pair.as_str().to_string()) + "\"");
            }
            Rule::not => {
                res.push("(!\"".to_string() + &escape_special_chars(pairs.as_str().to_string()) + "\")");
                return res.join(" ");
            }
            Rule::special_chars => {
                res.push(escape_special_chars(pair.as_str().to_string()));
            }
            Rule::chars_range => {
                let mut chars_range = pair.into_inner();
                let start_char = chars_range.next().unwrap().as_str();
                let end_char = chars_range.skip(1).next().unwrap().as_str();
                res.push("'".to_string() + start_char + "'..'" + end_char + "'");
                if pairs.peek().is_some_and(|r| r.as_rule() == Rule::chars_range) {
                    res.push("|".to_string());
                }
            }
            _ => {
            }
        }
        if concatable(pair_rule) && pairs.peek().is_some_and(|r| concatable(r.as_rule())) {
            res.push("~".to_string());
        }
    }
    return res.join(" ");
}


fn base_to_str(pairs: Pairs<Rule>) -> String {
    let mut res:Vec<String> = Vec::new();
    let mut pairs = pairs.into_iter();
    while let Some(pair) = pairs.next() {
        let pair_rule = pair.as_rule();
        match pair_rule {
            Rule::non_terminal => {
                res.push(escape_special_chars(pair.clone().into_inner().next().unwrap().as_str().to_string()));
            }
            Rule::pattern | Rule::class_bracketed_expansion | Rule::non_terminal_reg
            | Rule::regex_bracketed_expansion
            | Rule::regex_pattern | Rule::regex_expression | Rule::regex_expansion
            | Rule::class_pattern | Rule::class_expression | Rule::class_expansion => {
                res.push(base_to_str(pair.into_inner()));
            }
            Rule::token => {
                res.push("\"".to_string() + &escape_special_chars(pair.as_str().to_string()) + "\"");
            }
            Rule::or | Rule::star | Rule::name | Rule::lparen | Rule::rparen => {
                res.push(escape_special_chars(pair.as_str().to_string()));
            }
            Rule::dot => {
                res.push("ASCII".to_string());
            }
            Rule::regex_chars => {
                res.push(inner_regex_to_str(pair.into_inner()));
            }
            Rule::class_chars => {
                let mut class_chars_inner = pair.into_inner();
                if class_chars_inner.peek().is_some_and(|r| r.as_rule() == Rule::not) {
                    res.push(inner_regex_to_str(class_chars_inner));
                }
                else if class_chars_inner.peek().is_some_and(|r| r.as_rule() == Rule::chars_range) {
                    res.push(inner_regex_to_str(class_chars_inner));
                }
                else {
                    res.push("(".to_string());
                    while class_chars_inner.peek().is_some() {
                        let next_str = "\"".to_string() 
                            + &escape_special_chars(class_chars_inner.next().unwrap().as_str().to_string())
                            + "\"";
                        res.push(next_str);
                        if class_chars_inner.peek().is_some() {
                            res.push("|".to_string());
                        }
                    }
                    res.push(")".to_string());
                }
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
            Rule::syntax_rule | Rule::regex_rule | Rule::char_class_rule => {
                let mut rule = pair.into_inner();
                //print!("rule: {} \n", rule.clone());

                let non_terminal = rule.next().unwrap();
                let name = base_to_str(non_terminal.into_inner());

                let expansion = rule.skip(1).next().unwrap().into_inner();

                print!("\n{} = ", name);
                print!("{{ {} }}\n", base_to_str(expansion));
            }
            Rule::file | Rule::rule => {
                generate_pest(pair.into_inner());
            }
            Rule::pattern | Rule::regex_pattern | Rule::class_pattern => {
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

mod tptp {
    use crate::Parser;
    #[derive(Parser)]
    #[grammar = "tptp.pest"]
    pub struct TPTPParser;
}

fn main() {
    let tptp_bnf = fs::read_to_string("src/tptp.bnf").unwrap();
    let tptp_parse = BNFParser::parse(Rule::file, &tptp_bnf);
    println!("{:?}", tptp_parse.is_ok());
    generate_pest(tptp_parse.unwrap());
    //let problem = fs::read_to_string("src/COM001_10.p").unwrap();
    //let tptp_problem = tptp::TPTPParser::parse(tptp::Rule::TPTP_file, &problem);
    //println!("{:?}", tptp_problem.is_ok());

}
