use bindings::*;
use std::mem::uninitialized;
use std::ops::*;
use std::fmt;
use std::ffi::CString;
use libc::c_long;
use fmpz::Fmpz;

pub struct FmpzPoly {
    fmpz_poly: fmpz_poly_struct,
}

impl fmt::Display for FmpzPoly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let var = CString::new("x").unwrap();
            let raw_str = fmpz_poly_get_str_pretty(self.as_raw(), var.as_ptr());
            let s = CString::from_raw(raw_str);
            write!(f, "{}", s.into_string().unwrap())
        }
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
        set_coeff_fmpz,
        fmpz_poly_set_coeff_fmpz,
        (n: c_long, x: FmpzRef),
        doc = "`self[n] = x`"
    );

    pub fn get_coeff(&self, res: &mut Fmpz, n: c_long) {
        unsafe {
            fmpz_poly_get_coeff_fmpz(res.as_raw_mut(), self.as_raw(), n);
        }
    }
}
