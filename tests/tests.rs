extern crate flint;
extern crate libc;

use libc::c_ulong;
use flint::fmpz::{Fmpz, FmpzFactor};

fn fac_to_fmpz(f: &FmpzFactor) -> Fmpz {
    let mut res = Fmpz::from_si(1);
    let mut tmp = Fmpz::new();
    for &(ref a, e) in &(f.to_vec()) {
        tmp.pow_ui_mut(&a, e as c_ulong);
        res *= &tmp;
    }
    res
}

#[test]
fn factor_test() {
    let mut a = Fmpz::from_str("1844674407370955161", 10).unwrap();
    let b = Fmpz::from_si(340394);
    a -= &b;
    // println!("res1={}", res);
    // res.set_mul_ui(&a, 10);
    // println!("res2={}", res);
    // res.pow_ui(&a, 12);
    // println!("{}", res);
    let fac = a.to_factor();
    assert!(fac_to_fmpz(&fac).get_str(10) == a.get_str(10));
}
