use bindings::*;
use std::mem::uninitialized;
use std::ops::*;
use std::fmt;
use std::ffi::CString;

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
