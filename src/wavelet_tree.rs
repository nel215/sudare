use std::collections;

pub struct WaveletTree {
    pub height: usize,
}

pub fn new(_text: &str) -> WaveletTree {
    let mut alphabet = collections::HashMap::<char, usize>::new();
    for c in _text.chars() {
        if !alphabet.contains_key(&c) {
            let id = alphabet.len();
            alphabet.insert(c, id);
        }
    }
    let mut height = 1;
    while (1 << height) < alphabet.len() {
        height += 1;
    }
    return WaveletTree { height };
}

#[test]
fn test_new() {
    let wt1 = new("a");
    assert_eq!(wt1.height, 1);
    let wt2 = new("ab");
    assert_eq!(wt2.height, 1);
    let wt3 = new("abc");
    assert_eq!(wt3.height, 2);
    let wt4 = new("abcd");
    assert_eq!(wt4.height, 2);
}
