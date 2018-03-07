use std::collections;
use bitvector;

pub struct WaveletTree {
    pub height: usize,
    pub root: bitvector::BitVector,
    utoc: Vec<char>,
}

impl WaveletTree {
    pub fn access(&self, i: usize) -> char {
        let b = self.root.get(i) as usize;
        return self.utoc[b];
    }
    pub fn new(_text: &str) -> WaveletTree {
        let n = _text.len();
        let mut alphabet = collections::HashMap::<char, usize>::new();
        let mut utoc = Vec::new();
        for c in _text.chars() {
            if !alphabet.contains_key(&c) {
                let id = alphabet.len();
                alphabet.insert(c, id);
                utoc.push(c);
            }
        }
        // char to usize
        let mut text = vec![0; n];
        for (i, c) in _text.chars().enumerate() {
            text[i] = alphabet[&c];
        }
        let text = text;
        let mut height = 1;
        while (1 << height) < alphabet.len() {
            height += 1;
        }
        // construct nodes
        let mut root = bitvector::BitVectorBuilder::new();
        for b in text {
            root.push((b >> (height - 1)) & 1);
        }
        let root = root.build();
        return WaveletTree { height, root, utoc };
    }
}



#[test]
fn test_new() {
    let wt1 = WaveletTree::new("a");
    assert_eq!(wt1.height, 1);
    let wt2 = WaveletTree::new("ab");
    assert_eq!(wt2.height, 1);
    assert_eq!(wt2.root.get(0), 0);
    assert_eq!(wt2.root.get(1), 1);
    let wt3 = WaveletTree::new("abc");
    assert_eq!(wt3.height, 2);
    let wt4 = WaveletTree::new("abcd");
    assert_eq!(wt4.height, 2);
}

#[test]
fn test_access() {
    let wt1 = WaveletTree::new("a");
    assert_eq!(wt1.access(0), 'a');
}
