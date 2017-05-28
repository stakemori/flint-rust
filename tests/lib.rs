extern crate flint;
use flint::bindings::{mp_limb_t};
use flint::fmpz::*;

#[test]
fn it_works() {
    let mut res = Fmpz::new();
    let a = Fmpz::from_str("239023902390239032920930920", 10).unwrap();
    let b = Fmpz::from_si(344349839938948);
    res.mul_mut(&a, &b);
    println!("res1={}", res);
    res.mul_mut(&a, 10 as mp_limb_t);
    println!("res2={}", res);
    res.pow_ui(&a, 12);
    println!("{}", res);
    println!("{:?}", res.factor());
}

