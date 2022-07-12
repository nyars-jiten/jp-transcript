use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

macro_rules! template {
    () => {
        r#"
#[allow(dead_code)]
const MAX_CHUNK_SIZE_KANA: i32 = {max_chunk_size_kana};
#[allow(dead_code)]
const MAX_CHUNK_SIZE_STD: i32 = {max_chunk_size_std};
#[allow(dead_code)]
const MAX_CHUNK_SIZE_MISC: i32 = {max_chunk_size_misc};
#[allow(dead_code)]
static FROM_HIRAGANA: phf::Map<&'static str, &'static str> = {from_hiragana};
#[allow(dead_code)]
static FROM_KATAKANA: phf::Map<&'static str, &'static str> = {from_katakana};
#[allow(dead_code)]
static FROM_POLIVANOV: phf::Map<&'static str, &'static str> = {from_polivanov};
#[allow(dead_code)]
static FROM_HEPBURN: phf::Map<&'static str, &'static str> = {from_hepburn};
#[allow(dead_code)]
static FROM_NIHON: phf::Map<&'static str, &'static str> = {from_nihon};
#[allow(dead_code)]
static TO_HIRAGANA: phf::Map<&'static str, &'static str> = {to_hiragana};
#[allow(dead_code)]
static TO_KATAKANA: phf::Map<&'static str, &'static str> = {to_katakana};
#[allow(dead_code)]
static TO_POLIVANOV: phf::Map<&'static str, &'static str> = {to_polivanov};
#[allow(dead_code)]
static TO_HEPBURN: phf::Map<&'static str, &'static str> = {to_hepburn};
#[allow(dead_code)]
static TO_NIHON: phf::Map<&'static str, &'static str> = {to_nihon};
"#
    };
}

// macro_rules! p {
//     ($($tokens: tt)*) => {
//         println!("cargo:warning={}", format!($($tokens)*))
//     }
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let mut max_chunk_size_kana = 0;
    let mut max_chunk_size_std = 0;
    let mut max_chunk_size_misc = 0;

    let mut from_hiragana_builder = phf_codegen::Map::new();
    let mut from_katakana_builder = phf_codegen::Map::new();
    let mut from_polivanov_builder = phf_codegen::Map::new();
    let mut from_hepburn_builder = phf_codegen::Map::new();
    let mut from_nihon_builder = phf_codegen::Map::new();

    let mut to_hiragana_builder = phf_codegen::Map::new();
    let mut to_katakana_builder = phf_codegen::Map::new();
    let mut to_polivanov_builder = phf_codegen::Map::new();
    let mut to_hepburn_builder = phf_codegen::Map::new();
    let mut to_nihon_builder = phf_codegen::Map::new();

    let mut stored_hiragana: Vec<String> = Vec::new();
    let mut stored_katakana: Vec<String> = Vec::new();
    let mut stored_polivanov: Vec<String> = Vec::new();
    let mut stored_hepburn: Vec<String> = Vec::new();
    let mut stored_nihon: Vec<String> = Vec::new();
    let mut stored_std: Vec<String> = Vec::new();

    let reader = BufReader::new(File::open("./data/dict.txt")?);

    for line in reader.lines() {
        let current_line = line?;
        let literals: Vec<&str> = current_line.split('\t').collect();
        if current_line.is_empty() || current_line.starts_with("#") || literals.len() != 6 {
            continue;
        }

        for i in 0..literals.len() {
            if (i == 1 || i == 0) && max_chunk_size_kana < literals[i].chars().count() {
                max_chunk_size_kana = literals[i].chars().count();
            } else if i == 2 && max_chunk_size_std < literals[i].chars().count() {
                max_chunk_size_std = literals[i].chars().count();
            } else if max_chunk_size_misc < literals[i].chars().count() {
                max_chunk_size_misc = literals[i].chars().count();
            }
        }

        let hiragana_literal = literals[0].to_string();
        let katakana_literal = literals[1].to_string();
        let std_literal = literals[2].to_string();
        let polivanov_literal = literals[3].to_string();
        let hepburn_literal = literals[4].to_string();
        let nihon_literal = literals[5].to_string();

        if !stored_hiragana.contains(&hiragana_literal) {
            from_hiragana_builder.entry(hiragana_literal.clone(), &format!("\"{}\"", &std_literal));
            stored_hiragana.push(hiragana_literal.clone());
        }

        if !stored_katakana.contains(&katakana_literal) {
            from_katakana_builder.entry(katakana_literal.clone(), &format!("\"{}\"", &std_literal));
            stored_katakana.push(katakana_literal.clone());
        }

        if !stored_polivanov.contains(&polivanov_literal) {
            from_polivanov_builder
                .entry(polivanov_literal.clone(), &format!("\"{}\"", &std_literal));
            stored_polivanov.push(polivanov_literal.clone());
        }

        if !stored_hepburn.contains(&hepburn_literal) {
            from_hepburn_builder.entry(hepburn_literal.clone(), &format!("\"{}\"", &std_literal));
            stored_hepburn.push(hepburn_literal.clone());
        }

        if !stored_nihon.contains(&nihon_literal) {
            from_nihon_builder.entry(nihon_literal.clone(), &format!("\"{}\"", &std_literal));
            stored_nihon.push(nihon_literal.clone());
        }

        if !stored_std.contains(&std_literal) {
            to_hiragana_builder.entry(std_literal.clone(), &format!("\"{}\"", &hiragana_literal));
            to_katakana_builder.entry(std_literal.clone(), &format!("\"{}\"", &katakana_literal));
            to_polivanov_builder.entry(std_literal.clone(), &format!("\"{}\"", &polivanov_literal));
            to_hepburn_builder.entry(std_literal.clone(), &format!("\"{}\"", &hepburn_literal));
            to_nihon_builder.entry(std_literal.clone(), &format!("\"{}\"", &nihon_literal));
            stored_std.push(std_literal);
        }

        // p!("{}", literals[2]);
    }

    write!(
        &mut file,
        template!(),
        max_chunk_size_kana = max_chunk_size_kana,
        max_chunk_size_std = max_chunk_size_std,
        max_chunk_size_misc = max_chunk_size_misc,
        from_hiragana = from_hiragana_builder.build(),
        from_katakana = from_katakana_builder.build(),
        from_polivanov = from_polivanov_builder.build(),
        from_hepburn = from_hepburn_builder.build(),
        from_nihon = from_nihon_builder.build(),
        to_hiragana = to_hiragana_builder.build(),
        to_katakana = to_katakana_builder.build(),
        to_polivanov = to_polivanov_builder.build(),
        to_hepburn = to_hepburn_builder.build(),
        to_nihon = to_nihon_builder.build(),
    )
    .unwrap();

    Ok(())
}
