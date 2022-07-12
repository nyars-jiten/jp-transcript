include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

// transliterate tlite

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

pub fn convert_from_hiragana() {
    //
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
    }
}
