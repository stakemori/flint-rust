extern crate flint;
extern crate num;

use num::FromPrimitive;
use num::bigint::BigInt;

use flint::fmpz::{Fmpz};

#[test]
fn add_test() {
    let mut res = Fmpz::new();
    let mut res1 = Fmpz::new();
    let mut a = Fmpz::new();
    let mut b = Fmpz::new();
    for i in 0..100 {
        let x: BigInt = FromPrimitive::from_u64(i).unwrap();
        a.set_ui(i);
        for j in 0..1000 {
            let y: BigInt = FromPrimitive::from_u64(j).unwrap();
            b.set_ui(j);
            let z = &x + &y;
            res.set_add(&a, &b);
            res1.set_add_ui(&a, j);
            assert!(z.to_str_radix(10) == res.get_str(10));
            assert!(z.to_str_radix(10) == res1.get_str(10));
        }
    }
}

// #[test]
// fn it_works() {
//     let mut res = Fmpz::new();
//     let a = Fmpz::from_str("239023902390239032920930920", 10).unwrap();
//     let b = Fmpz::from_si(344349839938948);
//     res.set_mul(&a, &b);
//     println!("res1={}", res);
//     res.set_mul_ui(&a, 10);
//     println!("res2={}", res);
//     res.pow_ui(&a, 12);
//     println!("{}", res);
//     println!("{:?}", res.factor());
// }

