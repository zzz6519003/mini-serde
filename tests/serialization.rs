use mini_serde::{from_str, to_string};
// use crate::li

#[test]
fn test_num() {
    let num: u64 = 25;
    let s = to_string(&num);
    let parsed: u64 = from_str(&s).expect("deseriallize");
    assert_eq!(num, parsed);
}
