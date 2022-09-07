#![allow(dead_code)]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
#[cfg(test)]
use wasm_bindgen_test::*;

#[wasm_bindgen]
pub fn parse(corn: &str) -> Result<JsValue, JsValue> {
    console_error_panic_hook::set_once();

    let res = crate::parse(corn);

    match res {
        Ok(parsed) => Ok(JsValue::from_serde(&parsed).unwrap()),
        Err(err) => Err(JsValue::from_str(err.to_string().as_str())),
    }
}

#[cfg(test)]
#[wasm_bindgen_test]
fn test_wasm_parse_valid() {
    let res = parse("{foo = \"bar\"}");
    assert!(res.is_ok())
}

#[cfg(test)]
#[wasm_bindgen_test]
fn test_wasm_parse_invalid() {
    let res = parse("{foo = \"$bar\"}");
    assert!(res.is_err())
}
