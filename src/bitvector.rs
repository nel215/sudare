pub struct BitVector {
    pub data: Vec<u32>,
    pub large_sum: Vec<u32>,
    pub small_sum: Vec<u32>,
}
impl BitVector {
    pub fn set(&mut self, i: usize, b: usize) {
        self.data[i / 32] |= (b as u32) << (i % 32);
    }
    pub fn get(&self, i: usize) -> u32 {
        return (self.data[i / 32] >> (i % 32)) & 1;
    }
}
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

#[test]
fn check_size() {
    let bv1 = new(1);
    assert_eq!(bv1.large_sum.len(), 1);
    assert_eq!(bv1.small_sum.len(), 1);
    let bv2 = new(256);
    assert_eq!(bv2.large_sum.len(), 1);
    assert_eq!(bv2.small_sum.len(), 8);
    let bv3 = new(257);
    assert_eq!(bv3.large_sum.len(), 2);
    assert_eq!(bv3.small_sum.len(), 9);
}
