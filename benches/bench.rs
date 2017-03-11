#![feature(test)]

extern crate test;
extern crate pinyin_order;

use test::Bencher;
use pinyin_order::as_pinyin;

static FILE: &'static str = include_str!("../pinyin-data/pinyin.txt");

#[bench]
fn bench_sort_default(b: &mut Bencher) {
    let mut l: Vec<&str> = FILE.lines().collect();
    b.iter(|| { l.sort(); });
}

#[bench]
fn bench_sort_pinyin(b: &mut Bencher) {
    let mut l: Vec<&str> = FILE.lines().collect();
    b.iter(|| { l.sort_by_key(|s| as_pinyin(s)); });
}

#[bench]
fn bench_as_pinyin(b: &mut Bencher) {
    let l: Vec<&str> = FILE.lines().collect();
    b.iter(|| for s in &l {
        as_pinyin(s);
    });
}
