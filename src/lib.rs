pub mod bitvector;

pub fn rank(x: i64, i: i32) -> i32 {
    let mut res = 0;
    for idx in 0..i {
        if (x >> idx & 1) > 0 {
            res += 1;
        }
    }
    return res;
}
#[test]
fn it_works() {
    assert_eq!(rank(0b10, 1), 0);
    assert_eq!(rank(0b10, 2), 1);
}
