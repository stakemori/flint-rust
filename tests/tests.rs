extern crate flint;
extern crate libc;

use libc::c_ulong;
use flint::fmpz::{Fmpz, FmpzFactor};

mod fmpz {
    use super::*;

    #[test]
    fn test_hilbert_symbol_odd() {
        let p: Fmpz = From::from(5);
        let a: Fmpz = From::from(2);
        assert_eq!(Fmpz::hilbert_symbol_odd(&a, &a, &p), 1);
        assert_eq!(Fmpz::hilbert_symbol_odd(&a, &p, &p), -1);
        assert_eq!(Fmpz::hilbert_symbol_odd(&p, &a, &p), -1);
        assert_eq!(Fmpz::hilbert_symbol_odd(&p, &p, &p), 1);
        assert_eq!(Fmpz::hilbert_symbol_odd(&p, &(&p * &a), &p), -1);

        let p: Fmpz = From::from(7);
        let a: Fmpz = From::from(3);
        assert_eq!(Fmpz::hilbert_symbol_odd(&a, &a, &p), 1);
        assert_eq!(Fmpz::hilbert_symbol_odd(&a, &p, &p), -1);
        assert_eq!(Fmpz::hilbert_symbol_odd(&p, &a, &p), -1);
        assert_eq!(Fmpz::hilbert_symbol_odd(&p, &p, &p), -1);
        assert_eq!(Fmpz::hilbert_symbol_odd(&p, &(&p * &a), &p), 1);
    }

    #[test]
    fn test_div_r_2exp() {
        let a: Fmpz = From::from(11);
        let mut res: Fmpz = Default::default();
        res.fdiv_r_2exp_mut(&a, 2);
        assert_eq!(res, 3 as c_ulong);
        let a: Fmpz = From::from(13);
        let mut res: Fmpz = Default::default();
        res.fdiv_r_2exp_mut(&a, 2);
        assert_eq!(res, 1 as c_ulong);
}

    #[test]
    fn test_remove() {
        let mut a = Fmpz::new();
        let c = 131;
        let b: Fmpz = From::from(131 << 15);
        let p: Fmpz = From::from(2);
        assert_eq!(a.remove(&b, &p), 15);
        assert_eq!(a, c as c_ulong);

        let p: Fmpz = From::from(3);
        let b: Fmpz = From::from(236196);
        assert_eq!(a.remove(&b, &p), 10);
        assert_eq!(a, 4 as c_ulong);
    }
}

mod factor {
    use super::*;
    fn fac_to_fmpz(f: &FmpzFactor) -> Fmpz {
        let mut res = From::from(1);
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
        let b = From::from(340394);
        a -= &b;
        // println!("res1={}", res);
        // res.set_mul_ui(&a, 10);
        // println!("res2={}", res);
        // res.pow_ui(&a, 12);
        // println!("{}", res);
        let fac = a.to_factor();
        assert!(fac_to_fmpz(&fac).get_str(10) == a.get_str(10));
    }
}
