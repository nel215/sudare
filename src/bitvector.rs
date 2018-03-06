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
