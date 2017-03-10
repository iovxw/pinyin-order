extern crate phf_codegen;
extern crate regex;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut out_file = BufWriter::new(File::create(&path).unwrap());

    let mut pinyin_data = String::new();
    File::open("./pinyin-data/pinyin.txt").unwrap().read_to_string(&mut pinyin_data).unwrap();
    let re = regex::Regex::new(r"^U\+([^#:\n]+):\s*([^ \n,]+)").unwrap();

    write!(&mut out_file,
           "static PINYIN_MAP: phf::Map<char, &'static str> = ")
        .unwrap();
    let mut m = phf_codegen::Map::new();
    for line in pinyin_data.lines() {
        match re.captures(line) {
            Some(group) => {
                let hex = &group[1];
                let code_point = u32::from_str_radix(hex, 16).unwrap();
                let c = std::char::from_u32(code_point).unwrap();
                let pinyin = &group[2];
                m.entry(c, &escape_pinyin(pinyin));
            }
            None => continue,
        }
    }
    m.build(&mut out_file)
        .unwrap();
    write!(&mut out_file, ";\n").unwrap();
}

fn escape_pinyin(s: &str) -> String {
    let mut r = String::new();
    r.push('"');
    for c in s.chars() {
        match c {
            'ā' | 'á' | 'ǎ' | 'à' => r.push('a'),
            'ē' | 'é' | 'ě' | 'è' => r.push('e'),
            'ō' | 'ó' | 'ǒ' | 'ò' => r.push('o'),
            'ī' | 'í' | 'ǐ' | 'ì' => r.push('i'),
            'ū' | 'ú' | 'ǔ' | 'ù' => r.push('u'),
            'ü' | 'ǘ' | 'ǚ' | 'ǜ' => r.push('v'),
            'ń' | 'ň' | '' => r.push('n'),
            _ => r.push(c),
        }
    }
    r.push('"');
    r
}
