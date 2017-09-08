#![feature(test)]
extern crate libc;
extern crate test;
extern crate flint;
use libc::{c_ulong, c_long};

use test::Bencher;

mod fmpz {
    use super::*;
    use flint::fmpz::{Fmpz, FmpzFactor};

    #[bench]
    fn square_sum(b: &mut Bencher) {
        let mut s = Fmpz::new();
        let mut a = Fmpz::new();
        let mut tmp = Fmpz::new();
        b.iter(|| for i in 0..1000000 {
            a.set_si(i);
            tmp.pow_ui_mut(&a, 2);
            s += &tmp;
        })
    }

    #[bench]
    fn set_bench(bn: &mut Bencher) {
        let a: Fmpz = From::from(9223372036854775807);
        let mut c = Fmpz::new();
        c.pow_ui_mut(&a, 100);
        let mut b = Fmpz::new();
        bn.iter(|| { b.set(&a); })
    }

    #[bench]
    fn get_ui_bench(b: &mut Bencher) {
        let a: Fmpz = From::from(9223372036854775807);
        b.iter(|| { let _ = a.to_slong().unwrap(); })
    }

    #[bench]
    fn get_ui_unchecked_bench(b: &mut Bencher) {
        let a: Fmpz = From::from(9223372036854775807);
        b.iter(|| { let _ = a.get_ui_unchecked(); })
    }

    #[bench]
    fn mod4_bench1(b: &mut Bencher) {
        b.iter(|| {
            let mut s = 0;
            let mut tmp: Fmpz = Default::default();
            for i in 1..1_000_000 {
                let a: Fmpz = From::from(2 * i + 1);
                let cnd = {
                    tmp.sub_ui_mut(&a, 1);
                    tmp >>= 1;
                    tmp.is_even()
                };
                if cnd {
                    s += 1;
                }
            }
            s
        })
    }

    #[bench]
    fn mod4_bench2(b: &mut Bencher) {
        b.iter(|| {
            let mut s = 0;
            let mut tmp: Fmpz = Default::default();
            for i in 1..1_000_000 {
                let a: Fmpz = From::from(2 * i + 1);
                let cnd = {
                    tmp.sub_ui_mut(&a, 1);
                    tmp.fdiv_r_2exp_mut(&a, 2);
                    tmp == 1 as c_ulong
                };
                if cnd {
                    s += 1;
                }
            }
            s
        })
    }

    #[bench]
    fn jacobi_bench(b: &mut Bencher) {
        b.iter(|| {
            let p: Fmpz = From::from(1031);
            let mut a: Fmpz = From::from(1);
            while a < p {
                a.jacobi(&p);
                a += 1;
            }
        })
    }

    // Sage benchmark:
    // %time l = [factor(i) for i in range(1, 100000)]
    // CPU times: user 1.13 s, sys: 15.8 ms, total: 1.14 s
    // Wall time: 1.14 s

    // test fmpz::prime_factor_bench ... bench:  20,701,657 ns/iter (+/- 416,316)
    #[bench]
    fn prime_factor_bench(b: &mut Bencher) {
        b.iter(|| {
            let mut fac = FmpzFactor::new();
            for i in 1..100_000 {
                let a: Fmpz = From::from(i);
                fac.factor_mut(&a);
            }
        })
    }

    // test fmpz::prime_factor_si_bench ... bench:  19,990,004 ns/iter (+/- 317,584)
    #[bench]
    fn prime_factor_si_bench(b: &mut Bencher) {
        b.iter(|| {
            let mut fac = FmpzFactor::new();
            for i in 1..100_000 {
                fac.factor_si_mut(i);
            }
        })
    }

}

mod fmpz_mat {
    use super::*;
    use flint::fmpz::Fmpz;
    use flint::fmpz_mat::FmpzMat;

    #[bench]
    fn test_row_vec_add(b: &mut Bencher) {
        let v: Vec<_> = (0..100).map(|i| Fmpz::from_ui(i)).collect();
        let mut w = v.clone();
        b.iter(|| for i in 0..100 {
            w[i] += &v[i];
        })
    }

    #[bench]
    fn test_row_vec_add_using_mat(b: &mut Bencher) {
        let mut m = FmpzMat::new(100, 100);
        let mut res = FmpzMat::new(100, 100);
        for i in 0..100 {
            m.set_entry_si(i, 0, i as c_long);
            m.set_entry_si(i, 1, i as c_long);
        }
        let mut n = FmpzMat::new(100, 100);
        n.set_entry_si(0, 1, 1);
        b.iter(|| res.mul_mut(&m, &n))
    }
}
