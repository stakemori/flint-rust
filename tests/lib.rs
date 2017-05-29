extern crate flint;
extern crate num;

use num::FromPrimitive;
use num::bigint::BigInt;

use flint::bindings::{mp_limb_t};
use flint::fmpz::{Fmpz, MulSet};

#[test]
fn add_test() {
    let mut res = Fmpz::new();
    let mut a = Fmpz::new();
    let mut b = Fmpz::new();
    for i in 0..1000 {
        let x: BigInt = FromPrimitive::from_u64(i).unwrap();
        a.set_ui(i);
        for j in 0..1000 {
            let y: BigInt = FromPrimitive::from_u64(j).unwrap();
            b.set_ui(j);
            let z = &x + &y;
            res.add(&a, &b);
            assert!(z.to_str_radix(10) == res.get_str(10));
        }
    }
}

#[test]
fn it_works() {
    let mut res = Fmpz::new();
    let a = Fmpz::from_str("239023902390239032920930920", 10).unwrap();
    let b = Fmpz::from_si(344349839938948);
    res.mul_set(&a, &b);
    println!("res1={}", res);
    res.mul_set(&a, 10 as mp_limb_t);
    println!("res2={}", res);
    res.pow_ui(&a, 12);
    println!("{}", res);
    println!("{:?}", res.factor());
}

