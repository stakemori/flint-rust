extern crate flint;
extern crate libc;
extern crate gmp;
extern crate serde_json;

use libc::{c_ulong, c_long};
use flint::fmpz::{Fmpz, FmpzFactor};

mod fmpz {
    use super::*;
    use gmp::mpz::Mpz;
    // use flint::fmpz::FlintRandState;

    #[test]
    fn test_divisible() {
        let a: Fmpz = From::from(91);
        let b: Fmpz = From::from(7);
        let c: Fmpz = From::from(3);
        assert!(a.is_divisible(&b));
        assert!(!a.is_divisible(&c));
        assert!(a.is_divisible_si(13));
        assert!(!a.is_divisible_si(5));
    }

    #[test]
    fn test_op_with_ptr() {
        let mut a: Fmpz = From::from(3);
        let b: Fmpz = From::from(2);
        a += b.as_raw();
        assert_eq!(a, 5_u64);
    }

    #[test]
    fn test_from_mpz() {
        let a: Mpz = From::from(10);
        let mut b: Fmpz = From::from(&a);
        assert_eq!(b, 10_u64);
        b *= 2_u64;
        let a: Mpz = From::from(&b);
        assert_eq!(a, 20_u64);
    }

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
        let b: Fmpz = From::from(-243);
        assert_eq!(a.remove(&b, &p), 5);
        assert_eq!(a, -1 as c_long);
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

    // #[test]
    // fn test_rand_and_factor() {
    //     let mut m: Fmpz = From::from(1);
    //     let mut n: Fmpz = From::from(1);
    //     let mut s = FlintRandState::new();
    //     let mut b = Fmpz::new();
    //     let mut f = FmpzFactor::new();
    //     n <<= 70;
    //     m <<= 90;
    //     let mut a = Fmpz::new();
    //     for i in 0..100 {
    //         loop {
    //             a.randm_mut(&mut s, &m);
    //             if n < a {
    //                 break;
    //             }
    //         }
    //         b.set(&a);
    //         b += 1;
    //         b *= &a;
    //         println!("i: {}, a: {}, b: {}", i, a, b);
    //         f.factor_mut(&b);
    //         println!("f: {:?}", f.to_vec());
    //     }
    // }
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
        let b: Fmpz = From::from(340_394);
        a -= &b;
        // println!("res1={}", res);
        // res.set_mul_ui(&a, 10);
        // println!("res2={}", res);
        // res.pow_ui(&a, 12);
        // println!("{}", res);
        let fac = a.to_factor();
        assert_eq!(fac_to_fmpz(&fac).get_string(10), a.get_string(10));
    }
}

mod fmpq {
    use super::*;
    use flint::fmpq::Fmpq;
    use flint::fmpz::Fmpz;
    use libc::c_long;
    use flint::bindings::fmpz_set;

    #[test]
    fn test_serialize_fmpq() {
        let a: Fmpq = From::from((
            Fmpz::from_str("4304903940901920901902911", 10).unwrap(),
            Fmpz::from_str("1332343034094031188388181332", 10).unwrap(),
        ));
        let serialized = serde_json::to_string(&a).unwrap();
        let deserialized: Fmpq = serde_json::from_str(&serialized).unwrap();
        assert_eq!(a, deserialized);
    }

    #[test]
    fn test_op() {
        let a: Fmpq = From::from((1, 2));
        let b: Fmpq = From::from((1, 3));
        let c = &a + &b;
        assert_eq!(c.num_new(), 5_u64);
        assert_eq!(c.den_new(), 6_u64);

        let mut a: Fmpq = From::from(&a.den_new());

        a.set_pow_si(-1);
        assert_eq!(a.num_new(), 1_u64);
        assert_eq!(a.den_new(), 2_u64);

        a *= 3;
        assert_eq!(a.num_new(), 3_u64);
        assert_eq!(a.den_new(), 2_u64);

        a /= 5;
        assert_eq!(a.num_new(), 3_u64);
        assert_eq!(a.den_new(), 10_u64);

        a += 1;
        assert_eq!(a.num_new(), 13_u64);
        assert_eq!(a.den_new(), 10_u64);

        a -= 2;
        assert_eq!(a.num_new(), -7_i64);
        assert_eq!(a.den_new(), 10_u64);

        let mut a: Fmpq = From::from((1, 2));
        let b: Fmpz = From::from(2);
        a *= &b;
        assert_eq!(a.num_new(), 1_i64);
        assert_eq!(a.den_new(), 1_i64);
        a -= &b;
        assert_eq!(a.num_new(), -1_i64);
        assert_eq!(a.den_new(), 1_i64);

        a += &b;
        assert_eq!(a.num_new(), 1_i64);
        assert_eq!(a.den_new(), 1_i64);
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
        a.num(&mut num);
        a.den(&mut den);
        assert_eq!(num, -12 as c_long);
        assert_eq!(den, 7 as c_long);
    }

