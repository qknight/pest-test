use crate::NumberFormat;
use crate::Emission;
use crate::Emission::{EdgeDefined, EdgeUndefined};
use crate::parse_emission;

#[test]
fn empty() {
    let input = "";
    let output = parse_emission(input, NumberFormat::DE).unwrap();
    let q: Vec<Emission> = vec![];
    assert_eq!(output.as_slice(), q.as_slice());
}

#[test]
fn empty_spaces() {
    let input = "  ";
    let output = parse_emission(input, NumberFormat::DE).unwrap();
    let q: Vec<Emission> = vec![];
    assert_eq!(output.as_slice(), q.as_slice());
}

#[test]
fn empty_spaces_tab() {
    let input = " \t ";
    let output = parse_emission(input, NumberFormat::DE).unwrap();
    let q: Vec<Emission> = vec![];
    assert_eq!(output.as_slice(), q.as_slice());
}

#[test]
fn edge_undefined() {
    let input = "\"asdf1\" \"asdf2\"";
    let output = parse_emission(input, NumberFormat::DE).unwrap();
    let q: Vec<Emission> = vec![EdgeUndefined(1, "asdf1".to_string(), "asdf2".to_string())];
    assert_eq!(output.as_slice(), q.as_slice());
}

#[test]
fn edge_undefined_umlauts() {
    let input = "\"H₂ Generator\"   \"fällmittel\"";
    let output = parse_emission(input, NumberFormat::DE).unwrap();
    let q: Vec<Emission> = vec![EdgeUndefined(1, "H₂ Generator".to_string(), "fällmittel".to_string())];
    assert_eq!(output.as_slice(), q.as_slice());
}

#[test]
fn edge_undefined_other() {
    let input = "\"H₂ Generator\"   \"Midtbø\"";
    let output = parse_emission(input, NumberFormat::DE).unwrap();
    let q: Vec<Emission> = vec![EdgeUndefined(1, "H₂ Generator".to_string(), "Midtbø".to_string())];
    assert_eq!(output.as_slice(), q.as_slice());
}

#[test]
fn edge_defined() {
    let input = "\"asdf1\" 1,1 \"asdf2\"";
    let output = parse_emission(input, NumberFormat::DE).unwrap();
    let q: Vec<Emission> = vec![EdgeDefined(1, "asdf1".to_string(), 1.1, "asdf2".to_string())];
    assert_eq!(output.as_slice(), q.as_slice());
}

#[test]
fn edge_defined_umlauts() {
    let input = "\"H₂ Generator\" 1,1 \"fällmittel\"";
    let output = parse_emission(input, NumberFormat::DE).unwrap();
    let q: Vec<Emission> = vec![EdgeDefined(1, "H₂ Generator".to_string(), 1.1, "fällmittel".to_string())];
    assert_eq!(output.as_slice(), q.as_slice());
}

#[test]
fn fail1() {
    let input = "\"asdf1\" aaa";
    let output = parse_emission(input, NumberFormat::DE);
    assert!(output.is_err());
}

#[test]
fn fail2() {
    let input = "\"asdf1\" 1,1 aaa";
    let output = parse_emission(input, NumberFormat::DE);
    assert!(output.is_err());
}

#[test]
fn fail3() {
    let input = "\"asdf1\" 1,1";
    let output = parse_emission(input, NumberFormat::DE);
    assert!(output.is_err());
}

#[test]
fn fail4() {
    let input = "a 1,1 a";
    let output = parse_emission(input, NumberFormat::DE);
    assert!(output.is_err());
}

#[test]
fn fail5() {
    let input = "a a";
    let output = parse_emission(input, NumberFormat::DE);
    assert!(output.is_err());
}

#[test]
fn edge_defined_de_floats_de() {
    let floats = vec![
        "1",
        "1,",
        "1,",
        "1,1",
        "2,",
        "1,0",
        "1,000",
        "1,0000",
        "0,5",
        "10,00",
        "1000",
        "1000,00",
        "1000,1",
        "1200,000",
        "100000,23",
        "1,0000",
        "0,0001",
        "2345,67",
        "1234567,89",
        "1000000,000",
        "10000,0001",
        "1,2345",
        "0,000001",
        "999,999"
    ];
    for f in floats {
        let input = format!("\"a\" {f} \"a\"");
        let number_format = NumberFormat::DE;
        // println!("input {input}");
        let output = parse_emission(&input, NumberFormat::DE).unwrap();
        let v = number_format.parse_number(f);
        let q: Vec<Emission> = vec![EdgeDefined(1, "a".to_string(), v, "a".to_string())];
        assert_eq!(output.as_slice(), q.as_slice());
    }
}


#[test]
fn edge_defined_us() {
    let input = "\"asdf1\" 1.1 \"asdf2\"";
    let output = parse_emission(input, NumberFormat::US).unwrap();
    let q: Vec<Emission> = vec![EdgeDefined(1, "asdf1".to_string(), 1.1, "asdf2".to_string())];
    assert_eq!(output.as_slice(), q.as_slice());
}

#[test]
fn edge_defined_de_floats_us() {
    let floats = vec![
        "1",
        ".1",
        "1.",
        "1.1",
        "2.",
        "1.0",
        "1.000",
        "1.0000",
        "0.5",
        "10.00",
        "1.000",
        "1000.00",
        "1000.1",
        "1200.000",
        "100000.23",
        "1.0000",
        "0.0001",
        "2345.67",
        "1234.56789",
        "1000000.000",
        "10000.0001",
        "1.2345",
        "0.000001",
        "999.999"
    ];
    for f in floats {
        let input = format!("\"a\" {f} \"a\"");
        // println!("input {input}");
        let number_format = NumberFormat::US;
        let output = parse_emission(&input, number_format).unwrap();
        let v = number_format.parse_number(f);

        let q: Vec<Emission> = vec![EdgeDefined(1, "a".to_string(), v, "a".to_string())];
        assert_eq!(output.as_slice(), q.as_slice());
    }
}