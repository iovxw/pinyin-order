//! Example:
//!
//! ```
//! extern crate pinyin_order;
//!
//! use pinyin_order::as_pinyin;
//!
//! fn main() {
//!     let mut l = vec!["中文", "中国", "abc", "重工", "abc中文"];
//!     l.sort_by_key(|s| as_pinyin(s));
//!     assert_eq!(l, vec!["abc", "abc中文", "中国", "中文", "重工"]);
//! }
//! ```

extern crate phf;

use std::cmp::Ordering;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

#[derive(PartialEq, Eq, PartialOrd, Debug)]
pub enum PinYin {
    Other(char),
    Chinese(char, &'static str),
}

impl Ord for PinYin {
    fn cmp(&self, other: &PinYin) -> Ordering {
        match *self {
            PinYin::Other(ref sc) => {
                match *other {
                    PinYin::Other(ref oc) => sc.cmp(oc),
                    PinYin::Chinese(_, _) => Ordering::Less,
                }
            }
            PinYin::Chinese(ref sc, ref sp) => {
                match *other {
                    PinYin::Other(_) => Ordering::Greater,
                    PinYin::Chinese(ref oc, ref op) => {
                        let pinyin_order = sp.cmp(op);
                        if pinyin_order == Ordering::Equal {
                            sc.cmp(oc)
                        } else {
                            pinyin_order
                        }
                    }
                }
            }
        }
    }
}

/// 将字符串转换为拼音用于排序
pub fn as_pinyin(s: &str) -> Vec<PinYin> {
    s.chars()
        .map(|c| match PINYIN_MAP.get(&c) {
            Some(pinyin) => PinYin::Chinese(c, pinyin),
            None => PinYin::Other(c),
        })
        .collect()
}

