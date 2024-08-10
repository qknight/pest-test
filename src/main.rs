use pest::Parser;
use pest_derive::Parser;
use indoc::indoc;

#[derive(Parser)]
#[grammar = "emission.pest"] // relative path to your .pest file
struct EmissionParser;

#[derive(Debug)]
enum Emission {
    EdgeDefined(usize, String, f64, String),
    EdgeUndefined(usize, String, String),
}

#[derive(Debug, Clone, Copy)]
enum NumberFormat {
    DE,
    US,
}

impl NumberFormat {
    fn parse_number(&self, num_str: &str) -> f64 {
        match self {
            NumberFormat::DE => {
                let normalized = num_str.replace('.', "").replace(',', ".");
                normalized.parse::<f64>().unwrap()
            }
            NumberFormat::US => {
                let normalized = num_str.replace(',', "");
                normalized.parse::<f64>().unwrap()
            }
        }
    }
}

fn detect_number_format(number: &str) -> NumberFormat {
    if number.contains(',') && !number.contains('.') {
        NumberFormat::DE
    } else if number.contains('.') && number.contains(',') {
        if number.rfind('.').unwrap() < number.rfind(',').unwrap() {
            NumberFormat::DE
        } else {
            NumberFormat::US
        }
    } else {
        NumberFormat::US
    }
}

fn parse_emission(input: &str) -> Result<Vec<Emission>, pest::error::Error<Rule>> {
    let mut result = Vec::new();
    
    let mut detected_format = None;
    
    for (line_number, line) in input.lines().enumerate() {
        let parsed = EmissionParser::parse(Rule::line, line)?.next().unwrap();
        
        for record in parsed.into_inner() {
            // println!("record: {}", record);
            match record.as_rule() {
                Rule::line => {
                    println!("found line");
                }
                Rule::entry => {
                    println!("entry rule: {}", record);

                    let mut inner_rules = record.into_inner();
                    let label = inner_rules.next().unwrap().as_str().trim_matches('"').to_string();
                    let number_str = inner_rules.next().unwrap().as_str();
                    
                    if detected_format.is_none() {
                        detected_format = Some(detect_number_format(number_str));
                    }
                    
                    let number_format = detected_format.unwrap();
                    let number = number_format.parse_number(number_str);
                    
                    let category = inner_rules.next().unwrap().as_str().to_string();
                    result.push(Emission::EdgeDefined(line_number + 1, label, number, category));
                }
                Rule::text_entry => {
                    println!("text_entry rule: {}", record);

                    let mut inner_rules = record.into_inner();
                    let label = inner_rules.next().unwrap().as_str().trim_matches('"').to_string();
                    let category = inner_rules.next().unwrap().as_str().to_string();
                    result.push(Emission::EdgeUndefined(line_number + 1, label, category));
                }
                Rule::section => {
                    println!("section rule: {}", record);
                }

                _ => {
                    println!("no rule detected");

                }
            }
        }
    }

    Ok(result)
}

fn main() {
    let input = indoc! {"
    \"a\" 1,233   asdf asdf
    \"a\" 1,233   asdf \n \"asdf\" asdf
    "};
    //[ a0_ a ]
    // let input = indoc! {" // FIXME broken
    // \"aaa\" 1,233.423.423.44 asdf
    // "};    
    // let input = indoc! {"
    // [a0_]
    // [a1_]

    // \"aaa\" 1,233 asdf
    // [a2_]
    // naaa bbb
    // \"asdf\" asdf
    // [a3_]
    // "};
    // let input = r#"
    // [a0_]
    // [a1_]
    // [a2_]
    // naaa bbb
    // "#;
    // let input = r#"
    // "H₂ Generator" 1.223,2 other_indirect_emissions
    // "Fällmittel (AI)" 400 "fällmittel"
    // "Fällmittel (Eisen(II)-sulfat)" 200 "fällmittel"
    // "fällmittel" emissions
    // "Flüssiggas" 23 emissions
    // "#;


    println!("input: {}", input);

    match parse_emission(input) {
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
