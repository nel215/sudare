pub struct BitVector {
    pub data: Vec<u64>,
    pub large_sum: Vec<u64>,
}
impl BitVector {
    pub fn new(n: usize) -> BitVector {
        let mut large_size: usize = 0;
        if n % 256 > 0 {
            large_size += 1;
        }
        large_size += (n / 256) as usize;
        let data = vec![0; n];
        let large_sum = vec![0; large_size];
        return BitVector { data, large_sum };
    }
    pub fn get(&self, i: usize) -> u64 {
        return (self.data[i / 64] >> (i % 64)) & 1;
    }
    pub fn _rank(&self, i: usize) -> usize {
        let mut res = 0;
        let j = i / 256;
        if j > 0 {
            res += self.large_sum[j - 1];
        }
        let mut j = j * 256 / 64;
        while (j + 1) * 64 < i {
            res += rank(self.data[j]);
            j += 1;
        }
        let mask = (1 << (i % 64)) - 1;
        res += rank(self.data[j] & mask);
        return res as usize;
    }
    pub fn rank(&self, i: usize, b: usize) -> usize {
        if b == 1 {
            self._rank(i)
        } else {
            i - self._rank(i)
        }
    }
}

pub struct BitVectorBuilder {
    pub data: Vec<u64>,
    pub size: usize,
}

impl BitVectorBuilder {
    pub fn new() -> BitVectorBuilder {
        let data = Vec::new();
        BitVectorBuilder { data, size: 0 }
    }
    pub fn push(&mut self, b: usize) {
        if self.data.len() * 64 <= self.size {
            self.data.push(0);
        }
        if b > 0 {
            let last = self.data.len() - 1;
            self.data[last] |= 1 << (self.size % 64);
        }
        self.size += 1
    }
    pub fn build(self) -> BitVector {
        let n = self.data.len();
        let mut large_size = (n / 4) as usize;
        if n % 4 != 0 {
            large_size += 1;
        }
        let mut large_sum = vec![0; large_size];
        for i in 0..n {
            let j = i / 4;
            if j > 0 {
                large_sum[j] = large_sum[j - 1];
            }
            large_sum[j] += rank(self.data[i]);
        }
        BitVector {
            data: self.data,
            large_sum,
        }
    }
}

pub fn rank(b: u64) -> u64 {
    let mut b = (b & 0x55555555) + (b >> 1 & 0x55555555);
    b = (b & 0x33333333) + (b >> 2 & 0x33333333);
    b = (b & 0x0f0f0f0f) + (b >> 4 & 0x0f0f0f0f);
    b = (b & 0x00ff00ff) + (b >> 8 & 0x00ff00ff);
    b = (b & 0x0000ffff) + (b >> 16 & 0x0000ffff);
    b
}

#[test]
fn check_size() {
    let bv1 = BitVector::new(1);
    assert_eq!(bv1.large_sum.len(), 1);
    let bv2 = BitVector::new(256);
    assert_eq!(bv2.large_sum.len(), 1);
    let bv3 = BitVector::new(257);
    assert_eq!(bv3.large_sum.len(), 2);
}

#[test]
fn test_builder() {
    let mut builder = BitVectorBuilder::new();
    builder.push(1);
    for _ in 0..255 {
        builder.push(0);
    }
    builder.push(1);
    assert_eq!(builder.size, 257);
    let bv = builder.build();
    assert_eq!(bv.large_sum.len(), 2);
    assert_eq!(bv.large_sum[0], 1);
    assert_eq!(bv.large_sum[1], 2);
    assert_eq!(bv.rank(0, 1), 0);
    assert_eq!(bv.rank(1, 1), 1);
    assert_eq!(bv.rank(256, 1), 1);
    assert_eq!(bv.rank(257, 1), 2);
}

#[test]
fn test_rank() {
    assert_eq!(rank(0b1), 1);
    assert_eq!(rank(0b111), 3);
    assert_eq!(rank(0b11111), 5);
}
