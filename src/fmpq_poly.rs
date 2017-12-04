use bindings::*;
use std::mem::uninitialized;
use std::ops::*;
use std::fmt;
use std::ffi::CString;
use fmpq::Fmpq;
use libc::c_long;
use fmpz_poly::FmpzPoly;

#[derive(Debug)]
pub struct FmpqPoly {
    fmpq_poly: fmpq_poly_struct,
}

impl fmt::Display for FmpqPoly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let var = CString::new("x").unwrap();
            let raw_str = fmpq_poly_get_str_pretty(self.as_raw(), var.as_ptr());
            let s = CString::from_raw(raw_str);
            write!(f, "{}", s.into_string().unwrap())
        }
    }
}

impl Deref for FmpqPoly {
    type Target = fmpq_poly_struct;

    fn deref(&self) -> &fmpq_poly_struct {
        self.as_raw()
    }
}

impl DerefMut for FmpqPoly {
    fn deref_mut(&mut self) -> &mut fmpq_poly_struct {
        self.as_raw_mut()
    }
}

impl Clone for FmpqPoly {
    fn clone(&self) -> Self {
        let mut a = FmpqPoly::new();
        a.set(self);
        a
    }

    fn clone_from(&mut self, other: &Self) {
        self.set(other);
    }
}

impl Drop for FmpqPoly {
    fn drop(&mut self) {
        unsafe {
            fmpq_poly_clear(&mut self.fmpq_poly);
        }
    }
}

define_assign_wref!(FmpqPoly, AddAssign, add_assign, fmpq_poly_add, FmpqPoly);
define_assign_wref!(FmpqPoly, SubAssign, sub_assign, fmpq_poly_sub, FmpqPoly);
define_assign_wref!(FmpqPoly, MulAssign, mul_assign, fmpq_poly_scalar_mul_fmpq, Fmpq);

impl FmpqPoly {
    pub fn new() -> Self {
        unsafe {
            let mut fmpq_poly: fmpq_poly_struct = uninitialized();
            fmpq_poly_init(&mut fmpq_poly);
            FmpqPoly { fmpq_poly: fmpq_poly }
        }
    }

    pub fn as_raw(&self) -> &fmpq_poly_struct {
        &self.fmpq_poly
    }

    pub fn as_raw_mut(&mut self) -> &mut fmpq_poly_struct {
        &mut self.fmpq_poly
    }

    impl_mut_c_wrapper!(set, fmpq_poly_set, (x: SelfRef), doc = "`self = x`");

    pub fn set_fmpz_poly(&mut self, x: &FmpzPoly) {
        unsafe {
            fmpq_poly_set_fmpz_poly(self.as_raw_mut(), x.as_raw());
        }
    }

    impl_mut_c_wrapper!(
        inv_series_newton_mut,
        fmpq_poly_inv_series_newton,
        (x: SelfRef, n: c_long),
        doc = "`self = x`"
    );

    impl_mut_c_wrapper!(
        add_mut,
        fmpq_poly_add,
        (x: SelfRef, y: SelfRef),
        doc = "`self = x + y`"
    );

    impl_mut_c_wrapper!(
        sub_mut,
        fmpq_poly_sub,
        (x: SelfRef, y: SelfRef),
        doc = "`self = x + y`"
    );

    pub fn get_coeff(&self, res: &mut Fmpq, n: c_long) {
        unsafe {
            fmpq_poly_get_coeff_fmpq(res.as_raw_mut(), self.as_raw(), n);
        }
    }

    pub fn set_coeff(&mut self, a: &Fmpq, n: c_long) {
        unsafe {
            fmpq_poly_set_coeff_fmpq(self.as_raw_mut(), n, a.as_raw());
        }
    }

    pub fn eval_fmpq(&self, res: &mut fmpq, a: &fmpq) {
        unsafe {
            fmpq_poly_evaluate_fmpq(res, self.as_raw(), a);
        }
    }

    pub fn eval_fmpz(&self, res: &mut fmpq, a: &fmpz) {
        unsafe {
            fmpq_poly_evaluate_fmpz(res, self.as_raw(), a);
        }
    }
}
