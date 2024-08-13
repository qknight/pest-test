#[cfg(test)]
mod tests;

use pest::Parser;
use pest_derive::Parser;
use indoc::indoc;

#[derive(Parser)]
#[grammar = "emission.pest"] // relative path to your .pest file
struct EmissionParser;

#[derive(Debug, PartialEq)]
enum Emission {
    EdgeDefined(usize, String, f64, String),
    EdgeUndefined(usize, String, String),
}

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
enum NumberFormat {
    DE,
    US,
}

impl NumberFormat {
    fn parse_number(&self, num_str: &str) -> f64 {
        match self {
            NumberFormat::DE => {
                println!("DE");
                let normalized = num_str.replace('.', "").replace(',', ".");
                normalized.parse::<f64>().unwrap()
            }
            NumberFormat::US => {
                println!("US");
                let normalized = num_str.replace(',', "");
                normalized.parse::<f64>().unwrap()
            }
        }
    }
}

fn parse_emission(input: &str, number_format: NumberFormat) -> Result<Vec<Emission>, pest::error::Error<Rule>> {
    let mut result = Vec::new();
    for (line_number, line) in input.lines().enumerate() {
        println!("line: {line}");
        let parsed = EmissionParser::parse(Rule::line, line);
        // println!("{parsed:?}");
        match parsed {
            Ok(mut v) => {
                let t = v.next().unwrap();
                for record in t.into_inner() {
                    println!("record: {}", record);
                    match record.as_rule() {
                        Rule::EdgeD => {
                            // println!("EdgeD rule: {}", record);
                            let mut inner_rules = record.into_inner();
                            let source = inner_rules.next().unwrap().as_str().trim_matches('"').to_string();
                            let number_str = inner_rules.next().unwrap().as_str();
                            println!("number_str: {number_str}");
                            let number = number_format.parse_number(number_str);
                            let destination = inner_rules.next().unwrap().as_str().to_string();
                            result.push(Emission::EdgeDefined(line_number + 1, source, number, destination));
                        }
                        Rule::EdgeU => {
                            // println!("EdgeU rule: {}", record);
                            let mut inner_rules = record.into_inner();
                            let source = inner_rules.next().unwrap().as_str().trim_matches('"').to_string();
                            let destination = inner_rules.next().unwrap().as_str().to_string();
                            result.push(Emission::EdgeUndefined(line_number + 1, source, destination));
                        }
                        _ => {}
                    }
                }
            },
            Err(e) => {
                println!("error parsing line {}", line_number + 1);
                return Err(e);
            }
        }
    }
    Ok(result)
}

fn main() {
    let input = indoc! {"
    \"asdf\" 11 \"asdf\"
    "};
    print!("input: {input}");
    match parse_emission(input, NumberFormat::DE) {
        Ok(emissions) => {
            for emission in emissions {
                println!("{:?}", emission);
            }
        }
        Err(e) => {
            eprintln!("Error parsing: {}", e);
        }
    }
}
