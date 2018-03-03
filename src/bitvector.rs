pub struct BitVector {
    pub size: i32,
}
pub fn new(n: i32) -> BitVector {
    let mut size = 0;
    if n % 256 > 0 {
        size += 1;
    }
    size += n / 256;
    return BitVector { size };
}
