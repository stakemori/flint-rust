#![feature(test)]

extern crate test;
extern crate flint;

use flint::fmpz::Fmpz;
use test::Bencher;
use flint::bindings::bench_square_sum_native;

fn square_sum(n: u64) {
    let mut res = Fmpz::from_ui(0);
    let mut a = Fmpz::new();
    let mut tmp = Fmpz::new();
    for i in 1..n {
        a.set_ui(i);
        tmp.pow_ui_mut(&a, 2);
        res += &tmp;
    }
}

#[bench]
fn square_sum_bench(b: &mut Bencher) {
    b.iter(|| square_sum(1000000))
}

#[bench]
fn square_sum_native_bench(b: &mut Bencher) {
    b.iter(|| unsafe {
        bench_square_sum_native(1000000);
    })
}
