#![feature(test)]
extern crate test;
extern crate flint;

use flint::fmpz::Fmpz;
use test::Bencher;


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
