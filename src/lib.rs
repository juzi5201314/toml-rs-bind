use wasm_bindgen::prelude::*;

pub fn set_console_error_panic_hook() {
    console_error_panic_hook::set_once()
}

#[wasm_bindgen]
pub fn stringify(v: &JsValue) -> String {
    set_console_error_panic_hook();
    toml::to_string(&v.into_serde::<serde_json::Value>().unwrap()).unwrap()
}

#[wasm_bindgen]
pub fn parse(s: String) -> JsValue {
    set_console_error_panic_hook();
    JsValue::from_serde::<toml::Value>(&toml::from_str::<toml::Value>(&s).unwrap()).unwrap()
}