extern crate libc;
extern crate gmp;

use self::gmp::mpz::mp_limb_t;
use std::ops::{Shl, Shr, ShlAssign, ShrAssign};
use std::mem::uninitialized;
use bindings::*;
use self::libc::{c_ulong, c_long};
use fmpz::Fmpz;
use std::fmt;

#[derive(Debug, Clone)]
pub struct FmpzMat {
    fmpz_mat: fmpz_mat_struct,
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
                .map(|j| self.entry_cloned(i, j).get_str(10))
                .collect();
            vv.push(format!("[{}]", v.join(", ")));
        }
        write!(f, "{}", vv.join("\n"))
    }
}

impl PartialEq for FmpzMat {
    fn eq(&self, other: &Self) -> bool {
        unsafe { int_to_bool!(fmpz_mat_equal(self.as_ptr(), other.as_ptr())) }
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
            fmpz_mat_scalar_mul_2exp(res.as_mut_ptr(), self.as_ptr(), other);
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
            fmpz_mat_scalar_tdiv_q_2exp(res.as_mut_ptr(), self.as_ptr(), other);
        }
        res
    }
}

impl ShlAssign<mp_limb_t> for FmpzMat {
    fn shl_assign(&mut self, other: mp_limb_t) {
        unsafe {
            fmpz_mat_scalar_mul_2exp(self.as_mut_ptr(), self.as_ptr(), other);
        }
    }
}

