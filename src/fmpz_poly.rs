// TODO: refactor code
use bindings::*;
use std::mem::uninitialized;
use std::ops::*;
use std::fmt;
use std::ffi::{CStr, CString};
use libc::{c_long, c_ulong, c_void};
use fmpz::Fmpz;
use traits::*;

#[derive(Debug)]
pub struct FmpzPoly {
    fmpz_poly: fmpz_poly_struct,
}

impl fmt::Display for FmpzPoly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let var = CString::new("x").unwrap();
            let raw_str = fmpz_poly_get_str_pretty(self.as_raw(), var.as_ptr());
            let s = CStr::from_ptr(raw_str);
            flint_free(raw_str as *mut c_void);
            write!(f, "{}", s.to_str().unwrap().to_string())
        }
    }
}

impl Default for FmpzPoly {
    fn default() -> Self {
        FmpzPoly::new()
    }
}

impl Clone for FmpzPoly {
    fn clone(&self) -> Self {
        let mut res = FmpzPoly::new();
        res.set(self);
        res
    }

    fn clone_from(&mut self, other: &Self) {
        self.set(other);
    }
}

impl Drop for FmpzPoly {
    fn drop(&mut self) {
        unsafe {
            fmpz_poly_clear(&mut self.fmpz_poly);
        }
    }
}

impl Deref for FmpzPoly {
    type Target = fmpz_poly_struct;

    fn deref(&self) -> &fmpz_poly_struct {
        self.as_raw()
    }
}

impl DerefMut for FmpzPoly {
    fn deref_mut(&mut self) -> &mut fmpz_poly_struct {
        self.as_raw_mut()
    }
}

impl PartialEq for FmpzPoly {
    fn eq(&self, other: &Self) -> bool {
        unsafe { int_to_bool!(fmpz_poly_equal(self.as_raw(), other.as_raw())) }
    }
}

define_assign_wref!(FmpzPoly, AddAssign, add_assign, fmpz_poly_add, FmpzPoly);
define_assign_wref!(FmpzPoly, SubAssign, sub_assign, fmpz_poly_sub, FmpzPoly);
define_assign_wref!(
    FmpzPoly,
    MulAssign,
    mul_assign,
    fmpz_poly_scalar_mul_fmpz,
    Fmpz
);
impl_operator!(Add, FmpzPoly, add, fmpz_poly_add);
impl_operator!(Sub, FmpzPoly, sub, fmpz_poly_sub);
impl_operator_w_ref!(Mul, FmpzPoly, mul, fmpz_poly_scalar_mul_fmpz, Fmpz);
impl_operator_c!(Mul, FmpzPoly, mul, c_long, fmpz_poly_scalar_mul_si);

impl FmpzPoly {
    pub fn new() -> Self {
        unsafe {
            let mut fmpz_poly: fmpz_poly_struct = uninitialized();
            fmpz_poly_init(&mut fmpz_poly);
            Self { fmpz_poly: fmpz_poly }
        }
    }

    pub fn as_raw(&self) -> &fmpz_poly_struct {
        &self.fmpz_poly
    }

    pub fn as_raw_mut(&mut self) -> &mut fmpz_poly_struct {
        &mut self.fmpz_poly
    }

    impl_mut_c_wrapper!(
        set,
        fmpz_poly_set,
        (x: SelfRef),
    );

    impl_mut_c_wrapper!(
        add_mut,
        fmpz_poly_add,
        (x: SelfRef, y: SelfRef),
        doc = "`self = x + y`"
    );

    impl_mut_c_wrapper!(
        sub_mut,
        fmpz_poly_sub,
        (x: SelfRef, y: SelfRef),
        doc = "`self = x + y`"
    );

    impl_mut_c_wrapper!(
        mullow_mut,
        fmpz_poly_mullow,
        (x: SelfRef, y: SelfRef, n: c_long),
        doc = "`self = x * y trancated length n`"
    );



    impl_mut_c_wrapper!(
        inv_series_mut,
        fmpz_poly_inv_series,
        (x: SelfRef, n: c_long),
        doc = "`self = x^{-1} trancated length n`"
    );

    impl_mut_c_wrapper!(
        set_coeff_si,
        fmpz_poly_set_coeff_si,
        (n: c_long, x: c_long),
        doc = "`self[n] = x`"
    );

    impl_mut_c_wrapper!(
        set_coeff,
        fmpz_poly_set_coeff_fmpz,
        (n: c_long, x: FmpzRef),
        doc = "`self[n] = x`"
    );

    impl_mut_c_wrapper!(
        pow_trunc_mut,
        fmpz_poly_pow_trunc,
        (poly: SelfRef, e: c_ulong, n: c_long),
        doc = "`self = poly^e mod x^n`"
    );

    pub fn pow_trunc(&self, e: c_ulong, n: c_long) -> FmpzPoly {
        let mut res = FmpzPoly::new();
        res.pow_trunc_mut(&self, e, n);
        res
    }

    impl_mut_c_wrapper!(
        addmul_scalar_mut,
        fmpz_poly_scalar_addmul_fmpz,
        (x: SelfRef, a: FmpzRef),
        doc = "`self += ax`"
    );

    impl_mut_c_wrapper!(
        submul_scalar_mut,
        fmpz_poly_scalar_submul_fmpz,
        (x: SelfRef, a: FmpzRef),
        doc = "`self -= ax`"
    );

    pub fn get_coeff(&self, res: &mut Fmpz, n: c_long) {
        unsafe {
            fmpz_poly_get_coeff_fmpz(res.as_raw_mut(), self.as_raw(), n);
        }
    }

    pub fn mullow_assign(&mut self, other: &Self, n: c_long) {
        unsafe {
            fmpz_poly_mullow(self.as_raw_mut(), self.as_raw(), other.as_raw(), n);
        }
    }
}

impl SetCoefficient<c_long> for FmpzPoly {
    fn set_coefficient(&mut self, n: c_long, x: c_long) {
        self.set_coeff_si(n, x);
    }
}

impl<'a> SetCoefficient<&'a Fmpz> for FmpzPoly {
    fn set_coefficient(&mut self, n: c_long, x: &Fmpz) {
        self.set_coeff(n, x);
    }
}
