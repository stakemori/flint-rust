extern crate libc;
extern crate gmp;

use self::gmp::mpz::mp_limb_t;
use std::ops::{Shl, Shr, ShlAssign, ShrAssign};
use std::mem::uninitialized;
use bindings::*;
use self::libc::{c_ulong, c_long};
use fmpz::Fmpz;
use std::fmt;

#[derive(Debug)]
pub struct FmpzMat {
    fmpz_mat: fmpz_mat_struct,
}

impl Clone for FmpzMat {
    fn clone(&self) -> Self {
        let mut a = FmpzMat::new(self.nrows() as i64, self.ncols() as i64);
        a.set(&self);
        a
    }

    fn clone_from(&mut self, other: &Self) {
        debug_assert_eq!((self.nrows(), self.ncols()), (other.nrows(), other.ncols()));
        self.set(other);
    }
}

impl Drop for FmpzMat {
    fn drop(&mut self) {
        unsafe {
            fmpz_mat_clear(&mut self.fmpz_mat);
        }
    }
}

impl fmt::Display for FmpzMat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut vv = Vec::new();
        for i in 0..self.nrows() {
            let v: Vec<String> = (0..self.ncols())
                .map(|j| self.entry_cloned(i, j).get_string(10))
                .collect();
            vv.push(format!("[{}]", v.join(", ")));
        }
        write!(f, "{}", vv.join("\n"))
    }
}

impl PartialEq for FmpzMat {
    fn eq(&self, other: &Self) -> bool {
        unsafe { int_to_bool!(fmpz_mat_equal(self.as_raw(), other.as_raw())) }
    }
}

impl<'a> Shl<mp_limb_t> for &'a FmpzMat {
    type Output = FmpzMat;
    fn shl(self, other: mp_limb_t) -> FmpzMat {
        let mut res = FmpzMat::new(
            self.nrows() as mp_limb_signed_t,
            self.ncols() as mp_limb_signed_t,
        );
        unsafe {
            fmpz_mat_scalar_mul_2exp(res.as_raw_mut(), self.as_raw(), other);
        }
        res
    }
}

impl<'a> Shr<mp_limb_t> for &'a FmpzMat {
    type Output = FmpzMat;
    fn shr(self, other: mp_limb_t) -> FmpzMat {
        let mut res = FmpzMat::new(
            self.nrows() as mp_limb_signed_t,
            self.ncols() as mp_limb_signed_t,
        );
        unsafe {
            fmpz_mat_scalar_tdiv_q_2exp(res.as_raw_mut(), self.as_raw(), other);
        }
        res
    }
}

impl ShlAssign<mp_limb_t> for FmpzMat {
    fn shl_assign(&mut self, other: mp_limb_t) {
        unsafe {
            fmpz_mat_scalar_mul_2exp(self.as_raw_mut(), self.as_raw(), other);
        }
    }
}

impl ShrAssign<mp_limb_t> for FmpzMat {
    fn shr_assign(&mut self, other: mp_limb_t) {
        unsafe {
            fmpz_mat_scalar_tdiv_q_2exp(self.as_raw_mut(), self.as_raw(), other);
        }
    }
}

impl FmpzMat {
    pub fn new(rows: mp_limb_signed_t, cols: mp_limb_signed_t) -> Self {
        unsafe {
            let mut fmpz_mat: fmpz_mat_struct = uninitialized();
            fmpz_mat_init(&mut fmpz_mat, rows, cols);
            FmpzMat { fmpz_mat: fmpz_mat }
        }
    }

    impl_mut_c_wrapper!(set, fmpz_mat_set, (m: SelfRef),);

    fn as_raw(&self) -> *const fmpz_mat_struct {
        &self.fmpz_mat
    }

    fn as_raw_mut(&mut self) -> *mut fmpz_mat_struct {
        &mut self.fmpz_mat
    }

    pub fn set_zero(&mut self) {
        unsafe {
            fmpz_mat_zero(self.as_raw_mut());
        }
    }