impl ShrAssign<mp_limb_t> for FmpzMat {
    fn shr_assign(&mut self, other: mp_limb_t) {
        unsafe {
            fmpz_mat_scalar_tdiv_q_2exp(self.as_mut_ptr(), self.as_ptr(), other);
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

    fn as_ptr(&self) -> *const fmpz_mat_struct {
        &self.fmpz_mat
    }

    fn as_mut_ptr(&mut self) -> *mut fmpz_mat_struct {
        &mut self.fmpz_mat
    }

    pub fn set_zero(&mut self) {
        unsafe {
            fmpz_mat_zero(self.as_mut_ptr());
        }
    }

    pub fn set_one(&mut self) {
        debug_assert_eq!(self.ncols(), self.nrows());
        unsafe {
            fmpz_mat_one(self.as_mut_ptr());
        }
    }

    pub fn set_entry_ui(&mut self, r: isize, c: isize, x: c_ulong) {
        unsafe {
            fmpz_set_ui(self.entry_raw_mut(r, c), x);
        }
    }

    pub fn set_entry(&mut self, r: isize, c: isize, x: &Fmpz) {
        unsafe {
            fmpz_set(self.entry_raw_mut(r, c), x.as_ptr());
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
        let p = self.entry_raw(r, c);
        unsafe {
            let res = Fmpz { fmpz: [*p] };
            res.clone()
        }
    }

    pub fn is_zero(&self) -> bool {
        unsafe { int_to_bool!(fmpz_mat_is_zero(self.as_ptr())) }
    }

    pub fn is_one(&self) -> bool {
        unsafe { int_to_bool!(fmpz_mat_is_one(self.as_ptr())) }
    }

    pub fn ncols(&self) -> isize {
        self.fmpz_mat.c as isize
    }

    pub fn nrows(&self) -> isize {
        self.fmpz_mat.r as isize
    }

    pub fn negate(&mut self) {
        unsafe {
            fmpz_mat_neg(self.as_mut_ptr(), self.as_ptr());
        }
    }
    impl_c_wrapper!(add_mut, fmpz_mat_add, Self, Self, "self = x + y");
    impl_c_wrapper!(sub_mut, fmpz_mat_sub, Self, Self, "self = x - y");
    impl_c_wrapper!(neg_mut, fmpz_mat_neg, Self, "self = -x");
    impl_c_wrapper!(
        scalar_mul_si_mut,
        fmpz_mat_scalar_mul_si,
        Self,
        Si,
        "self = x * y"
    );
    impl_c_wrapper!(
        scalar_mul_ui_mut,
        fmpz_mat_scalar_mul_ui,
        Self,
        Ui,
        "self = x * y"
    );
    impl_c_wrapper!(
        scalar_mul_fmpz_mut,
        fmpz_mat_scalar_mul_fmpz,
        Self,
        Fmpz,
        "self = x * y"
    );

    impl_c_wrapper!(
        scalar_addmul_fmpz_mut,
        fmpz_mat_scalar_addmul_fmpz,
        Self,
        Fmpz,
        "self += x * y"
    );

    impl_c_wrapper!(
        scalar_addmul_si_mut,
        fmpz_mat_scalar_addmul_si,
        Self,
        Si,
        "self += x * y"
    );

    impl_c_wrapper!(
        scalar_addmul_ui_mut,
        fmpz_mat_scalar_addmul_ui,
        Self,
        Ui,
        "self += x * y"
    );


    impl_c_wrapper!(
        scalar_submul_fmpz_mut,
        fmpz_mat_scalar_submul_fmpz,
        Self,
        Fmpz,
        "self -= x * y"
    );

    impl_c_wrapper!(
        scalar_submul_si_mut,
        fmpz_mat_scalar_submul_si,
        Self,
        Si,
        "self -= x * y"
    );

    impl_c_wrapper!(
        scalar_submul_ui_mut,
        fmpz_mat_scalar_submul_ui,
        Self,
        Ui,
        "self -= x * y"
    );


    impl_c_wrapper!(
        scalar_divexact_fmpz_mut,
        fmpz_mat_scalar_divexact_fmpz,
        Self,
        Fmpz,
        "self = x/y"
    );

    impl_c_wrapper!(
        scalar_divexact_si_mut,
        fmpz_mat_scalar_divexact_si,
        Self,
        Si,
        "self = x/y"
    );

    impl_c_wrapper!(
        scalar_divexact_ui_mut,
        fmpz_mat_scalar_divexact_ui,
        Self,
        Ui,
        "self = x/y"
    );

    impl_c_wrapper!(mul_mut, fmpz_mat_mul, Self, Self, "self = x * y");

    impl_c_wrapper!(
        mul_classical_mut,
        fmpz_mat_mul_classical,
        Self,
        Self,
        "self = x * y"
    );

    impl_c_wrapper!(sqr_mut, fmpz_mat_sqr, Self, "self = x * x");

    pub fn pow_mut(&mut self, m: &Self, exp: mp_limb_t) {
        unsafe {
            fmpz_mat_pow(self.as_mut_ptr(), m.as_ptr(), exp);
        }
    }

    pub fn content_mut(&self, res: &mut Fmpz) {
        unsafe {
            fmpz_mat_content(res.as_mut_ptr(), self.as_ptr());
        }
    }

    /// `res = m.trace()`
    pub fn trace_mut(&self, res: &mut Fmpz) {
        unsafe {
            fmpz_mat_trace(res.as_mut_ptr(), self.as_ptr());
        }
    }

    /// `res = m.det()`
    pub fn det_mut(&self, res: &mut Fmpz) {
        unsafe {
            fmpz_mat_det(res.as_mut_ptr(), self.as_ptr());
        }
    }

    pub fn rank(&self) -> u64 {
        unsafe { fmpz_mat_rank(self.as_ptr()) as u64 }
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
            self.nrows() as mp_limb_signed_t,
            self.nrows() as mp_limb_signed_t,
        );
        let r = unsafe { fmpz_mat_nullspace(b.as_mut_ptr(), self.as_ptr()) };
        b.column_vectors().into_iter().take(r as usize).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut m = FmpzMat::new(2, 2);
        m.set_entry_ui(0, 0, 1);
        m.set_entry_ui(0, 1, 2);
        m.set_entry_ui(1, 0, 2);
        m.set_entry_ui(1, 1, 4);
        println!("{}", m);
        println!("{:?}", m.nullspace_basis());
    }
}
