# pinyin-order

[![Travis Build Status](https://travis-ci.org/iovxw/pinyin-order.svg)](https://travis-ci.org/iovxw/pinyin-order)
[![Crates](https://img.shields.io/crates/v/pinyin-order.svg)](https://crates.io/crates/pinyin-order)
[![Documentation](https://docs.rs/pinyin-order/badge.svg)](https://docs.rs/pinyin-order)

Rust 字符串向量按拼音排序

## Example

```rust
extern crate pinyin_order;

use pinyin_order::as_pinyin;

fn main() {
    let mut l = vec!["中文", "中国", "abc", "重工", "abc中文"];
    l.sort_by_key(|s| as_pinyin(s));
    assert_eq!(l, vec!["abc", "abc中文", "中国", "中文", "重工"]);
}
```

## License

This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or distribute this software, either in source code form or as a compiled binary, for any purpose, commercial or non-commercial, and by any means.
