pub struct BitVector {
    pub large_sum: Vec<u32>,
}
pub fn new(n: i32) -> BitVector {
    let mut size: usize = 0;
    if n % 256 > 0 {
        size += 1;
    }
    size += (n / 256) as usize;
    let large_sum = vec![0; size];
    return BitVector { large_sum };
}

#[test]
fn check_size() {
    let bv1 = new(1);
    assert_eq!(bv1.large_sum.len(), 1);
    let bv2 = new(256);
    assert_eq!(bv2.large_sum.len(), 1);
    let bv3 = new(257);
    assert_eq!(bv3.large_sum.len(), 2);
}
