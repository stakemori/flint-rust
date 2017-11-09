use bindings::*;
use fmpz_poly::FmpzPoly;


pub fn bernoulli_number(x: &mut fmpq, n: u64) {
    unsafe {
        arith_bernoulli_number(x, n);
    }
}

pub fn bernoulli_poly(p: &mut fmpq_poly_struct, n: u64) {
    unsafe {
        arith_bernoulli_polynomial(p, n);
    }
}

pub fn ramanujan_tau_series(p: &mut fmpz_poly_struct, n: i64) {
    unsafe {
        arith_ramanujan_tau_series(p, n);
    }
}

pub fn ramanujan_tau_series_new(n: i64) -> FmpzPoly {
    unsafe {
        let mut res = FmpzPoly::new();
        arith_ramanujan_tau_series(res.as_raw_mut(), n);
        res
    }
}
