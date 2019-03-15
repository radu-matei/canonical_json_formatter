use crate::formatter;
use serde::Serialize;
use spectral::prelude::*;
use std::str;

#[test]
fn test_can_use_formatter() {
    let mut out = Vec::new();
    let obj = json!({"foo":true,"bar":false});
    let mut ser = serde_json::Serializer::with_formatter(&mut out, formatter::Canonical {});
    obj.serialize(&mut ser).unwrap();

    let x: serde_json::Value = serde_json::from_str(&str::from_utf8(&out).unwrap()).unwrap();
    assert_that(&x["foo"].as_bool().unwrap()).is_true();
    assert_that(&x["bar"].as_bool().unwrap()).is_false();
}
