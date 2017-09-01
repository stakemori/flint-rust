#![feature(test)]
extern crate libc;
extern crate test;
extern crate flint;
use libc::c_ulong;

use test::Bencher;

mod fmpz {
    use super::*;
    use flint::fmpz::Fmpz;

    #[bench]
    fn mod4_bench1(b: &mut Bencher) {
        b.iter(|| {
            let mut s = 0;
            let mut tmp: Fmpz = Default::default();
            for i in 1..1000_000 {
                let a: Fmpz = From::from(2 * i + 1);
                if {
                    tmp.sub_ui_mut(&a, 1);
                    tmp >>= 1;
                    tmp.is_even()
                }
                {
                    s += 1;
                }
            }
            s})
    }

    #[bench]
    fn mod4_bench2(b: &mut Bencher) {
        b.iter(|| {
            let mut s = 0;
            let mut tmp: Fmpz = Default::default();
            for i in 1..1000_000 {
                let a: Fmpz = From::from(2 * i + 1);
                if {
                    tmp.sub_ui_mut(&a, 1);
                    tmp.fdiv_r_2exp_mut(&a, 2);
                    tmp == 1 as c_ulong
                }
                {
                    s += 1;
                }
            }
            s})
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
}
