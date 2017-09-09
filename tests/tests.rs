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
    fn test_hilbert_symbol_2() {
        let two: Fmpz = From::from(2);
        let mut res = Vec::new();
        let mut res1 = Vec::new();
        let mut res2 = Vec::new();
        for &i in &[1, 3, 5, 7] {
            for &j in &[1, 3, 5, 7] {
                let a = From::from(i);
                let b = From::from(j);
                let c = &two * &From::from(i);
                let d = &two * &From::from(j);
                assert_eq!(
                    Fmpz::hilbert_symbol_2(&a, &b),
                    Fmpz::hilbert_symbol_2(&b, &a)
                );
                assert_eq!(
                    Fmpz::hilbert_symbol_2(&a, &d),
                    Fmpz::hilbert_symbol_2(&d, &a)
                );
                if Fmpz::hilbert_symbol_2(&a, &d) == -1 {
                    res.push((i, j));
                }
                if Fmpz::hilbert_symbol_2(&c, &d) == -1 {
                    res1.push((i, j));
                }
                if Fmpz::hilbert_symbol_2(&a, &b) == -1 {
                    res2.push((i, j));
                }
            }
        }
        assert_eq!(
            res,
            vec![
                (3, 1),
                (3, 5),
                (5, 1),
                (5, 3),
                (5, 5),
                (5, 7),
                (7, 3),
                (7, 7),
            ]
        );
        assert_eq!(
            res1,
            vec![
                (1, 3),
                (1, 5),
                (3, 1),
                (3, 3),
                (5, 1),
                (5, 7),
                (7, 5),
                (7, 7),
            ]
        );
        assert_eq!(res2, vec![(3, 3), (3, 7), (7, 3), (7, 7)]);
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
        let b: Fmpz = From::from(236_196);
        assert_eq!(a.remove(&b, &p), 10);
        assert_eq!(a, 4 as c_ulong);
    }

    #[test]
    fn test_size() {
        let a = Fmpz::from_str("-9223372036854775807", 10).unwrap();
        let b = Fmpz::from_str("18446744073709551615", 10).unwrap();
        assert_eq!(a.bits(), 63);
        assert_eq!(b.bits(), 64);
    }

    #[test]
    fn test_to_slong() {
        let a = Fmpz::from_str("-9223372036854775807", 10).unwrap();
        let mut b = Fmpz::from_str("-9223372036854775808", 10).unwrap();
        b <<= 1;
        assert_eq!(a.to_slong().unwrap(), -9223372036854775807);
        assert_eq!(b.to_slong(), None);
    }

    #[test]
    fn test_set_remove() {
        let mut a = Fmpz::from_str("6338253001141147007483516026880", 10).unwrap();
        let two: Fmpz = From::from(2);
        assert_eq!(a.set_remove(&two), 100);
        assert_eq!(a, 5 as c_ulong);
    }
}

mod factor {
    use super::*;
    fn fac_to_fmpz(f: &FmpzFactor) -> Fmpz {
        let mut res = From::from(1);
        let mut tmp = Fmpz::new();
        for &(ref a, e) in &(f.to_vec()) {
            tmp.pow_ui_mut(a, e as c_ulong);
            res *= &tmp;
        }
        res
    }

    #[test]
    fn factor_test() {
        let mut a = Fmpz::from_str("1844674407370955161", 10).unwrap();
        let b = From::from(340_394);
        a -= &b;
        // println!("res1={}", res);
        // res.set_mul_ui(&a, 10);
        // println!("res2={}", res);
        // res.pow_ui(&a, 12);
        // println!("{}", res);
        let fac = a.to_factor();
        assert_eq!(fac_to_fmpz(&fac).get_str(10), a.get_str(10));
    }
}

mod fmpq {
    use flint::fmpq::Fmpq;
    use flint::fmpz::Fmpz;
    use libc::c_long;
    use flint::bindings::fmpz_set;

    #[test]
    fn test_op() {
        let a: Fmpq = From::from((1, 2));
        let b: Fmpq = From::from((1, 3));
        let c = &a + &b;
        assert_eq!(c.num(), 5_u64);
        assert_eq!(c.den(), 6_u64);

        let mut a: Fmpq = From::from(&a.den());

        a.set_pow_si(-1);
        assert_eq!(a.num(), 1_u64);
        assert_eq!(a.den(), 2_u64);

        a *= 3;
        assert_eq!(a.num(), 3_u64);
        assert_eq!(a.den(), 2_u64);

        a /= 5;
        assert_eq!(a.num(), 3_u64);
        assert_eq!(a.den(), 10_u64);

        a += 1;
        assert_eq!(a.num(), 13_u64);
        assert_eq!(a.den(), 10_u64);

        a -= 2;
        assert_eq!(a.num(), -7_i64);
        assert_eq!(a.den(), 10_u64);
    }

    #[test]
    fn test_sgn() {
        let a: Fmpq = From::from((-12, 5));
        assert_eq!(a.sgn(), -1);
    }

    #[test]
    fn test_set_num_den() {
        let a: Fmpq = From::from((-24, 14));
        let mut num = Fmpz::new();
        let mut den = Fmpz::new();
        a.set_num_den(&mut num, &mut den);
        assert_eq!(num, -12 as c_long);
        assert_eq!(den, 7 as c_long);
    }

    #[test]
    fn test_num_ptr() {
        let n = Fmpz::from_str(
            "290329093209403904940394039049093409403903430909093820983209",
            10,
        ).unwrap();
        let d = Fmpz::from_str("2", 10).unwrap();
        let a: Fmpq = From::from((&n, &d));
        let mut b = Fmpz::new();
        unsafe {
            fmpz_set(b.as_mut_ptr(), a.num_as_ptr());
        }
        assert_eq!(b, n);
    }
}
