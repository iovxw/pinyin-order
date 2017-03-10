//! Example:
//!
//! ```
//! extern crate pinyin_order;
//!
//! use pinyin_order::as_pinyin;
//!
//! fn main() {
//!     let mut l = vec!["中文", "中国", "abc", "重工", "abc中文"];
//!     l.sort_by_key(|ref s| as_pinyin(s));
//!     assert_eq!(l, vec!["abc", "abc中文", "中国", "中文", "重工"]);
//! }
//! ```

extern crate phf;

use std::cmp::Ordering;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

#[derive(Eq, Debug)]
pub enum PinYin {
    Other(char),
    Chinese(char, &'static str),
}

impl PinYin {
    fn is_other(&self) -> bool {
        match *self {
            PinYin::Other(_) => true,
            PinYin::Chinese(_, _) => false,
        }
    }

    fn is_chinese(&self) -> bool {
        match *self {
            PinYin::Other(_) => false,
            PinYin::Chinese(_, _) => true,
        }
    }

    fn get_other(&self) -> char {
        match *self {
            PinYin::Other(r) => r,
            PinYin::Chinese(_, _) => panic!(),
        }
    }

    fn get_chinese(&self) -> (char, &str) {
        match *self {
            PinYin::Other(_) => panic!(),
            PinYin::Chinese(c, p) => (c, p),
        }
    }
}

impl PartialEq for PinYin {
    fn eq(&self, other: &PinYin) -> bool {
        if self.is_other() && other.is_other() {
            self.get_other().eq(&other.get_other())
        } else if self.is_chinese() && other.is_chinese() {
            self.get_chinese().0.eq(&other.get_chinese().0)
        } else {
            false
        }
    }
}

impl PartialOrd for PinYin {
    fn partial_cmp(&self, other: &PinYin) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PinYin {
    fn cmp(&self, other: &PinYin) -> Ordering {
        if self.is_other() && other.is_other() {
            self.get_other().cmp(&other.get_other())
        } else if self.is_chinese() && other.is_chinese() {
            let (sc, sp) = self.get_chinese();
            let (oc, op) = other.get_chinese();
            let pinyin_order = sp.cmp(&op);
            if pinyin_order == Ordering::Equal {
                sc.cmp(&oc)
            } else {
                pinyin_order
            }
        } else {
            if self.is_other() {
                Ordering::Less
            } else {
                Ordering::Greater
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

