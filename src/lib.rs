use wasm_bindgen::prelude::*;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

// #[wasm_bindgen]
// pub fn convert_to_kana(jsvalue: JsValue) -> String {
//     let value: String = jsvalue.into_serde().unwrap();
//     convert(&value, Translit::Std, Translit::Hiragana).to_string()
// }

#[wasm_bindgen]
pub fn convert_to_kana(request: String) -> String {
    convert(&request, Translit::Std, Translit::Hiragana)
}

// #[wasm_bindgen]
// pub fn convert_to_polivanov(jsvalue: JsValue) -> String {
//     let value: String = jsvalue.into_serde().unwrap();
//     convert(&value, Translit::Std, Translit::Polivanov).to_string()
// }

#[allow(dead_code)]
enum Translit {
    Hiragana,
    Katakana,
    Std,
    Polivanov,
    Hepburn,
    Nihon,
}

fn convert(value: &str, _from: Translit, to: Translit) -> String {
    // result = match from {
    //     // Translit::Hiragana => &"",
    //     // Translit::Katakana => &"",
    //     _ => value,
    // };

    let result: String = match to {
        Translit::Hiragana => convert_std_to_kana(value),
        Translit::Katakana => convert_std_to_kana(value),
        // Translit::Polivanov => value,
        // Translit::Hepburn => value,
        // Translit::Nihon => value,
        _ => String::from(""),
    };

    result
}

fn convert_std_to_kana(value: &str) -> String {
    let excluded_strs = vec!['-', ' ', '\'', '(', ')', '^'];
    // let excluded_strs_latk = vec!["'", "(", ")", "^"];

    let mut table = Translit::Hiragana;
    let raw = value.to_lowercase();
    let raw_chars: Vec<char> = raw.chars().collect();
    let raw_len = raw_chars.len();
    let mut ci: usize = 0;
    let mut result = String::from("");
    let mut chunk = String::from("");

    while ci < raw_len {
        if ci == 0 && raw_chars[ci] == '!' {
            table = Translit::Katakana;
            ci += 1;
            continue;
        }
        let mut j = MAX_CHUNK_SIZE_STD + 1;
        while j > 0 {
            j -= 1;
            if ci + j > raw_len || j == 0 {
                continue;
            }

            chunk = raw_chars[ci..(ci + j)].iter().collect();

            // println!("{}", chunk);

            match get_table_value(&table, &chunk) {
                Some(val) => {
                    result += val;
                    ci += &chunk.len();
                    break;
                }
                None => {
                    continue;
                }
            };
        }

        // если j не 0, значит выше нашёлся подходящий чанк и особые условия не нужны
        if j != 0 {
            continue;
        }

        // Блок особых условий для символов, которые не найдены в таблице

        let mut skip_i: usize = 1; // относительный индекс последнего значимого символа
        let first_chunk_sym = chunk.chars().next().unwrap();

        // двоеточие конвертируется в удвоение
        if chunk.eq(":") && ci >= skip_i {
            if excluded_strs.contains(&raw_chars[ci - 1]) {
                skip_i += 1;
            }

            match table {
                Translit::Hiragana => match &raw_chars[ci - skip_i] {
                    'o' => result += "う",
                    'u' => result += "う",
                    'a' => result += "あ",
                    'i' => result += "い",
                    'e' => result += "え",
                    _ => result += ":",
                },
                _ => result += "ー",
            }
        } else if chunk.eq("!") {
            table = match table {
                Translit::Hiragana => Translit::Katakana,
                _ => Translit::Hiragana,
            }
        } else if ci + 2 < raw_chars.len()
            && raw_chars[ci + 2] == first_chunk_sym
            && raw_chars[ci + 1] == '!'
        {
            result += extract_table_value(&table, "*tu");
        } else if ci + 1 < raw_chars.len() && chunk.eq("\\") {
            ci += 1;
            let not_changed_raw: Vec<char> = value.chars().collect();
            result += &not_changed_raw[ci].to_string();
        } else if ci + 1 < raw_chars.len()
            && ((first_chunk_sym == raw_chars[ci + 1]
                && "qwrtpsdfghkljzxcvbnm".chars().any(|x| x == first_chunk_sym))
                || (first_chunk_sym == 't' && raw_chars[ci + 1] == 'c'))
        {
            result += extract_table_value(&table, "*tu");
        } else if excluded_strs.contains(&first_chunk_sym) {
            //
        } else {
            result += &chunk;
        }

        ci += 1;
    }

    result
}

fn extract_table_value<'a>(table: &'a Translit, value: &'a str) -> &'a str {
    match get_table_value(table, value) {
        Some(val) => val,
        None => "",
    }
}

fn get_table_value<'a>(table: &'a Translit, value: &'a str) -> Option<&'a str> {
    match table {
        Translit::Hiragana => TO_HIRAGANA.get(value).cloned(),
        Translit::Katakana => TO_KATAKANA.get(value).cloned(),
        Translit::Polivanov => TO_POLIVANOV.get(value).cloned(),
        Translit::Hepburn => TO_HEPBURN.get(value).cloned(),
        Translit::Nihon => TO_NIHON.get(value).cloned(),
        _ => TO_HIRAGANA.get(value).cloned(),
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn converts_from_kana() {
    //     let conditions = &[("あ", "a")];

    //     for (source, result) in conditions {
    //         assert_eq!(convert(*source), *result, "{} => {}", *source, *result);
    //     }
    // }

    #[test]
    fn converts_to_kana() {
        let conditions = &[
            ("(ba)ngo^:", "ばんごう"),
            ("!kit!to", "キッと"),
            ("gunki", "ぐんき"),
            ("wyiwye!wyiwye", "ゐゑヰヱ"),
            ("chokko:-!Nikoru", "ちょっこうニコル"),
            ("!cho:ku", "チョーク"),
            ("!Chu:gaefu !no hanno:", "チューガエフのはんのう"),
            ("daiichi-!fosufin", "だいいちフォスフィン"),
            ("en'yoku-yakiire", "えんよくやきいれ"),
            ("ju:mo:-!feruto", "じゅうもうフェルト"),
            ("!kwakwikwukwekwo", "クァクィクゥクェクォ"),
            ("!tsottsa", "ツォッツァ"),
            ("!kondishona:", "コンディショナー"),
            ("aiibisha", "あいいびしゃ"),
            ("be:a:i:u:e:o:", "べえああいいううええおう"),
            ("兼兼", "兼兼"),
            ("ooguma", "おおぐま"),
            (":次系", ":次系"),
            ("!vo:paru", "ヴォーパル"),
            ("!Wi:n !no hen'isoku", "ウィーンのへんいそく"),
            ("!u*o:ki:to:ki:", "ウォーキートーキー"),
        ];

        for (source, result) in conditions {
            assert_eq!(
                convert(source, Translit::Std, Translit::Hiragana),
                *result,
                "{} => {}",
                *source,
                *result
            );
        }
    }
}
