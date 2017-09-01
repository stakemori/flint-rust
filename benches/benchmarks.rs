#![feature(test)]
extern crate test;
extern crate flint;

use test::Bencher;

mod fmpz {
    use super::*;
    use flint::fmpz::Fmpz;
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
