use wasm_bindgen::prelude::*;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

/**
* convert any -> std
* std -> any
* unknown -> std
*
* furigana ? (allow check no in a kanji)
* HIRAGANA
   KATAKANA
   POLIVANOV
   HEPBURN
   NIHON
*/

#[wasm_bindgen]
pub fn convert_from_hiragana(key: JsValue) -> String {
    let akey: String = key.into_serde().unwrap();
    convert(&akey).to_string()
}

pub fn convert_to_polivanov() {
    //
}

fn convert(key: &str) -> &str {
    FROM_HIRAGANA.get(key).cloned().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = convert("あ");
        assert_eq!(result, "a");
        assert_eq!(convert("あ"), "aa");
    }
}
