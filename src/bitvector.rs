pub struct BitVector {
    pub data: Vec<u32>,
    pub large_sum: Vec<u32>,
    pub small_sum: Vec<u32>,
}
impl BitVector {
    pub fn new(n: usize) -> BitVector {
        let mut large_size: usize = 0;
        let mut small_size: usize = 0;
        if n % 256 > 0 {
            large_size += 1;
        }
        if n % 32 > 0 {
            small_size += 1;
        }
        large_size += (n / 256) as usize;
        small_size += (n / 32) as usize;
        let data = vec![0; n];
        let large_sum = vec![0; large_size];
        let small_sum = vec![0; small_size];
        return BitVector {
            data,
            large_sum,
            small_sum,
        };
    }
    pub fn set(&mut self, i: usize, b: usize) {
        self.data[i / 32] |= (b as u32) << (i % 32);
    }
    pub fn get(&self, i: usize) -> u32 {
        return (self.data[i / 32] >> (i % 32)) & 1;
    }
    pub fn rank(&self, i: usize) -> usize {
        let mut res = 0;
        for j in 0..i {
            res += (self.data[j / 32] >> (j % 32)) & 1;
        }
        return res as usize;
    }
}

pub struct BitVectorBuilder {
    pub data: Vec<u32>,
    pub size: usize,
}

impl BitVectorBuilder {
    pub fn new() -> BitVectorBuilder {
        let data = Vec::new();
        BitVectorBuilder { data, size: 0 }
    }
    pub fn push(&mut self, b: usize) {
        if self.data.len() * 32 <= self.size {
            self.data.push(0);
        }
        if b > 0 {
            let last = self.data.len() - 1;
            self.data[last] |= 1 << (self.size % 32);
        }
        self.size += 1
    }
}

pub fn rank(b: u64) -> usize {
    let mut b = (b & 0x55555555) + (b >> 1 & 0x55555555);
    b = (b & 0x33333333) + (b >> 2 & 0x33333333);
    b = (b & 0x0f0f0f0f) + (b >> 4 & 0x0f0f0f0f);
    b = (b & 0x00ff00ff) + (b >> 8 & 0x00ff00ff);
    b = (b & 0x0000ffff) + (b >> 16 & 0x0000ffff);
    b as usize
}

#[test]
fn check_size() {
    let bv1 = BitVector::new(1);
    assert_eq!(bv1.large_sum.len(), 1);
    assert_eq!(bv1.small_sum.len(), 1);
    let bv2 = BitVector::new(256);
    assert_eq!(bv2.large_sum.len(), 1);
    assert_eq!(bv2.small_sum.len(), 8);
    let bv3 = BitVector::new(257);
    assert_eq!(bv3.large_sum.len(), 2);
    assert_eq!(bv3.small_sum.len(), 9);
}

#[test]
fn test_builder() {
    let mut builder = BitVectorBuilder::new();
    builder.push(0);
    builder.push(1);
    assert_eq!(builder.size, 2);
}

#[test]
fn test_rank() {
    assert_eq!(rank(0b1), 1);
    assert_eq!(rank(0b111), 3);
    assert_eq!(rank(0b11111), 5);
}