    #[test]
    fn test_op_mut() {
        let a: Fmpq = From::from((1, 2));
        let b: Fmpq = From::from((1, 3));
        let mut res = Fmpq::new();

        res.add_mut(&a, &b);
        assert_eq!(res, Fmpq::from((5, 6)));

        res.sub_mut(&a, &b);
        assert_eq!(res, Fmpq::from((1, 6)));

        res.mul_mut(&a, &b);
        assert_eq!(res, Fmpq::from((1, 6)));

        res.div_mut(&a, &b);
        assert_eq!(res, Fmpq::from((3, 2)));
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
            fmpz_set(b.as_raw_mut(), a.num_as_raw());
        }
        assert_eq!(b, n);
    }
}


mod fmpq_mat {
    use flint::fmpq::Fmpq;
    use flint::fmpq_mat::FmpqMat;

    #[test]
    fn test_right_kernel() {
        {
            let vv: Vec<Vec<Fmpq>> = vec![
                vec![From::from(1), From::from(2), From::from(3)],
                vec![From::from(2), From::from(4), From::from(6)],
                vec![From::from(0), From::from(0), From::from(1)],
            ];
            let mat: FmpqMat = From::from(vv);
            let mut res = FmpqMat::new(3, 3);
            let r = res.rref_mut(&mat);
            assert_eq!(r, 2);
            let v = mat.right_kernel_basis();
            // println!("{}", res);
            assert_eq!(v[0], vec![From::from(-2), From::from(1), From::from(0)]);
        }

        {
            let vv: Vec<Vec<Fmpq>> = vec![vec![From::from(0), From::from(0), From::from(1)]];
            let mat: FmpqMat = From::from(vv);
            let v = mat.right_kernel_basis();
            let mut mat_mul: FmpqMat = FmpqMat::new(1, 2);
            let km: FmpqMat = From::from(v);
            let mut v_t = FmpqMat::new(km.ncols() as i64, km.nrows() as i64);
            v_t.transpose_mut(&km);
            // println!("{}", km);
            mat_mul.mul_mut(&mat, &v_t);
            assert!(mat_mul.is_zero());
        }

        {
            let vv: Vec<Vec<Fmpq>> = vec![
                vec![
                    From::from(-12),
                    From::from(-5),
                    From::from(-1),
                    From::from(-1),
                    From::from(-3),
                ],
                vec![
                    From::from(-1),
                    From::from(1),
                    From::from(1),
                    From::from(0),
                    From::from(-1),
                ],
                vec![
                    From::from(3),
                    From::from(-1),
                    From::from(1),
                    From::from(-1),
                    From::from(1),
                ],
            ];
            let mat: FmpqMat = From::from(vv);
            let v = mat.right_kernel_basis();
            let mut mat_mul: FmpqMat = FmpqMat::new(3, 2);
            let km: FmpqMat = From::from(v);
            let mut v_t = FmpqMat::new(km.ncols() as i64, km.nrows() as i64);
            v_t.transpose_mut(&km);
            println!("{}", km);
            mat_mul.mul_mut(&mat, &v_t);
            assert!(mat_mul.is_zero());
        }
    }

}

mod fmpq_poly {
    use flint::fmpq_poly::FmpqPoly;
    use flint::arith::bernoulli_poly;
    use flint::fmpq::Fmpq;

    #[test]
    fn test_bernoulli_poly() {
        let mut a = FmpqPoly::new();
        bernoulli_poly(&mut a, 10);
        assert_eq!(
            format!("{}", a),
            "x^10 - 5*x^9 + 15/2*x^8 - 7*x^6 + 5*x^4 - 3/2*x^2 + 5/66"
        );
        let x: Fmpq = From::from(10);
        let mut res = Fmpq::new();
        a.eval_fmpq(&mut res, &x);
        assert_eq!(res.num_new().to_slong().unwrap(), 379041290105);
        assert_eq!(res.den_new().to_slong().unwrap(), 66);
    }
}

mod fmpz_mat {
    use flint::fmpz_mat::FmpzMat;
    #[test]
    fn test_snf() {
        let mut a = FmpzMat::new(2, 2);
        let mut b = FmpzMat::new(2, 2);
        b.set_entry_si(0, 0, 2);
        b.set_entry_si(0, 1, 4);
        b.set_entry_si(1, 0, 6);
        b.set_entry_si(1, 1, 12);
        a.snf_mut(&b);
        println!("{}", a);
    }
}

mod fmpz_poly {
    use flint::fmpz_poly::*;
    use flint::arith::*;

    #[test]
    fn test_fmpz_poly() {
        let mut a = FmpzPoly::new();
        let b = ramanujan_tau_series_new(10);
        a.set_coeff_si(0, 1);
        a.set_coeff_si(1, -1);
        let mut c = FmpzPoly::new();
        c.inv_series_mut(&a, 10);
        println!("{}", c);
        println!("{}", b);
    }
}