    pub fn set_one(&mut self) {
        debug_assert_eq!(self.ncols(), self.nrows());
        unsafe {
            fmpz_mat_one(self.as_raw_mut());
        }
    }

    pub fn set_entry_ui(&mut self, r: isize, c: isize, x: c_ulong) {
        unsafe {
            fmpz_set_ui(self.entry_raw_mut(r, c), x);
        }
    }

    pub fn set_entry(&mut self, r: isize, c: isize, x: &fmpz) {
        unsafe {
            fmpz_set(self.entry_raw_mut(r, c), x);
        }
    }

    pub fn get_entry(&self, r: isize, c: isize, x: &mut Fmpz) {
        unsafe {
            fmpz_set(x.as_raw_mut(), self.entry_raw(r, c));
        }
    }

    pub fn set_entry_si(&mut self, r: isize, c: isize, x: c_long) {
        unsafe {
            fmpz_set_si(self.entry_raw_mut(r, c), x);
        }
    }

    pub fn entry_raw(&self, r: isize, c: isize) -> *const fmpz {
        unsafe { (*self.fmpz_mat.rows.offset(r)).offset(c) }
    }

    pub fn entry_raw_mut(&self, r: isize, c: isize) -> *mut fmpz {
        unsafe { (*self.fmpz_mat.rows.offset(r)).offset(c) }
    }

    pub fn entry_cloned(&self, r: isize, c: isize) -> Fmpz {
        unsafe {
            let p = self.entry_raw(r, c);
            let mut res = Fmpz::new();
            fmpz_set(res.as_raw_mut(), p);
            res
        }
    }

    pub fn is_zero(&self) -> bool {
        unsafe { int_to_bool!(fmpz_mat_is_zero(self.as_raw())) }
    }

    pub fn is_one(&self) -> bool {
        unsafe { int_to_bool!(fmpz_mat_is_one(self.as_raw())) }
    }

    pub fn ncols(&self) -> isize {
        self.fmpz_mat.c as isize
    }

    pub fn nrows(&self) -> isize {
        self.fmpz_mat.r as isize
    }

    pub fn negate(&mut self) {
        unsafe {
            fmpz_mat_neg(self.as_raw_mut(), self.as_raw());
        }
    }

    impl_mut_c_wrapper!(
        add_mut,
        fmpz_mat_add,
        (x: SelfRef, y: SelfRef),
        doc = "self = x + y"
    );
    impl_mut_c_wrapper!(
        sub_mut,
        fmpz_mat_sub,
        (x: SelfRef, y: SelfRef),
        doc = "self = x - y"
    );
    impl_mut_c_wrapper!(neg_mut, fmpz_mat_neg, (x: SelfRef), doc = "self = -x");
    impl_mut_c_wrapper!(
        scalar_mul_si_mut,
        fmpz_mat_scalar_mul_si,
        (x: SelfRef, y: Si),
        doc = "self = x * y"
    );
    impl_mut_c_wrapper!(
        scalar_mul_ui_mut,
        fmpz_mat_scalar_mul_ui,
        (x: SelfRef, y: Ui),
        doc = "self = x * y"
    );

    impl_mut_c_wrapper!(
        scalar_mul_fmpz_mut,
        fmpz_mat_scalar_mul_fmpz,
        (x: SelfRef, y: FmpzRef),
        doc = "self = x * y"
    );
    impl_mut_c_wrapper!(
        scalar_addmul_fmpz_mut,
        fmpz_mat_scalar_addmul_fmpz,
        (x: SelfRef, y: FmpzRef),
        doc = "self += x * y"
    );

    impl_mut_c_wrapper!(
        scalar_addmul_si_mut,
        fmpz_mat_scalar_addmul_si,
        (x: SelfRef, y: Si),
        doc = "self += x * y"
    );

    impl_mut_c_wrapper!(
        scalar_addmul_ui_mut,
        fmpz_mat_scalar_addmul_ui,
        (x: SelfRef, y: Ui),
        doc = "self += x * y"
    );

