use bindings::*;

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
