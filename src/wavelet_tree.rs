use std::collections;
use bitvector;

pub struct WaveletTree {
    pub height: usize,
    pub nodes: Vec<bitvector::BitVector>,
    utoc: Vec<char>,
}

impl WaveletTree {
    pub fn access(&self, i: usize) -> char {
        let mut i = i;
        let mut bs = 0;
        let mut node_id = 0;
        for _ in 0..self.height {
            let b = self.nodes[node_id].get(i) as usize;
            i = self.nodes[node_id].rank(i, b);
            bs = (bs << 1) | b;
            node_id = 2 * node_id + 1 + b;
        }
        return self.utoc[bs];
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
        let mut builders = Vec::new();
        for _ in 0..((1 << height) - 1) {
            builders.push(bitvector::BitVectorBuilder::new());
        }
        for bs in text {
            let mut i = 0;
            for h in 0..height {
                let b = (bs >> (height - 1 - h)) & 1;
                builders[i].push(b);
                i = 2 * i + 1 + b;
            }
        }
        let mut nodes = Vec::new();
        for b in builders {
            nodes.push(b.build());
        }
        WaveletTree {
            height,
            nodes,
            utoc,
        }
    }
}

#[test]
fn test_new() {
    let wt1 = WaveletTree::new("a");
    assert_eq!(wt1.height, 1);
    assert_eq!(wt1.nodes.len(), 1);
    let wt2 = WaveletTree::new("ab");
    assert_eq!(wt2.height, 1);
    assert_eq!(wt2.nodes.len(), 1);
    assert_eq!(wt2.nodes[0].get(0), 0);
    assert_eq!(wt2.nodes[0].get(1), 1);
    let wt3 = WaveletTree::new("abc");
    assert_eq!(wt3.height, 2);
    assert_eq!(wt3.nodes.len(), 3);
    let wt4 = WaveletTree::new("abcd");
    assert_eq!(wt4.height, 2);
}

#[test]
fn test_access() {
    let wt1 = WaveletTree::new("abcd");
    assert_eq!(wt1.access(0), 'a');
    assert_eq!(wt1.access(1), 'b');
    assert_eq!(wt1.access(2), 'c');
    assert_eq!(wt1.access(3), 'd');
}