    impl_mut_c_wrapper!(
        scalar_submul_fmpz_mut,
        fmpz_mat_scalar_submul_fmpz,
        (x: SelfRef, y: FmpzRef),
        doc = "self -= x * y"
    );

    impl_mut_c_wrapper!(
        scalar_submul_si_mut,
        fmpz_mat_scalar_submul_si,
        (x: SelfRef, y: Si),
        doc = "self -= x * y"
    );

    impl_mut_c_wrapper!(
        scalar_submul_ui_mut,
        fmpz_mat_scalar_submul_ui,
        (x: SelfRef, y: Ui),
        doc = "self -= x * y"
    );

    impl_mut_c_wrapper!(
        scalar_divexact_fmpz_mut,
        fmpz_mat_scalar_divexact_fmpz,
        (x: SelfRef, y: FmpzRef),
        doc = "self = x/y"
    );

    impl_mut_c_wrapper!(
        scalar_divexact_si_mut,
        fmpz_mat_scalar_divexact_si,
        (x: SelfRef, y: Si),
        doc = "self = x/y"
    );

    impl_mut_c_wrapper!(
        scalar_divexact_ui_mut,
        fmpz_mat_scalar_divexact_ui,
        (x: SelfRef, y: Ui),
        doc = "self = x/y"
    );

    impl_mut_c_wrapper!(
        mul_mut,
        fmpz_mat_mul,
        (x: SelfRef, y: SelfRef),
        doc = "self = x * y"
    );

    impl_mut_c_wrapper!(
        mul_classical_mut,
        fmpz_mat_mul_classical,
        (x: SelfRef, y: SelfRef),
        doc = "self = x * y"
    );
    impl_mut_c_wrapper!(sqr_mut, fmpz_mat_sqr, (x: SelfRef), doc = "self = x * x");

    impl_mut_c_wrapper!(pow_mut, fmpz_mat_pow, (m: SelfRef, exp: mp_limb_t),);

    impl_mut_c_wrapper!(
        snf_mut,
        fmpz_mat_snf,
        (a: SelfRef),
        doc = "Set `self` to the Smith normal form of `a`"
    );

    pub fn content_mut(&self, res: &mut Fmpz) {
        unsafe {
            fmpz_mat_content(res.as_raw_mut(), self.as_raw());
        }
    }

    /// `res = m.trace()`
    pub fn trace_mut(&self, res: &mut Fmpz) {
        unsafe {
            fmpz_mat_trace(res.as_raw_mut(), self.as_raw());
        }
    }

    /// `res = m.det()`
    pub fn det_mut(&self, res: &mut Fmpz) {
        unsafe {
            fmpz_mat_det(res.as_raw_mut(), self.as_raw());
        }
    }

    pub fn rank(&self) -> u64 {
        unsafe { fmpz_mat_rank(self.as_raw()) as u64 }
    }

    fn column_vector(&self, i: isize) -> Vec<Fmpz> {
        let n = self.nrows();
        (0..n).map(|j| self.entry_cloned(j, i)).collect()
    }

    fn column_vectors(&self) -> Vec<Vec<Fmpz>> {
        let m = self.ncols();
        (0..m).map(|j| self.column_vector(j)).collect()
    }

    pub fn nullspace_basis(&self) -> Vec<Vec<Fmpz>> {
        let mut b = FmpzMat::new(
            self.ncols() as mp_limb_signed_t,
            self.ncols() as mp_limb_signed_t,
        );
        let r = unsafe { fmpz_mat_nullspace(b.as_raw_mut(), self.as_raw()) };
        b.column_vectors().into_iter().take(r as usize).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut m = FmpzMat::new(2, 3);
        m.set_entry_ui(0, 0, 1);
        m.set_entry_ui(0, 1, 2);
        m.set_entry_ui(1, 0, 2);
        m.set_entry_ui(1, 1, 4);
        println!("{}", m);
        println!("{:?}", m.nullspace_basis());
    }
}
