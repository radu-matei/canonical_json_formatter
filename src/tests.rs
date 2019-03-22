use crate::formatter;
use serde::Serialize;
use serde_derive::*;
use serde_json::*;
use spectral::prelude::*;
use std::io::Read;

#[test]
fn test_can_use_formatter() {
    let mut out = Vec::new();
    let obj = json!({"foo":true,"bar":false});
    let mut ser = serde_json::Serializer::with_formatter(&mut out, formatter::Canonical {});
    obj.serialize(&mut ser).unwrap();

    let x: serde_json::Value = serde_json::from_slice(&out).unwrap();
    assert_that(&x["foo"].as_bool().unwrap()).is_true();
    assert_that(&x["bar"].as_bool().unwrap()).is_false();
}

#[test]
fn test_compact_from_struct() {
    let p = Person {
        name: "John Doe".to_string(),
        age: 46,
        phones: vec!["+1 234 567".to_string(), "+1 345 678".to_string()],
    };

    let file = tempfile::NamedTempFile::new().expect("cannot create temp file");
    let mut ser = serde_json::Serializer::with_formatter(&file, formatter::Canonical {});
    p.serialize(&mut ser).unwrap();

    let mut content = String::new();
    file.reopen().unwrap().read_to_string(&mut content).unwrap();

    assert_that(&content.contains("\n")).is_false();
    // TODO - check for other escape characters that are invalid in canonical JSON
}

#[test]
fn test_compact_from_string() {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let p: Person = serde_json::from_str(data).unwrap();
    let file = tempfile::NamedTempFile::new().expect("cannot create temp file");
    let mut ser = serde_json::Serializer::with_formatter(&file, formatter::Canonical {});
    p.serialize(&mut ser).unwrap();

    let mut content = String::new();
    file.reopen().unwrap().read_to_string(&mut content).unwrap();

    assert_that(&content.contains("\n")).is_false();
    // TODO - check for other escape characters that are invalid in canonical JSON
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}
