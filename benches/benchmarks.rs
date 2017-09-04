#![feature(test)]
extern crate libc;
extern crate test;
extern crate flint;
use libc::c_ulong;

use test::Bencher;

mod fmpz {
    use super::*;
    use flint::fmpz::{Fmpz, FmpzFactor};

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
