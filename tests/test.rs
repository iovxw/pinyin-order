extern crate pinyin_order;

use pinyin_order::{PinYin, as_pinyin};

#[test]
fn test_pinyin() {
    assert_eq!(as_pinyin("中文"),
               vec![PinYin::Chinese('中', "zhong"), PinYin::Chinese('文', "wen")]);
}

#[test]
fn test_sort() {
    let mut l = vec!["中文", "中国", "abc", "重工", "abc中文"];
    l.sort_by_key(|s| as_pinyin(s));
    assert_eq!(l, vec!["abc", "abc中文", "中国", "中文", "重工"]);
}
